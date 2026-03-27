use minlin::{Rect, RectExt, Vec2};

use crate::event::{Event, EventFlags, EventKind, MouseRelation};

#[derive(Clone, Copy, Debug)]
pub struct EventInfo<E> {
    event: E,
    kind: EventKind,
    flags: EventFlags,
    old_mouse: Option<Vec2<f32>>,
    new_mouse: Option<Vec2<f32>>,
}

impl<E: Event> EventInfo<E> {
    pub fn new(event: E, old_mouse: Option<Vec2<f32>>) -> Self {
        let kind = event.get_type();
        let flags = event.get_flags();

        let new_mouse = match kind {
            EventKind::MouseMove(pos) => Some(pos),
            EventKind::MouseLeaveWindow => None,
            _ => old_mouse,
        };

        Self {
            event,
            kind,
            flags,
            old_mouse,
            new_mouse,
        }
    }

    pub fn mouse_relate_to(&self, bounds: Rect<f32>) -> MouseRelation {
        if !self.is_mouse() {
            return MouseRelation::None;
        }
        let old = self
            .old_mouse
            .map(|p| bounds.contains(p))
            .unwrap_or_default();
        let new = self
            .new_mouse
            .map(|p| bounds.contains(p))
            .unwrap_or_default();
        match (old, new) {
            (false, false) => MouseRelation::Elswhere,
            (false, true) => MouseRelation::Enter,
            (true, false) => MouseRelation::Leave,
            (true, true) => {
                if self.old_mouse == self.new_mouse {
                    MouseRelation::Hover
                } else {
                    MouseRelation::Enter
                }
            }
        }
    }

    pub fn is_for(&self, bounds: Rect<f32>) -> bool {
        !self.is_mouse()
            || self
                .old_mouse
                .map(|p| bounds.contains(p))
                .unwrap_or_default()
            || self
                .new_mouse
                .map(|p| bounds.contains(p))
                .unwrap_or_default()
    }
}

impl<E: Event> Event for EventInfo<E> {
    fn get_type(&self) -> EventKind {
        self.kind
    }

    fn key_char(&self) -> Option<smol_str::SmolStr> {
        self.event.key_char()
    }

    fn get_flags(&self) -> EventFlags {
        self.flags
    }
}
