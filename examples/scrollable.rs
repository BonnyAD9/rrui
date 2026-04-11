use minlin::Vec2;
use rrui::{
    Application, Bytes, Color, Element, ImageFill, ImageRenderer,
    LayerRenderer, QuadRenderer, Shell, SvgRenderer, Theme,
    config::IcedWgpuWinit,
    event::Event,
    widgets::{Image, Scrollable},
};
use winit::error::EventLoopError;

pub fn main() -> Result<(), EventLoopError> {
    let mut config = IcedWgpuWinit::default();
    config.render_config.clear_color = Some(Color::xrgb(0x181818));
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
    R: QuadRenderer + SvgRenderer + LayerRenderer + ImageRenderer + 'static,
    E: Event + 'static,
{
    type Message = ();
    type Theme = Theme;

    fn message(
        &mut self,
        _: &mut rrui::Shell<R, Self::Message, E, Self::Theme>,
        _: Self::Message,
    ) {
    }

    fn root(
        &mut self,
        _: &mut Shell<R, Self::Message, E, Self::Theme>,
    ) -> Element<R, Self::Message, E, Self::Theme> {
        let mut img =
            Image::encoded(Bytes::from_static(include_bytes!("bonnyad9.jpg")));
        img.set_fill(ImageFill::fill_center());
        img.size = Some(Vec2::new(2000., 2000.));

        Scrollable::both(img).into()
    }

    fn theme(&self) -> &Self::Theme {
        &self.theme
    }
}
