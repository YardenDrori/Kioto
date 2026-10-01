use std::{
    pin::Pin,
    sync::{Arc, Mutex},
    task::{Context, Wake, Waker},
};

use crate::executor::{executor::Executor, waker::MyWaker};

pub struct Task {
    pub future: Mutex<Pin<Box<dyn Future<Output = ()> + Send>>>,
    pub task_state: Mutex<TaskState>,
    pub executor: Arc<Executor>,
}

impl Task {
    fn requeue(self: &Arc<Self>) {
        //PERF: potential dumb atomic counter operaion
        let task_clone = self.clone();

        self.executor.register_task(task_clone);
    }
}

impl Wake for Task {
    fn wake(self: Arc<Self>) {
        self.requeue();
    }
    fn wake_by_ref(self: &Arc<Self>) {
        self.requeue();
    }
}

pub enum TaskState {
    QUEUED,
    PENDING,
    DONE,
}
