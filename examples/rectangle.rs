use std::fmt::Debug;

use rrui::{
    Application, Border, Color, Element, QuadRenderer, Theme,
    config::IcedWgpuWinit,
    widgets::{Container, MarginExt, Rectangle},
};
use winit::error::EventLoopError;

pub fn main() -> Result<(), EventLoopError> {
    let mut config = IcedWgpuWinit::default();
    config.render_config.clear_color = Some(Color::xrgb(0xdddd55));
    config.window_config.title = "hello rrui window".into();

    rrui::run(App::new(), config)?;

    Ok(())
}

pub struct App {
    theme: Theme,
}

impl App {
    pub fn new() -> Self {
        Self {
            theme: Theme::dark(),
        }
    }
}

impl<R: QuadRenderer, E: Debug> Application<R, E> for App {
    type Message = ();
    type Theme = Theme;

    fn message(&mut self, _msg: Self::Message) {}

    fn root(&mut self) -> Element<R, Self::Message, E, Self::Theme> {
        Container::styled(
            true,
            Rectangle::new(
                (500., 500.),
                Color::xrgb(0x123456),
                Border::new(Color::xrgb(0xdd5555), 5., 10.),
            )
            .marge([10., 5.]),
        )
        .into()
    }

    fn theme(&self) -> &Self::Theme {
        &self.theme
    }
}
