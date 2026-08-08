use rrui::{
    Application, Color, ControlRenderer, Element, QuadRenderer, Shell,
    TextRenderer, Theme,
    config::IcedWgpuWinit,
    event::Event,
    widgets::{Container, Input, Stack, TextBlock},
};
use winit::error::EventLoopError;

pub fn main() -> Result<(), EventLoopError> {
    let mut config = IcedWgpuWinit::default();
    config.render_config.clear_color = Some(Color::xrgb(0xdddd55));
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

impl<
    R: QuadRenderer + TextRenderer + ControlRenderer + 'static,
    E: Event + 'static,
> Application<R, E> for App
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
        shell: &mut Shell<R, Self::Message, E, Self::Theme>,
    ) -> Element<R, Self::Message, E, Self::Theme> {
        let (txt_in, txt_out) = shell.make_variable("Waiting...");
        let text = TextBlock::variable(txt_out).centered();

        let mut input = Input::new();
        input.width(200.);
        input.on_change(|a| (None, Some(a.parse::<f32>().is_ok())));
        input.on_confirm(move |v| {
            if let Ok(n) = v.parse::<f32>() {
                txt_in.set(format!("You entered the number {n}."));
                (None, Some(true))
            } else {
                txt_in.set(format!("`{}` is not a number.", **v));
                (None, Some(false))
            }
        });

        let mut stack = Stack::<Element<_, _, _, _>>::new(vec![
            TextBlock::new("Enter a number").centered().into(),
            Container::center(input).into(),
            text.into(),
        ]);
        stack.spacing(5.);

        Container::center_styled(true, stack).into()
    }

    fn theme(&self) -> &Self::Theme {
        &self.theme
    }
}
