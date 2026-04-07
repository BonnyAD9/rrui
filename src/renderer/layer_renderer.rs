use minlin::Rect;

pub trait LayerRenderer {
    fn start_layer(&mut self, bounds: impl Into<Rect<f32>>);

    fn end_layer(&mut self);

    fn clip_bounds(&self) -> Rect<f32>;

    fn with_layer<T>(
        &mut self,
        bounds: impl Into<Rect<f32>>,
        f: impl FnOnce(&mut Self) -> T,
    ) -> T {
        self.start_layer(bounds);
        let res = f(self);
        self.end_layer();
        res
    }

    fn start_clip(&mut self, bounds: impl Into<Rect<f32>>) {
        self.start_layer(bounds);
    }

    fn end_clip(&mut self) {
        self.end_layer();
    }

    fn with_clip<T>(
        &mut self,
        bounds: impl Into<Rect<f32>>,
        f: impl FnOnce(&mut Self) -> T,
    ) -> T {
        self.start_clip(bounds);
        let res = f(self);
        self.end_clip();
        res
    }

    fn start_on_top(&mut self) {
        self.start_layer(self.clip_bounds());
    }

    fn end_on_top(&mut self) {
        self.end_layer();
    }

    fn with_on_top<T>(&mut self, f: impl FnOnce(&mut Self) -> T) -> T {
        self.start_on_top();
        let res = f(self);
        self.end_on_top();
        res
    }
}
