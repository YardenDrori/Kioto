use std::{
    sync::{Arc, Mutex},
    task::{RawWaker, RawWakerVTable, Wake, Waker},
};

use crate::executor::{executor::Executor, task::Task};

pub struct MyWaker {
    executor: Arc<Executor>,
    pub task: Arc<Mutex<Task>>,
}

impl Wake for MyWaker {
    fn wake(self: Arc<Self>) {
        let mut tasks = self.executor.tasks_sct.lock().expect("lock is poisoned");
        tasks.push(self.task.clone());
    }
    fn wake_by_ref(self: &Arc<Self>) {
        let mut tasks = self.executor.tasks_sct.lock().expect("lock is poisoned");
        tasks.push(self.task.clone());
    }
}

// enum MyRawWaker {}
//
// impl MyRawWaker {
//     fn new() {
//         let data: *const ();
//         let v_table = RawWakerVTable::new(clone, wake, wake_by_ref, drop);
//         let waker = RawWaker::new(data, &v_table);
//     }
// }
