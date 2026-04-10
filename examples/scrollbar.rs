use minlin::{Infinity, Vec2};
use rrui::{
    Application, Color, Element, LayerRenderer, QuadRenderer, Shell,
    SvgRenderer, TextAlign, TextRenderer, Theme,
    config::IcedWgpuWinit,
    event::Event,
    widgets::{
        Button, Container, Grid, Layout, Scrollbar, ScrollbarState, TextBlock,
    },
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
    R: QuadRenderer + TextRenderer + SvgRenderer + 'static,
    E: Event + 'static,
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
        shell: &mut Shell<Self::Message>,
    ) -> Element<R, Self::Message, E, Self::Theme> {
        let (text_in, text_out) = shell.make_variable("0 / 100");

        let mut text = TextBlock::variable(text_out);
        text.size = Some(Vec2::INFINITY);
        text.align_x = TextAlign::Center;

        let mut scrollbar = Scrollbar::vertical();
        scrollbar.configure(ScrollbarState::new(110., 10., 0.));
        scrollbar.on_scroll(move |pos| {
            text_in.set(format!("{pos:.0} / 100"));
            None
        });

        let mut layout = Layout::<Element<_, _, _, _>>::horizontal();
        layout.add_unit(text.into());
        layout.add_abs(20., scrollbar.into());

        layout.into()
    }

    fn theme(&self) -> &Self::Theme {
        &self.theme
    }
}
