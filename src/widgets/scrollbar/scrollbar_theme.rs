use minlin::{Padding, Vec2};

use crate::{
    Orientation, SvgData, SvgParameters,
    widgets::{
        ButtonState, ButtonTheme, ContainerAppereance, ScrollbarSizes,
        ThumbTheme, TrackTheme, scrollbar::scrollbar_style::ScrollbarStyle,
    },
};

pub trait ScrollbarTheme: ButtonTheme + ThumbTheme + TrackTheme {
    type Style: ScrollbarStyle<
            ButtonStyle = <Self as ButtonTheme>::Style,
            TrackStyle = <Self as TrackTheme>::Style,
            ThumbStyle = <Self as ThumbTheme>::Style,
        >;

    fn sizes(&self, style: &<Self as ScrollbarTheme>::Style)
    -> ScrollbarSizes;

    fn min_thumb(&self, style: &<Self as ScrollbarTheme>::Style) -> f32;

    fn appereance(
        &self,
        style: &<Self as ScrollbarTheme>::Style,
        orientation: Orientation,
    ) -> Option<ContainerAppereance>;

    fn padding(
        &self,
        style: &<Self as ScrollbarTheme>::Style,
        orientation: Orientation,
    ) -> Padding<f32>;

    fn top_button<Svg: SvgData>(
        &self,
        style: &<Self as ScrollbarTheme>::Style,
    ) -> (Svg, SvgParameters);

    fn bottom_button<Svg: SvgData>(
        &self,
        style: &<Self as ScrollbarTheme>::Style,
    ) -> (Svg, SvgParameters);

    fn left_button<Svg: SvgData>(
        &self,
        style: &<Self as ScrollbarTheme>::Style,
    ) -> (Svg, SvgParameters);

    fn right_button<Svg: SvgData>(
        &self,
        style: &<Self as ScrollbarTheme>::Style,
    ) -> (Svg, SvgParameters);
}
