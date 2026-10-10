use std::{
    pin::{self},
    sync::Arc,
    task::{Context, Poll::Ready, Waker},
    thread::park,
};

use crate::waker::blocking_waker::BlockingWaker;

pub mod executor;
pub mod waker;

pub fn block_on(fut: impl Future<Output = ()>) {
    let mut pinned_fut = pin::pin!(fut);

    let waker = Waker::from(Arc::new(BlockingWaker::new()));
    let mut cx = Context::from_waker(&waker);

    let mut res = pinned_fut.as_mut().poll(&mut cx);
    while !matches!(res, Ready(_)) {
        park();
        res = pinned_fut.as_mut().poll(&mut cx);
    }
}
