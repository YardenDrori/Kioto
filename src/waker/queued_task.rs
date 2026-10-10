use std::{
    pin::Pin,
    sync::{Arc, Mutex, MutexGuard, Weak},
    task::Wake,
};

use crate::executor::executor::Executor;

// TODO: replace this in favor of an AtomicU8 to save perf on mutex aquisitions
#[derive(PartialEq)]
pub enum TaskState {
    QUEUED,  // in the queue, waiting to be polled
    RUNNING, // actively running, not allowed to push to queue, change status to ALERTED to queue
    ALERTED, // has been awoken while RUNNING, when the poll ends, push to the queue
    IDLE,    //not doing anything, nor in the queue, waiting a waker to wake it
    DONE,    //finshed execution, a poll would panic. Under any circumstance do NOT poll
}

pub struct Task {
    pub future: Mutex<Pin<Box<dyn Future<Output = ()> + Send>>>,
    pub task_state: Mutex<TaskState>,

    // TODO: make this a list of executors with a cheap way to pick the emptieset one
    executor: Weak<Executor>,
}

// NOTE: allows using the function in existing critical sections
pub fn set_state_locked(lock: &mut MutexGuard<TaskState>, state: TaskState) {
    **lock = state;
}

pub fn replace_state_locked(
    lock: &mut MutexGuard<TaskState>,
    state_to_replace: TaskState,
    new_state: TaskState,
) -> bool {
    if **lock == state_to_replace {
        **lock = new_state;
        return true;
    }
    return false;
}

// TODO: push to the emptiest executor
pub fn queue_new_task(fut: Mutex<Pin<Box<dyn Future<Output = ()> + Send>>>, executor: &Arc<Executor>) {
    let task = Arc::new(Task {
        future: fut,
        task_state: Mutex::new(TaskState::QUEUED),
        executor: Arc::downgrade(&executor),
    });
    task.enqueue_self();
}

impl Wake for Task {
    fn wake(self: Arc<Self>) {
        println!("BEEP BEEP BEEP");
        let mut lock = self.task_state.lock().unwrap();
        if replace_state_locked(&mut lock, TaskState::IDLE, TaskState::QUEUED) {
            drop(lock);
            self.enqueue_self();
            return;
        }
        replace_state_locked(&mut lock, TaskState::RUNNING, TaskState::ALERTED);
    }
    fn wake_by_ref(self: &Arc<Self>) {
        self.clone().wake();
    }
}

impl Task {
    //example docs
    ///sets the task's state to the provided value.
    ///Blocking.
    ///
    ///
    ///also see:
    ///```rust
    ///pub fn replace_state(&self, state_to_replace: TaskState, new_state: TaskState) -> bool {
    ///pub fn swap_state(&self, new_state: TaskState) -> TaskState {
    ///```
    pub fn set_state(&self, state: TaskState) {
        let mut lock = self.task_state.lock().unwrap();
        set_state_locked(&mut lock, state);
    }
    pub fn replace_state(&self, state_to_replace: TaskState, new_state: TaskState) -> bool {
        let mut lock = self.task_state.lock().unwrap();
        replace_state_locked(&mut lock, state_to_replace, new_state)
    }
    pub fn swap_state(&self, new_state: TaskState) -> TaskState {
        let mut lock = self.task_state.lock().unwrap();
        std::mem::replace(&mut *lock, new_state)
    }

    fn enqueue_self(self: Arc<Self>) {
        let Some(executor) = self.executor.upgrade() else {
            panic!("executor not found");
        };
        executor.push_task(self.clone());
    }
}
