// ladata::line::link
//
//! Linked lists.
//

#[cfg(test)]
mod tests;

mod builder;
pub use builder::{LinkedList16, LinkedList32, LinkedList8};

/// A doubly linked list, re-exported from [`std`].
#[cfg(feature = "std")]
pub use std::collections::LinkedList as DoublyLinkedList;
