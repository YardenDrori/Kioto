use std::{
    pin::Pin,
    task::{Context, Waker},
};

pub struct TaskStructAttemptThree<T> {
    pub future: Pin<Box<dyn Future<Output = Option<Box<T>>>>>,
}
