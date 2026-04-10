#[derive(Debug, Clone, Copy, Default)]
pub struct ScrollbarState {
    pub len: f32,
    pub view: f32,
    pub pos: f32,
}

impl ScrollbarState {
    pub fn new(len: f32, view: f32, pos: f32) -> Self {
        Self { len, view, pos }
    }

    pub fn rel_pos(&self) -> f32 {
        self.pos / (self.len - self.view)
    }

    pub fn from_rel(&self, rel: f32) -> f32 {
        rel * (self.len - self.view)
    }
}
