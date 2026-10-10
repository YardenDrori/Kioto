use std::{
    sync::{Arc, Mutex, MutexGuard},
    task::{
        Context,
        Poll::{self},
        Waker,
    },
    thread::{self, current, park},
};

use crate::task::queued_task::{Task, TaskState, replace_state_locked, set_state_locked};

pub(crate) struct Executor {
    pub task_queue: Mutex<Vec<Arc<Task>>>,

    //PERF: this field exists to avoid locking the queue twice per poll
    task_to_queue: Mutex<Option<Arc<Task>>>,
    thread: thread::Thread,
    // TODO: refs to other executors
}

impl Executor {
    pub fn new() -> Self {
        println!("thread id registered: {:?}", thread::current().id());
        return Executor {
            task_queue: Mutex::new(Vec::new()),
            task_to_queue: Mutex::new(None),
            thread: thread::current(), //temp, might change in future
        };
    }

    // HACK:
    pub fn update_thread(self: &mut Arc<Self>) {
        let Some(executor) = Arc::get_mut(self) else {
            panic!("temp error msg");
        };
        executor.thread = current();
    }

    fn wake(&self) {
        println!("goood morning");

        println!("thread id awoken: {:?}", thread::current().id());
        self.thread.unpark();
    }

    pub fn loop_tasks(self: Arc<Self>) {
        // INFO: this needs to be a mutex because an executor lives as a ref in all other executors
        // for work stealing but only the executor's main thread is allowed to interact with the
        // task_to_queue variable thus we lock it only once and pass that lock
        let mut task_to_queue_lock = self.task_to_queue.lock().unwrap();

        loop {
            self.poll_task(&mut task_to_queue_lock);
        }
    }

    pub fn push_task(&self, task: Arc<Task>) {
        self.wake();
        let mut tasks = self.task_queue.lock().expect("lock is poisoned");
        tasks.push(task);
    }

    fn pop_task(&self, task_to_queue_lock: &mut MutexGuard<Option<Arc<Task>>>) -> Option<Arc<Task>> {
        if let Some(task) = Option::take(&mut **task_to_queue_lock) {
            return Some(task);
        }

        let mut tasks = self.task_queue.lock().expect("lock is poisoned");

        tasks.pop()
    }

    fn poll_task(&self, task_to_queue_lock: &mut MutexGuard<Option<Arc<Task>>>) {
        let Some(task) = self.pop_task(task_to_queue_lock) else {
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
                    **task_to_queue_lock = Some(task);
                    // self.task_to_queue = Some(task);
                } else {
                    set_state_locked(&mut lock, TaskState::IDLE);
                }
            }
        }
    }
}
