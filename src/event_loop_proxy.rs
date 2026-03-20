pub trait EventLoopProxy<M>: Clone + Send + Sync {
    fn send(&self, m: M);
}

impl<M: Clone + Send + Sync> EventLoopProxy<M>
    for winit::event_loop::EventLoopProxy<M>
{
    fn send(&self, m: M) {
        _ = self.send_event(m);
    }
}
