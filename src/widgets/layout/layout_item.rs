use crate::{Orientation, Size, Space, Widget};

#[derive(Debug, Copy, Clone)]
pub struct LayoutItem<W> {
    pub widget: W,
    pub space: Space,
}

impl<W> LayoutItem<W> {
    pub fn new(space: impl Into<Space>, widget: W) -> Self {
        Self {
            space: space.into(),
            widget,
        }
    }

    pub fn size<Rend, Msg, Evt, Theme>(
        &mut self,
        theme: &Theme,
        orientation: Orientation,
    ) -> Size
    where
        W: Widget<Rend, Msg, Evt, Theme>,
    {
        match self.space {
            Space::Relative(v) => Size::Relative(v),
            Space::Absolute(v) => Size::Absolute(v),
            Space::Auto => {
                let v = orientation.component(self.widget.size(theme));
                if v.is_finite() {
                    Size::Absolute(v)
                } else {
                    Size::Relative(1.)
                }
            }
        }
    }
}
