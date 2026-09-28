use std::{
    sync::Arc,
    task::Wake,
    thread::{self, Thread},
};

pub struct IdkWhatImDoing {
    thread: Thread,
}

impl Wake for IdkWhatImDoing {
    fn wake(self: Arc<Self>) {
        self.thread.unpark();
    }
    fn wake_by_ref(self: &Arc<Self>) {
        self.thread.unpark();
    }
}

impl IdkWhatImDoing {
    pub fn new() -> Self {
        Self {
            thread: thread::current(),
        }
    }
}
