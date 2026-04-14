use crate::Color;

pub trait ControlRenderer {
    fn replace_foreground(&mut self, color: Option<Color>) -> Option<Color>;

    fn with_foreground<T>(
        &mut self,
        color: Option<Color>,
        f: impl FnOnce(&mut Self) -> T,
    ) -> T {
        let old = self.replace_foreground(color);
        let res = f(self);
        self.replace_foreground(old);
        res
    }

    fn foreground(&self) -> Option<Color>;
}
