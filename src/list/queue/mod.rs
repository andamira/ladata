// ladata::list::queue
//
//! Queues.
//

use crate::{list::Array, mem::Storage};

#[cfg(feature = "std")]
use crate::mem::Boxed;

mod methods;
mod std_impls;

/// A queue, backed by an [`Array`].
pub struct ArrayQueue<T, S: Storage, const CAP: usize> {
    pub(crate) array: Array<T, S, CAP>,
    pub(crate) len: usize,
    pub(crate) front: usize,
    pub(crate) back: usize,
}

// /// A queue, backed by a [`Vec`].
// pub struct ArrayQueue<T> {
//     vec: Vec<T>,
//     front: usize,
//     back: usize,
// }

/// An [`ArrayQueue`] stored in the stack.
pub type Queue<T, const CAP: usize> = ArrayQueue<T, (), CAP>;

/// An [`ArrayQueue`] stored in the heap.
#[cfg(feature = "std")]
pub type BoxedQueue<T, const CAP: usize> = ArrayQueue<T, Boxed, CAP>;
