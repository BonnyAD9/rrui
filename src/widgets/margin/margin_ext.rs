use minlin::Padding;

use crate::{WidgetExt, widgets::Margin};

pub trait MarginExt: Sized {
    fn marge(self, additional: impl Into<Padding<f32>>) -> Margin<Self> {
        Margin::new(additional, self)
    }

    fn margin(self, total: impl Into<Padding<f32>>) -> Margin<Self> {
        Margin::new(total, self)
    }
}

impl<T: WidgetExt> MarginExt for T {}
