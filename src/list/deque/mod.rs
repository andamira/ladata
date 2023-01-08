// ladata::list::deque
//
//! Double-ended queues are linear lists for which any accesses are made from
//! either end.
//

use crate::mem::{Array, Storage};

#[cfg(feature = "std")]
use crate::mem::Boxed;

mod methods;
mod std_impls;

/// A double-ended queue, backed by an [`Array`].
///
/// It has the [`Queue`] and [`Stack`] methods implemented for both the front
/// and the back sides.
///
/// [`Queue`]: crate::all::Queue
/// [`Stack`]: crate::all::Stack
pub struct ArrayDeque<T, S: Storage, const CAP: usize> {
    pub(crate) array: Array<T, S, CAP>,
    pub(crate) len: usize,
    pub(crate) front: usize,
    pub(crate) back: usize,
}

/// An [`ArrayDeque`] stored in the stack.
pub type Deque<T, const CAP: usize> = ArrayDeque<T, (), CAP>;

/// An [`ArrayDeque`] stored in the heap.
#[cfg(feature = "std")]
pub type BoxedDeque<T, const CAP: usize> = ArrayDeque<T, Boxed, CAP>;
