// ladata::list::stack
//
//! Stacks ar linear lists for which all accesses are made from one end.
//!
//! <https://en.wikipedia.org/wiki/Stack_(abstract_data_type)>
//

use crate::mem::{Array, Storage};

#[cfg(feature = "std")]
use crate::mem::Boxed;

mod methods;
mod std_impls;

/// A Stack, backed by an [`Array`].
pub struct ArrayStack<T, S: Storage, const CAP: usize> {
    pub(crate) array: Array<T, S, CAP>,
    pub(crate) len: usize,
}

// TODO
// /// A Stack, backed by a [`Vec`].
// pub struct VecStack<T> {
//     vec: Vec<T>,
// }

/// An [`ArrayStack`] stored in the stack.
pub type Stack<T, const CAP: usize> = ArrayStack<T, (), CAP>;

/// An [`ArrayStack`] stored in the heap.
#[cfg(feature = "std")]
pub type BoxedStack<T, const CAP: usize> = ArrayStack<T, Boxed, CAP>;
