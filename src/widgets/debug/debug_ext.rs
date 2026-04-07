use crate::{WidgetExt, widgets::Debug};

pub trait DebugExt: Sized {
    fn debug(self) -> Debug<Self> {
        Debug::r(self)
    }
}

impl<T: WidgetExt> DebugExt for T {}
