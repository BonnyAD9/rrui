use rrui::{
    Application, Color, Element, QuadRenderer, TextRenderer, Theme,
    config::IcedWgpuWinit,
    event::Event,
    widgets::{
        self, Button, ButtonTheme, Container, ContainerTheme, Stack,
        TextBlock, TextBlockTheme,
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

/*type Renderer = rrui::iced_wgpu::Renderer;
type TextBlock = widgets::TextBlock<
    <Theme as TextBlockTheme>::Style,
    <Renderer as TextRenderer>::Font,
    <Renderer as TextRenderer>::LayedText,
>;
type Button<W> = widgets::Button<
    W,
    <Theme as ButtonTheme>::Style,
    (),
>;

type Container<W> = widgets::Container<
    W,
    <Theme as ContainerTheme>::Style,
>;*/

impl<R: QuadRenderer + TextRenderer + 'static, E: Event + 'static>
    Application<R, E> for App
{
    type Message = ();
    type Theme = Theme;

    fn message(
        &mut self,
        _: &mut rrui::Shell<Self::Message>,
        _: Self::Message,
    ) {
    }

    fn root(&mut self) -> Element<R, Self::Message, E, Self::Theme> {
        let text = TextBlock::new("Hello there!");

        let mut but = Button::text("Click me!");
        but.size([100., 30.]);
        but.on_press(|_| {
            println!("Hello there!");
            None
        });

        let but = Container::center_styled(true, but);

        Stack::<Element<_, _, _, _>>::from_top([text.into(), but.into()])
            .into()
    }

    fn theme(&self) -> &Self::Theme {
        &self.theme
    }
}
