pub struct Sender<T> {
    // TODO
}

pub struct Receiver<T> {
    // TODO
}

pub mod error {

    use std::fmt;

    #[derive(Debug)]
    pub struct SendError<T>(pub T);

    impl<T> fmt::Display for SendError<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "channel closed")
        }
    }

    impl<T: fmt::Debug> std::error::Error for SendError<T> {}

    #[derive(Debug, PartialEq, Eq, Clone)]
    pub enum RecvError {
        Closed,
        Lagged(u64),
    }

    impl fmt::Display for RecvError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                RecvError::Closed => write!(f, "channel closed"),
                RecvError::Lagged(amt) => write!(f, "channel lagged by {}", amt),
            }
        }
    }

    impl std::error::Error for RecvError {}

    #[derive(Debug, PartialEq, Eq, Clone)]
    pub enum TryRecvError {
        Empty,
        Closed,
        Lagged(u64),
    }

    impl fmt::Display for TryRecvError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                TryRecvError::Empty => write!(f, "channel empty"),
                TryRecvError::Closed => write!(f, "channel closed"),
                TryRecvError::Lagged(amt) => write!(f, "channel lagged by {}", amt),
            }
        }
    }

    impl std::error::Error for TryRecvError {}
}

use self::error::{RecvError, SendError, TryRecvError};

struct Shared<T> {
    // TODO
}

const MAX_RECEIVERS: usize = usize::MAX >> 2;

#[track_caller]
pub fn channel<T: Clone>(capacity: usize) -> (Sender<T>, Receiver<T>) {
    todo!();
}

unsafe impl<T: Send> Send for Sender<T> {}
unsafe impl<T: Send> Sync for Sender<T> {}

unsafe impl<T: Send> Send for Receiver<T> {}
unsafe impl<T: Send> Sync for Receiver<T> {}

impl<T> Sender<T> {
    #[track_caller]
    pub fn new(capacity: usize) -> Self {
        todo!();
    }

    pub fn send(&self, value: T) -> Result<usize, SendError<T>> {
        todo!();
    }

    pub fn subscribe(&self) -> Receiver<T> {
        todo!();
    }

    pub fn len(&self) -> usize {
        todo!();
    }

    pub fn is_empty(&self) -> bool {
        todo!();
    }

    pub fn receiver_count(&self) -> usize {
        todo!();
    }

    pub fn same_channel(&self, other: &Self) -> bool {
        todo!();
    }
}

impl<T> Clone for Sender<T> {
    fn clone(&self) -> Self {
        todo!();
    }
}

impl<T> Drop for Sender<T> {
    fn drop(&mut self) {
        todo!();
    }
}

impl<T> Receiver<T> {
    pub fn len(&self) -> usize {
        todo!();
    }

    pub fn is_empty(&self) -> bool {
        todo!();
    }

    pub fn same_channel(&self, other: &Self) -> bool {
        todo!();
    }
}

impl<T: Clone> Receiver<T> {
    pub fn resubscribe(&self) -> Self {
        todo!();
    }

    pub async fn recv(&mut self) -> Result<T, RecvError> {
        todo!()
    }

    pub fn try_recv(&mut self) -> Result<T, TryRecvError> {
        todo!()
    }

    pub fn blocking_recv(&mut self) -> Result<T, RecvError> {
        todo!()
    }
}

fn is_unpin<T: Unpin>() {}

#[cfg(not(loom))]
#[cfg(test)]
mod tests {
    use super::*;

    //TODO
}
