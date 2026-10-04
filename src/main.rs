use std::{
    pin::Pin,
    sync::{Arc, Mutex},
    task::Poll::{Pending, Ready},
    thread,
    time::{Duration, Instant},
};

use crate::executor::{executor::Executor, task::queue_new_task};
pub mod executor;

fn main() {
    let executor = Executor::new();

    do_the_thing(&executor);
    do_the_thing(&executor);
    do_the_thing(&executor);

    let thread = thread::spawn(move || {
        executor.loop_tasks();
    });

    thread.join().unwrap();
}

fn do_the_thing(executor: &Arc<Executor>) {
    let thing: Pin<Box<dyn Future<Output = ()> + Send>> = Box::pin(sleep());
    let fut = Mutex::new(thing);
    queue_new_task(fut, &executor);
}

//wait 3 seconds, print the input number, wait 3 seconds, print the input num times 2
async fn sleep() {
    println!("not done 1");

    WAIT::BEGUN(Instant::now()).await;

    println!("done 1")
}

enum WAIT /*3 seconds*/ {
    BEGUN(Instant),
}

impl Future for WAIT {
    type Output = ();

    fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        match *self {
            WAIT::BEGUN(start) => {
                if start.elapsed() > std::time::Duration::from_secs(3) {
                    println!("done 2");
                    return Ready(());
                }

                println!("not done 2");

                //NOTE: no reactor yet
                let cx_c = cx.waker().clone();
                thread::spawn(move || {
                    thread::sleep(Duration::from_secs(3));
                    cx_c.wake();
                });

                return Pending;
            }
        }
    }
}
