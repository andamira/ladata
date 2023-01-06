// ladata::list::deque
//
//! Queues.
//

use crate::{list::Array, mem::Storage};

#[cfg(feature = "std")]
use crate::mem::Boxed;

mod methods;
mod std_impls;

/// A double-ended queue, backed by an [`Array`].
///
/// A double-ended queue, specifically, since the only difference between them
/// are the restriction of methods.
///
/// is a linear list for which accesses are made from both ends.
///
/// You can get the classic Queue methods by using enqueue() and dequeue() which
/// are aliases for push_back, and pop_front() respectively.
pub struct ArrayDeque<T, S: Storage, const CAP: usize> {
    pub(crate) array: Array<T, S, CAP>,
    pub(crate) len: usize,
    pub(crate) front: usize,
    pub(crate) back: usize,
}

/// An [`ArrayDeque`] stored in the stack.
pub type Deque<T, const CAP: usize> = ArrayDeque<T, (), CAP>;

/// An [`Array`] stored in the heap.
#[cfg(feature = "std")]
pub type BoxedDeque<T, const CAP: usize> = ArrayDeque<T, Boxed, CAP>;
