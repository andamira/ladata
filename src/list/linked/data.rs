// ladata::list::link::adt
//
//!
//

use crate::{error::LadataResult as Result, misc::DataCollection};

/// An abstract Singly Linked List.
pub trait DataSinglyLinkedList: DataCollection {
    type Node: DataSinglyLinkedListNode<Data = Self::Element>;

    ///
    fn linked_first(&mut self) -> Result<Self::Node>;

    ///
    fn linked_insert_after(
        &mut self,
        node: &mut Self::Node,
        element: Self::Element,
    ) -> Result<Self::Node>;
    ///
    fn linked_remove_after(&mut self, node: &mut Self::Node) -> Result<Option<Self::Node>>;
}

/// An abstract Singly Linked List Node.
pub trait DataSinglyLinkedListNode: Sized {
    type Data;
    ///
    fn linked_data(&self) -> &Self::Data;
    ///
    fn linked_data_mut(&mut self) -> &mut Self::Data;

    /// Returns a shared reference to the next node.
    fn linked_next(&self) -> Option<&Self>;
    /// Returns an exclusive reference to the next node.
    fn linked_next_mut(&mut self) -> Option<&mut Self>;
}

/* doubly */

/// An abstract Doubly Linked List.
pub trait DataDoublyLinkedList: DataSinglyLinkedList {
    ///
    fn linked_last(&mut self) -> Result<Option<Self::Node>>;

    ///
    fn linked_insert_before(
        &mut self,
        node: &mut Self::Node,
        element: Self::Element,
    ) -> Result<Self::Node>;
    ///
    fn linked_remove_before(&mut self, node: &mut Self::Node) -> Result<Option<Self::Node>>;
}

/// An abstract Doubly Linked List Node.
pub trait DataDoublyLinkedListNode: DataSinglyLinkedListNode {
    /// Returns a shared reference to the previous node.
    fn linked_prev(&self) -> Option<&Self>;
    /// Returns an exclusive reference to the previous node.
    fn linked_prev_mut(&mut self) -> Option<&mut Self>;
}

/* thirdly */

// /// An abstract Thirdly Linked List.
// pub trait DataThirdlyLinkedList: DataDoublyLinkedList {
//     fn linked_parent(&self) -> Result<Self::Node>;
// }
//
// /// An abstract Thirdly Linked List Node.
// pub trait DataThirdlyLinkedListNode: DataDoublyLinkedListNode {
//     /// Returns a shared reference to the parent node.
//     fn linked_parent(&self) -> Option<&Self>;
//     /// Returns an excusive reference to the parent node.
//     fn linked_set_parent(&mut self, parent: Option<Self>);
// }
