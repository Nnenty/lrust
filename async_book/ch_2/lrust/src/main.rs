pub use own_future_realizaiton::{Poll, SimpleFuture};
pub use simultaneous_calls::Join;
pub use timer::{SharedState, TimerFuture};

// Module 'own_future_realizaiton' description:
// We create own simple Future trait and struct pseudo-Socket,
// that implement Future trait, where 'fn pool(...) -> ...'
// return 'Pool::Ready' if socket has data to read,
// otherwise it does its best to wake the socket and returns 'Pool::Pending'
mod own_future_realizaiton {
    pub trait SimpleFuture {
        type Output;
        fn poll(&mut self, wake: fn()) -> Poll<Self::Output>;
    }
    pub enum Poll<'a, T> {
        Ready(&'a T),
        Pending,
    }
    pub struct Socket;
    impl Socket {
        pub fn has_data_to_read(&self) -> bool {
            false
        }
    }
    pub struct SocketRead {
        socket: Socket,
    }
    impl SocketRead {
        pub fn set_readable_callback(&self, f: fn()) {}
    }

    impl SimpleFuture for SocketRead {
        type Output = Socket;

        fn poll(&mut self, wake: fn()) -> Poll<Self::Output> {
            if self.socket.has_data_to_read() {
                Poll::Ready(&self.socket)
            } else {
                self.set_readable_callback(test_run);
                Poll::Pending
            }
        }
    }

    fn test_run() {}
}

// Module 'simultaneous_calls' description:
// We create struct Join, that implement 'SimpleFuture' trait
// from module 'own_future_realizaiton'. 'fn pool(...) -> ...'
// takes 'a' if 'a' have some data and ready,
// takes 'b' if 'b' have some data and ready,
// returns 'Pool::Ready' if futures was completed or
// returns 'Pool::Pending'.
// They will call `wake()` when progress can be made.
mod simultaneous_calls {
    use super::*;
    pub struct Join<FutureA, FutureB> {
        // Each field may contain a future that should be run to completion.
        // If the future has already completed, the field is set to `None`.
        // This prevents us from polling a future after it has completed, which
        // would violate the contract of the `Future` trait.
        a: Option<FutureA>,
        b: Option<FutureB>,
    }

    impl<FutureA, FutureB> SimpleFuture for Join<FutureA, FutureB>
    where
        FutureA: SimpleFuture<Output = ()>,
        FutureB: SimpleFuture<Output = ()>,
    {
        type Output = ();
        fn poll(&mut self, wake: fn()) -> Poll<Self::Output> {
            // Attempt to complete future `a`.
            if let Some(a) = &mut self.a {
                if let Poll::Ready(()) = a.poll(wake) {
                    self.a.take();
                }
            }

            // Attempt to complete future `b`.
            if let Some(b) = &mut self.b {
                if let Poll::Ready(()) = b.poll(wake) {
                    self.b.take();
                }
            }

            if self.a.is_none() && self.b.is_none() {
                // Both futures have completed -- we can return successfully
                Poll::Ready(&())
            } else {
                // One or both futures returned `Poll::Pending` and still have
                // work to do. They will call `wake()` when progress can be made.
                Poll::Pending
            }
        }
    }
}
mod alternate_calls {
    use super::*;
    pub struct AndThenFut<FutureA, FutureB> {
        first: Option<FutureA>,
        second: FutureB,
    }

    impl<FutureA, FutureB> SimpleFuture for AndThenFut<FutureA, FutureB>
    where
        FutureA: SimpleFuture<Output = ()>,
        FutureB: SimpleFuture<Output = ()>,
    {
        type Output = ();
        fn poll(&mut self, wake: fn()) -> Poll<Self::Output> {
            if let Some(first) = &mut self.first {
                match first.poll(wake) {
                    // We've completed the first future -- remove it and start on
                    // the second!
                    Poll::Ready(()) => self.first.take(),
                    // We couldn't yet complete the first future.
                    Poll::Pending => return Poll::Pending,
                };
            }
            // Now that the first future is done, attempt to complete the second.
            self.second.poll(wake)
        }
    }
}
mod timer {
    use super::*;

    use futures::{
        executor,
        future::{BoxFuture, FutureExt},
        task::{waker_ref, ArcWake},
    };
    use std::{
        future::Future,
        pin::Pin,
        sync::{
            mpsc::{sync_channel, Receiver, SyncSender},
            Arc, Mutex,
        },
        task::{Context, Poll, Waker},
        thread,
        time::Duration,
    };

    pub struct TimerFuture {
        shared_state: Arc<Mutex<SharedState>>,
    }
    pub struct SharedState {
        completed: bool,

        waker: Option<Waker>,
    }

    impl Future for TimerFuture {
        type Output = ();

        fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
            let mut shared_state = self.shared_state.lock().unwrap();

            if shared_state.completed {
                Poll::Ready(())
            } else {
                shared_state.waker = Some(cx.waker().clone());
                Poll::Pending
            }
        }
    }

    impl TimerFuture {
        pub fn new(duration: Duration) -> Self {
            let shared_state = Arc::new(Mutex::new(SharedState {
                completed: false,
                waker: None,
            }));

            let thread_shared_state = shared_state.clone();
            thread::spawn(move || {
                thread::sleep(duration);

                let mut shared_state = thread_shared_state.lock().unwrap();
                shared_state.completed = true;

                if let Some(waker) = shared_state.waker.take() {
                    waker.wake();
                }
            });

            Self { shared_state }
        }
    }
    pub struct Task {
        future: Mutex<Option<BoxFuture<'static, ()>>>,
        task_sender: SyncSender<Arc<Task>>,
    }

    #[derive(Clone)]
    pub struct Spawner {
        task_sender: SyncSender<Arc<Task>>,
    }
    pub struct Executor {
        reciever: Receiver<Arc<Task>>,
    }

    pub fn new_executor_and_spawner() -> (Executor, Spawner) {
        const MAX_QUEUED_TASKS: usize = 10_000;

        let (spawner, executor) = sync_channel(MAX_QUEUED_TASKS);

        (
            Executor { reciever: executor },
            Spawner {
                task_sender: spawner,
            },
        )
    }
}
fn main() {
    // Modules in this program:
    //
    // 1. own_future_realizaiton
    //
    // 2. simultaneous_calls
    //
    // 3. timer
}
