// ladata::line::queue
//
//! Queues.
//

use crate::mem::Storage;

#[cfg(feature = "std")]
use crate::mem::Boxed;

mod methods;
mod std_impls;

/// A double-ended queue, backed by an array.
///
/// A double-ended queue is a linear list for which accesses are made from both ends.
pub struct Queue<T, S: Storage, const CAP: usize> {
    array: S::Container<[T; CAP]>,
    front: usize,
    back: usize,
    len: usize,
}

/// A [`Queue`] stored in the heap.
#[cfg(feature = "std")]
pub type BoxedQueue<T, const CAP: usize> = Queue<T, Boxed, CAP>;

/// A [`Queue`] stored in the stack.
pub type RawQueue<T, const CAP: usize> = Queue<T, (), CAP>;
