// ladata::adt
//
//! Abstract Data Types.
//!
//! - <https://en.wikipedia.org/wiki/Abstract_data_type>
//
// WAIT:
// - associated type defaults https://github.com/rust-lang/rust/issues/29661
//   can't use this in subtraits `type Element = <Self as Collection>::Element;`

use crate::error::LadataResult as Result;

mod linked;

pub use linked::*;

/// An abstract Collection.
pub trait Collection {
    type Element;
    fn collection_is_empty(&self) -> bool;
    fn collection_len(&self) -> usize;

    // CHECK: size, element, clone
    fn collection_new() -> Self;

    fn collection_clear(&mut self) -> Result<()>;
}

/// An abstract growable (dynamically-sized) Collection.
pub trait GrowCollection: Collection {
    fn collection_with_capacity(capacity: usize) -> Self;
    fn collection_capacity(&self) -> usize;
    fn collection_set_capacity(&mut self, capacity: usize) -> Result<()>;
    //
    fn collection_remaining_capacity(&self) -> usize {
        self.collection_capacity() - self.collection_len()
    }
}

/// An abstract Array.
///
/// - <https://en.wikipedia.org/wiki/Array_(data_type)#Abstract_arrays>
pub trait Array: Collection {
    ///
    fn array_get(&mut self, index: usize) -> Result<<Self as Collection>::Element>;
    ///
    fn array_set(&mut self, index: usize, element: <Self as Collection>::Element) -> Result<()>;
}

/// An abstract ranked (multi-dimensional) Array.
pub trait RankedArray<const R: usize = 2>: Collection {
    ///
    fn ranked_array_get(
        &mut self,
        multi_index: [usize; R],
    ) -> Result<<Self as Collection>::Element>;
    ///
    fn ranked_array_set(
        &mut self,
        multi_index: [usize; R],
        element: <Self as Collection>::Element,
    ) -> Result<()>;
}

/// An abstract Stack.
pub trait Stack: Collection {
    ///
    fn stack_pop(&mut self) -> Result<<Self as Collection>::Element>;
    ///
    fn stack_push(&mut self, element: <Self as Collection>::Element) -> Result<()>;
}

/// An abstract Queue.
pub trait Queue: Collection {
    fn queue_dequeue(&mut self) -> Result<<Self as Collection>::Element>;
    fn queue_enqueue(&mut self, element: <Self as Collection>::Element) -> Result<()>;
}

/// An abstract Deque.
pub trait Deque: Collection + Queue {
    ///
    fn deque_dequeue_back(&mut self) -> Result<<Self as Collection>::Element>;
    ///
    fn deque_enqueue_front(&mut self, element: <Self as Collection>::Element) -> Result<()>;
    //
    ///
    fn deque_dequeue_front(&mut self) -> Result<<Self as Collection>::Element> {
        self.queue_dequeue()
    }
    ///
    fn deque_enqueue_back(&mut self, element: <Self as Collection>::Element) -> Result<()> {
        self.queue_enqueue(element)
    }
}

/// An abstract Priority Queue.
pub trait PriorityQueue: Collection {
    ///
    fn prio_queue_insert(&mut self, element: <Self as Collection>::Element) -> Result<()>;
    /// Pulls the element with the highest priority.
    fn prio_queue_pull_highest(&mut self) -> Result<<Self as Collection>::Element>;
}
