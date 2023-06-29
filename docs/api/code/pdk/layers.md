#[derive(Layers)]
pub struct ExamplePdkLayers {
    #[layer(gds = "66/20")]
    pub poly: Poly,
    #[layer(gds = "68/20")]
    #[pin(pin = "met1_pin", label = "met1_label")]
    pub met1: Met1,
    #[layer(alias = "met1")]
    pub met1_drawing: Met1,
    #[layer(gds = "68/16")]
    pub met1_pin: Met1Pin,
    #[layer(gds = "68/5")]
    pub met1_label: Met1Label,
    #[layer(name = "met2", gds = "69/20")]
    pub met2: Met2,
}