use rrui::{
    Application, Bytes, Element, ImageRenderer, QuadRenderer, Shell, Theme,
    config::IcedWgpuWinit, event::Event, widgets::Image,
};
use winit::error::EventLoopError;

pub fn main() -> Result<(), EventLoopError> {
    let mut config = IcedWgpuWinit::default();
    config.render_config.clear_color = None;
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

impl<R, E> Application<R, E> for App
where
    R: ImageRenderer + QuadRenderer,
    E: Event,
{
    type Message = ();
    type Theme = Theme;

    fn message(
        &mut self,
        _: &mut rrui::Shell<Self::Message>,
        _: Self::Message,
    ) {
    }

    fn root(
        &mut self,
        _: &mut Shell<Self::Message>,
    ) -> Element<R, Self::Message, E, Self::Theme> {
        Image::encoded(Bytes::from_static(include_bytes!("bonnyad9.jpg")))
            .into()
    }

    fn theme(&self) -> &Self::Theme {
        &self.theme
    }
}
