use minlin::Rect;

#[derive(Debug, Default)]
pub struct Shell {
    pub(crate) window_bounds: Rect<f32>,
    pub(crate) redraw: bool,
    pub(crate) relayout: bool,
}

impl Shell {
    pub fn request_redraw(&mut self) {
        self.redraw = true;
    }

    pub fn redraw_requested(&self) -> bool {
        self.redraw
    }

    pub fn request_relayout(&mut self) {
        self.relayout = true;
    }

    pub fn relayout_requested(&self) -> bool {
        self.relayout
    }
}
