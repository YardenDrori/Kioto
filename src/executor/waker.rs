use std::{
    sync::{Arc, Mutex},
    task::Wake,
};

use crate::executor::executor::Executor;

struct Waker {
    executor: Arc<Executor>,
}

//wake trait doesnt accept lifetimes
impl Wake for Waker {
    fn wake(self: Arc<Self>) {
        todo!()
    }
    fn wake_by_ref(self: &Arc<Self>) {
        todo!()
    }
}
