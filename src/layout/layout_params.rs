use crate::Shell;

#[derive(Debug)]
pub struct LayoutParams<'a, Rend, Msg, Evt, Theme> {
    pub shell: &'a mut Shell<Rend, Msg, Evt, Theme>,
    pub theme: &'a Theme,
    pub renderer: &'a Rend,
}

impl<'a, Rend, Msg, Evt, Theme> LayoutParams<'a, Rend, Msg, Evt, Theme> {
    pub fn new(
        shell: &'a mut Shell<Rend, Msg, Evt, Theme>,
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
