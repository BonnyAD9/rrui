use std::fmt::Debug;

use minlin::Vec4;
use rrui::{config::IcedWgpuWinit, widgets, Application};
use winit::error::EventLoopError;

pub fn main() -> Result<(), EventLoopError> {
    let mut config = IcedWgpuWinit::default();
    config.render_config.clear_color = Some(Vec4::new(0.8, 0.8, 0.2, 1.));
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
