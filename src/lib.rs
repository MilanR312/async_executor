#![feature(local_waker)]
#![feature(noop_waker)]
#![no_std]

use core::{future::Future, task::Poll};

use executor::Executor;

pub mod executor;
mod mpsc;
mod task;
mod waker;
