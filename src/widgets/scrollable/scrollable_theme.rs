use minlin::Vec2;

pub trait ScrollableTheme {
    type Style;

    fn scrollbar_size(&self, style: &Self::Style) -> Vec2<f32>;
}
