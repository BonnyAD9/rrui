#[derive(Debug, Default)]
pub struct Shell {
    pub(crate) redraw: bool,
}

impl Shell {
    pub fn request_redraw(&mut self) {
        self.redraw = true;
    }

    pub fn redraw_requested(&mut self) -> bool {
        self.redraw
    }
}
