pub trait Font: Clone + Default + 'static {
    fn name_static(name: &'static str) -> Self;

    fn serif() -> Self;

    fn sans_serif() -> Self;

    fn cursive() -> Self;

    fn fantasy() -> Self;

    fn monospace() -> Self;
}
