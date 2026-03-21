use std::fmt::Debug;

use rrui::{Application, Color, config::IcedWgpuWinit, widgets};
use winit::error::EventLoopError;

pub fn main() -> Result<(), EventLoopError> {
    let mut config = IcedWgpuWinit::default();
    config.render_config.clear_color = Some(Color::xrgb(0xdddd55));
    config.window_config.title = "hello rrui window".into();

    rrui::run(App, config)?;

    Ok(())
}

pub struct App;

impl<R, E: Debug> Application<R, E> for App {
    type Message = ();

    fn message(&mut self, _msg: Self::Message) {}

    fn root(&mut self) -> rrui::Element<R, Self::Message, E> {
        widgets::Nothing.into()
    }
}
