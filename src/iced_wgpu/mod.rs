mod renderer;

use iced_wgpu::core::{
    Background, Border, Color, Rectangle, Shadow, Vector, border::Radius,
    renderer::Quad,
};
use minlin::Rect;

pub use self::renderer::*;

fn color(color: crate::Color) -> Color {
    Color::from_rgba(color.x, color.y, color.z, color.w)
}

fn rect(rect: Rect<f32>) -> Rectangle {
    Rectangle {
        x: rect.x,
        y: rect.y,
        width: rect.z,
        height: rect.w,
    }
}

impl From<crate::Radius> for Radius {
    fn from(value: crate::Radius) -> Self {
        Self {
            top_left: value.tl,
            top_right: value.tr,
            bottom_right: value.br,
            bottom_left: value.bl,
        }
    }
}

impl From<crate::Border> for Border {
    fn from(value: crate::Border) -> Self {
        Self {
            color: color(value.color),
            width: value.width,
            radius: value.radius.into(),
        }
    }
}

impl From<crate::Shadow> for Shadow {
    fn from(value: crate::Shadow) -> Self {
        Self {
            color: color(value.color),
            offset: Vector::new(value.offset.x, value.offset.y),
            blur_radius: value.radius,
        }
    }
}

impl From<&crate::Quad> for Quad {
    fn from(value: &crate::Quad) -> Self {
        Self {
            bounds: rect(value.bounds),
            border: value.border.into(),
            shadow: value.shadow.into(),
            snap: value.snap,
        }
    }
}

impl From<crate::Background> for Background {
    fn from(value: crate::Background) -> Self {
        match value {
            crate::Background::Solid(c) => Self::Color(color(c)),
            crate::Background::None => Self::Color(Color::TRANSPARENT),
        }
    }
}
