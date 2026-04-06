pub trait EventLoopProxy<M>: Clone + Send + Sync {
    fn send(&self, m: M);
}
