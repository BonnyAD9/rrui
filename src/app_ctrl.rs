use crate::Window;

pub trait AppCtrl {
    type Window: Window;
    type Error: std::error::Error;

    fn exit(self);

    fn create_window(
        self,
        cfg: <Self::Window as Window>::Config,
    ) -> Result<Self::Window, Self::Error>;
}
