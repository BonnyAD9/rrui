use std::fmt::Debug;

use rrui::{
    config::IcedWgpuWinit, widgets, Application, Border, Color, Element,
    QuadRenderer,
};
use winit::error::EventLoopError;

pub fn main() -> Result<(), EventLoopError> {
    let mut config = IcedWgpuWinit::default();
    config.render_config.clear_color = Some(Color::xrgb(0xdddd55));
    config.window_config.title = "hello rrui window".into();

    rrui::run(App, config)?;

    Ok(())
}

pub struct App;

impl<R: QuadRenderer, E: Debug> Application<R, E> for App {
    type Message = ();
    type Theme = ();

    fn message(&mut self, _msg: Self::Message) {}

    fn root(&mut self) -> Element<R, Self::Message, E, Self::Theme> {
        widgets::Rectangle::new(
            (500., 500.),
            Color::xrgb(0x123456),
            Border::new(Color::xrgb(0xdd5555), 5., 10.),
        )
        .into()
    }

    fn theme(&self) -> &Self::Theme {
        &()
    }
}
