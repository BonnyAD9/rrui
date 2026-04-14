mod button_style;

use minlin::Padding;

pub use self::button_style::*;

use crate::{
    Background, Border, Color, Orientation, Radius, SvgParameters,
    widgets::{
        ButtonState, ButtonTheme, ContainerAppereance, ContainerTheme,
        ScrollableStyle, ScrollableTheme, ScrollbarSizes, ScrollbarStyle,
        ScrollbarTheme, TextBlockTheme, ThumbState, ThumbTheme, TrackTheme,
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

    fn foreground(&self, _: &Self::Style, requested: Option<Color>) -> Color {
        requested.unwrap_or(self.fg)
    }
}

impl ButtonTheme for Theme {
    type Style = ButtonStyle;

    fn appereance(
        &self,
        style: &Self::Style,
        state: ButtonState,
    ) -> Option<ContainerAppereance> {
        match style {
            ButtonStyle::Normal => match state {
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
                ButtonState::Disabled => Some(ContainerAppereance {
                    border: self
                        .border
                        .with_color(self.border.color.rgb_mul(0.7)),
                    background: self.bg_norm.clone(),
                }),
            },
            ButtonStyle::Scrollbar => None,
        }
    }

    fn foreground(
        &self,
        _: &Self::Style,
        state: ButtonState,
    ) -> Option<Color> {
        match state {
            ButtonState::Normal => None,
            ButtonState::Hover => None,
            ButtonState::Pressed => None,
            ButtonState::Disabled => Some(self.border.color),
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

impl TrackTheme for Theme {
    type Style = ();

    fn appereance(
        &self,
        _: &Self::Style,
        _: crate::widgets::TrackState,
        _: Orientation,
    ) -> Option<ContainerAppereance> {
        None
    }

    fn is_different(
        &self,
        _: &Self::Style,
        _: crate::widgets::TrackState,
        _: crate::widgets::TrackState,
    ) -> bool {
        false
    }

    fn padding(
        &self,
        _: &Self::Style,
        _: Orientation,
    ) -> minlin::Padding<f32> {
        Padding::default()
    }
}

impl ThumbTheme for Theme {
    type Style = ();

    fn appereance(
        &self,
        _: &Self::Style,
        state: crate::widgets::ThumbState,
        _: Orientation,
    ) -> Option<ContainerAppereance> {
        match state {
            ThumbState::Normal => Some(ContainerAppereance {
                border: self.border.with_radius(5.),
                background: self.border.color.rgb_mul(0.7).into(),
            }),
            ThumbState::Hover | ThumbState::Dragging(_) => {
                Some(ContainerAppereance {
                    border: Border::round(5.),
                    background: self.accent.rgb_mul(0.6).into(),
                })
            }
            ThumbState::TrackHover => Some(ContainerAppereance {
                border: self.border.with_radius(4.),
                background: self.border.color.rgb_mul(0.8).into(),
            }),
        }
    }

    fn is_different(
        &self,
        _: &Self::Style,
        a: crate::widgets::ThumbState,
        b: crate::widgets::ThumbState,
    ) -> bool {
        fn state_group(s: ThumbState) -> u32 {
            match s {
                ThumbState::Normal => 0,
                ThumbState::Hover | ThumbState::Dragging(_) => 1,
                ThumbState::TrackHover => 2,
            }
        }
        state_group(a) != state_group(b)
    }

    fn padding(
        &self,
        _: &Self::Style,
        orientation: Orientation,
    ) -> minlin::Padding<f32> {
        match orientation {
            Orientation::Horizontal => Padding::new(0., 7., 0., 5.),
            Orientation::Vertical => Padding::new(7., 0., 5., 0.),
        }
    }
}

impl ScrollbarStyle for () {
    type ButtonStyle = ButtonStyle;
    type TrackStyle = ();
    type ThumbStyle = ();

    fn button_style(&self) -> Self::ButtonStyle {
        ButtonStyle::Scrollbar
    }

    fn track_style(&self) -> Self::TrackStyle {}

    fn thumb_style(&self) -> Self::ThumbStyle {}
}

impl ScrollbarTheme for Theme {
    type Style = ();

    fn sizes(
        &self,
        _: &<Self as ScrollbarTheme>::Style,
        orientation: Orientation,
    ) -> crate::widgets::ScrollbarSizes {
        ScrollbarSizes {
            size: 20.,
            button: 20.,
            button_padding: match orientation {
                Orientation::Horizontal => Padding::new(0., 1., 0., 0.),
                Orientation::Vertical => Padding::new(1., 0., 0., 0.),
            },
        }
    }

    fn min_thumb(&self, _: &<Self as ScrollbarTheme>::Style) -> f32 {
        20.
    }

    fn appereance(
        &self,
        _: &<Self as ScrollbarTheme>::Style,
        _: Orientation,
    ) -> Option<ContainerAppereance> {
        Some(ContainerAppereance {
            border: Border::none(),
            background: self.border.color.into(),
        })
    }

    fn padding(
        &self,
        _: &<Self as ScrollbarTheme>::Style,
        orientation: Orientation,
    ) -> minlin::Padding<f32> {
        match orientation {
            Orientation::Horizontal => Padding::new(0., 0., 0., 19.),
            Orientation::Vertical => Padding::new(0., 0., 19., 0.),
        }
    }

    fn top_button<Svg: crate::SvgData>(
        &self,
        _: &<Self as ScrollbarTheme>::Style,
        state: ButtonState,
    ) -> (Svg, crate::SvgParameters) {
        (
            Svg::from_static(include_bytes!("point_up.svg")),
            SvgParameters::colored(match state {
                ButtonState::Normal => self.fg,
                ButtonState::Hover | ButtonState::Pressed => {
                    self.accent.rgb_mul(0.8)
                }
                ButtonState::Disabled => self.fg.rgb_mul(0.5),
            }),
        )
    }

    fn bottom_button<Svg: crate::SvgData>(
        &self,
        _: &<Self as ScrollbarTheme>::Style,
        state: ButtonState,
    ) -> (Svg, crate::SvgParameters) {
        (
            Svg::from_static(include_bytes!("point_down.svg")),
            SvgParameters::colored(match state {
                ButtonState::Normal => self.fg,
                ButtonState::Hover | ButtonState::Pressed => {
                    self.accent.rgb_mul(0.8)
                }
                ButtonState::Disabled => self.fg.rgb_mul(0.5),
            }),
        )
    }

    fn left_button<Svg: crate::SvgData>(
        &self,
        _: &<Self as ScrollbarTheme>::Style,
        state: ButtonState,
    ) -> (Svg, crate::SvgParameters) {
        (
            Svg::from_static(include_bytes!("point_left.svg")),
            SvgParameters::colored(match state {
                ButtonState::Normal => self.fg,
                ButtonState::Hover | ButtonState::Pressed => {
                    self.accent.rgb_mul(0.8)
                }
                ButtonState::Disabled => self.fg.rgb_mul(0.5),
            }),
        )
    }

    fn right_button<Svg: crate::SvgData>(
        &self,
        _: &<Self as ScrollbarTheme>::Style,
        state: ButtonState,
    ) -> (Svg, crate::SvgParameters) {
        (
            Svg::from_static(include_bytes!("point_right.svg")),
            SvgParameters::colored(match state {
                ButtonState::Normal => self.fg,
                ButtonState::Hover | ButtonState::Pressed => {
                    self.accent.rgb_mul(0.8)
                }
                ButtonState::Disabled => self.fg.rgb_mul(0.5),
            }),
        )
    }
}

impl ScrollableStyle for () {
    type ScrollbarStyle = ();

    fn scrollbar_style(&self) -> Self::ScrollbarStyle {}
}

impl ScrollableTheme for Theme {
    type Style = ();

    fn appereance(
        &self,
        _: &<Self as ScrollableTheme>::Style,
    ) -> Option<ContainerAppereance> {
        None
    }
}
