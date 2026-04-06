use std::path::Path;

use bytes::Bytes;
use minlin::{Infinity, MapExt, Rect, RectExt, Vec2};

use crate::{
    Element, ImageData, ImageParameters, ImageRenderer, LayoutFlags,
    RefRedrawSlot, RelPos, Widget, WidgetExt,
};

#[derive(Debug)]
pub struct Image<I> {
    pub image: I,
    pub params: RefRedrawSlot<ImageParameters>,
    pub size: Option<Vec2<f32>>,
    img_size: Option<Vec2<u32>>,
    rel_pos: RelPos,
    bounds: Rect<f32>,
}

impl<I: ImageData> Image<I> {
    pub fn new(image: I) -> Self {
        Self {
            image,
            params: ImageParameters::default().into(),
            size: None,
            img_size: None,
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
}

impl<Rend, Msg, Evt, Theme> Widget<Rend, Msg, Evt, Theme>
    for Image<Rend::ImageData>
where
    Rend: ImageRenderer,
{
    fn layout(
        &mut self,
        lp: &mut crate::LayoutParams<'_, Rend, Msg, Theme>,
        bounds: &crate::LayoutBounds,
        pos_base: RelPos,
        flags: crate::LayoutFlags,
    ) -> Rect<f32> {
        self.rel_pos.update(pos_base);
        if flags.contains(LayoutFlags::WIDGET_MODIFIED) {
            self.img_size = None;
        }

        self.bounds = if let Some(s) = self.size {
            bounds.clamp(s)
        } else {
            bounds.clamp(self.img_size(lp.renderer).cast())
        };
        self.bounds
    }

    fn size(&mut self, _: &Theme) -> Vec2<f32> {
        self.size.unwrap_or(Vec2::INFINITY)
    }

    fn reposition(&mut self, _: &Theme, pos: Vec2<f32>) {
        self.bounds.set_pos(pos);
    }

    fn event(
        &mut self,
        _: &mut crate::Shell<Msg>,
        _: &Theme,
        _: &crate::event::EventInfo<Evt>,
    ) -> bool {
        false
    }

    fn draw(
        &mut self,
        _: &mut crate::Shell<Msg>,
        _: &Theme,
        renderer: &mut Rend,
    ) {
        self.params.update();
        let params = self.params.borrow();
        let bounds = self.rel_pos.position_rect(self.bounds);
        renderer.draw_image(bounds, &self.image, &params);
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
