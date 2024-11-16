use alloc::boxed::Box;
use core::{future::Future, pin::Pin, task::Context, task::Poll};

pub mod simple_executor;

pub struct Task {
    future: Pin<Box<dyn Future<Output = ()>>>,
}

impl Task {
    pub fn new(future: impl Future<Output = ()> + 'static) -> Task {
        Task {
            future: Box::pin(future),
        }
    }

    fn poll(&mut self, context: &mut Context) -> Poll<()> {
        // as_mut: Pin<Box<Future>> -> Pin<&mut Future>
        self.future.as_mut().poll(context)
    }
}
