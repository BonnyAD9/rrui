use std::{borrow::Cow, path::Path};

use bytes::Bytes;

pub trait SvgData: Clone {
    fn from_path(path: impl AsRef<Path>) -> Self;

    fn from_memory(data: Bytes) -> Self;

    fn from_static(data: impl Into<Cow<'static, [u8]>>) -> Self {
        Self::from_memory(Bytes::from_owner(data.into()))
    }
}
