use std::{
    ops::{Deref, DerefMut},
    sync::{Arc, Mutex},
    task::{
        Context,
        Poll::{self, Ready},
        Waker,
    },
};

use crate::executor::{
    task::{Task, TaskState},
    waker::MyWaker,
};

pub struct Executor {
    //Mutex<Vec<...>> - the queue can live in multiple threads for stealing work thus we make it as such
    //Arc<Mutex<Task>> - task can live in multiple threads because of select!() for instance read io and timer (for a
    //timeout of the request) both running in paralell thus seperate threads while also being part of
    //the same task
    pub tasks_sct: Mutex<Vec<Arc<Task>>>,
}

impl Executor {
    pub fn register_task(self: &Self, task: Arc<Task>) {
        let mut task_state = task.task_state.lock().unwrap();
        *task_state = TaskState::QUEUED;
        drop(task_state);

        let mut tasks = self.tasks_sct.lock().expect("lock is poisoned");
        tasks.push(task);
    }

    fn new() -> Arc<Self> {
        return Arc::new(Executor {
            tasks_sct: Mutex::new(Vec::new()),
        });
    }

    fn poll_task(self: &Self) {
        //lock the queue
        let mut tasks = self.tasks_sct.lock().expect("lock is poisoned");
        let Some(mut task) = tasks.pop() else {
            return;
        };
        //release the queue
        drop(tasks);

        //PERF: inefficient atomic operation
        let waker = Waker::from(task.clone());

        //lock the task
        let mut task_state = task.task_state.lock().unwrap();
        *task_state = TaskState::PENDING;
        drop(task_state);

        if matches!(
            task.future
                .lock()
                .unwrap()
                .as_mut()
                .poll(&mut Context::from_waker(&waker)),
            Poll::Ready(_)
        ) {
            let mut task_state = task.task_state.lock().unwrap();
            *task_state = TaskState::DONE;
            drop(task_state);
        }
        //release the lock
        //if a waker wanted to call wake while for this task it would NOW be able to do so
        //PERF: this blocks a worker thread for potentially the entire poll duration
    }
}
