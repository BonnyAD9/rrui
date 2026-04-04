use minlin::{Infinity, Vec2};
use rrui::{
    Application, Color, Element, QuadRenderer, Shell, Size, TextAlign,
    TextRenderer, Theme,
    config::IcedWgpuWinit,
    event::Event,
    widgets::{Button, Container, Grid, TextBlock},
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

    fn root(
        &mut self,
        shell: &mut Shell<Self::Message>,
    ) -> Element<R, Self::Message, E, Self::Theme> {
        let (text_in, text_out) = shell.make_variable("You clicked 0 times!");

        let mut text = TextBlock::variable(text_out);
        text.size = Some(Vec2::INFINITY);
        text.align_x = TextAlign::Center;

        let mut but = Button::text("Click me!");
        but.size([100., 30.]);

        let mut counter: u32 = 0;
        but.on_press(move |_| {
            counter += 1;
            text_in.set(format!("You clicked {counter} times!"));
            None
        });

        let mut but = Container::new(but);
        but.pad_rel([1., 1., 1., 4.]);

        let mut grid = Grid::<Element<_, _, _, _>>::new(
            [],
            [Size::Relative(1.), Size::Relative(1.)],
        );
        grid.add([0, 0], text.into());
        grid.add([0, 1], but.into());

        grid.into()
    }

    fn theme(&self) -> &Self::Theme {
        &self.theme
    }
}
