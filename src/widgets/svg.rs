use std::{borrow::Cow, path::Path};

use bytes::Bytes;
use minlin::{Infinity, MapExt, Rect, RectExt, Vec2};

use crate::{
    Angle, ControlRenderer, Element, ImageFill, LayoutFlags, RedrawSlot,
    RelPos, RelayoutSlot, SvgData, SvgParameters, SvgRenderer, Widget,
    WidgetExt,
};

#[derive(Debug)]
pub struct Svg<S> {
    pub svg: S,
    pub params: RedrawSlot<SvgParameters>,
    pub size: Option<Vec2<f32>>,
    pub fill: RelayoutSlot<ImageFill>,
    pub use_requested_color: bool,
    svg_size: Option<Vec2<u32>>,
    sbounds: Rect<f32>,
    rel_pos: RelPos,
    bounds: Rect<f32>,
}

impl<S: SvgData> Svg<S> {
    pub fn new(svg: S) -> Self {
        Self {
            svg,
            params: Default::default(),
            size: None,
            fill: Default::default(),
            use_requested_color: false,
            svg_size: None,
            sbounds: Rect::default(),
            rel_pos: RelPos::default(),
            bounds: Rect::default(),
        }
    }

    pub fn path(path: impl AsRef<Path>) -> Self {
        Self::new(S::from_path(path))
    }

    pub fn memory(data: Bytes) -> Self {
        Self::new(S::from_memory(data))
    }

    pub fn from_static(data: impl Into<Cow<'static, [u8]>>) -> Self {
        Self::new(S::from_static(data))
    }

    pub fn size(&mut self, size: impl Into<Vec2<f32>>) -> &mut Self {
        self.size = Some(size.into());
        self
    }

    pub fn set_fill(
        &mut self,
        fill: impl Into<RelayoutSlot<ImageFill>>,
    ) -> &mut Self {
        self.fill = fill.into();
        self
    }

    pub fn fill(&mut self) -> &mut Self {
        self.fill = ImageFill::fill_center().into();
        self
    }

    pub fn fit(&mut self) -> &mut Self {
        self.fill = ImageFill::fit_center().into();
        self
    }

    pub fn params(
        &mut self,
        params: impl Into<RedrawSlot<SvgParameters>>,
    ) -> &mut Self {
        self.params = params.into();
        self
    }

    pub fn rotation(&mut self, rotation: Angle) -> &mut Self {
        self.params.rotation = rotation;
        self
    }

    pub fn opacity(&mut self, opacity: f32) -> &mut Self {
        self.params.opacity = opacity;
        self
    }
}

impl<Rend, Msg, Evt, Theme> Widget<Rend, Msg, Evt, Theme>
    for Svg<Rend::SvgData>
where
    Rend: SvgRenderer + ControlRenderer,
{
    fn layout(
        &mut self,
        lp: &mut crate::LayoutParams<'_, Rend, Msg, Evt, Theme>,
        bounds: &crate::LayoutBounds,
        pos_base: RelPos,
        flags: crate::LayoutFlags,
    ) -> Rect<f32> {
        self.rel_pos.update(pos_base);
        self.fill.update();
        if flags.contains(LayoutFlags::WIDGET_MODIFIED) {
            self.svg_size = None;
        }

        let svg_size = lp.renderer.measure_svg(&self.svg).cast();
        self.bounds = if let Some(s) = self.size {
            bounds.clamp(s)
        } else {
            bounds.clamp(svg_size)
        };

        self.sbounds = self.fill.calculate(self.bounds, svg_size);
        self.bounds
    }

    fn size(&mut self, _: &Theme) -> Vec2<f32> {
        self.size.unwrap_or(Vec2::INFINITY)
    }

    fn reposition(&mut self, _: &Theme, pos: Vec2<f32>) {
        let diff = self.sbounds.pos() - self.bounds.pos();
        self.sbounds.set_pos(pos + diff);
        self.bounds.set_pos(pos)
    }

    fn event(
        &mut self,
        _: &mut crate::Shell<Rend, Msg, Evt, Theme>,
        _: &Theme,
        _: &crate::event::EventInfo<Evt>,
    ) -> bool {
        false
    }

    fn draw(
        &mut self,
        _: &mut crate::Shell<Rend, Msg, Evt, Theme>,
        _: &Theme,
        renderer: &mut Rend,
    ) {
        self.params.update();
        let bounds = self.rel_pos.position_rect(self.bounds);
        let ibounds = self.rel_pos.position_rect(self.sbounds);
        if self.use_requested_color
            && let Some(c) = renderer.foreground()
        {
            let params = SvgParameters {
                color: Some(c),
                ..*self.params
            };
            renderer.draw_svg_clipped(ibounds, bounds, &self.svg, &params);
        } else {
            renderer.draw_svg_clipped(
                ibounds,
                bounds,
                &self.svg,
                &self.params,
            );
        }
    }
}

impl<Rend, Msg, Evt, Theme> From<Svg<Rend::SvgData>>
    for Element<Rend, Msg, Evt, Theme>
where
    Rend: SvgRenderer + ControlRenderer,
    Rend::SvgData: 'static,
{
    fn from(value: Svg<Rend::SvgData>) -> Self {
        Self::new(value)
    }
}

impl<I> WidgetExt for Svg<I> {}
