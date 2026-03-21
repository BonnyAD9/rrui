use std::fmt::Debug;

use rrui::{
    config::IcedWgpuWinit, widgets, Application, Border, Color, QuadRenderer,
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

    fn message(&mut self, _msg: Self::Message) {}

    fn root(&mut self) -> rrui::Element<R, Self::Message, E> {
        widgets::Rectangle::new(
            (500., 500.),
            Color::xrgb(0x123456),
            Border::new(Color::xrgb(0xdd5555), 5., 10.),
        )
        .into()
    }
}
