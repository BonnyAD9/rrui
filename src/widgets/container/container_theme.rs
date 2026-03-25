use crate::widgets::ContainerAppereance;

pub trait ContainerTheme {
    type Style;

    fn border_width(&self, style: &Self::Style) -> f32;

    fn appereance(&self, style: &Self::Style) -> Option<ContainerAppereance>;
}
