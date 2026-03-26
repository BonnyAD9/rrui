use crate::{
    Background, Border, Color, Radius,
    widgets::{ContainerAppereance, ContainerTheme},
};

#[derive(Debug, Clone)]
pub struct Theme {
    pub border: Border,
    pub bg: Background,
}

impl Theme {
    pub fn dark() -> Self {
        Self {
            border: Border {
                color: Color::xrgb(0x555555),
                width: 1.,
                radius: Radius::same(5.),
            },
            bg: Color::xrgb(0x222222).into(),
        }
    }
}

impl ContainerTheme for Theme {
    type Style = bool;

    fn border_width(&self, style: &Self::Style) -> f32 {
        if *style { self.border.width } else { 0. }
    }

    fn appereance(&self, style: &Self::Style) -> Option<ContainerAppereance> {
        if *style && self.border.width != 0. {
            Some(ContainerAppereance {
                border: self.border,
                background: self.bg.clone(),
            })
        } else {
            None
        }
    }
}
