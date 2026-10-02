use std::{
    sync::{Arc, Mutex},
    task::{
        Context,
        Poll::{self},
        Waker,
    },
};

use crate::executor::task::{Task, TaskState, replace_state_locked, set_state_locked};

pub struct Executor {
    //Mutex<Vec<...>> - the queue can live in multiple threads for stealing work thus we make it as such
    //Arc<Mutex<Task>> - task can live in multiple threads because of select!() for instance read io and timer (for a
    //timeout of the request) both running in paralell thus seperate threads while also being part of
    //the same task
    pub task_queue: Mutex<Vec<Arc<Task>>>,

    //PERF: this field exists to avoid locking the queue twice per poll
    task_to_queue: Option<Arc<Task>>,
}

impl Executor {
    fn new() -> Arc<Self> {
        return Arc::new(Executor {
            task_queue: Mutex::new(Vec::new()),
            task_to_queue: None,
        });
    }

    fn pop_task(&mut self) -> Option<Arc<Task>> {
        let mut tasks = self.task_queue.lock().expect("lock is poisoned");

        if let Some(task) = Option::take(&mut self.task_to_queue) {
            return Some(task);
        }

        tasks.pop()
    }

    pub fn register_task(self: &Self, task: Arc<Task>) {
        let mut tasks = self.task_queue.lock().expect("lock is poisoned");
        tasks.push(task);
    }

    fn poll_task(self: &mut Self) {
        let Some(task) = self.pop_task() else {
            //TODO: steal task from other executors
            return;
        };

        if !task.replace_state(TaskState::QUEUED, TaskState::RUNNING) {
            return;
        }

        let waker = Waker::from(task.clone());

        let poll = task
            .future
            .lock()
            .unwrap()
            .as_mut()
            .poll(&mut Context::from_waker(&waker));

        match poll {
            Poll::Ready(_) => {
                task.set_state(TaskState::DONE);
            }
            Poll::Pending => {
                let mut lock = task.task_state.lock().unwrap();
                if replace_state_locked(&mut lock, TaskState::ALERTED, TaskState::QUEUED) {
                    drop(lock);
                    self.task_to_queue = Some(task);
                } else {
                    set_state_locked(&mut lock, TaskState::IDLE);
                }
            }
        }
        //release the lock
    }
}
