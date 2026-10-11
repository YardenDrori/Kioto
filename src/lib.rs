use std::{
    pin::{self},
    sync::Arc,
    task::{Context, Poll::Ready, Waker},
    thread::park,
};

use crate::waker::blocking_waker::BlockingWaker;

pub(crate) mod executor;
pub(crate) mod reactor;
pub(crate) mod waker;

///takes an async function and blocks the current thread until its future is ready
///this allows running async code in sync functions
///
///if the caller is async itself you probably wanna call await instead to resolve the future without blocking
///
///usage:
///```rust
///fn main() {
///    todo!() // TODO: add example once reactor is done
///}
///```
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
