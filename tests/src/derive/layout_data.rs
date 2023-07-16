use substrate::layout::{HasLayout, Instance};
use substrate::LayoutData;

#[derive(Default, LayoutData)]
pub struct LayoutInstances<T: HasLayout> {
    #[substrate(transform)]
    pub instances: Vec<Instance<T>>,
    pub field: i64,
}

#[derive(LayoutData)]
pub enum EnumInstances<T: HasLayout> {
    One {
        #[substrate(transform)]
        one: Instance<T>,
        field: i64,
    },
    Two(
        #[substrate(transform)] Instance<T>,
        #[substrate(transform)] Instance<T>,
        i64,
    ),
}

#[derive(LayoutData)]
pub struct TwoInstances<T: HasLayout>(
    #[substrate(transform)] pub Instance<T>,
    #[substrate(transform)] pub Instance<T>,
    pub i64,
);