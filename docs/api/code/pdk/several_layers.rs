#[derive(Layers)]
pub struct ExamplePdkALayers {
    #[layer(gds = "66/20")]
    pub polya: PolyA,
    #[layer_family]
    pub met1a: Met1A,
    #[layer(name = "met2", gds = "69/20")]
    pub met2a: Met2A,
}

#[derive(LayerFamily, Clone, Copy)]
pub struct Met1A {
    #[layer(gds = "68/20", primary)]
    pub drawing: Met1ADrawing,
    #[layer(gds = "68/16", pin)]
    pub pin: Met1APin,
    #[layer(gds = "68/5", label)]
    pub label: Met1ALabel,
}

#[derive(Layers)]
pub struct ExamplePdkBLayers {
    #[layer(gds = "66/20")]
    pub polyb: PolyB,
    #[layer_family]
    pub met1b: Met1B,
    #[layer(name = "met2", gds = "69/20")]
    pub met2b: Met2B,
}

#[derive(LayerFamily, Clone, Copy)]
pub struct Met1B {
    #[layer(gds = "68/20", primary)]
    pub drawing: Met1BDrawing,
    #[layer(gds = "68/16", pin)]
    pub pin: Met1BPin,
    #[layer(gds = "68/5", label)]
    pub label: Met1BLabel,
}