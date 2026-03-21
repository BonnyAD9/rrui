use crate::{event::Event, EventLoopProxy};

pub trait EventLoop<Message, AppState>: Sized {
    type Event: Event;
    type Proxy: EventLoopProxy<Message>;
    type Window;
    type Error;

    fn create() -> Result<Self, Self::Error>;

    fn run(self, app: &mut AppState) -> Result<(), Self::Error>;
}
