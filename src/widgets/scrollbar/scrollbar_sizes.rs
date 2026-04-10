use minlin::Padding;

#[derive(Debug, Clone, Copy)]
pub struct ScrollbarSizes {
    // W
    pub size: f32,

    // H
    pub button: f32,

    pub button_padding: Padding<f32>,
}
