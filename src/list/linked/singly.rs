// ladata::list::link::singly
//
//! A macro builder for linked lists backed by a const-sized array.
//
// ```diagram
//
// Concrete list data representation:
// count:3  front:2  free:_  nodes:↓
//    i0       i1       i2       i3
// [__3rd_] [__2nd_] [__1st_] [______]
//       n_       n0       n1       n_
//
// Abstract list representation:
// [i2] → [i1] → [i0]
// front         last
//
// each node has a successor (next), pointing towards the back.
// ```

use unicode_segmentation::UnicodeSegmentation;

use core::{
    fmt::{self, Debug},
    mem::size_of,
};

use crate::{
    error::{LadataError as Error, LadataResult as Result},
    list::Array,
    mem::Storage,
    misc::*,
};

#[cfg(feature = "std")]
use crate::mem::Boxed;

/// Generates a singly linked list backed by an array, with custom index size.
#[rustfmt::skip]
macro_rules! linked_list_array {
    // $name : name prefix. E.g.: SinglyLinked
    // $B : byte size
    // $b : bit size
    // $t : inner index type
    // $nmt: nonmax inner index type
    ( $name:ident, $B:literal, $b:literal, $t:ty, $nmt:ty) => { paste::paste! {

        // Node ----------------------------------------------------------------

        /// The list node.
        pub(super) struct [<$name$b Node>] <T> {
            /// The index of the next element, towards the back of the list.
            next: [<Index$b>],
            /// The node's data.
            data: T,
        }

        impl<T: Default> Default for [<$name$b Node>]<T> {
            fn default() -> Self {
                Self {
                    next: None.into(),
                    data: T::default(),
                }
            }
        }

        impl<T: Clone> Clone for [<$name$b Node>]<T> {
            #[inline]
            fn clone(&self) -> Self {
                Self {
                    next: self.next.clone(),
                    data: self.data.clone(),
                }
            }
        }
        impl<T: Copy> Copy for [<$name$b Node>]<T> {}

        // IMPROVE
        impl<T: Debug> Debug for [<$name$b Node>]<T> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.debug_struct(stringify![[<$name$b Node>]])
                .field("next", &self.next)
                .field("data", &self.data)
                .finish()
            }
        }

        impl<T> [<$name$b Node>]<T> {
            // /// Returns a new node, with `data`, and custom `next` index.
            // #[inline]
            // pub(super) const fn new(
            //     next: [<Index$b>],
            //     data: T
            // ) -> Self {
            //     Self {
            //         next,
            //         data,
            //     }
            // }

            /// Returns an empty node, with unlinked `next` index.
            #[inline]
            pub(super) const fn new_unlinked(data: T) -> Self {
                Self {
                    next: [<Index$b>]::none(),
                    data,
                }
            }

            //

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

            /// Unlinks the node, clearing the next index.
            #[inline]
            pub(super) fn unlink(&mut self) {
                self.next = [<Index$b>]::none();
            }

            /// Sets the `data` and unlinks the node, clearing the next index.
            #[inline]
            pub(super) fn reset(&mut self, data: T) {
                self.data = data;
                self.next = [<Index$b>]::none();
            }

            //

            // /// Returns a shared reference to the data.
            // #[inline]
            // pub(super) fn ref_data(&self) -> &T {
            //     &self.data
            // }

            // /// Returns an exclusive reference to the data.
            // #[inline]
            // pub(super) fn mut_data(&mut self) -> &mut T {
            //     &mut self.data
            // }

            // /// Returns the data.
            // #[inline]
            // pub(super) fn into_data(self) -> T {
            //     self.data
            // }

            /// Returns the inner components (next, data).
            #[inline]
            pub(super) fn into_components(self) -> ([<Index$b>], T) {
                (self.next, self.data)
            }
        }

        // List ----------------------------------------------------------------

        #[doc = "A singly linked list, backed by an [`Array`], using " $b "-bit indices."]
        ///
        #[doc = "- It has a maximum length of [`" $t "::MAX`]` -1` elements."]
        #[doc = "- An empty list has a minimum size of `3 * " $B "` bytes."]
        #[doc = "- Each element occupies `2 * " $B " + size_of::<T>()` bytes,"]
        #[doc = "plus any padding."]
        pub struct [<$name$b>]<T, S: Storage, const CAP: usize> {
            /// The index of the current element at the front.
            front: [<Index$b>],
            /// The current counted number of nodes.
            count: [<Index$b>],
            /// The array of nodes, stored in the generic container.
            nodes: Array<[<$name$b Node>]<T>, S, CAP>,
            /// The index of the front free element.
            free: [<Index$b>],
        }

        /// impl Clone, Copy, Debug, Default…
        mod [<impls_$b>] {
            use super::*;

            // T:Clone
            impl<T: Clone, S: Storage, const CAP: usize> Clone for [<$name$b>]<T, S, CAP>
                where S::Stored<[[<$name$b Node>]<T>; CAP]>: Clone {
                fn clone(&self) -> Self {
                    Self {
                        front: self.front.clone(),
                        free: self.free.clone(),
                        count: self.count.clone(),
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
                    write![f, "{} {{ cap:{} len:{} front:{:?} free:{:?} sizeof:{} }}",
                        stringify!([<$name$b>]),
                        CAP, self.len(), self.front, self.free, size_of::<Self>()]?;

                    if self.len() > 0 {

                        // TODO: save ordered list of indexes, from iteration.

                        /* show the list diagram */
                        write![f, "\n## array of nodes:"]?;

                        //    i0       i1       i2       i3
                        write![f, "\n"]?;
                        for (idx, _) in self.nodes.iter().enumerate() {
                            write![f, "   i{}    ", idx]?;
                        }
                        // [__3rd_] [__2nd_] [__1st_] [______]
                        write![f, "\n"]?;
                        for node in self.nodes.iter() {
                            // if node.is_in_the_list() { // TEMP
                                let graphemes: String = format!["{:?}",
                                    node.data].graphemes(true).take(6).collect();
                                write![f, "[{:_^6}] ", graphemes]?;
                            // } else {
                            //     write![f, "[______] "]?;
                            // }
                        }
                        //       n_       n0       n1       n_
                        write![f, "\n"]?;
                        for node in self.nodes.iter() {
                            write![f, "      n{} ", node.next]?;
                        }

                        /* show the list of nodes */
                        write![f, "\n## node list from the front:"]?;

                        // A) iterate in array order
                        // let mut counter = 0;
                        // let mut idx_ptr = self.front;
                        // IMPROVE: use iterator when implemented
                        // for idx in 0..self.len() {
                        //     write!(f, "\nidx:{idx}: {:?}", self.nodes[idx as usize])?;
                        // }

                        // B) iterate in list order
                        let mut current_idx = self.front;
                        while current_idx.is_some() {
                            let node = &self.nodes[current_idx.as_usize()];
                            write![f, "\ni{current_idx}: {node:?}"]?;
                            // current_idx = self.nodes[current_idx.as_usize()].next();
                            current_idx = node.next();
                        }

                        // write![f, "\n{:#?}", self.nodes]?; // TEMP print array
                    }
                    Ok(())
                }
            }

            /// `S=(); T:Default`
            impl<T: Default, const CAP: usize> Default for [<$name$b>]<T, (), CAP>
                where [<$name$b Node>]<T>: Default
            {
                /// Returns an empty, non-circular, singly linked list,
                /// allocated in the stack, and filled with unlinked `CAP`
                /// elements set to their default value.
                ///
                /// # Panics
                #[doc = "Panics if `CAP` is > [`" $t "::MAX`]."]
                ///
                /// # Examples
                /// ```
                /// use ladata::list::SinglyLinkedList8;
                /// let l = SinglyLinkedList8::<u8, (), 100>::default();
                /// ```
                fn default() -> Self {
                    assert![(CAP as $t) < $t::MAX];
                    Self {
                        front: None.into(),
                        free: None.into(),
                        count: [<Index$b>]::new(0).unwrap(),
                        nodes: Array::default(),
                    }
                }
            }

            /// `S=Boxed; T:Default`
            #[cfg(feature = "std")]
            #[cfg_attr(feature = "nightly", doc(cfg(feature = "std")))]
            impl<T: Default, const CAP: usize> Default for [<$name$b>]<T, Boxed, CAP>
                where [<$name$b Node>]<T>: Default
            {
                /// Returns an empty, non-circular, singly linked list,
                /// allocated in the stack, and filled with unlinked `CAP`
                /// elements set to their default value.
                ///
                /// # Examples
                /// ```
                /// use ladata::all::{Boxed, SinglyLinkedList8};
                /// let l = SinglyLinkedList8::<u8, Boxed, 10>::default();
                /// ```
                ///
                /// # Panics
                #[doc = "Panics if `CAP` is > [`" $t "::MAX`]."]
                fn default() -> Self {
                    assert![(CAP as $t) < $t::MAX];
                    Self {
                        front: None.into(),
                        free: None.into(),
                        count: [<Index$b>]::new(0).unwrap(),
                        nodes: Array::default(),
                    }
                }
            }
        }

        /// `S=(); T:Clone`
        impl<T: Clone, const CAP: usize> [<$name$b>]<T, (), CAP> {
            /// Returns a singly linked list, allocated in the stack,
            /// filled with `CAP` unlinked elements set to `value`.
            ///
            /// # Errors
            #[doc = "If `CAP` is >= [`" $t "::MAX`]."]
            ///
            /// # Examples
            /// ```
            /// use ladata::list::DirectSinglyLinkedList8;
            /// # fn main() -> ladata::error::LadataResult<()> {
            ///
            /// let l = DirectSinglyLinkedList8::<u8, 100>::new(0)?;
            /// # Ok(()) }
            /// ```
            pub fn new(value: T) -> Result<Self> {
                if (CAP as $t) < $t::MAX {
                    Ok(Self {
                        front: None.into(),
                        free: None.into(),
                        count: [<Index$b>]::new(0).unwrap(),
                        nodes: Array::<[<$name$b Node>]<T>, (), CAP>::
                            with([<$name$b Node>]::new_unlinked(value)),
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
            /// Returns a singly linked list, allocated in the heap,
            /// filled with `CAP` unlinked elements set to `value`.
            ///
            /// # Errors
            #[doc = "If `CAP` is >= [`" $t "::MAX`]."]
            ///
            /// # Examples
            /// ```
            /// use ladata::list::{BoxedSinglyLinkedList8};
            /// # fn main() -> ladata::error::LadataResult<()> {
            ///
            /// let l = BoxedSinglyLinkedList8::<u8, 100>::new(0)?;
            /// # Ok(()) }
            /// ```
            pub fn new(value: T) -> Result<Self> {
                if (CAP as $t) < $t::MAX {
                    Ok(Self {
                        front: None.into(),
                        free: None.into(),
                        count: [<Index$b>]::new(0).unwrap(),
                        nodes: Array::<[<$name$b Node>]<T>, Boxed, CAP>::
                            with([<$name$b Node>]::new_unlinked(value)),
                    })
                } else {
                    Err(Error::IndexOutOfBounds(CAP))
                }
            }
        }

        /// `*`
        impl<T, S: Storage, const CAP: usize> [<$name$b>]<T, S, CAP> {
            /// Returns the number of elements.
            ///
            /// # Examples
            /// ```
            /// use ladata::list::DirectSinglyLinkedList8;
            /// # fn main() -> ladata::error::LadataResult<()> {
            ///
            /// let mut s = DirectSinglyLinkedList8::<i32, 3>::default();
            /// s.push_front(1)?;
            /// assert_eq![1, s.len()];
            /// # Ok(()) }
            /// ```
            pub fn len(&self) -> usize {
                debug_assert![self.count.as_primitive() < $t::MAX];
                self.count.as_usize()
            }

            /// Checks if the list is empty.
            ///
            /// # Examples
            /// ```
            /// use ladata::list::DirectSinglyLinkedList8;
            ///
            /// let s = DirectSinglyLinkedList8::<i32, 5>::default();
            /// assert![s.is_empty()];
            /// ```
            pub fn is_empty(&self) -> bool {
                // debug_assert![
                //     (self.len() == 0 && self.front.is_none()) ||
                //     (self.len() > 0 && self.front.is_some())
                // ];
                self.len() == 0
            }

            /// Checks if the list is full.
            ///
            /// # Examples
            /// ```
            /// use ladata::list::DirectSinglyLinkedList8;
            /// # fn main() -> ladata::error::LadataResult<()> {
            ///
            /// let mut s = DirectSinglyLinkedList8::<i32, 2>::default();
            /// s.push_front(1)?;
            /// s.push_front(2)?;
            /// assert![s.is_full()];
            /// # Ok(()) }
            /// ```
            pub fn is_full(&self) -> bool {
                self.len() == CAP
            }

            /// Returns the maximum number of elements.
            ///
            /// # Examples
            /// ```
            /// use ladata::list::DirectSinglyLinkedList8;
            ///
            /// let s = DirectSinglyLinkedList8::<i32, 3>::default();
            /// assert_eq![3, s.capacity()];
            /// ```
            pub const fn capacity(&self) -> usize {
                CAP
            }

            /// Returns the available free elements.
            ///
            /// # Examples
            /// ```
            /// use ladata::list::DirectSinglyLinkedList8;
            /// # fn main() -> ladata::error::LadataResult<()> {
            ///
            /// let mut s = DirectSinglyLinkedList8::<i32, 3>::default();
            /// s.push_front(1)?;
            /// assert_eq![2, s.remaining_capacity()];
            /// # Ok(()) }
            /// ```
            pub fn remaining_capacity(&self) -> usize {
                CAP - self.len()
            }

            /// Clears the list, unlinking all values.
            ///
            /// # Examples
            /// ```
            /// use ladata::list::DirectSinglyLinkedList8;
            /// # fn main() -> ladata::error::LadataResult<()> {
            ///
            /// let mut s = DirectSinglyLinkedList8::<i32, 2>::default();
            /// s.push_front(1)?;
            /// s.push_front(2)?;
            /// s.clear();
            /// assert![s.is_empty()];
            /// # Ok(()) }
            /// ```
            pub fn clear(&mut self) {
                self.count = [<Index$b>]::new(0).unwrap();
                self.front = None.into();
                self.free = None.into();
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
            /// use ladata::list::DirectSinglyLinkedList8;
            /// # fn main() -> ladata::error::LadataResult<()> {
            ///
            /// let mut s = DirectSinglyLinkedList8::<i32, 3>::default();
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

            /// Returns a shared reference to the front element.
            ///
            /// # Errors
            /// If the list is empty.
            ///
            /// # Examples
            /// ```
            /// use ladata::list::DirectSinglyLinkedList8;
            /// # fn main() -> ladata::error::LadataResult<()> {
            ///
            /// let mut s = DirectSinglyLinkedList8::<i32, 3>::default();
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

            /// Returns an exclusive reference to the front element.
            ///
            /// # Errors
            /// If the list is empty.
            ///
            /// # Examples
            /// ```
            /// use ladata::list::DirectSinglyLinkedList8;
            /// # fn main() -> ladata::error::LadataResult<()> {
            ///
            /// let mut s = DirectSinglyLinkedList8::<i32, 3>::default();
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

            /// Pushes an element as the new front element of the list,
            /// and returns its index.
            ///
            /// # Errors
            /// If the list is full.
            ///
            /// # Examples
            /// ```
            /// use ladata::list::DirectSinglyLinkedList8;
            /// # fn main() -> ladata::error::LadataResult<()> {
            ///
            /// let mut s = DirectSinglyLinkedList8::<i32, 3>::default();
            /// s.push_front(1)?;
            /// assert_eq![1, s.len()];
            /// # Ok(()) }
            /// ```
            //
            // # Diagram
            // ```_
            // count:0  front:_  free:_
            //    i0       i1       i2       i3
            // [______] [______] [______] [______]
            //       n_       n_       n_       n_
            //
            // count:1  front:0  free:_                          push_front()
            //    i0       i1       i2       i3
            // [__1st_] [______] [______] [______]
            //       n_       n_       n_       n_
            //
            // count:2  front:1  free:_                          push_front()
            //    i0       i1       i2       i3
            // [__2nd_] [__1st_] [______] [______]
            //       n_       n0       n_       n_
            // ```
            pub fn push_front(&mut self, value: T) -> Result<[<Index$b>]> {
                if self.is_full() {
                    Err(Error::NotEnoughSpace(Some(1)))
                } else {
                    // The new front node, and its new index:
                    let mut new_node = [<$name$b Node>]::new_unlinked(value);
                    let new_index = self.first_free_index();

                    // If there was already a front node…
                    if self.front.is_some() {
                        // …make the new front node point to it.
                        new_node.set_next(self.front);
                    }

                    // Update the node, the count and the front index:
                    self.nodes[new_index.as_usize()] = new_node;
                    self.count.increment().unwrap();
                    self.front = new_index;

                    Ok(new_index)
                }
            }

            /// Provides a forward iterator.
            ///
            /// # Example
            /// ```
            /// use ladata::list::DirectSinglyLinkedList8;
            /// # fn main() -> ladata::error::LadataResult<()> {
            ///
            /// // let l = DirectSinglyLinkedList8::<i32, 4>::from([1, 2]); // TODO
            /// let mut l = DirectSinglyLinkedList8::<i32, 4>::default();
            /// l.push_front(1)?;
            /// l.push_front(2)?;
            ///
            /// let mut li = l.iter();
            /// assert_eq![Some(&2), li.next()];
            /// assert_eq![Some(&1), li.next()];
            /// assert_eq![None, li.next()];
            /// # Ok(()) }
            /// ```
            pub fn iter(&self) -> [<$name$b Iter>]<'_, T, S, CAP> {
                [<$name$b Iter>] {
                    list: self,
                    current: self.front,
                }
            }
        }

        /// `T:Clone`
        impl<T: Clone, S: Storage, const CAP: usize> [<$name$b>]<T, S, CAP> {
            /// Removes the element at the front of the array and returns it.
            ///
            /// # Errors
            /// If the list is empty.
            ///
            /// # Examples
            /// ```
            /// use ladata::list::DirectSinglyLinkedList8;
            /// # fn main() -> ladata::error::LadataResult<()> {
            ///
            /// let mut s = DirectSinglyLinkedList8::<i32, 3>::default();
            /// s.push_front(1)?;
            /// s.push_front(2)?;
            /// assert_eq![Ok(2), s.pop_front()];
            /// assert_eq![Ok(1), s.pop_front()];
            /// assert![s.pop_front().is_err()];
            /// # Ok(()) }
            /// ```
            //
            // # Diagram
            // ```_
            // count:2  front:1  free:_
            //    i0       i1       i2       i3
            // [__2nd_] [__1st_] [______] [______]
            //       n_       n0       n_       n_
            //
            // count:2  front:0  free:_                           pop_front()
            //    i0       i1       i2       i3
            // [__1st_] [______] [______] [______]
            //       n_       n_       n_       n_
            // ```
            #[cfg(feature = "safe")]
            pub fn pop_front(&mut self) -> Result<T> {
                if self.is_empty() {
                    Err(Error::NotEnoughElements(1))
                } else {
                    todo![]
                }
            }

            /// Resets the list, unlinking all elements and setting them to `value`.
            pub fn reset(&mut self, value: T) {
                self.front = None.into();
                self.free = None.into();
                self.count = [<Index$b>]::new(0).unwrap();
                self.reset_all_nodes(value);
            }
        }

        // Iter ----------------------------------------------------------------

        #[doc ="A `" [<$name$b>] "` iterator."]
        pub struct [<$name$b Iter>]<'s, T, S: Storage, const CAP: usize> {
            list: &'s [<$name$b>]<T, S, CAP>,
            /// The current node index.
            current: [<Index$b>],
        }

        impl<'a, T, S: Storage, const CAP: usize> Iterator for [<$name$b Iter>]<'a, T, S, CAP> {
            type Item = &'a T;

            fn next(&mut self) -> Option<Self::Item> {
                if self.list.is_empty() || self.current.is_none() {
                    None
                } else {
                    let node = &self.list.nodes[self.current.as_usize()];
                    self.current = node.next();
                    Some(&node.data)
                }
            }

            fn size_hint(&self) -> (usize, Option<usize>) {
                (self.list.len(), Some(self.list.len()))
            }
        }

        impl<'a, T, S: Storage, const CAP: usize>
            ExactSizeIterator for [<$name$b Iter>]<'a, T, S, CAP> {}

        // Private -------------------------------------------------------------

        /// Private utility methods
        #[allow(dead_code)]
        impl<T, S: Storage, const CAP: usize> [<$name$b>]<T, S, CAP> {
            /// Returns the index of the front free node.
            const fn first_free_index(&self) -> [<Index$b>] {
                if self.free.is_some() {
                    self.free
                } else {
                    self.count
                }
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
linked_list_array![SinglyLinkedList, 1, 8, u8, nonmax::NonMaxU8];

// #[cfg(any(
//     target_pointer_width = "16",
//     target_pointer_width = "32",
//     target_pointer_width = "64",
//     target_pointer_width = "128"
// ))]
// linked_list_array![SinglyLinkedList, 2, 16, u16, nonmax::NonMaxU16];
//
// #[cfg(any(
//     target_pointer_width = "32",
//     target_pointer_width = "64",
//     target_pointer_width = "128"
// ))]
// linked_list_array![SinglyLinkedList, 4, 32, u32, nonmax::NonMaxU32];
