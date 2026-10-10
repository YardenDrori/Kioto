use std::{
    task::Wake,
    thread::{self, current, park},
};

pub struct BlockingWaker {
    thread: thread::Thread,
}

impl BlockingWaker {
    pub fn new() -> Self {
        BlockingWaker { thread: current() }
    }
    pub fn block_until_wake() {
        park();
    }
}

impl Wake for BlockingWaker {
    fn wake(self: std::sync::Arc<Self>) {
        self.thread.unpark();
    }
    fn wake_by_ref(self: &std::sync::Arc<Self>) {
        self.thread.unpark();
    }
}
