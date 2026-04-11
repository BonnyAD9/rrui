use minlin::{Infinity, Vec2};
use rrui::{
    Align, Application, Bytes, Element, ImageFill, ImageRenderer,
    LayerRenderer, QuadRenderer, Shell, TextRenderer, Theme, VariableIn,
    config::IcedWgpuWinit,
    event::Event,
    widgets::{Button, Container, Grid, Image, Stack},
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
    R: ImageRenderer + QuadRenderer + TextRenderer + LayerRenderer + 'static,
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
        shell: &mut Shell<R, Self::Message, E, Self::Theme>,
    ) -> Element<R, Self::Message, E, Self::Theme> {
        let (fill_in, fill_out) = shell.make_variable(ImageFill::default());
        let mut img =
            Image::encoded(Bytes::from_static(include_bytes!("bonnyad9.jpg")));
        img.set_fill(fill_out);
        img.size = Some(Vec2::INFINITY);

        let mut grid =
            Grid::<Element<_, _, _, _>>::new_rel([1., 1., 1.], [1., 1., 1.]);
        grid.add_z(-1., [0, 0]..[3, 3], img.into());
        grid.add(
            [0, 0],
            align_buttons(fill_in.clone(), [Align::Start, Align::Start]),
        );
        grid.add(
            [1, 0],
            align_buttons(fill_in.clone(), [Align::Center, Align::Start]),
        );
        grid.add(
            [2, 0],
            align_buttons(fill_in.clone(), [Align::End, Align::Start]),
        );
        grid.add(
            [0, 1],
            align_buttons(fill_in.clone(), [Align::Start, Align::Center]),
        );
        grid.add([1, 1], center_buttons(fill_in.clone()));
        grid.add(
            [2, 1],
            align_buttons(fill_in.clone(), [Align::End, Align::Center]),
        );
        grid.add(
            [0, 2],
            align_buttons(fill_in.clone(), [Align::Start, Align::End]),
        );
        grid.add(
            [1, 2],
            align_buttons(fill_in.clone(), [Align::Center, Align::End]),
        );
        grid.add([2, 2], align_buttons(fill_in, [Align::End, Align::End]));

        grid.into()
    }

    fn theme(&self) -> &Self::Theme {
        &self.theme
    }
}

fn align_buttons<Rend, Evt>(
    var: VariableIn<ImageFill>,
    align: impl Into<Vec2<Align>>,
) -> Element<Rend, (), Evt, Theme>
where
    Rend: QuadRenderer + TextRenderer,
    Evt: Event,
{
    let var2 = var.clone();
    let align = align.into();

    let mut but_fill = Button::text("fill");
    but_fill.size([100., 30.]);
    but_fill.on_press(move |_| {
        var.set(ImageFill::Fill(align));
        None
    });

    let mut but_fit = Button::text("fit");
    but_fit.size([100., 30.]);
    but_fit.on_press(move |_| {
        var2.set(ImageFill::Fit(align));
        None
    });

    let mut stack = Stack::new([but_fill, but_fit]);
    stack.spacing(5.);
    Container::center(stack).into()
}

fn center_buttons<Rend, Evt>(
    var: VariableIn<ImageFill>,
) -> Element<Rend, (), Evt, Theme>
where
    Rend: QuadRenderer + TextRenderer,
    Evt: Event,
{
    let var2 = var.clone();
    let var3 = var.clone();

    let mut but_fill = Button::text("fill");
    but_fill.size([100., 30.]);
    but_fill.on_press(move |_| {
        var.set(ImageFill::fill_center());
        None
    });

    let mut but_strech = Button::text("strech");
    but_strech.size([100., 30.]);
    but_strech.on_press(move |_| {
        var2.set(ImageFill::Strech);
        None
    });

    let mut but_fit = Button::text("fit");
    but_fit.size([100., 30.]);
    but_fit.on_press(move |_| {
        var3.set(ImageFill::fit_center());
        None
    });

    let mut stack = Stack::new([but_fill, but_strech, but_fit]);
    stack.spacing(5.);
    Container::center(stack).into()
}
