use core::{marker::PhantomData, ptr::NonNull, cell::RefCell};



pub struct Channel<T, const N: usize>{
    data: RefCell<heapless::Deque<T, N>>,
}
impl<const N: usize, T> Channel<T, N>{
    pub const fn new() -> Self {
        Self {
            data: RefCell::new(heapless::Deque::new())
        }
    }
    pub fn recv(&self) -> Option<T>{
        self.data.borrow_mut().pop_front()
    }
    pub fn sender(&self) -> Sender<T, N>{
        Sender {
            ch: NonNull::new(self as *const Channel<T, N> as *mut _).unwrap()
        }
    }
}
pub struct Sender<T, const N: usize>{
    ch: NonNull<Channel<T, N>>
}
impl<T, const N: usize> Clone for Sender<T, N>{
    fn clone(&self) -> Self {
        Self {
            ch: self.ch
        }
    }
}
impl<T, const N: usize> Sender<T, N>{
    pub fn send(&self, data: T) -> Result<(), T>{
        // SAFETY: a Sender is created by the Channel and
        // remains valid since the channel is contained by the Executor struct
        let channel = unsafe {
            self.ch.as_ref()
        };
        channel.data.borrow_mut().push_back(data)
    }
}