#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ImageFilter {
    #[default]
    Linear,
    Nearest,
}
