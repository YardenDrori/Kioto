use std::{
    task::Poll::{Pending, Ready},
    thread,
    time::{Duration, Instant},
};

use kioto::block_on;

fn main() {
    // let executor_raw = Executor::new();
    // let thread = thread::spawn(move || {
    //     let mut executor = Arc::new(executor_raw);
    //     executor.update_thread();
    //     do_the_thing(&executor);
    //     do_the_thing(&executor);
    //     do_the_thing(&executor);
    //     executor.loop_tasks();
    // });
    //
    // thread.join().unwrap();

    block_on(sleep());
}

//wait 3 seconds, print the input number, wait 3 seconds, print the input num times 2
// let the rust compiler build state machine
async fn sleep() {
    println!("not done 1");

    WAIT::BEGUN(Instant::now()).await;

    println!("done 1")
}

//manual state machine -- will be the leafs of all async method stack
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

                // NOTE: no reactor yet
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
