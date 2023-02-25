// ladata::list::link
//
//! Linked lists.
//

#[cfg(feature = "std")]
use crate::mem::Boxed;

#[cfg(test)]
mod tests;

mod adt;
pub use adt::{
    DoublyLinkedListAdt, DoublyLinkedListAdtNode, SinglyLinkedListAdt, SinglyLinkedListAdtNode,
};

mod doubly;
pub use doubly::{DoublyLinkedList16, DoublyLinkedList32, DoublyLinkedList8};

/// A dynamic doubly linked list, re-exported from `std`.
///
#[cfg(feature = "std")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "std")))]
pub use std::collections::LinkedList as DynDoublyLinkedList;

/// A [`DoublyLinkedList8`] stored in the stack.
pub type DirectDoublyLinkedList8<T, const CAP: usize> = DoublyLinkedList8<T, (), CAP>;
/// A [`DoublyLinkedList16`] stored in the stack.
pub type DirectDoublyLinkedList16<T, const CAP: usize> = DoublyLinkedList16<T, (), CAP>;
/// A [`DoublyLinkedList32`] stored in the stack.
pub type DirectDoublyLinkedList32<T, const CAP: usize> = DoublyLinkedList32<T, (), CAP>;

/// A [`DoublyLinkedList8`] stored in the heap.
#[cfg(feature = "std")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "std")))]
pub type BoxedDoublyLinkedList8<T, const CAP: usize> = DoublyLinkedList8<T, Boxed, CAP>;
/// A [`DoublyLinkedList16`] stored in the heap.
#[cfg(feature = "std")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "std")))]
pub type BoxedDoublyLinkedList16<T, const CAP: usize> = DoublyLinkedList16<T, Boxed, CAP>;
/// A [`DoublyLinkedList32`] stored in the heap.
#[cfg(feature = "std")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "std")))]
pub type BoxedDoublyLinkedList32<T, const CAP: usize> = DoublyLinkedList32<T, Boxed, CAP>;
