#[derive(Debug, Copy, Clone)]
pub enum MayInit<Conf, Val> {
    Uninitialized(Conf),
    Initialized(Val),
    None,
}

impl<C, V> MayInit<C, V> {
    pub fn init(&mut self, f: impl FnOnce(C) -> V) -> &mut V {
        match self {
            Self::Uninitialized(_) => {
                let Self::Uninitialized(v) =
                    std::mem::replace(self, Self::None)
                else {
                    unreachable!();
                };
                *self = Self::Initialized(f(v));
                self.unwrap_mut()
            }
            Self::Initialized(v) => v,
            _ => panic!(),
        }
    }

    pub fn unwrap_mut(&mut self) -> &mut V {
        match self {
            Self::Initialized(v) => v,
            _ => panic!(),
        }
    }
}
