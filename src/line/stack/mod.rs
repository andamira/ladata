// ladata::line::stack
//
//! Stacks.
//!
//! <https://en.wikipedia.org/wiki/Stack_(abstract_data_type)>
//

use crate::mem::Storage;

#[cfg(feature = "std")]
use crate::mem::Boxed;

mod methods;
mod std_impls;

/// A Stack, backed by an array.
///
/// A stack is a linear list for which all accesses are made from one end.
pub struct Stack<T, S: Storage, const CAP: usize> {
    array: S::Container<[T; CAP]>,
    len: usize,
}

/// A [`Stack`] stored in the heap.
#[cfg(feature = "std")]
pub type BoxedStack<T, const CAP: usize> = Stack<T, Boxed, CAP>;

/// A [`Stack`] stored in the stack.
pub type RawStack<T, const CAP: usize> = Stack<T, (), CAP>;
