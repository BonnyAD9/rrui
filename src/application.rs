use crate::{
    Element, Shell,
    event::{EventCtrl, EventInfo},
};

pub trait Application<Renderer, Event> {
    type Message: Clone + Sync + Send + 'static;
    type Theme;

    fn pre_event(
        &mut self,
        shell: &mut Shell<Self::Message>,
        event: &EventInfo<Event>,
        evt_ctrl: &mut EventCtrl,
    ) {
        _ = shell;
        _ = event;
        _ = evt_ctrl;
    }

    fn post_event(
        &mut self,
        shell: &mut Shell<Self::Message>,
        event: &EventInfo<Event>,
    ) {
        _ = shell;
        _ = event;
    }

    fn message(
        &mut self,
        shell: &mut Shell<Self::Message>,
        msg: Self::Message,
    );

    fn messages(
        &mut self,
        shell: &mut Shell<Self::Message>,
        msgs: &mut Vec<Self::Message>,
    ) {
        for msg in msgs.splice(.., []) {
            self.message(shell, msg);
        }
    }

    fn root(
        &mut self,
        shell: &mut Shell<Self::Message>,
    ) -> Element<Renderer, Self::Message, Event, Self::Theme>;

    fn theme(&self) -> &Self::Theme;
}
