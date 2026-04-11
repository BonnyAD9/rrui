use crate::event::EventFlags;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EventTarget {
    Nothing,
    Root,
    DragCapture,
    DragCaptureEnd,
}

impl EventTarget {
    pub fn target_flags(&self) -> EventFlags {
        match self {
            EventTarget::Nothing => EventFlags::empty(),
            EventTarget::Root => EventFlags::empty(),
            EventTarget::DragCapture => EventFlags::DRAG_CAPTURE,
            EventTarget::DragCaptureEnd => EventFlags::DRAG_CAPTURE,
        }
    }
}

impl Iterator for EventTarget {
    type Item = EventTarget;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            EventTarget::Nothing => None,
            EventTarget::Root => {
                *self = EventTarget::Nothing;
                Some(EventTarget::Root)
            }
            EventTarget::DragCapture => {
                *self = EventTarget::Root;
                Some(EventTarget::DragCapture)
            }
            EventTarget::DragCaptureEnd => {
                *self = EventTarget::Root;
                Some(EventTarget::DragCaptureEnd)
            }
        }
    }
}
