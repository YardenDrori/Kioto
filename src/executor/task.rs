use std::{
    pin::Pin,
    sync::{Arc, Mutex, MutexGuard, Weak},
    task::Wake,
};

use crate::executor::executor::Executor;

pub struct Task {
    pub future: Mutex<Pin<Box<dyn Future<Output = ()> + Send>>>,
    pub task_state: Mutex<TaskState>,
    executor: Weak<Executor>,
}

impl Task {
    ///blocking
    pub fn set_state(self: &Arc<Self>, state: TaskState) {
        let mut lock = self.task_state.lock().unwrap();
        set_state_locked(&mut lock, state);
    }
    pub fn replace_state(&self, state_to_replace: TaskState, new_state: TaskState) -> bool {
        let mut lock = self.task_state.lock().unwrap();
        replace_state_locked(&mut lock, state_to_replace, new_state)
    }

    pub fn enqueue_self(self: Arc<Self>) {
        let Some(executor) = self.executor.upgrade() else {
            panic!("executor not found");
        };
        executor.register_task(self.clone());
    }
}

// NOTE: allows using the function in existing critical sections
// otherwise it would be
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

impl Wake for Task {
    fn wake(self: Arc<Self>) {
        let mut lock = self.task_state.lock().unwrap();
        if replace_state_locked(&mut lock, TaskState::IDLE, TaskState::QUEUED) {
            drop(lock);
            self.enqueue_self();
            return;
        }
        if replace_state_locked(&mut lock, TaskState::RUNNING, TaskState::ALERTED) {
            return;
        }

        panic!("attempted to awake task that is not RUNNING, nor IDLE.");

        // NOTE: no special case for ALERTED cause once its in the queue both this and the
        // waker in the other thread agree, it should be polled thus no conflict.
    }
    fn wake_by_ref(self: &Arc<Self>) {
        self.clone().wake();
    }
}

#[derive(PartialEq)]
pub enum TaskState {
    QUEUED,  // in the queue, waiting to be polled
    RUNNING, // actively running, not allowed to push to queue, change status to ALERTED to queue
    ALERTED, // has been awoken while RUNNING, when the poll ends, push to the queue
    IDLE,    //not doing anything, nor in the queue, waiting a waker to wake it
    DONE,    //finshed execution, a poll would panic. Under any circumstance do NOT poll
}
