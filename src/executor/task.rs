use std::{
    pin::Pin,
    task::{Context, Waker},
};

//return value must be a known size at compile time this kinda
//forces a heap allocation unless im missing something which i hope i am
//attempt 2: failed
pub struct TaskStructAttempt<'a, T> {
    pub future: Pin<Box<dyn Future<Output = Option<Box<T>>>>>,
    pub context: Context<'a>,
}

pub struct TaskStructAttemptThree<T> {
    pub future: Pin<Box<dyn Future<Output = Option<Box<T>>>>>,
}

impl<'a, T> TaskStructAttempt<'a, T> {
    fn new(future: Pin<Box<dyn Future<Output = Option<Box<T>>>>>, waker: &'a Waker) -> Self {
        Self {
            future: future,
            context: Context::from_waker(&waker),
        }
    }
}

//attempt 1: failed
pub enum TaskEnumAttempt<T> {
    Pending(Pin<Box<dyn Future<Output = Option<Box<T>>>>>),
    Done,
}
