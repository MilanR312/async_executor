use core::{
    fmt::Debug,
    task::{LocalWaker, RawWaker, RawWakerVTable},
};

use crate::{
    mpsc::Sender,
    task::{Task, TaskId},
};

pub struct TaskWaker<const N: usize> {
    task_id: TaskId,
    scheduler: Sender<TaskId, N>,
}
impl<const N: usize> Debug for TaskWaker<N> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("TaskWaker")
            .field("task_id", &self.task_id)
            .finish()
    }
}

fn raw_waker<const N: usize>(waker: &TaskWaker<N>) -> RawWaker {
    unsafe fn clone_waker<const N: usize>(waker: *const ()) -> RawWaker {
        raw_waker(&*(waker as *const TaskWaker<N>))
    }
    unsafe fn wake<const N: usize>(waker: *const ()) {
        let waker = &*(waker as *const TaskWaker<N>);
        waker.scheduler.send(waker.task_id);
    }
    unsafe fn wake_by_ref<const N: usize>(waker: *const ()) {
        let waker = &*(waker as *const TaskWaker<N>);
        waker.scheduler.send(waker.task_id);
    }
    unsafe fn drop_waker<const N: usize>(waker: *const ()) {
        // the waker is a & so no drop is needed
    }

    RawWaker::new(
        waker as *const _ as *const (),
        &RawWakerVTable::new(
            clone_waker::<N>,
            wake::<N>,
            wake_by_ref::<N>,
            drop_waker::<N>,
        ),
    )
}
/// the caller must guarantee that the LocalWaker never outlives the TaskWaker object
pub unsafe fn waker<const N: usize>(waker: &TaskWaker<N>) -> LocalWaker {
    unsafe { LocalWaker::from_raw(raw_waker(waker)) }
}

impl<const N: usize> TaskWaker<N> {
    pub(crate) fn new(task_id: TaskId, scheduler: Sender<TaskId, N>) -> Self {
        Self { task_id, scheduler }
    }

    pub(crate) fn wake_task(&self) {
        self.scheduler
            .send(self.task_id)
            .expect("executor is not alive");
    }
    pub(crate) unsafe fn waker(&self) -> LocalWaker {
        waker(self)
    }
}
