use minlin::{Infinity, Vec2};
use rrui::{
    Application, Color, Element, Font, QuadRenderer, Shell, Size, TextAlign,
    TextRenderer, TextWrap, Theme,
    config::IcedWgpuWinit,
    event::Event,
    widgets::{Button, Container, Grid, Stack, TextBlock, Variable},
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
        let mut text =
            TextBlock::new("A quick brown fox jumped over the lazy dog.");
        text.size = Some(Vec2::INFINITY);
        text.wrapping = TextWrap::WordOrGlyph;
        text.align_x = TextAlign::Center;
        let (text_in, text_out) = shell.make_ref_variable(text);
        let text: Variable<TextBlock<_, R::Font, _>> = Variable::new(text_out);

        let db_text = text_in.clone();
        let mut def_but = Button::text("Default");
        def_but.size([100., 30.]);
        def_but.on_press(move |_| {
            db_text.borrow_mut().font = None;
            None
        });

        let db_text = text_in.clone();
        let mut mono_but = Button::text("Monospace");
        mono_but.size([100., 30.]);
        mono_but.on_press(move |_| {
            db_text.borrow_mut().font = Some(Font::monospace());
            None
        });

        let mut buts = Stack::from_left([def_but, mono_but]);
        buts.spacing(6.);
        let mut buts = Container::new(buts);
        buts.pad_rel([1., 1., 1., 4.]);

        let mut grid = Grid::<Element<_, _, _, _>>::new(
            [],
            [Size::Relative(1.), Size::Relative(1.)],
        );
        grid.add([0, 0], text.into());
        grid.add([0, 1], buts.into());

        grid.into()
    }

    fn theme(&self) -> &Self::Theme {
        &self.theme
    }
}
