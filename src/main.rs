#![feature(local_waker)]
#![feature(noop_waker)]
#![no_std]

use core::{future::Future, task::Poll};

use executor::Executor;

mod executor;
mod log;
mod mpsc;
mod task;
mod waker;

// added for the asyncsleep future
extern crate std;

pub struct AsyncSleep {
    to_wake: std::time::Instant
}
impl Future for AsyncSleep {
    type Output = ();
    fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        let current = std::time::Instant::now();
        if self.to_wake > current {
            cx.local_waker().wake_by_ref();
            Poll::Pending
        } else {
            Poll::Ready(())
        }
    }
}
pub fn sleep(duration: core::time::Duration) -> AsyncSleep {
    let to_wake = std::time::Instant::now().checked_add(duration).unwrap();
    AsyncSleep {
        to_wake
    }
}


async fn async_number() -> u32 {
    6
}
async fn async_task() {
    let number = async_number().await;
    debug!("num1 = {number}");
    sleep(core::time::Duration::from_millis(10)).await;
    let number2 = async_number().await;
    debug!("num = {}", number + number2);
}

async fn lots_of_prints(){
    for i in 0..5 {
        debug!("hello world {i}");
        sleep(core::time::Duration::from_millis(5)).await;
    }
}



fn main() {
    let mut printer = lots_of_prints();
    let mut task = async_task();

    let mut executor = Executor::<'_, 4>::new();
    executor.schedule(&mut printer);
    executor.schedule(&mut task);
    executor.run();
}