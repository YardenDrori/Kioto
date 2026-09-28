use std::{
    sync::{Arc, Mutex},
    task::Wake,
};

use crate::executor::{
    executor::Executor,
    task::{Task, TaskStructAttempt},
};

struct Waker {
    //never mind! adding the return value generic breaks it cause once again waker doesnt take
    //generics or anything! how fun! we can just make a RawWaker perhaps? idk im genuinely stuck
    //here
    executor: Arc<Executor>,
    //need some way to pass task to waker so it can be readded to the queue
    // task: &'a TaskStructAttempt, --doesnt work Waker doesnt want lifetimes or generics
    // obscure_ptr: *const (), --could maybe? be a pointer to the task?? prob unsafe tho and doesnt work
}

//wake trait doesnt accept lifetimes
impl Wake for Waker {
    fn wake(self: Arc<Self>) {
        let mut task_queue = self.executor.tasks_sct.lock().expect("lock poisoned");
        task_queue.push(self.task);
    }
    fn wake_by_ref(self: &Arc<Self>) {}
}
