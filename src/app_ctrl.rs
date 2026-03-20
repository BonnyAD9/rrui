pub trait AppCtrl {
    fn exit(self);
}

impl AppCtrl for &winit::event_loop::ActiveEventLoop {
    fn exit(self) {
        winit::event_loop::ActiveEventLoop::exit(self)
    }
}
