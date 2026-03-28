use crate::Shell;

#[derive(Debug)]
pub struct LayoutParams<'a, Rend, Msg, Theme> {
    pub shell: &'a mut Shell<Msg>,
    pub theme: &'a Theme,
    pub renderer: &'a Rend,
}

impl<'a, Rend, Msg, Theme> LayoutParams<'a, Rend, Msg, Theme> {
    pub fn new(
        shell: &'a mut Shell<Msg>,
        theme: &'a Theme,
        renderer: &'a Rend,
    ) -> Self {
        Self {
            shell,
            theme,
            renderer,
        }
    }
}
