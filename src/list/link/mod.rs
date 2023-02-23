// ladata::list::link
//
//! Linked lists.
//

#[cfg(test)]
mod tests;

mod doubly;
pub use doubly::{DoublyLinked16, DoublyLinked32, DoublyLinked8};

/// A growable doubly linked list, re-exported from [`std`].
///
#[cfg(feature = "std")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "std")))]
pub use std::collections::LinkedList as GrowDoublyLinked;
