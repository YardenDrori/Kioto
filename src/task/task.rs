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
    Queued = 0,
    Running = 1,
    Alerted = 2,
    Idle = 3,
    Done = 4,
}

impl From<TaskState> for u8 {
    fn from(value: TaskState) -> Self {
        value as u8
    }
}

impl From<u8> for TaskState {
    fn from(v: u8) -> Self {
        match v {
            0 => Self::Queued,
            1 => Self::Running,
            2 => Self::Alerted,
            3 => Self::Idle,
            4 => Self::Done,
            _ => unreachable!("invalid task state {v}"),
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
        AtomicU8::swap(&self.state, new_state.into(), std::sync::atomic::Ordering::AcqRel).into()
    }

    pub fn set_state(&self, new_state: TaskState) {
        AtomicU8::store(&self.state, new_state.into(), std::sync::atomic::Ordering::Release);
    }

    pub fn update_state(&self, func: impl FnMut(u8) -> u8) {
        AtomicU8::update(
            &self.state,
            std::sync::atomic::Ordering::AcqRel,
            std::sync::atomic::Ordering::Acquire,
            func,
        );
    }

    pub fn try_update_state(&self, func: impl FnMut(u8) -> Option<u8>) -> Result<TaskState, TaskState> {
        match AtomicU8::try_update(
            &self.state,
            std::sync::atomic::Ordering::AcqRel,
            std::sync::atomic::Ordering::Acquire,
            func,
        ) {
            Ok(val) => return Ok(val.into()),
            Err(val) => return Err(val.into()),
        }
    }
}
