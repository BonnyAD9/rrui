use minlin::{Rect, Vec2};

use crate::{SvgData, SvgParameters};

pub trait SvgRenderer {
    type SvgData: SvgData;

    fn measure_svg(&self, data: &Self::SvgData) -> Vec2<u32>;

    fn draw_svg_clipped(
        &mut self,
        bounds: impl Into<Rect<f32>>,
        clip_bounds: impl Into<Rect<f32>>,
        data: &Self::SvgData,
        params: &SvgParameters,
    );

    fn draw_svg(
        &mut self,
        bounds: impl Into<Rect<f32>>,
        data: &Self::SvgData,
        params: &SvgParameters,
    ) {
        let bounds = bounds.into();
        self.draw_svg_clipped(bounds, bounds, data, params);
    }
}
