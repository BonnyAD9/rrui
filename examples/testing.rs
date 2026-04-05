use rrui::{
    Application, Element, QuadRenderer, Shell, TextAlign, TextRenderer,
    TextWrap, Theme,
    config::IcedWgpuWinit,
    event::Event,
    widgets::{Container, MarginExt, TextBlock},
};
use winit::error::EventLoopError;

pub fn main() -> Result<(), EventLoopError> {
    let mut config = IcedWgpuWinit::default();
    //config.render_config.clear_color = Some(Color::xrgb(0x0));
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

impl<R: QuadRenderer + TextRenderer, E: Event> Application<R, E> for App {
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
        let mut text = TextBlock::new(
            "Hello rrui! Or maybe some longer text to try here.\n\
            Even on multiple lines!",
        );
        text.wrapping = TextWrap::WordOrGlyph;
        text.align_x = TextAlign::Center;

        let mut con = Container::styled(true, text.margin(5.));
        con.size([200., 300.]);

        con.into()
    }

    fn theme(&self) -> &Self::Theme {
        &self.theme
    }
}
