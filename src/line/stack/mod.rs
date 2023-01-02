// ladata::line::stack
//
//! LIFO Stacks.
//!
//! Logical structure:
//! - Linear list with LIFO
//! - Control parameter: Stack-top pointer
//!
//! Storage structure:
//! - Sequential stack
//! - Linked stack
//!
//! Basic operations:
//! - Init
//! - is_empty, len, remaining
//! - clear
//! - push, pop
//!
//! [w]: https://en.wikipedia.org/wiki/Stack_(abstract_data_type)
//

use crate::mem::Storage;

#[cfg(feature = "std")]
use crate::mem::Boxed;

#[cfg(test)]
mod tests;

mod methods;

mod std_impls;

/// A constant-capacity [`LIFO Stack`], implemented over an array.
///
/// A stack is a linear list for which practically all accesses are made at one end.
pub struct Stack<T, S: Storage, const CAP: usize> {
    /// The stack stored in the generic container.
    stack: S::Container<[T; CAP]>,
    /// The len index == the count of elements.
    len: usize,
}

/// A [`Stack`] stored in the heap.
#[cfg(feature = "std")]
pub type BoxedStack<T, const CAP: usize> = Stack<T, Boxed, CAP>;

/// A [`Stack`] stored in the stack.
pub type RawStack<T, const CAP: usize> = Stack<T, (), CAP>;
