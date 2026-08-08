use minlin::Vec2;

use crate::{
    EditorAction, EditorParams, Font, primitives::selection::Selection,
};

pub trait Editor: Default + 'static {
    type Font: Font;

    fn with_text(text: &str) -> Self;

    fn is_empty(&self) -> bool;

    fn copy(&self) -> Option<String>;

    fn do_action(&mut self, action: EditorAction);

    fn update(
        &mut self,
        bounds: impl Into<Vec2<f32>>,
        params: &EditorParams<Self::Font>,
    );

    fn set_text(&mut self, text: &str);

    fn get_text(&self) -> String;

    fn selection(&self) -> Selection;

    fn min_bounds(&self) -> Vec2<f32>;
}
