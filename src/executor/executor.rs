use std::{
    sync::{Arc, Mutex},
    task::{
        Context,
        Poll::{self},
        Waker,
    },
    thread::{self, park},
};

use crate::waker::queued_task::{Task, TaskState, replace_state_locked, set_state_locked};

pub struct Executor {
    // TODO: change data structure
    pub task_queue: Mutex<Vec<Arc<Task>>>,

    // BUG: issue of creator of the executor not updating thread when moving executor to its own thread
    thread: thread::Thread,
    // TODO: refs to other executors for work stealing
}

impl Executor {
    pub fn new() -> Self {
        println!("thread id registered: {:?}", thread::current().id());
        return Executor {
            task_queue: Mutex::new(Vec::new()),
            thread: thread::current(), //temp, might change in future
        };
    }

    fn wake(&self) {
        println!("goood morning");

        println!("thread id awoken: {:?}", thread::current().id());
        self.thread.unpark();
    }

    pub fn loop_tasks(self: Arc<Self>) {
        //PERF: this field exists to avoid locking the queue twice per poll
        let mut task_to_queue: Option<Arc<Task>> = None;

        loop {
            self.poll_task(&mut task_to_queue);
        }
        // TODO: add exit condition
    }

    pub fn push_task(&self, task: Arc<Task>) {
        let mut tasks = self.task_queue.lock().expect("lock is poisoned");
        tasks.push(task);
        self.wake();
    }

    fn pop_task(&self, task_to_queue: &mut Option<Arc<Task>>) -> Option<Arc<Task>> {
        if let Some(task) = Option::take(task_to_queue) {
            return Some(task);
        }

        let mut tasks = self.task_queue.lock().expect("lock is poisoned");

        tasks.pop()
    }

    fn poll_task(&self, task_to_queue_ref: &mut Option<Arc<Task>>) {
        let Some(task) = self.pop_task(task_to_queue_ref) else {
            //TODO: steal task from other executors
            println!("goood night");
            park();
            return;
        };

        //i know we need this for 100% but I cant fluently explain why
        //the only reason i can think of is if, somehow, two executor threads owned the same task
        //and wanted to poll it at the same time, this blocks against that but A) how would that
        //even happen and B) is that it?
        if !task.replace_state(TaskState::QUEUED, TaskState::RUNNING) {
            return;
        }

        let old_state = task.swap_state(TaskState::RUNNING);
        debug_assert!(
            old_state == TaskState::QUEUED,
            "Queue had a task in a non queued status"
        );

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
                    *task_to_queue_ref = Some(task);
                } else {
                    set_state_locked(&mut lock, TaskState::IDLE);
                }
            }
        }
    }
}
