use std::{
    pin::Pin,
    sync::{Arc, Mutex, atomic::AtomicU8},
    task::{Context, Poll, Waker},
};

pub struct Task {
    future: Mutex<Pin<Box<dyn Future<Output = ()> + Send>>>, //this is send to move the task
    state: AtomicU8,
}

#[repr(u8)]
#[derive(PartialEq)]
pub enum TaskState {
    QUEUED = 0,
    RUNNING = 1,
    ALERTED = 2,
    IDLE = 3,
    DONE = 4,
}

impl From<TaskState> for u8 {
    fn from(value: TaskState) -> Self {
        value as u8
    }
}

impl From<u8> for TaskState {
    fn from(value: u8) -> Self {
        if value <= 4 {
            unsafe { std::mem::transmute::<u8, TaskState>(value) }
        } else {
            panic!("invalid state value");
        }
    }
}

impl Task {
    fn new(fut: impl Future<Output = ()> + 'static + Send, state: TaskState) -> Arc<Self> {
        let dyn_fut: Pin<Box<dyn Future<Output = ()> + Send>> = Box::pin(fut);
        Arc::new(Task {
            future: Mutex::new(dyn_fut),
            state: AtomicU8::new(state.into()),
        })
    }

    pub fn waker(&self) -> Waker {
        todo!()
    }

    pub fn poll(&self, cx: &mut Context) -> Poll<()> {
        self.future.lock().unwrap().as_mut().poll(cx)
    }

    pub fn compare_exchange_state(&self, old_state: TaskState, new_state: TaskState) -> bool {
        AtomicU8::compare_exchange(
            &self.state,
            old_state.into(),
            new_state.into(),
            std::sync::atomic::Ordering::AcqRel,
            std::sync::atomic::Ordering::Acquire,
        )
        .is_ok()
    }

    pub fn swap_state(&self, new_state: TaskState) -> TaskState {
        AtomicU8::swap(&self.state, new_state.into(), std::sync::atomic::Ordering::Release).into()
    }

    pub fn set_state(&self, new_state: TaskState) {
        AtomicU8::store(&self.state, new_state.into(), std::sync::atomic::Ordering::Release);
    }

    pub fn update_state(&self, func: impl FnMut(u8) -> u8) {
        AtomicU8::update(
            &self.state,
            std::sync::atomic::Ordering::Release,
            std::sync::atomic::Ordering::Acquire,
            func,
        );
    }

    pub fn try_update_state(&self, func: impl FnMut(u8) -> Option<u8>) {
        AtomicU8::try_update(
            &self.state,
            std::sync::atomic::Ordering::Release,
            std::sync::atomic::Ordering::Acquire,
            func,
        );
    }
}
