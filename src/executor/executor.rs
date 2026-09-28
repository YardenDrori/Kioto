use std::{
    future::Pending,
    sync::{Arc, Mutex, RwLock},
    task::{Context, Poll::Ready, Waker},
};

use crate::{
    executor::{self, task::TaskStructAttemptThree},
    waker,
};

pub struct Executor {
    pub tasks_sct: Mutex<Vec<TaskStructAttemptThree>>,
}

impl Executor {
    fn new() -> Arc<Self> {
        return Arc::new(Executor {
            tasks_sct: Mutex::new(Vec::new()),
        });
    }

    fn loop_iter(self: &Self) {
        let mut tasks_sct = self.tasks_sct.lock().expect("lock is poisoned");
        let Some(mut task_sct) = tasks_sct.pop() else {
            return;
        };
        drop(tasks_sct); //free the mutex as soon as possible for other threads
        match task_sct.future.as_mut().poll(&mut Context::from_waker(Waker::noop())) {
            Ready(_) => {
                return;
            }
            _ => {
                return;
            }
        }
    }
}
