use crate::Shell;

pub trait Widget<Rend, Msg, Evt> {
    fn event(&mut self, event: &Evt, shell: &mut Shell);

    fn draw(&mut self, renderer: &mut Rend, shell: &mut Shell);
}
