use crate::Color;

pub trait TextBlockTheme {
    type Style;

    fn foreground(
        &self,
        style: &Self::Style,
        requested: Option<Color>,
    ) -> Color;
}
