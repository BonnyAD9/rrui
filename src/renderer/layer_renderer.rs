use minlin::Rect;

pub trait LayerRenderer {
    fn with_clip<T>(
        &mut self,
        bounds: impl Into<Rect<f32>>,
        f: impl FnOnce(&mut Self) -> T,
    ) -> T;
}
