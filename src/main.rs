use std::future::Future;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Waker};
use std::time::Duration;
use std::{pin, thread, time};

use crate::waker::IdkWhatImDoing;

pub mod executor;

fn main() {
    let fuck_this_shit = is_this_just_state_for_the_future_bullshit_stuff_question_mark::new();
    println!("about to catch some Zs {:?}", time::Instant::now());
    sleep(fuck_this_shit);
    println!(
        "Ughh fuck, what time is it? (its {:?}) I fucking hate mondays.",
        time::Instant::now()
    );
}

fn sleep<Fut: Future<Output = ()> + Unpin + Clone>(mut fut: Fut) {
    let i_phone_alarm_clock = Arc::new(IdkWhatImDoing::new());
    let waker = Waker::from(i_phone_alarm_clock);
    let mut i_still_dont_get_why_we_wrap_the_waker_in_a_context = Context::from_waker(&waker);

    let pinned_fut = pin::Pin::new(&mut fut);
    match pinned_fut.poll(&mut i_still_dont_get_why_we_wrap_the_waker_in_a_context) {
        Poll::Pending => {
            thread::park();
        }
        _ => {
            todo!()
        }
    }
}

#[derive(Clone)]
pub struct is_this_just_state_for_the_future_bullshit_stuff_question_mark {
    virgin: bool,
}

impl is_this_just_state_for_the_future_bullshit_stuff_question_mark {
    fn new() -> Self {
        return Self { virgin: true };
    }
    fn have_sex(self: &mut Self) {
        self.virgin = false;
    }
}

impl Future for is_this_just_state_for_the_future_bullshit_stuff_question_mark {
    type Output = ();

    fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        let state_clone = self.get_mut();

        if state_clone.virgin {
            state_clone.have_sex();

            let fml = cx.waker().clone();
            thread::spawn(move || {
                thread::sleep(Duration::from_secs(2));
                fml.wake();
            });

            println!("pending");
            return Poll::Pending;
        }

        println!("ready");
        return Poll::Ready(());
    }
}
