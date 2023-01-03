// ladata::line::link::builder
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

use core::fmt;

#[cfg(not(feature = "no_unsafe"))]
use core::{
    mem::{self, MaybeUninit},
};

use crate::mem::{Raw, Storage};

#[cfg(feature = "std")]
use crate::mem::Boxed;

/// Generates a doubly linked list backed by an array, with custom index size.
macro_rules! linked_list_array {
    // $name : name prefix. E.g.: LinkedList
    // $B : byte size
    // $b : bit size
    // $t : inner index type
    // $nmt: nonmax inner index type
    ( $name:ident, $B:literal, $b:literal, $t:ty, $nmt:ty) => { paste::paste! {

        // Index ---------------------------------------------------------------

        /// The private list index type.
        ///
        /// There's a maximum of `$t::MAX` -1 nodes.
        #[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
        pub(super) struct [<$name$b Index>](Option<$nmt>);

        impl fmt::Debug for [<$name$b Index>] {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f,  "{:?}", self.0)
            }
        }

        #[allow(dead_code)]
        impl [<$name$b Index>] {
            /// Returns a new index pointing to some node.
            ///
            #[doc = "Returns `None` if index is [`" $t "::MAX`]."]
            #[inline]
            pub(super) const fn new(index: $t) -> Option<Self> {
                if let Some(i) = $nmt::new(index) {
                    Some(Self(Some(i)))
                } else {
                    None
                }
            }

            /// Returns a new index that doesn't point to any node.
            #[inline]
            pub(super) const fn none() -> Self {
                Self(None)
            }

            //

            /// Returns `true` if the index points to some node.
            #[inline]
            pub(super) const fn is_some(&self) -> bool {
                self.0.is_some()
            }

            /// Returns `true` if the index points to no node.
            #[inline]
            pub(super) const fn is_none(&self) -> bool {
                self.0.is_none()
            }

            //

            /// Retuns the inner primitive type, or `None` if == `$t::MAX`.
            #[inline]
            pub(super) const fn get(&self) -> Option<$t> {
                if let Some(i) = self.0 {
                    Some(i.get())
                } else {
                    None
                }
            }

            /// Returns the inner primitive type as a `usize`.
            // THINK MAYBE DELETE
            #[inline]
            pub(super) const fn as_usize(&self) -> usize {
                if let Some(v) = self.get() {
                    v as usize
                } else {
                    // RETHINK?
                    $t::MAX as usize
                }
            }

            /// Increments by 1 the inner value, if not `None`.
            // TODO TEST when very close to MAX... in e.g. push_front
            // IMPROVE: Result?
            #[must_use]
            #[inline]
            pub(super) fn increment(&mut self) -> Option<()> {
                if let Some(i) = self.0 {
                    self.0 = $nmt::new(i.get().checked_add(1)?);
                    Some(())
                } else {
                    None
                }
            }
            /// Decrements by 1 the inner value, if not `None`.
            // IMPROVE when very close to MAX... in e.g. push_front
            // IMPROVE Result?
            #[must_use]
            #[inline]
            pub(super) fn decrement(&mut self) -> Option<()> {
                if let Some(i) = self.0 {
                    self.0 = $nmt::new(i.get().checked_sub(1)?);
                    Some(())
                } else {
                    None
                }
            }
        }

        impl From<$t> for [<$name$b Index>] {
            /// Converts $t::MAX to None
            #[inline]
            fn from(index: $t) -> Self {
                if let Some(i) = $nmt::new(index) {
                    Self(Some(i))
                } else {
                    Self(None)
                }
            }
        }
        impl From<Option<$t>> for [<$name$b Index>] {
            #[inline]
            fn from(index: Option<$t>) -> Self {
                if let Some(i) = index {
                    Self($nmt::new(i))
                } else {
                    Self(None)
                }
            }
        }

        // Node ----------------------------------------------------------------

        /// The list node.
        //
        // Doesn't derive `Default` on purpose.
        pub(super) struct [<$name$b Node>] <T> {
            /// The index of the previous element, towards the front of the list.
            prev: [<$name$b Index>],
            /// The index of the next element, towards the back of the list.
            next: [<$name$b Index>],
            /// The node's data.
            data: T,
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

        impl<T: fmt::Debug> fmt::Debug for [<$name$b Node>]<T> {
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
                prev: [<$name$b Index>],
                next: [<$name$b Index>],
                data: T
            ) -> Self {
                Self {
                    prev,
                    next,
                    data,
                }
            }

            /// Returns an empty node, with custom `prev`ious and `next` indices.
            // RETHINK: Default?
            #[inline]
            pub(super) const fn new_unlinked(data: T) -> Self {
                Self {
                    prev: [<$name$b Index>]::none(),
                    next: [<$name$b Index>]::none(),
                    data,
                }
            }

            /// Returns a new node intended to be the new front of the list.
            /// Expects the index of the current front node.
            #[inline]
            pub(super) const fn new_front(current_front: [<$name$b Index>], data: T) -> Self {
                Self {
                    prev: [<$name$b Index>]::none(),
                    next: current_front,
                    data,
                }
            }

            /// Returns a new node intended to be the new back of the list.
            /// Expects the index of the current back node.
            #[inline]
            pub(super) const fn new_back(current_back: [<$name$b Index>], data: T) -> Self {
                Self {
                    prev: current_back,
                    next: [<$name$b Index>]::none(),
                    data,
                }
            }

            /// Returns this node's next index (towards the back).
            #[inline]
            pub(super) fn next(&self) -> [<$name$b Index>] {
                self.next
            }
            /// Sets this node's next index (towards the back).
            #[inline]
            pub(super) fn set_next(&mut self, index: [<$name$b Index>]) {
                self.next = index;
            }

            /// Returns this node's previous index (towards the front).
            #[inline]
            pub(super) fn prev(&self) -> [<$name$b Index>] {
                self.prev
            }
            /// Sets this node's previous index (towards the front).
            #[inline]
            pub(super) fn set_prev(&mut self, index: [<$name$b Index>]) {
                self.prev = index;
            }

            /// Unlinks the node, clearing both prev and next indexes.
            pub(super) fn unlink(&mut self) {
                self.next = [<$name$b Index>]::none();
                self.prev = [<$name$b Index>]::none();
            }

            /// Sets the `value` and unlinks the node,
            /// clearing both prev and next indexes.
            pub(super) fn reset(&mut self, value: T) {
                self.data = value;
                self.next = [<$name$b Index>]::none();
                self.prev = [<$name$b Index>]::none();
            }
        }

        // List ----------------------------------------------------------------

        #[doc = "A doubly linked list, backed by an array, using " $b "-bit indices."]
        ///
        #[doc = "- It has a maximum length of [`" $t "::MAX`]` -1` elements."]
        #[doc = "- An empty list has a minimum size of `3 * " $B "` bytes."]
        #[doc = "- Each element occupies `2 * " $B " + core::mem::size_of::<T>()` bytes,"]
        #[doc = "plus any padding."]
        pub struct [<$name$b>]<T, S: Storage, const CAP: usize> {
            /// The current number of nodes.
            count: [<$name$b Index>],
            /// The index of the current element at the front.
            front: [<$name$b Index>],
            /// The index of the current element at the back.
            back: [<$name$b Index>],
            /// The array of nodes, stored in the generic container.
            nodes: S::Container<[ [<$name$b Node>]<T>; CAP]>,
        }

        /// impl Clone, Copy, Debug, Default…
        mod [<impls_$b>] {
            use super::*;

            // T:Clone
            impl<T: Clone, S: Storage, const CAP: usize> Clone for [<$name$b>]<T, S, CAP>
                where S::Container<[[<$name$b Node>]<T>; CAP]>: Clone {
                fn clone(&self) -> Self {
                    Self {
                        count: self.count.clone(),
                        front: self.front.clone(),
                        back: self.back.clone(),
                        nodes: self.nodes.clone(),
                    }
                }
            }

            /// `T:Copy`
            impl<T: Copy, S: Storage, const CAP: usize> Copy for [<$name$b>]<T, S, CAP>
                where S::Container<[[<$name$b Node>]<T>; CAP]>: Copy {}

            /// `T:Debug`
            impl<T: fmt::Debug, S: Storage, const CAP: usize> fmt::Debug for [<$name$b>]<T, S, CAP>
                where S::Container<[[<$name$b Node>]<T>; CAP]>: fmt::Debug {
                fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    let mut debug = f.debug_struct(stringify![[<$name$b>]]);
                    debug
                        .field("CAP", &CAP)
                        .field("count", &self.count)
                        .field("front", &self.front)
                        .field("back", &self.back);

                    if CAP <= 6 {
                        debug.field("nodes", &self.nodes);
                    } else {
                        // IMPROVE: show first 3 and last 3
                        debug.field("nodes { ... }", &());
                    }

                    debug.finish()
                }
            }

            /// `S=(); T:Default`
            impl<T: Default, const CAP: usize> Default for [<$name$b>]<T, (), CAP> {
                /// Returns an empty, non-circular, doubly linked list.
                ///
                /// # Examples
                /// ```
                /// use ladata::all::LinkedList8;
                /// let l = LinkedList8::<u8, (), 100>::default();
                /// ```
                ///
                /// # Panics
                #[doc = "Panics if `CAP` is > [`" $t "::MAX`]."]
                fn default() -> Self {
                    assert![CAP < $t::MAX as usize];
                    #[cfg(not(feature = "no_unsafe"))]
                    let data = Raw::new({
                        let mut arr: [MaybeUninit<[<$name$b Node>]<T>>; CAP] = unsafe {
                            MaybeUninit::uninit().assume_init()
                        };
                        for i in &mut arr[..] {
                            let _ = i.write([<$name$b Node>]::new_unlinked(T::default()));
                        }
                        unsafe { mem::transmute_copy::<_, [ [<$name$b Node>]<T>; CAP]>(&arr) }
                    });
                    #[cfg(feature = "no_unsafe")]
                    let data = Raw::new(
                        core::array::from_fn(|_| [<$name$b Node>]::new_unlinked(T::default())));

                    Self {
                        count: 0.into(),
                        front: None.into(),
                        back: None.into(),
                        nodes: data,
                    }
                }
            }

            /// `S=Boxed; T:Default`
            #[cfg(feature = "std")]
            impl<T: Default, const CAP: usize> Default for [<$name$b>]<T, Boxed, CAP> {
                /// Returns an empty, non-circular, doubly linked list.
                ///
                /// # Examples
                /// ```
                /// use ladata::all::{Boxed, LinkedList8};
                /// let l = LinkedList8::<u8, Boxed, 10>::default();
                /// ```
                ///
                /// # Panics
                #[doc = "Panics if `CAP` is > [`" $t "::MAX`]."]
                fn default() -> Self {
                    assert![CAP < $t::MAX as usize];
                    let data = {
                        let mut v = Vec::<[<$name$b Node>]<T>>::with_capacity(CAP);

                        for _ in 0..CAP {
                            v.push([<$name$b Node>]::new_unlinked(T::default()));
                        }

                        let Ok(array) = v.into_boxed_slice().try_into() else {
                            panic!("Can't turn the boxed slice into a boxed array");
                        };
                        array
                    };


                    Self {
                        count: 0.into(),
                        front: None.into(),
                        back: None.into(),
                        nodes: data,
                    }
                }
            }
        }

        /// `S=(); T:Clone`
        impl<T: Clone, const CAP: usize> [<$name$b>]<T, (), CAP> {
            /// Returns a doubly linked list, filled with unlinked `value` elements.
            ///
            /// # Examples
            /// ```
            /// use ladata::all::LinkedList8;
            /// let l = LinkedList8::<u8, (), 100>::new(0);
            /// ```
            ///
            /// # Panics
            #[doc = "Panics if `CAP` is >= [`" $t "::MAX`]."]
            pub fn new(value: T) -> Self {
                assert![CAP < $t::MAX as usize];

                #[cfg(not(feature = "no_unsafe"))]
                let data = {
                    let mut arr: [MaybeUninit<[<$name$b Node>]<T>>; CAP] = unsafe {
                        MaybeUninit::uninit().assume_init()
                    };

                    for i in &mut arr[..] {
                        let _ = i.write([<$name$b Node>]::new_unlinked(value.clone()));
                    }

                    // TEMP:FIX: can't use transmute for now:
                    // - https://github.com/rust-lang/rust/issues/62875
                    // - https://github.com/rust-lang/rust/issues/61956
                    // unsafe { mem::transmute::<_, [ [<$name$b Node>]<T>; CAP]>(&arr) }
                    unsafe { mem::transmute_copy::<_, [ [<$name$b Node>]<T>; CAP]>(&arr) }
                }.into();

                #[cfg(feature = "no_unsafe")]
                let data = Raw::new(
                    core::array::from_fn(|_| [<$name$b Node>]::new_unlinked(value.clone())));

                Self {
                    count: 0.into(),
                    front: None.into(),
                    back: None.into(),
                    nodes: data,
                }
            }
        }

        /// `S:Boxed + T:Clone`
        #[cfg(feature = "std")]
        impl<T: Clone, const CAP: usize> [<$name$b>]<T, Boxed, CAP> {
            /// Returns a doubly linked list, filled with unlinked `value` elements.
            ///
            /// # Examples
            /// ```
            /// use ladata::all::{Boxed, LinkedList8};
            /// let l = LinkedList8::<u8, Boxed, 100>::new(0);
            /// ```
            ///
            /// # Panics
            #[doc = "Panics if `CAP` is >= [`" $t "::MAX`]."]
            pub fn new(value: T) -> Self {
                assert![CAP < $t::MAX as usize];

                let data = {
                    let mut v = Vec::<[<$name$b Node>]<T>>::with_capacity(CAP);

                    for _ in 0..CAP {
                        v.push([<$name$b Node>]::new_unlinked(value.clone()));
                    }

                    let Ok(array) = v.into_boxed_slice().try_into() else {
                        panic!("Can't turn the boxed slice into a boxed array");
                    };
                    array
                };

                Self {
                    count: 0.into(),
                    front: None.into(),
                    back: None.into(),
                    nodes: data,
                }
            }
        }

        /// `T:Clone`
        impl<T: Clone, S: Storage, const CAP: usize> [<$name$b>]<T, S, CAP> {
            /// Resets the list, unlinking all elements and setting them to `value`.
            pub fn reset(&mut self, value: T) {
                self.count = 0.into();
                self.front = None.into();
                self.back = None.into();
                self.reset_all_nodes(value);
            }
        }

        /// `*`
        impl<T, S: Storage, const CAP: usize> [<$name$b>]<T, S, CAP> {
            /// Returns the number of elements.
            pub const fn len(&self) -> $t {
                if let Some(c) = self.count.get() {
                    c
                } else {
                    0
                }
            }

            /// Checks if the list is empty.
            pub const fn is_empty(&self) -> bool {
                self.front.is_none() && self.back.is_none()
            }

            /// Checks if the list is full.
            pub const fn is_full(&self) -> bool {
                self.len() as usize == CAP
            }

            /// Returns the maximum number of elements.
            pub const fn capacity(&self) -> usize {
                CAP
            }

            /// Returns the available free elements.
            pub const fn remaining_capacity(&self) -> usize {
                CAP - self.len() as usize
            }

            /// Clears the list, unlinking all values.
            pub fn clear(&mut self) {
                self.count = 0.into();
                self.front = None.into();
                self.back = None.into();
                self.unlink_all_nodes();
            }

            /* front & back */

            /// Returns the index of the first element,
            /// or `None` if the list is empty.
            pub const fn front_index(&self) -> Option<$t> {
                if self.front.is_some() {
                    self.front.get()
                } else {
                    None
                }
            }

            /// Returns the index of the last element,
            /// or `None` if the list is empty.
            pub const fn back_index(&self) -> Option<$t> {
                if self.back.is_some() {
                    self.back.get()
                } else {
                    None
                }
            }

            /// Returns a shared reference to the first element,
            /// or `None` if the list is empty.
            pub fn front(&self) -> Option<&T> {
                if self.front.is_some() {
                    Some(&self.nodes[self.front.as_usize()].data)
                } else {
                    None
                }
            }

            /// Returns a shared reference to the last element,
            /// or `None` if the list is empty.
            pub fn back(&self) -> Option<&T> {
                if self.back.is_some() {
                    Some(&self.nodes[self.back.as_usize()].data)
                } else {
                    None
                }
            }

            /// Returns an exclusive reference to the first element,
            /// or `None` if the list is empty.
            pub fn front_mut(&mut self) -> Option<&mut T> {
                if self.front.is_some() {
                    Some(&mut self.nodes[self.front.as_usize()].data)
                } else {
                    None
                }
            }

            /// Returns an exclusive reference to the last element,
            /// or `None` if the list is empty.
            pub fn back_mut(&mut self) -> Option<&mut T> {
                if self.back.is_some() {
                    Some(&mut self.nodes[self.back.as_usize()].data)
                } else {
                    None
                }
            }

            /// Returns a shared reference to the element at `index`,
            /// or `None` if the index is out of bounds.
            pub fn at(&self, index: $t) -> Option<&T> {
                if index < self.count.get()? {
                    Some(&self.nodes[index as usize].data)
                } else {
                    None
                }
            }

            /// Returns an exclusive reference to the element at `index`,
            /// or `None` if the index is out of bounds.
            pub fn at_mut(&mut self, index: $t) -> Option<&mut T> {
                if index < self.count.get()? {
                    Some(&mut self.nodes[index as usize].data)
                } else {
                    None
                }
            }

            /// Adds an element at the front of the array and returns its index.
            ///
            /// Returns `None` on overflow.
            pub fn push_front(&mut self, value: T) -> Option<$t> {
                if self.is_full() {
                    None
                } else {
                    // create the new element to put at the front.
                    let element = [<$name$b Node>]::new_front(self.front, value);

                    // find where the new element will be inserted.
                    let element_idx = self.count;

                    // the first front element is also the back element.
                    if element_idx.get().unwrap() == 0 {
                        self.back = element_idx;
                    } else {
                        // otherwise update the previous front element
                        // self.mut_node_at(self.front)?.set_prev(&element_idx);
                        self.set_prev_at(self.front, element_idx)?;
                    }

                    // update the element count.
                    self.count.increment()?;

                    // insert the new element
                    self.nodes[element_idx.as_usize()] = element;

                    // update current front element.
                    self.front = element_idx;

                    // return the index of the inserted element
                    Some(self.count.get().unwrap() - 1)
                }
            }

            /// Removes the element at the front of the array and returns it.
            ///
            /// Returns `None` if the list is empty.
            // TODO WIP
            #[allow(warnings)] // TEMP
            #[cfg(not(feature = "no_unsafe"))]
            pub fn pop_front(&mut self) -> Option<T> {
                if self.front.is_none() {
                    return None;
                }

                    todo![]
                }

            // /// Adds an element at the back of the array and returns its index.
            // ///
            // /// Returns `None` on overflow.
            // pub fn push_back(&mut self, value: T) -> Option<$t> {
            //     if self.is_full() {
            //         None
            //     } else {
            //         // 1. create the new element to put at the back.
            //         // its previous_index will link to the current back_index
            //         let element = [<$name$b Node>]::back(self.back, value);
            //         // 2.
            //         let prev = self.insert_free_element(element);
            //
            //         // ???
            //         *self.next_of_prev(self.back, true) = prev;
            //
            //         self.back = prev;
            //         self.count += 1;
            //
            //         Some(prev - 1)
            //     }
            // }

            // TODO: pop_back
        }

        //

        /// Private utility methods
        #[allow(dead_code)]
        impl<T, S: Storage, const CAP: usize> [<$name$b>]<T, S, CAP> {
            /// Returns a reference to the node at `index`,
            /// or `None` if either the index is `None`, or overflows.
            fn ref_node_at(&self, index: [<$name$b Index>]) -> Option<&[<$name$b Node>]<T>> {
                if let Some(i) = index.get() {
                    if i < self.count.get()? {
                        // self.nodes[i as usize].data.as_ref()
                        self.nodes.get(i as usize)
                    } else {
                        None
                    }
                } else {
                    None
                }

                // CHECK whether there's a performance improvement by using:
                // Some(
                //     self.nodes[i as usize].data.as_ref()
                //         .unwrap_or_else(|| unsafe { hint::unreachable_unchecked() }),
                // )
            }

            /// Returns an exclusive reference to the node at `index`,
            /// or `None` if either the index is `None`, or overflows.
            fn mut_node_at(&mut self, index: [<$name$b Index>])
                -> Option<&mut [<$name$b Node>]<T>> {
                if let Some(i) = index.get() {
                    if i < self.count.get()? {
                        self.nodes.get_mut(i as usize)
                    } else {
                        None
                    }
                } else {
                    None
                }
            }

            /// Returns the `prev` index of the node at `index`,
            /// or `None` if either the index is `None`, or it overflows.
            fn prev_at(&self, index: [<$name$b Index>]) -> Option<[<$name$b Index>]> {
                Some(self.ref_node_at(index)?.prev())
            }
            /// Sets the `prev` index of the node at `index` with `new_prev`.
            ///
            /// Returns `None` if either the index is `None`, or it overflows.
            fn set_prev_at(&mut self, index: [<$name$b Index>], new_prev: [<$name$b Index>])
                -> Option<()> {
                self.mut_node_at(index)?.set_prev(new_prev);
                Some(())
            }

            /// Returns the `next` node-index of the provided node-`index`,
            /// or `None` if either the `index` is `None`, or it overflows.
            fn next_at(&self, index: [<$name$b Index>]) -> Option<[<$name$b Index>]> {
                Some(self.ref_node_at(index)?.next())
            }
            /// Sets the `next` node-index of the provided node-`index`,
            /// with `new_next`.
            ///
            /// Returns `None` if either the index is `None`, or it overflows.
            fn set_next_at(&mut self, index: [<$name$b Index>], new_next: [<$name$b Index>])
                -> Option<()> {
                self.mut_node_at(index)?.set_next(new_next);
                Some(())
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

#[cfg(any(
    target_pointer_width = "8",
    target_pointer_width = "16",
    target_pointer_width = "32",
    target_pointer_width = "64",
    target_pointer_width = "128"
))]
linked_list_array![LinkedList, 1, 8, u8, nonmax::NonMaxU8];

#[cfg(any(
    target_pointer_width = "16",
    target_pointer_width = "32",
    target_pointer_width = "64",
    target_pointer_width = "128"
))]
linked_list_array![LinkedList, 2, 16, u16, nonmax::NonMaxU16];

#[cfg(any(
    target_pointer_width = "16",
    target_pointer_width = "32",
    target_pointer_width = "64",
    target_pointer_width = "128"
))]
linked_list_array![LinkedList, 4, 32, u32, nonmax::NonMaxU32];
