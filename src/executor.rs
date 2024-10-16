use core::{future::Future, task::{ContextBuilder, LocalWaker, Poll, Waker}};

use heapless::{FnvIndexMap, Entry};

use crate::{mpsc::Channel, task::{Task, TaskId}, waker::{waker, TaskWaker}};




pub struct Executor<'a, const CAP: usize>{
    tasks: FnvIndexMap<TaskId, Task<'a>, CAP>,
    task_queue: Channel<TaskId, CAP>,
    waker_cache: FnvIndexMap<TaskId, TaskWaker<CAP>, CAP>
}
impl<'a, const CAP: usize> Executor<'a, CAP>{
    pub const fn new() -> Self {
        Self {
            task_queue: Channel::new(),
            tasks: FnvIndexMap::new(),
            waker_cache: FnvIndexMap::new()
        }
    }
    pub fn schedule(&mut self, fut: &'a mut dyn Future<Output = ()>){
        let task = Task::new(fut);
        let id = task.id();
        self.tasks.insert(id, task).unwrap();
        self.task_queue.sender().send(id).expect("executor is full");
    }
    fn run_ready_tasks(&mut self){
        let Self {
            tasks,
            task_queue,
            waker_cache
        } = self;

        while let Some(id) = task_queue.recv() {
            let task = match tasks.get_mut(&id){
                Some(task) => task,
                _ => continue
            };
            let waker = waker_cache
                .entry(id);
            let waker = match waker {
                Entry::Occupied(x) => x.into_mut(),
                Entry::Vacant(x) => {
                    let waker = TaskWaker::new(id, task_queue.sender());
                    x.insert(waker).unwrap()
                }                
            };
            let waker = unsafe { waker.waker() };
            let mut context = ContextBuilder::from_waker(&Waker::noop())
                .local_waker(&waker)
                .build();
            match task.poll(&mut context){
                Poll::Pending => {},
                Poll::Ready(x) => {
                    tasks.remove(&id);
                    waker_cache.remove(&id);
                }
            }
        }
    }
    pub fn run(&mut self){
        loop {
            if self.tasks.len() == 0 { return }
            self.run_ready_tasks();
        }
    }
}
