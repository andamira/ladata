// ladata::list::link::doubly
//
//! A macro builder for linked lists backed by a const-sized array.
//
// ```diagram
// FRONT ←NODE→ ←NODE→ ←NODE→ ←NODE→ (BACK)
//
//     head                        tail
//    (FRONT)                     (BACK)
//      [0]      [1]      [2]      [3]
//    !←P,S→1  0←P,S→2  1←P,S→3  2←P,S→!
//
// each node has a successor (next), pointing towards the back,
// and a predecessor (prev), pointing towards the front.
// ```

use core::{fmt::{self, Debug}, mem::size_of};

use crate::{
    error::{LadataError as Error, LadataResult as Result},
    list::Array,
    mem::Storage,
    misc::*,
};

#[cfg(feature = "std")]
use crate::mem::Boxed;

/// Generates a doubly linked list backed by an array, with custom index size.
#[rustfmt::skip]
macro_rules! linked_list_array {
    // $name : name prefix. E.g.: DoublyLinked
    // $B : byte size
    // $b : bit size
    // $t : inner index type
    // $nmt: nonmax inner index type
    ( $name:ident, $B:literal, $b:literal, $t:ty, $nmt:ty) => { paste::paste! {

        // Node ----------------------------------------------------------------

        /// The list node.
        pub(super) struct [<$name$b Node>] <T> {
            /// The index of the previous element, towards the front of the list.
            prev: [<Index$b>],
            /// The index of the next element, towards the back of the list.
            next: [<Index$b>],
            /// The node's data.
            data: T,
        }

        impl<T: Default> Default for [<$name$b Node>]<T> {
            fn default() -> Self {
                Self {
                    data: T::default(),
                    prev: None.into(),
                    next: None.into(),
                }
            }
        }

        impl<T: Clone> Clone for [<$name$b Node>]<T> {
            #[inline]
            fn clone(&self) -> Self {
                Self {
                    prev: self.prev.clone(),
                    next: self.next.clone(),
                    data: self.data.clone(),
                }
            }
        }
        impl<T: Copy> Copy for [<$name$b Node>]<T> {}

        impl<T: Debug> Debug for [<$name$b Node>]<T> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.debug_struct(stringify![[<$name$b Node>]])
                .field("prev", &self.prev)
                .field("next", &self.next)
                .field("data", &self.data)
                .finish()
            }
        }

        #[allow(dead_code)]
        impl<T> [<$name$b Node>]<T> {
            /// Returns a new node, with `data`, and custom `prev`ious and `next` indices.
            #[inline]
            pub(super) const fn new(
                prev: [<Index$b>],
                next: [<Index$b>],
                data: T
            ) -> Self {
                Self {
                    prev,
                    next,
                    data,
                }
            }

            /// Returns an empty node, with custom `prev`ious and `next` indices.
            #[inline]
            pub(super) const fn new_unlinked(data: T) -> Self {
                Self {
                    prev: [<Index$b>]::none(),
                    next: [<Index$b>]::none(),
                    data,
                }
            }

            /// Returns a new node intended to be the new front of the list.
            /// Expects the index of the current front node.
            #[inline]
            pub(super) const fn new_front(current_front: [<Index$b>], data: T) -> Self {
                Self {
                    prev: [<Index$b>]::none(),
                    next: current_front,
                    data,
                }
            }

            /// Returns a new node intended to be the new back of the list.
            /// Expects the index of the current back node.
            #[inline]
            pub(super) const fn new_back(current_back: [<Index$b>], data: T) -> Self {
                Self {
                    prev: current_back,
                    next: [<Index$b>]::none(),
                    data,
                }
            }

            /// Returns this node's next index (towards the back).
            #[inline]
            pub(super) fn next(&self) -> [<Index$b>] {
                self.next
            }
            /// Sets this node's next index (towards the back).
            #[inline]
            pub(super) fn set_next(&mut self, index: [<Index$b>]) {
                self.next = index;
            }

            /// Returns this node's previous index (towards the front).
            #[inline]
            pub(super) fn prev(&self) -> [<Index$b>] {
                self.prev
            }
            /// Sets this node's previous index (towards the front).
            #[inline]
            pub(super) fn set_prev(&mut self, index: [<Index$b>]) {
                self.prev = index;
            }

            /// Unlinks the node, clearing both prev and next indexes.
            pub(super) fn unlink(&mut self) {
                self.next = [<Index$b>]::none();
                self.prev = [<Index$b>]::none();
            }

            /// Sets the `value` and unlinks the node,
            /// clearing both prev and next indexes.
            pub(super) fn reset(&mut self, value: T) {
                self.data = value;
                self.next = [<Index$b>]::none();
                self.prev = [<Index$b>]::none();
            }
        }

        // List ----------------------------------------------------------------

        #[doc = "A doubly linked list, backed by an [`Array`], using " $b "-bit indices."]
        ///
        #[doc = "- It has a maximum length of [`" $t "::MAX`]` -1` elements."]
        #[doc = "- An empty list has a minimum size of `3 * " $B "` bytes."]
        #[doc = "- Each element occupies `2 * " $B " + size_of::<T>()` bytes,"]
        #[doc = "plus any padding."]
        pub struct [<$name$b>]<T, S: Storage, const CAP: usize> {
            /// The current number of nodes.
            len: $nmt,
            /// The index of the current element at the front.
            front: [<Index$b>],
            /// The index of the current element at the back.
            back: [<Index$b>],
            /// The array of nodes, stored in the generic container.
            nodes: Array<[<$name$b Node>]<T>, S, CAP>,
        }

        /// impl Clone, Copy, Debug, Default…
        mod [<impls_$b>] {
            use super::*;

            // T:Clone
            impl<T: Clone, S: Storage, const CAP: usize> Clone for [<$name$b>]<T, S, CAP>
                where S::Stored<[[<$name$b Node>]<T>; CAP]>: Clone {
                fn clone(&self) -> Self {
                    Self {
                        len: self.len.clone(),
                        front: self.front.clone(),
                        back: self.back.clone(),
                        nodes: self.nodes.clone(),
                    }
                }
            }

            /// `T:Copy`
            impl<T: Copy, S: Storage, const CAP: usize> Copy for [<$name$b>]<T, S, CAP>
                where S::Stored<[[<$name$b Node>]<T>; CAP]>: Copy {}

            /// `T:Debug`
            impl<T: Debug, S: Storage, const CAP: usize> Debug for [<$name$b>]<T, S, CAP>
                where S::Stored<[[<$name$b Node>]<T>; CAP]>: Debug {
                fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    // IMPROVE
                    write!(f, "{} {{ len:{} cap:{} front:{:?} back:{:?} size_of:{} }}",
                        stringify!([<$name$b>]),
                        self.len(), CAP, self.front, self.back, size_of::<Self>())?;

                    Ok(())
                }
            }

            /// `S=(); T:Default`
            impl<T: Default, const CAP: usize> Default for [<$name$b>]<T, (), CAP>
                where [<$name$b Node>]<T>: Default
            {
                /// Returns an empty, non-circular, doubly linked list,
                /// allocated in the stack, and filled with unlinked `CAP`
                /// elements set to their default value.
                ///
                /// # Panics
                #[doc = "Panics if `CAP` is > [`" $t "::MAX`]."]
                ///
                /// # Examples
                /// ```
                /// use ladata::list::DoublyLinkedList8;
                /// let l = DoublyLinkedList8::<u8, (), 100>::default();
                /// ```
                fn default() -> Self {
                    assert![CAP < $t::MAX as usize];
                    Self {
                        nodes: Array::default(),
                        len: $nmt::new(0).unwrap(),
                        front: None.into(),
                        back: None.into(),
                    }
                }
            }

            /// `S=Boxed; T:Default`
            #[cfg(feature = "std")]
            #[cfg_attr(feature = "nightly", doc(cfg(feature = "std")))]
            impl<T: Default, const CAP: usize> Default for [<$name$b>]<T, Boxed, CAP>
                where [<$name$b Node>]<T>: Default
            {
                /// Returns an empty, non-circular, doubly linked list,
                /// allocated in the stack, and filled with unlinked `CAP`
                /// elements set to their default value.
                ///
                /// # Examples
                /// ```
                /// use ladata::all::{Boxed, DoublyLinkedList8};
                /// let l = DoublyLinkedList8::<u8, Boxed, 10>::default();
                /// ```
                ///
                /// # Panics
                #[doc = "Panics if `CAP` is > [`" $t "::MAX`]."]
                fn default() -> Self {
                    assert![CAP < $t::MAX as usize];
                    Self {
                        nodes: Array::default(),
                        len: $nmt::new(0).unwrap(),
                        front: None.into(),
                        back: None.into(),
                    }
                }
            }
        }

        /// `S=(); T:Clone`
        impl<T: Clone, const CAP: usize> [<$name$b>]<T, (), CAP> {
            /// Returns a doubly linked list, allocated in the stack,
            /// filled with `CAP` unlinked elements set to `value`.
            ///
            /// # Errors
            #[doc = "If `CAP` is >= [`" $t "::MAX`]."]
            ///
            /// # Examples
            /// ```
            /// use ladata::list::DirectDoublyLinkedList8;
            /// # fn main() -> ladata::error::LadataResult<()> {
            ///
            /// let l = DirectDoublyLinkedList8::<u8, 100>::new(0)?;
            /// # Ok(()) }
            /// ```
            pub fn new(value: T) -> Result<Self> {
                if CAP < $t::MAX as usize {
                    Ok(Self {
                        nodes: Array::<[<$name$b Node>]<T>, (), CAP>::
                            with([<$name$b Node>]::new_unlinked(value)),
                        len: $nmt::new(0).unwrap(),
                        front: None.into(),
                        back: None.into(),
                    })
                } else {
                    Err(Error::IndexOutOfBounds(CAP))
                }
            }
        }

        /// `S:Boxed + T:Clone`
        #[cfg(feature = "std")]
        #[cfg_attr(feature = "nightly", doc(cfg(feature = "std")))]
        impl<T: Clone, const CAP: usize> [<$name$b>]<T, Boxed, CAP> {
            /// Returns a doubly linked list, allocated in the heap,
            /// filled with `CAP` unlinked elements set to `value`.
            ///
            /// # Errors
            #[doc = "If `CAP` is >= [`" $t "::MAX`]."]
            ///
            /// # Examples
            /// ```
            /// use ladata::list::{BoxedDoublyLinkedList8};
            /// # fn main() -> ladata::error::LadataResult<()> {
            ///
            /// let l = BoxedDoublyLinkedList8::<u8, 100>::new(0)?;
            /// # Ok(()) }
            /// ```
            pub fn new(value: T) -> Result<Self> {
                if CAP < $t::MAX as usize {
                    Ok(Self {
                        nodes: Array::<[<$name$b Node>]<T>, Boxed, CAP>::
                            with([<$name$b Node>]::new_unlinked(value)),
                        len: $nmt::new(0).unwrap(),
                        front: None.into(),
                        back: None.into(),
                    })
                } else {
                    Err(Error::IndexOutOfBounds(CAP))
                }
            }
        }

        /// `T:Clone`
        impl<T: Clone, S: Storage, const CAP: usize> [<$name$b>]<T, S, CAP> {
            /// Resets the list, unlinking all elements and setting them to `value`.
            pub fn reset(&mut self, value: T) {
                self.len = $nmt::new(0).unwrap();
                self.front = None.into();
                self.back = None.into();
                self.reset_all_nodes(value);
            }

            /// Removes the element at the front of the array and returns it.
            ///
            /// # Errors
            /// If the list is empty.
            ///
            /// # Examples
            /// ```
            /// use ladata::list::DirectDoublyLinkedList8;
            /// # fn main() -> ladata::error::LadataResult<()> {
            ///
            /// let mut s = DirectDoublyLinkedList8::<i32, 3>::default();
            /// s.push_front(1)?;
            /// s.push_front(2)?;
            /// assert_eq![Ok(2), s.pop_front()];
            /// assert_eq![Ok(1), s.pop_front()];
            /// assert![s.pop_front().is_err()];
            /// # Ok(()) }
            /// ```
            #[cfg(feature = "safe")]
            pub fn pop_front(&mut self) -> Result<T> {
                if self.is_empty() {
                    Err(Error::NotEnoughElements(1))
                } else {
                    todo![]
                    // FIXME
                    // // get the front node
                    // let front_idx = self.front.as_usize();
                    // let front_node = self.nodes[front_idx].clone();
                    //
                    // // update the front pointer
                    // self.front = front_node.next;
                    // self.nodes[front_idx].unlink();
                    //
                    // // update the previous front node
                    // if let Some(prev_idx) = front_node.prev.as_usize() {
                    //     self.nodes[prev_idx].next = front_node.next;
                    // } else {
                    //     // this was the only element in the list
                    //     self.back = None.into();
                    // }
                    //
                    // // update the number of elements
                    // self.decrement_len()?;
                    //
                    // // return the data of the front node
                    // Ok(front_node.data)
                }
            }

            /// Removes the element at the back of the array and returns it.
            ///
            /// # Errors
            /// If the list is empty.
            ///
            /// # Examples
            /// ```
            /// use ladata::list::DirectDoublyLinkedList8;
            /// # fn main() -> ladata::error::LadataResult<()> {
            ///
            /// let mut s = DirectDoublyLinkedList8::<i32, 3>::default();
            /// s.push_front(1)?;
            /// s.push_front(2)?;
            /// assert_eq![Ok(1), s.pop_back()];
            /// assert_eq![Ok(2), s.pop_back()];
            /// assert![s.pop_back().is_err()];
            /// # Ok(()) }
            /// ```
            #[cfg(feature = "safe")]
            pub fn pop_back(&mut self) -> Result<T> {
                if self.is_empty() {
                    Err(Error::NotEnoughElements(1))
                } else {
                    todo![]
                    // // get the back node
                    // // FIXME?
                    // let back_idx = self.back.as_usize();
                    // let back_node = self.nodes[back_idx].clone();
                    //
                    // // update the back pointer
                    // self.back = back_node.prev;
                    //
                    // // unlink the back node
                    // self.nodes[back_idx].unlink();
                    //
                    // // update the next node of the previous node
                    // if let Some(next_idx) = back_node.next.as_usize() {
                    //     self.nodes[next_idx].prev = back_node.prev;
                    // } else {
                    //     // this was the only element in the list
                    //     self.front = None.into();
                    // }
                    //
                    // // update the number of elements
                    // self.decrement_len()?;
                    //
                    // // return the data of the back node
                    // Ok(back_node.data)
                }
            }
        }

        /// `*`
        impl<T, S: Storage, const CAP: usize> [<$name$b>]<T, S, CAP> {
            /// Returns the number of elements.
            ///
            /// # Examples
            /// ```
            /// use ladata::list::DirectDoublyLinkedList8;
            /// # fn main() -> ladata::error::LadataResult<()> {
            ///
            /// let mut s = DirectDoublyLinkedList8::<i32, 3>::default();
            /// s.push_front(1)?;
            /// assert_eq![1, s.len()];
            /// # Ok(()) }
            /// ```
            pub const fn len(&self) -> $t {
                self.len.get()
            }

            /// Checks if the list is empty.
            ///
            /// # Examples
            /// ```
            /// use ladata::list::DirectDoublyLinkedList8;
            ///
            /// let s = DirectDoublyLinkedList8::<i32, 5>::default();
            /// assert![s.is_empty()];
            /// ```
            pub const fn is_empty(&self) -> bool {
                self.front.is_none() && self.back.is_none()
            }

            /// Checks if the list is full.
            ///
            /// # Examples
            /// ```
            /// use ladata::list::DirectDoublyLinkedList8;
            /// # fn main() -> ladata::error::LadataResult<()> {
            ///
            /// let mut s = DirectDoublyLinkedList8::<i32, 2>::default();
            /// s.push_front(1)?;
            /// s.push_front(2)?;
            /// assert![s.is_full()];
            /// # Ok(()) }
            /// ```
            pub const fn is_full(&self) -> bool {
                self.len() as usize == CAP
            }

            /// Returns the maximum number of elements.
            ///
            /// # Examples
            /// ```
            /// use ladata::list::DirectDoublyLinkedList8;
            ///
            /// let s = DirectDoublyLinkedList8::<i32, 3>::default();
            /// assert_eq![3, s.capacity()];
            /// ```
            pub const fn capacity(&self) -> usize {
                CAP
            }

            /// Returns the available free elements.
            ///
            /// # Examples
            /// ```
            /// use ladata::list::DirectDoublyLinkedList8;
            /// # fn main() -> ladata::error::LadataResult<()> {
            ///
            /// let mut s = DirectDoublyLinkedList8::<i32, 3>::default();
            /// s.push_front(1)?;
            /// assert_eq![2, s.remaining_capacity()];
            /// # Ok(()) }
            /// ```
            pub const fn remaining_capacity(&self) -> usize {
                CAP - self.len() as usize
            }

            /// Clears the list, unlinking all values.
            ///
            /// # Examples
            /// ```
            /// use ladata::list::DirectDoublyLinkedList8;
            /// # fn main() -> ladata::error::LadataResult<()> {
            ///
            /// let mut s = DirectDoublyLinkedList8::<i32, 2>::default();
            /// s.push_front(1)?;
            /// s.push_front(2)?;
            /// s.clear();
            /// assert![s.is_empty()];
            /// # Ok(()) }
            /// ```
            pub fn clear(&mut self) {
                self.len = $nmt::new(0).unwrap();
                self.front = None.into();
                self.back = None.into();
                self.unlink_all_nodes();
            }

            /* front & back */

            /// Returns the index of the front element.
            ///
            /// # Errors
            /// If the list is empty.
            ///
            /// # Examples
            /// ```
            /// use ladata::list::DirectDoublyLinkedList8;
            /// # fn main() -> ladata::error::LadataResult<()> {
            ///
            /// let mut s = DirectDoublyLinkedList8::<i32, 3>::default();
            /// s.push_front(1)?;
            /// assert_eq![0, s.front_index()?];
            /// # Ok(()) }
            /// ```
            pub const fn front_index(&self) -> Result<$t> {
                if let Some(i) = self.front.get() {
                    Ok(i)
                } else {
                    Err(Error::NotEnoughElements(1))
                }
            }

            /// Returns the index of the back element.
            ///
            /// # Errors
            /// If the list is empty.
            ///
            /// # Examples
            /// ```
            /// use ladata::list::DirectDoublyLinkedList8;
            /// # fn main() -> ladata::error::LadataResult<()> {
            ///
            /// let mut s = DirectDoublyLinkedList8::<i32, 3>::default();
            /// s.push_front(1)?;
            /// assert_eq![0, s.back_index()?];
            /// # Ok(()) }
            /// ```
            pub const fn back_index(&self) -> Result<$t> {
                if let Some(i) = self.back.get() {
                    Ok(i)
                } else {
                    Err(Error::NotEnoughElements(1))
                }
            }

            /// Returns a shared reference to the front element.
            ///
            /// # Errors
            /// If the list is empty.
            ///
            /// # Examples
            /// ```
            /// use ladata::list::DirectDoublyLinkedList8;
            /// # fn main() -> ladata::error::LadataResult<()> {
            ///
            /// let mut s = DirectDoublyLinkedList8::<i32, 3>::default();
            /// s.push_front(1)?;
            /// assert_eq![&1, s.front()?];
            /// # Ok(()) }
            /// ```
            pub fn front(&self) -> Result<&T> {
                if self.front.is_some() {
                    Ok(&self.nodes[self.front.as_usize()].data)
                } else {
                    Err(Error::NotEnoughElements(1))
                }
            }

            /// Returns a shared reference to the back element.
            ///
            /// # Errors
            /// If the list is empty.
            ///
            /// # Examples
            /// ```
            /// use ladata::list::DirectDoublyLinkedList8;
            /// # fn main() -> ladata::error::LadataResult<()> {
            ///
            /// let mut s = DirectDoublyLinkedList8::<i32, 3>::default();
            /// s.push_front(1)?;
            /// assert_eq![&1, s.back()?];
            /// # Ok(()) }
            /// ```
            pub fn back(&self) -> Result<&T> {
                if self.back.is_some() {
                    Ok(&self.nodes[self.back.as_usize()].data)
                } else {
                    Err(Error::NotEnoughElements(1))
                }
            }

            /// Returns an exclusive reference to the front element.
            ///
            /// # Errors
            /// If the list is empty.
            ///
            /// # Examples
            /// ```
            /// use ladata::list::DirectDoublyLinkedList8;
            /// # fn main() -> ladata::error::LadataResult<()> {
            ///
            /// let mut s = DirectDoublyLinkedList8::<i32, 3>::default();
            /// s.push_front(1)?;
            /// assert_eq![&mut 1, s.front_mut()?];
            /// # Ok(()) }
            /// ```
            pub fn front_mut(&mut self) -> Result<&mut T> {
                if self.front.is_some() {
                    Ok(&mut self.nodes[self.front.as_usize()].data)
                } else {
                    Err(Error::NotEnoughElements(1))
                }
            }

            /// Returns an exclusive reference to the back element.
            ///
            /// # Errors
            /// If the list is empty.
            ///
            /// # Examples
            /// ```
            /// use ladata::list::DirectDoublyLinkedList8;
            /// # fn main() -> ladata::error::LadataResult<()> {
            ///
            /// let mut s = DirectDoublyLinkedList8::<i32, 3>::default();
            /// s.push_front(1)?;
            /// assert_eq![&mut 1, s.back_mut()?];
            /// # Ok(()) }
            /// ```
            pub fn back_mut(&mut self) -> Result<&mut T> {
                if self.back.is_some() {
                    Ok(&mut self.nodes[self.back.as_usize()].data)
                } else {
                    Err(Error::NotEnoughElements(1))
                }
            }

            /// Returns a shared reference to the element at `index`.
            ///
            /// # Errors
            /// If the index is out of bounds.
            ///
            /// # Examples
            /// ```
            /// use ladata::list::DirectDoublyLinkedList8;
            /// # fn main() -> ladata::error::LadataResult<()> {
            ///
            /// let mut s = DirectDoublyLinkedList8::<i32, 3>::default();
            /// s.push_front(1)?;
            /// assert_eq![&1, s.at(0)?];
            /// # Ok(()) }
            /// ```
            pub fn at(&self, index: $t) -> Result<&T> {
                if index < self.len.get() {
                    return Ok(&self.nodes[index as usize].data);
                }
                Err(Error::IndexOutOfBounds(index as usize))
            }

            /// Returns an exclusive reference to the element at `index`.
            ///
            /// # Errors
            /// If the index is out of bounds.
            ///
            /// # Examples
            /// ```
            /// use ladata::list::DirectDoublyLinkedList8;
            /// # fn main() -> ladata::error::LadataResult<()> {
            ///
            /// let mut s = DirectDoublyLinkedList8::<i32, 3>::default();
            /// s.push_front(1);
            /// assert_eq![&mut 1, s.at_mut(0)?];
            /// # Ok(()) }
            /// ```
            pub fn at_mut(&mut self, index: $t) -> Result<&mut T> {
                if index < self.len.get() {
                    return Ok(&mut self.nodes[index as usize].data);
                }
                Err(Error::IndexOutOfBounds(index as usize))
            }

            /// Adds an element at the front of the list and returns its index.
            ///
            /// # Errors
            /// If the list is full.
            ///
            /// # Examples
            /// ```
            /// use ladata::list::DirectDoublyLinkedList8;
            /// # fn main() -> ladata::error::LadataResult<()> {
            ///
            /// let mut s = DirectDoublyLinkedList8::<i32, 3>::default();
            /// s.push_front(1)?;
            /// assert_eq![1, s.len()];
            /// # Ok(()) }
            /// ```
            pub fn push_front(&mut self, value: T) -> Result<$t> {
                if self.is_full() {
                    Err(Error::NotEnoughSpace(Some(1)))
                } else {
                    // create the new element to put at the front.
                    let element = [<$name$b Node>]::new_front(self.front, value);

                    // find where the new element will be inserted.
                    let element_idx = self.len;

                    // the first front element is also the back element.
                    if element_idx.get() == 0 {
                        self.front = element_idx.into(); // CHECK
                        self.back = element_idx.into();
                    } else {
                        // otherwise update the previous front element
                        // self.mut_node_at(self.front)?.set_prev(&element_idx);
                        self.set_prev_at(self.front, element_idx.into())?
                    }

                    // update the number of elements.
                    self.increment_len()?;

                    // insert the new element
                    self.nodes[element_idx.get() as usize] = element;

                    // update current front element.
                    self.front = element_idx.into();

                    // return the index of the inserted element
                    Ok(self.len.get() - 1)
                }
            }

            /// Adds an element at the back of the list and returns its index.
            ///
            /// # Errors
            /// If the list is full.
            ///
            /// # Examples
            /// ```
            /// use ladata::list::DirectDoublyLinkedList8;
            /// # fn main() -> ladata::error::LadataResult<()> {
            ///
            /// let mut s = DirectDoublyLinkedList8::<i32, 3>::default();
            /// s.push_back(1)?;
            /// assert_eq![1, s.len()];
            /// # Ok(()) }
            /// ```
            pub fn push_back(&mut self, value: T) -> Result<$t> {
                if self.is_full() {
                    Err(Error::NotEnoughSpace(Some(1)))
                } else {
                    // Create the new element to put at the back
                    let element = [<$name$b Node>]::new_back(self.back, value);

                    // Find where the new element will be inserted
                    let element_idx = self.len;

                    // The first back element is also the front element
                    if element_idx.get() == 0 {
                        self.front = element_idx.into();
                        self.back = element_idx.into();
                    } else {
                        // Otherwise, update the next element of the previous back element
                        self.set_next_at(self.back, element_idx.into())?;
                    }

                    // Update the number of elements
                    self.increment_len()?;

                    // Insert the new element
                    self.nodes[element_idx.get() as usize] = element;

                    // Update the current back element
                    self.back = element_idx.into();

                    // Return the index of the inserted element
                    Ok(self.len.get() - 1)
                }
            }

            /// Removes the element at the front of the array and returns it.
            ///
            /// # Errors
            /// If the list is empty.
            ///
            /// # Examples
            /// ```
            /// use ladata::list::DirectDoublyLinkedList8;
            /// # fn main() -> ladata::error::LadataResult<()> {
            ///
            /// let mut s = DirectDoublyLinkedList8::<i32, 3>::default();
            /// s.push_front(1)?;
            /// s.push_front(2)?;
            /// assert_eq![Ok(2), s.pop_front()];
            /// assert_eq![Ok(1), s.pop_front()];
            /// assert![s.pop_front().is_err()];
            /// # Ok(()) }
            /// ```
            #[cfg(not(feature = "safe"))]
            pub fn pop_front(&mut self) -> Result<T> {
                if self.is_empty() {
                    Err(Error::NotEnoughElements(1))
                } else {
                    todo![]
                    // Get the front node
                    // FIXME
                    // let front_idx = self.front.as_usize();
                    // let front_node_ptr = &self.nodes[front_idx] as *const [<$name$b Node>]<T>;
                    //
                    // // SAFETY: we're not gonna access the value, but move it out
                    // // MOTIVATION: to not depend on T: Clone
                    // let front_node = unsafe { core::ptr::read(front_node_ptr) };
                    //
                    // // Update the front pointer
                    // self.front = front_node.next;
                    //
                    // // Update the prev node of the next node
                    // if let Some(next_idx) = front_node.next.get() {
                    //     self.nodes[next_idx as usize].prev = None.into();
                    // } else {
                    //     self.back = None.into();
                    // }
                    //
                    // // Update the number of elements
                    // self.decrement_len()?;
                    //
                    // self.nodes[front_idx].unlink();
                    //
                    // // Return the data of the front node
                    // Ok(front_node.data)
                }
            }

            /// Removes the element at the back of the array and returns it.
            ///
            /// # Errors
            /// If the list is empty.
            ///
            /// # Examples
            /// ```
            /// use ladata::list::DirectDoublyLinkedList8;
            /// # fn main() -> ladata::error::LadataResult<()> {
            ///
            /// let mut s = DirectDoublyLinkedList8::<i32, 3>::default();
            /// s.push_front(1)?;
            /// s.push_front(2)?;
            /// assert_eq![Ok(1), s.pop_back()];
            /// assert_eq![Ok(2), s.pop_back()];
            /// assert![s.pop_back().is_err()];
            /// # Ok(()) }
            /// ```
            #[cfg(not(feature = "safe"))]
            pub fn pop_back(&mut self) -> Result<T> {
                if self.is_empty() {
                    Err(Error::NotEnoughElements(1))
                } else {
                    todo![]
                    // // get the back node
                    // let back_idx = self.back.as_usize();
                    // let back_node_ptr = &self.nodes[back_idx] as *const [<$name$b Node>]<T>;
                    //
                    // // SAFETY: we're not gonna access the value, but move it out
                    // // MOTIVATION: to not depend on T: Clone
                    // let back_node = unsafe { core::ptr::read(back_node_ptr) };
                    //
                    // // update the back pointer
                    // self.back = back_node.prev;
                    //
                    // // update the next node of the previous node
                    // if let Some(next_idx) = back_node.next.get() {
                    //     self.nodes[next_idx as usize].prev = back_node.prev;
                    // } else {
                    //     self.front = None.into();
                    // }
                    //
                    // // update the number elements
                    // self.decrement_len()?;
                    //
                    // self.nodes[back_idx].unlink();
                    //
                    // // return the data of the back node
                    // Ok(back_node.data)
                }
            }
        }

        //

        /// Private utility methods
        #[allow(dead_code)]
        impl<T, S: Storage, const CAP: usize> [<$name$b>]<T, S, CAP> {
            ///
            fn increment_len(&mut self) -> Result<()> {
                if let Some(i) = self.len.get().checked_add(1) {
                    // MAYBE
                    // #[cfg(feature = "safe")]
                    { self.len = $nmt::new(i).unwrap(); }
                    // #[cfg(not(feature = "safe"))]
                    // unsafe { self.len = $nmt::new_unchecked(i); }
                    Ok(())
                } else {
                    Err(Error::NotEnoughSpace(Some(1)))
                }
            }
            ///
            fn decrement_len(&mut self) -> Result<()> {
                if let Some(i) = self.len.get().checked_sub(1) {
                    // MAYBE
                    // #[cfg(feature = "safe")]
                    { self.len = $nmt::new(i).unwrap(); }
                    // #[cfg(not(feature = "safe"))]
                    // unsafe { self.len = $nmt::new_unchecked(i); }
                    Ok(())
                } else {
                    Err(Error::NotEnoughElements(1))
                }
            }
            /// Returns a shared reference to the node at `index`,
            ///
            /// # Errors
            /// If either the index is `None`, or out of bounds.
            fn ref_node_at(&self, index: [<Index$b>]) -> Result<&[<$name$b Node>]<T>> {
                if let Some(i) = index.get() {
                    if i < self.len.get() {
                        self.nodes.get(i as usize).ok_or(Error::IndexOutOfBounds(i as usize))
                    } else {
                        return Err(Error::IndexOutOfBounds(i as usize));
                    }
                } else {
                    Err(Error::EmptyNode)
                }

                // CHECK whether there's a performance improvement by using:
                // Some(
                //     self.nodes[i as usize].data.as_ref()
                //         .unwrap_or_else(|| unsafe { hint::unreachable_unchecked() }),
                // )
            }

            /// Returns an exclusive reference to the node at `index`.
            ///
            /// # Errors
            /// If either the index is `None`, or out of bounds.
            fn mut_node_at(&mut self, index: [<Index$b>])
                -> Result<&mut [<$name$b Node>]<T>> {
                if let Some(i) = index.get() {
                    if i < self.len.get() {
                        self.nodes.get_mut(i as usize).ok_or(Error::IndexOutOfBounds(i as usize))
                    } else {
                        return Err(Error::IndexOutOfBounds(i as usize));
                    }
                } else {
                    Err(Error::EmptyNode)
                }
            }

            /// Returns the `prev` field of the node at `index`.
            ///
            /// # Errors
            /// If either the index is `None`, or out of bounds.
            fn prev_at(&self, index: [<Index$b>]) -> Result<[<Index$b>]> {
                Ok(self.ref_node_at(index)?.prev())
            }
            /// Sets the `prev` field of the node at `index` with `new_prev`.
            ///
            /// # Errors
            /// If either the index is `None`, or out of bounds.
            fn set_prev_at(&mut self, index: [<Index$b>], new_prev: [<Index$b>])
                -> Result<()> {
                self.mut_node_at(index)?.set_prev(new_prev);
                Ok(())
            }

            /// Returns the `next` field of the provided node-`index`.
            ///
            /// # Errors
            /// If either the index is `None`, or out of bounds.
            fn next_at(&self, index: [<Index$b>]) -> Result<[<Index$b>]> {
                Ok(self.ref_node_at(index)?.next())
            }
            /// Sets the `next` field of the provided node-`index`, with `new_next`.
            ///
            /// # Errors
            /// If either the index is `None`, or out of bounds.
            fn set_next_at(&mut self, index: [<Index$b>], new_next: [<Index$b>])
                -> Result<()> {
                self.mut_node_at(index)?.set_next(new_next);
                Ok(())
            }

            /// Unlinks all the nodes.
            #[inline]
            fn unlink_all_nodes(&mut self) {
                if CAP == 0 {
                    return;
                }
                for i in 1..CAP-1 {
                    self.nodes[i].unlink();
                }
            }
        }

        /// Private utility methods, when T: Clone
        impl<T: Clone, S: Storage, const CAP: usize> [<$name$b>]<T, S, CAP> {
            /// Resets all the nodes with the provided value, and unlinks them.
            ///
            /// Uses `value` to fill the data of each node.
            #[inline]
            fn reset_all_nodes(&mut self, value: T) {
                if CAP == 0 {
                    return;
                }
                for i in 1..CAP-1 {
                    self.nodes[i].reset(value.clone());
                }
            }
        }

    }};
}

// Only generate lists with an index primitive bit size >= usize::BITS

#[cfg(any(
    target_pointer_width = "8",
    target_pointer_width = "16",
    target_pointer_width = "32",
    target_pointer_width = "64",
    target_pointer_width = "128"
))]
linked_list_array![DoublyLinkedList, 1, 8, u8, nonmax::NonMaxU8];

#[cfg(any(
    target_pointer_width = "16",
    target_pointer_width = "32",
    target_pointer_width = "64",
    target_pointer_width = "128"
))]
linked_list_array![DoublyLinkedList, 2, 16, u16, nonmax::NonMaxU16];

#[cfg(any(
    target_pointer_width = "32",
    target_pointer_width = "64",
    target_pointer_width = "128"
))]
linked_list_array![DoublyLinkedList, 4, 32, u32, nonmax::NonMaxU32];
