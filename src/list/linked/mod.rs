// ladata::list::link
//
//! Linked lists are linear lists of linked internal nodes.
//

#[cfg(feature = "std")]
use crate::mem::Boxed;

// TEMP
// #[cfg(test)]
// mod tests;

mod adt;
pub use adt::{
    DoublyLinkedListAdt, DoublyLinkedListAdtNode, SinglyLinkedListAdt, SinglyLinkedListAdtNode,
};

mod singly;
// pub use singly::{SinglyLinkedList16, SinglyLinkedList32, SinglyLinkedList8};
pub use singly::*;

// mod doubly;
// pub use doubly::{DoublyLinkedList16, DoublyLinkedList32, DoublyLinkedList8};

/* singly aliases */

/// A [`SinglyLinkedList8`] stored in the stack.
pub type DirectSinglyLinkedList8<T, const CAP: usize> = SinglyLinkedList8<T, (), CAP>;
// /// A [`SinglyLinkedList16`] stored in the stack.
// pub type DirectSinglyLinkedList16<T, const CAP: usize> = SinglyLinkedList16<T, (), CAP>;
// /// A [`SinglyLinkedList32`] stored in the stack.
// pub type DirectSinglyLinkedList32<T, const CAP: usize> = SinglyLinkedList32<T, (), CAP>;
//
/// A [`SinglyLinkedList8`] stored in the heap.
#[cfg(feature = "std")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "std")))]
pub type BoxedSinglyLinkedList8<T, const CAP: usize> = SinglyLinkedList8<T, Boxed, CAP>;
// /// A [`SinglyLinkedList16`] stored in the heap.
// #[cfg(feature = "std")]
// #[cfg_attr(feature = "nightly", doc(cfg(feature = "std")))]
// pub type BoxedSinglyLinkedList16<T, const CAP: usize> = SinglyLinkedList16<T, Boxed, CAP>;
// /// A [`SinglyLinkedList32`] stored in the heap.
// #[cfg(feature = "std")]
// #[cfg_attr(feature = "nightly", doc(cfg(feature = "std")))]
// pub type BoxedSinglyLinkedList32<T, const CAP: usize> = SinglyLinkedList32<T, Boxed, CAP>;

/* doubly aliases */

// /// A dynamic doubly linked list, re-exported from `std`.
// ///
// #[cfg(feature = "std")]
// #[cfg_attr(feature = "nightly", doc(cfg(feature = "std")))]
// pub use std::collections::LinkedList as DynDoublyLinkedList;

// /// A [`DoublyLinkedList8`] stored in the stack.
// pub type DirectDoublyLinkedList8<T, const CAP: usize> = DoublyLinkedList8<T, (), CAP>;
// /// A [`DoublyLinkedList16`] stored in the stack.
// pub type DirectDoublyLinkedList16<T, const CAP: usize> = DoublyLinkedList16<T, (), CAP>;
// /// A [`DoublyLinkedList32`] stored in the stack.
// pub type DirectDoublyLinkedList32<T, const CAP: usize> = DoublyLinkedList32<T, (), CAP>;
//
// /// A [`DoublyLinkedList8`] stored in the heap.
// #[cfg(feature = "std")]
// #[cfg_attr(feature = "nightly", doc(cfg(feature = "std")))]
// pub type BoxedDoublyLinkedList8<T, const CAP: usize> = DoublyLinkedList8<T, Boxed, CAP>;
// /// A [`DoublyLinkedList16`] stored in the heap.
// #[cfg(feature = "std")]
// #[cfg_attr(feature = "nightly", doc(cfg(feature = "std")))]
// pub type BoxedDoublyLinkedList16<T, const CAP: usize> = DoublyLinkedList16<T, Boxed, CAP>;
// /// A [`DoublyLinkedList32`] stored in the heap.
// #[cfg(feature = "std")]
// #[cfg_attr(feature = "nightly", doc(cfg(feature = "std")))]
// pub type BoxedDoublyLinkedList32<T, const CAP: usize> = DoublyLinkedList32<T, Boxed, CAP>;
