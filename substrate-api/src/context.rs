//! The global context.

use std::marker::PhantomData;
use std::path::Path;
use std::sync::{Arc, RwLock};

use once_cell::sync::OnceCell;

use crate::error::Result;
use crate::layout::builder::CellBuilder as LayoutCellBuilder;
use crate::layout::cell::Cell as LayoutCell;
use crate::layout::context::LayoutContext;
use crate::layout::error::{GdsExportError, LayoutError};
use crate::layout::gds::GdsExporter;
use crate::layout::HasLayoutImpl;
use crate::pdk::layers::GdsLayerSpec;
use crate::pdk::layers::LayerContext;
use crate::pdk::layers::LayerId;
use crate::pdk::layers::Layers;
use crate::pdk::Pdk;
use crate::schematic::{Cell as SchematicCell, FlatLen};
use crate::schematic::{CellBuilder as SchematicCellBuilder, HardwareType, NodeContext};
use crate::schematic::{HasSchematicImpl, SchematicContext};

/// The global context.
///
/// Stores configuration such as the PDK and tool plugins to use during generation.
///
/// # Examples
///
/// ```
#[doc = include_str!("../../docs/api/code/prelude.md.hidden")]
#[doc = include_str!("../../docs/api/code/pdk/layers.md.hidden")]
#[doc = include_str!("../../docs/api/code/pdk/pdk.md.hidden")]
#[doc = include_str!("../../docs/api/code/block/inverter.md.hidden")]
#[doc = include_str!("../../docs/api/code/layout/inverter.md.hidden")]
#[doc = include_str!("../../docs/api/code/block/buffer.md.hidden")]
#[doc = include_str!("../../docs/api/code/layout/buffer.md.hidden")]
#[doc = include_str!("../../docs/api/code/layout/generate.md")]
/// ```
pub struct Context<PDK: Pdk> {
    /// PDK-specific data.
    pub pdk: PdkData<PDK>,
    inner: Arc<RwLock<ContextInner>>,
}

impl<PDK: Pdk> Clone for Context<PDK> {
    fn clone(&self) -> Self {
        Self {
            pdk: self.pdk.clone(),
            inner: self.inner.clone(),
        }
    }
}

/// PDK data stored in the global context.
pub struct PdkData<PDK: Pdk> {
    /// PDK configuration and general data.
    pub pdk: Arc<PDK>,
    /// The PDK layer set.
    pub layers: Arc<PDK::Layers>,
}

impl<PDK: Pdk> Clone for PdkData<PDK> {
    fn clone(&self) -> Self {
        Self {
            pdk: self.pdk.clone(),
            layers: self.layers.clone(),
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct ContextInner {
    layers: LayerContext,
    schematic: SchematicContext,
    layout: LayoutContext,
}

impl<PDK: Pdk> Context<PDK> {
    /// Creates a new global context.
    pub fn new(pdk: PDK) -> Self {
        // Instantiate PDK layers.
        let mut layer_ctx = LayerContext::new();
        let layers = layer_ctx.install_layers::<PDK::Layers>();

        Self {
            pdk: PdkData {
                pdk: Arc::new(pdk),
                layers,
            },
            inner: Arc::new(RwLock::new(ContextInner::new(layer_ctx))),
        }
    }

    /// Generates a layout for `block` in the background.
    ///
    /// Returns a handle to the cell being generated.
    pub fn generate_layout<T: HasLayoutImpl<PDK>>(
        &mut self,
        block: T,
    ) -> Arc<OnceCell<Result<LayoutCell<T>>>> {
        let context_clone = self.clone();
        let mut inner_mut = self.inner.write().unwrap();
        let id = inner_mut.layout.get_id();
        inner_mut.layout.gen.generate(block.clone(), move || {
            let mut cell_builder = LayoutCellBuilder::new(id, block.name(), context_clone);
            let data = block.layout(&mut cell_builder);
            data.map(|data| LayoutCell::new(block, data, Arc::new(cell_builder.into())))
        })
    }

    /// Writes a layout to a GDS files.
    pub fn write_layout<T: HasLayoutImpl<PDK>>(
        &mut self,
        block: T,
        path: impl AsRef<Path>,
    ) -> Result<()> {
        let handle = self.generate_layout(block);
        let cell = handle.wait().as_ref().map_err(|e| e.clone())?;

        let inner = self.inner.read().unwrap();
        GdsExporter::new(cell.raw.clone(), &inner.layers)
            .export()
            .map_err(LayoutError::from)?
            .save(path)
            .map_err(GdsExportError::from)
            .map_err(LayoutError::from)?;
        Ok(())
    }

    /// Generates a schematic for `block` in the background.
    ///
    /// Returns a handle to the cell being generated.
    pub fn generate_schematic<T: HasSchematicImpl<PDK>>(
        &mut self,
        block: T,
    ) -> Arc<OnceCell<Result<SchematicCell<T>>>> {
        let context_clone = self.clone();
        let mut inner_mut = self.inner.write().unwrap();
        let id = inner_mut.schematic.get_id();
        inner_mut.schematic.gen.generate(block.clone(), move || {
            let mut node_ctx = NodeContext::new();
            let io = block.io();
            let nodes = node_ctx.nodes(io.len());
            let (io, nodes) = io.instantiate(&nodes);
            assert!(nodes.is_empty());
            let mut cell_builder = SchematicCellBuilder {
                id,
                ctx: context_clone,
                node_ctx,
                instances: Vec::new(),
                primitives: Vec::new(),
                phantom: PhantomData,
            };
            let data = block.schematic(io, &mut cell_builder);
            data.map(|data| SchematicCell::new(block, data, Arc::new(cell_builder.finish())))
        })
    }

    /// Installs a new layer set in the context.
    ///
    /// Allows for accessing GDS layers or other extra layers that are not present in the PDK.
    pub fn install_layers<L: Layers>(&mut self) -> Arc<L> {
        let mut inner = self.inner.write().unwrap();
        inner.layers.install_layers::<L>()
    }

    /// Gets a layer by its GDS layer spec.
    ///
    /// Should generally not be used except for situations involving GDS import, where
    /// layers may be imported at runtime.
    pub fn get_gds_layer(&self, spec: GdsLayerSpec) -> Option<LayerId> {
        let inner = self.inner.read().unwrap();
        inner.layers.get_gds_layer(spec)
    }
}

impl ContextInner {
    #[allow(dead_code)]
    pub(crate) fn new(layers: LayerContext) -> Self {
        Self {
            layers,
            schematic: Default::default(),
            layout: Default::default(),
        }
    }
}
