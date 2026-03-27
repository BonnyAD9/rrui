use crate::{
    Background, Border, Color, Radius,
    widgets::{
        ButtonState, ButtonTheme, ContainerAppereance, ContainerTheme,
        TextBlockTheme,
    },
};

#[derive(Debug, Clone)]
pub struct Theme {
    pub border: Border,
    pub bg_dark: Background,
    pub bg_norm: Background,
    pub fg: Color,
    pub accent: Color,
}

impl Theme {
    pub fn dark() -> Self {
        Self {
            border: Border {
                color: Color::xrgb(0x555555),
                width: 1.,
                radius: Radius::same(5.),
            },
            bg_dark: Color::xrgb(0x181818).into(),
            bg_norm: Color::xrgb(0x222222).into(),
            fg: Color::xrgb(0xeeeeee),
            accent: Color::xrgb(0xffd553),
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
                background: self.bg_norm.clone(),
            })
        } else {
            None
        }
    }
}

impl TextBlockTheme for Theme {
    type Style = ();

    fn foreground(&self, _: &Self::Style) -> Color {
        self.fg
    }
}

impl ButtonTheme for Theme {
    type Style = ();

    fn appereance(
        &self,
        _: &Self::Style,
        state: ButtonState,
    ) -> Option<ContainerAppereance> {
        match state {
            ButtonState::Normal => Some(ContainerAppereance {
                border: self.border,
                background: self.bg_norm.clone(),
            }),
            ButtonState::Hover => Some(ContainerAppereance {
                border: self.border.with_color(self.accent.rgb_mul(0.7)),
                background: self.bg_norm.clone(),
            }),
            ButtonState::Pressed => Some(ContainerAppereance {
                border: self.border.with_color(self.accent.rgb_mul(0.7)),
                background: self.accent.rgb_mul(0.3).into(),
            }),
        }
    }

    fn is_different(
        &self,
        _: &Self::Style,
        a: ButtonState,
        b: ButtonState,
    ) -> bool {
        a != b
    }
}
