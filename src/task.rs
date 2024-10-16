use core::{fmt::Debug, future::Future, pin::{pin, Pin}, sync::atomic::AtomicU64, task::{Context, Poll}};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub(crate) struct TaskId(u64);

impl TaskId {
    pub(crate)  fn new() -> Self {
        static NEXT_ID: AtomicU64 = AtomicU64::new(0);
        Self(NEXT_ID.fetch_add(1, core::sync::atomic::Ordering::Relaxed))
    }
}



pub(crate) struct Task<'a>{
    fut: &'a mut dyn Future<Output = ()>,
    id: TaskId
}
impl<'a> Debug for Task<'a> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Task")
            .field("id", &self.id)
            .finish()
    }
}
impl<'a> Task<'a>{
    pub fn new(fut: &'a mut dyn Future<Output = ()>) -> Self {
        Self {
            fut,
            id: TaskId::new()
        }
    }
    pub fn id(&self) -> TaskId {
        self.id
    }
    pub(crate) fn poll(&mut self, cx: &mut Context) -> Poll<()>{
        // SAFETY:
        // this struct contains the only &mut reference to the future so it should not be able to move
        let fut = unsafe { Pin::new_unchecked(&mut*self.fut)};
        fut.poll(cx)
    }
}