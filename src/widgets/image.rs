use std::path::Path;

use bytes::Bytes;
use minlin::{Infinity, MapExt, Rect, RectExt, Vec2};

use crate::{
    Angle, Element, ImageData, ImageFill, ImageFilter, ImageParameters,
    ImageRenderer, LayoutFlags, Radius, RedrawSlot, RelPos, RelayoutSlot,
    Widget, WidgetExt,
};

#[derive(Debug)]
pub struct Image<I> {
    pub image: I,
    pub params: RedrawSlot<ImageParameters>,
    pub size: Option<Vec2<f32>>,
    pub fill: RelayoutSlot<ImageFill>,
    img_size: Option<Vec2<u32>>,
    ibounds: Rect<f32>,
    rel_pos: RelPos,
    bounds: Rect<f32>,
}

impl<I: ImageData> Image<I> {
    pub fn new(image: I) -> Self {
        Self {
            image,
            params: Default::default(),
            size: None,
            fill: Default::default(),
            img_size: None,
            ibounds: Rect::default(),
            rel_pos: RelPos::default(),
            bounds: Rect::default(),
        }
    }

    pub fn path(path: impl AsRef<Path>) -> Self {
        Self::new(I::from_path(path))
    }

    pub fn encoded(data: Bytes) -> Self {
        Self::new(I::from_data(data))
    }

    pub fn rgba(size: impl Into<Vec2<u32>>, data: Bytes) -> Self {
        Self::new(I::from_rgba(size.into(), data))
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
        params: impl Into<RedrawSlot<ImageParameters>>,
    ) -> &mut Self {
        self.params = params.into();
        self
    }

    pub fn filter(&mut self, filter: ImageFilter) -> &mut Self {
        self.params.filter = filter;
        self
    }

    pub fn rotation(&mut self, rotation: Angle) -> &mut Self {
        self.params.rotation = rotation;
        self
    }

    pub fn border_radius(&mut self, radius: impl Into<Radius>) -> &mut Self {
        self.params.border_radius = radius.into();
        self
    }

    pub fn opacity(&mut self, opacity: f32) -> &mut Self {
        self.params.opacity = opacity;
        self
    }

    pub fn snap(&mut self, snap: bool) -> &mut Self {
        self.params.snap = snap;
        self
    }
}

impl<Rend, Msg, Evt, Theme> Widget<Rend, Msg, Evt, Theme>
    for Image<Rend::ImageData>
where
    Rend: ImageRenderer,
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
            self.img_size = None;
        }

        let img_size = self.img_size(lp.renderer).cast();
        self.bounds = if let Some(s) = self.size {
            bounds.clamp(s)
        } else {
            bounds.clamp(img_size)
        };

        self.ibounds = self.fill.calculate(self.bounds, img_size);
        self.bounds
    }

    fn size(&mut self, _: &Theme) -> Vec2<f32> {
        self.size.unwrap_or(Vec2::INFINITY)
    }

    fn reposition(&mut self, _: &Theme, pos: Vec2<f32>) {
        let diff = self.ibounds.pos() - self.bounds.pos();
        self.ibounds.set_pos(pos + diff);
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
        let ibounds = self.rel_pos.position_rect(self.ibounds);
        renderer.draw_image_clipped(
            ibounds,
            bounds,
            &self.image,
            &self.params,
        );
    }
}

impl<I: ImageData> Image<I> {
    fn img_size<Rend: ImageRenderer<ImageData = I>>(
        &mut self,
        rend: &Rend,
    ) -> Vec2<u32> {
        if let Some(s) = self.img_size {
            s
        } else {
            let s = rend.image_size(&self.image);
            self.img_size = Some(s);
            s
        }
    }
}

impl<Rend, Msg, Evt, Theme> From<Image<Rend::ImageData>>
    for Element<Rend, Msg, Evt, Theme>
where
    Rend: ImageRenderer,
    Rend::ImageData: 'static,
{
    fn from(value: Image<Rend::ImageData>) -> Self {
        Self::new(value)
    }
}

impl<I> WidgetExt for Image<I> {}
