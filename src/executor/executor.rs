use std::{
    future::Pending,
    sync::{Arc, Mutex, RwLock},
    task::{Context, Poll::Ready, Waker},
};

use crate::{
    executor::{
        self,
        task::{TaskEnumAttempt, TaskStructAttempt, TaskStructAttemptThree},
    },
    waker,
};

pub struct Executor<T> {
    pub tasks_sct: Mutex<Vec<TaskStructAttemptThree<T>>>,
    // pub tasks_enm: Mutex<Vec<TaskEnumAttempt<'a, T>>>,

    //another attempt instead of storing the task in waker we keep it here cause waker CAN reach this
    //slow, inefficient, inelegant
    //could convert to hashset, still not ideal tho
    pub inactive_tasks: Mutex<Vec<TaskStructAttemptThree<T>>>,
}

impl<T> Executor<T> {
    fn new() -> Arc<Self> {
        return Arc::new(Executor {
            tasks_sct: Mutex::new(Vec::new()),
            // tasks_enm: Mutex::new(Vec::new()),
            inactive_tasks: Mutex::new(Vec::new()),
        });
    }

    //gets called in a loop
    fn loop_iter(self: &Self) {
        // version 2: failed
        let mut tasks_sct = self.tasks_sct.lock().expect("lock is poisoned");
        let Some(mut task_sct) = tasks_sct.pop() else {
            return;
        };
        drop(tasks_sct); //free the mutex as soon as possible for other threads

        //version 1: failed
        // let mut tasks_enm = self.tasks_enm.lock().expect("lock is poisoned");
        // let Some(mut task_enm) = tasks_enm.pop() else {
        //     return;
        // };
        // drop(tasks_enm); //free the mutex as soon as possible for other threads

        //build the context, isnt it wasteful we build it every time? what if it returned pending?
        //theres no need for a new context, even if it returned success the waker does not change
        //which makes it feel realy wasteful
        //poll the task
        match task_sct.future.as_mut().poll(&mut Context::from_waker(Waker::noop())) {
            Ready(_) => {
                // the loop shouldnt actualy return anything right? it just runs the async code
                // that does lead me to question tho if a function has an async io where
                // does it get the result of that io? we dont have a place to return anything
                return;
            }
            _ => {
                //if the task is still pending we dont do anything the context and thust the waker
                //has already been sent via poll
                return;
            } //both branches doing the same thing is telling me i fucked up BIG somewhere
        }

        //failed attempt to do it like tokio
        // match tasks_enm {
        //     TaskEnumAttempt::Pending(future) => match future.as_mut().poll(&mut Context::from_waker(Waker::noop())) {
        //         Pending => {
        //             todo!()
        //         }
        //         Ready(_) => {
        //             todo!()
        //         }
        //     },
        // }
    }
}
