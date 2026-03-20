use crate::Element;

pub trait Application<Renderer, Event> {
    type Message: Clone + Sync + Send + 'static;

    fn message(&mut self, msg: Self::Message);

    fn root(&mut self) -> Element<Renderer, Self::Message, Event>;
}
