// ladata::structs::list::double
//
//! Linked lists.
//
// ```diagram
// FRONT ←NODE→ ←NODE→ ←NODE→ ←NODE→ (BACK)
//
//     head                        tail
//    (FRONT)                     (BACK)
//      [0]      [1]      [2]      [3]
//    !←P,S→1  0←P,S→2  1←P,S→3  2←P,S→!
//
// each node has a successor and a predecessor
// ```

#![allow(dead_code)] // TEMP

use core::{
    fmt,
    mem::{self, MaybeUninit},
};

use crate::storage::{Boxed, Storage};

/// Generates a doubly linked list tied to an array, with fixed-size index size.
macro_rules! linked_list_array {
    // $name : name prefix. E.g.: LinkedList
    // $B : byte size
    // $b : bit size
    // $t : inner index type
    // $nmt: nonmax inner index type
    // ;
    // $size128:  byte size for 128 × i32 nodes
    // $size128z: byte size for 128 × i32 nodes with zero-sized option.
    ( $name:ident, $B:literal, $b:literal, $t:ty, $nmt:ty) => { paste::paste! {

        // Index ---------------------------------------------------------------

        /// The private list index type.
        ///
        /// There's a maximum of `$t::MAX` -1 nodes.
        #[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
        struct [<$name$b Index>](Option<$nmt>);

        impl fmt::Debug for [<$name$b Index>] {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f,  "{:?}", self.0)
            }
        }

        impl [<$name$b Index>] {
            /// Returns a new index pointing to some node.
            ///
            #[doc = "Returns `None` if index is [`" $t "::MAX`]."]
            #[inline]
            const fn new(index: $t) -> Option<Self> {
                if let Some(i) = $nmt::new(index) {
                    Some(Self(Some(i)))
                } else {
                    None
                }
            }

            /// Returns a new index that doesn't point to any node.
            #[inline]
            const fn none() -> Self {
                Self(None)
            }

            //

            /// Returns `true` if the index points to some node.
            #[inline]
            const fn is_some(&self) -> bool {
                self.0.is_some()
            }

            /// Returns `true` if the index points to no node.
            #[inline]
            const fn is_none(&self) -> bool {
                self.0.is_none()
            }

            //

            /// Retuns the inner primitive type, or `None` if == `$t::MAX`.
            #[inline]
            const fn get(&self) -> Option<$t> {
                if let Some(i) = self.0 {
                    Some(i.get())
                } else {
                    None
                }
            }

            /// Returns the inner primitive type as a `usize`.
            // THINK MAYBE DELETE
            #[inline]
            const fn as_usize(&self) -> usize {
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
            fn increment(&mut self) -> Option<()> {
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
            fn decrement(&mut self) -> Option<()> {
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
        struct [<$name$b Node>] <T> {
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

        impl<T> [<$name$b Node>]<T> {
            /// Returns a new node, with `data`, and custom `prev`ious and `next` indices.
            #[inline]
            const fn new(
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
            const fn new_unlinked(data: T) -> Self {
                Self {
                    prev: [<$name$b Index>]::none(),
                    next: [<$name$b Index>]::none(),
                    data,
                }
            }

            /// Returns a new node intended to be the new front of the list.
            /// Expects the index of the current front node.
            #[inline]
            const fn new_front(current_front: [<$name$b Index>], data: T) -> Self {
                Self {
                    prev: [<$name$b Index>]::none(),
                    next: current_front,
                    data,
                }
            }

            /// Returns a new node intended to be the new back of the list.
            /// Expects the index of the current back node.
            #[inline]
            const fn new_back(current_back: [<$name$b Index>], data: T) -> Self {
                Self {
                    prev: current_back,
                    next: [<$name$b Index>]::none(),
                    data,
                }
            }

            /// Returns this node's next index (towards the back).
            #[inline]
            fn next(&self) -> [<$name$b Index>] {
                self.next
            }
            /// Sets this node's next index (towards the back).
            #[inline]
            fn set_next(&mut self, index: [<$name$b Index>]) {
                self.next = index;
            }

            /// Returns this node's previous index (towards the front).
            #[inline]
            fn prev(&self) -> [<$name$b Index>] {
                self.prev
            }
            /// Sets this node's previous index (towards the front).
            #[inline]
            fn set_prev(&mut self, index: [<$name$b Index>]) {
                self.prev = index;
            }

            /// Unlinks the node, clearing both prev and next indexes.
            fn unlink(&mut self) {
                self.next = [<$name$b Index>]::none();
                self.prev = [<$name$b Index>]::none();
            }

            /// Sets the `value` and unlinks the node,
            /// clearing both prev and next indexes.
            fn reset(&mut self, value: T) {
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
        pub struct [<$name$b>]<T, S: Storage, const LEN: usize> {
            /// The current number of nodes.
            count: [<$name$b Index>],
            /// The index of the current element at the front.
            front: [<$name$b Index>],
            /// The index of the current element at the back.
            back: [<$name$b Index>],
            /// The array of nodes, stored in the generic container.
            nodes: S::Container<[ [<$name$b Node>]<T>; LEN]>,
        }

        /// impl Clone, Copy, Debug, Default…
        mod [<impls_$b>] {
            use super::*;

            // T:Clone
            impl<T: Clone, S: Storage, const LEN: usize> Clone for [<$name$b>]<T, S, LEN>
                where S::Container<[[<$name$b Node>]<T>; LEN]>: Clone {
                fn clone(&self) -> Self {
                    Self {
                        count: self.count.clone(),
                        front: self.front.clone(),
                        back: self.back.clone(),
                        nodes: self.nodes.clone(),
                    }
                }
            }

            // T:Copy
            impl<T: Copy, S: Storage, const LEN: usize> Copy for [<$name$b>]<T, S, LEN>
                where S::Container<[[<$name$b Node>]<T>; LEN]>: Copy {}

            // T:Debug
            impl<T: fmt::Debug, S: Storage, const LEN: usize> fmt::Debug for [<$name$b>]<T, S, LEN>
                where S::Container<[[<$name$b Node>]<T>; LEN]>: fmt::Debug {
                fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    let mut debug = f.debug_struct(stringify![[<$name$b>]]);
                    debug
                        .field("LEN", &LEN)
                        .field("count", &self.count)
                        .field("front", &self.front)
                        .field("back", &self.back);

                    if LEN <= 6 {
                        debug.field("nodes", &self.nodes);
                    } else {
                        // IMPROVE: show first 3 and last 3
                        debug.field("nodes { ... }", &());
                    }
                    debug.finish()
                }
            }

            // S:() + T:Default
            impl<T: Default, const LEN: usize> Default for [<$name$b>]<T, (), LEN> {
                /// Returns an empty, non-circular, doubly linked list.
                ///
                /// # Panics
                #[doc = "Panics if `LEN` is > [`" $t "::MAX`]."]
                fn default() -> Self {
                    assert![LEN < $t::MAX as usize];
                    let data = {
                        let mut arr: [MaybeUninit<[<$name$b Node>]<T>>; LEN] = unsafe {
                            MaybeUninit::uninit().assume_init()
                        };
                        for i in &mut arr[..] {
                            let _ = i.write([<$name$b Node>]::new_unlinked(T::default()));
                        }
                        unsafe { mem::transmute_copy::<_, [ [<$name$b Node>]<T>; LEN]>(&arr) }
                    };

                    Self {
                        count: 0.into(),
                        front: None.into(),
                        back: None.into(),
                        nodes: data.into(),
                    }
                }
            }

            // S:Boxed + T:Default
            impl<T: Default, const LEN: usize> Default for [<$name$b>]<T, Boxed, LEN> {
                /// Returns an empty, non-circular, doubly linked list.
                ///
                /// # Panics
                #[doc = "Panics if `LEN` is > [`" $t "::MAX`]."]
                fn default() -> Self {
                    assert![LEN < $t::MAX as usize];
                    let data = {
                        let mut v = Vec::<[<$name$b Node>]<T>>::with_capacity(LEN);

                        for _ in 0..v.len() {
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

        /// # `S:() + T:Clone`
        impl<T: Clone, const LEN: usize> [<$name$b>]<T, (), LEN> {
            /// Returns a doubly linked list, filled with unlinked `value` elements.
            ///
            /// # Panics
            #[doc = "Panics if `LEN` is >= [`" $t "::MAX`]."]
            pub fn new(value: T) -> Self {
                assert![LEN < $t::MAX as usize];

                let data = {
                    let mut arr: [MaybeUninit<[<$name$b Node>]<T>>; LEN] = unsafe {
                        MaybeUninit::uninit().assume_init()
                    };

                    for i in &mut arr[..] {
                        let _ = i.write([<$name$b Node>]::new_unlinked(value.clone()));
                    }

                    // TEMP:FIX: can't use transmute for now:
                    // - https://github.com/rust-lang/rust/issues/62875
                    // - https://github.com/rust-lang/rust/issues/61956
                    // unsafe { mem::transmute::<_, [ [<$name$b Node>]<T>; LEN]>(&arr) }
                    unsafe { mem::transmute_copy::<_, [ [<$name$b Node>]<T>; LEN]>(&arr) }
                };

                Self {
                    count: 0.into(),
                    front: None.into(),
                    back: None.into(),
                    nodes: data.into(),
                }
            }
        }

        /// # `S:Boxed + T:Clone`
        impl<T: Clone, const LEN: usize> [<$name$b>]<T, Boxed, LEN> {
            /// Returns a doubly linked list, filled with unlinked `value` elements.
            ///
            /// # Panics
            #[doc = "Panics if `LEN` is >= [`" $t "::MAX`]."]
            pub fn new(value: T) -> Self {
                assert![LEN < $t::MAX as usize];

                let data = {
                    let mut v = Vec::<[<$name$b Node>]<T>>::with_capacity(LEN);

                    for _ in 0..v.len() {
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

        /// # `T:Clone`
        impl<T: Clone, S: Storage, const LEN: usize> [<$name$b>]<T, S, LEN> {
            /// Resets the list, unlinking all elements and setting them to `value`.
            pub fn reset(&mut self, value: T) {
                self.count = 0.into();
                self.front = None.into();
                self.back = None.into();
                self.reset_nodes(value);
            }
        }

        /// # ``
        impl<T, S: Storage, const LEN: usize> [<$name$b>]<T, S, LEN> {
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
                self.len() as usize == LEN
            }

            /// Returns the maximum number of elements.
            pub const fn capacity(&self) -> usize {
                LEN
            }

            /// Returns the available free elements.
            pub const fn remaining_capacity(&self) -> usize {
                LEN - self.len() as usize
            }

            /// Clears the list, unlinking all values.
            pub fn clear(&mut self) {
                self.count = 0.into();
                self.front = None.into();
                self.back = None.into();
                self.unlink_nodes();
            }

            /* front & back*/

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

            /// Deletes the element at `index`,
            /// returning `None` if `index` is out of bounds.
            // TODO
            pub fn delete_at(&mut self, _index: $t) -> Option<&mut T> {
                todo![]
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

                    // where the new element will be inserted.
                    let element_idx = self.count;

                    // the first front element is also the back element.
                    if element_idx.get().unwrap() == 0 {
                        self.back = element_idx;
                    } else {
                        // otherwise update the previous front element
                        // self.mut_node_at(self.front)?.set_prev(&element_idx);
                        self.set_prev_at(self.front, element_idx)?;
                    }

                    // update the element count
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
        impl<T, S: Storage, const LEN: usize> [<$name$b>]<T, S, LEN> {
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

            /// Returns the `next` index of the node at `index`,
            /// or `None` if either the index is `None`, or it overflows.
            fn next_at(&self, index: [<$name$b Index>]) -> Option<[<$name$b Index>]> {
                Some(self.ref_node_at(index)?.next())
            }
            /// Sets the `next` index of the node at `index` with `new_next`.
            ///
            /// Returns `None` if either the index is `None`, or it overflows.
            fn set_next_at(&mut self, index: [<$name$b Index>], new_next: [<$name$b Index>])
                -> Option<()> {
                self.mut_node_at(index)?.set_next(new_next);
                Some(())
            }

            /// Unlinks all the nodes.
            ///
            /// Uses `value` to fill each node.
            #[inline]
            fn unlink_nodes(&mut self) {
                if LEN == 0 {
                    return;
                }
                for i in 1..LEN-1 {
                    self.nodes[i].unlink();
                }
            }
        }

        /// Private utility methods, when T: Clone
        impl<T: Clone, S: Storage, const LEN: usize> [<$name$b>]<T, S, LEN> {
            /// Resets all the nodes with the provided value, and unlinks them.
            ///
            /// Uses `value` to fill each node.
            #[inline]
            fn reset_nodes(&mut self, value: T) {
                if LEN == 0 {
                    return;
                }
                for i in 1..LEN-1 {
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

#[cfg(test)]
mod tests {
    use super::*;
    use core::mem::size_of;

    #[test]
    fn sizes() {
        /* 8-bit index list */

        assert_eq!(1, size_of::<LinkedList8Index>());
        let index8 = 1;

        // the size of a node is the sum of:
        // - the size of its 2 indexes (2 * 1)
        // - the size of T
        // - any extra padding (NOTE: may depend on the platform)
        assert_eq!(2 + 0 + 0, size_of::<LinkedList8Node<()>>());
        assert_eq![2 + 1 + 0, size_of::<LinkedList8Node::<u8>>()];
        assert_eq![2 + 2 + 0, size_of::<LinkedList8Node::<u16>>()];
        assert_eq![2 + 4 + 2, size_of::<LinkedList8Node::<u32>>()];
        assert_eq![2 + 8 + 6, size_of::<LinkedList8Node::<u64>>()];
        assert_eq![2 + 16 + 6, size_of::<LinkedList8Node::<u128>>()];

        // the size of a list of 10 × u8:
        assert_eq![33, size_of::<LinkedList8::<u8, (), 10>>()];
        assert_eq![33, 3 * index8 + (2 * index8 + size_of::<u8>()) * 10];

        // the size of a list of 0 elements:
        assert_eq![3, size_of::<LinkedList8::<u8, (), 0>>()];
        assert_eq![4, size_of::<LinkedList8::<u16, (), 0>>()];
        assert_eq![4, size_of::<LinkedList8::<u32, (), 0>>()];
        assert_eq![8, size_of::<LinkedList8::<u64, (), 0>>()];
        assert_eq![8, size_of::<LinkedList8::<u128, (), 0>>()];

        // the size of a list of 1 element:
        assert_eq![3 + 1 + 2, size_of::<LinkedList8::<u8, (), 1>>()];
        assert_eq![4 + 2 + 2, size_of::<LinkedList8::<u16, (), 1>>()];
        assert_eq![4 + 4 + 4, size_of::<LinkedList8::<u32, (), 1>>()];
        assert_eq![8 + 8 + 8, size_of::<LinkedList8::<u64, (), 1>>()];
        assert_eq![8 + 16 + 8, size_of::<LinkedList8::<u128, (), 1>>()];

        // on the heap
        assert_eq![16, size_of::<LinkedList8::<u8, Boxed, 10>>()];
        assert_eq![16, size_of::<LinkedList8::<u128, Boxed, 10>>()];

        /* 16-bit index list */

        assert_eq!(2, size_of::<LinkedList16Index>());
        let index16 = 2;

        // the size of a node is the sum of:
        // - the size of its 2 indexes (2 * 2)
        // - the size of T
        // - any extra padding (NOTE: may depend on the platform)
        assert_eq!(4 + 0, size_of::<LinkedList16Node<()>>());
        assert_eq![4 + 1 + 1, size_of::<LinkedList16Node::<u8>>()];
        assert_eq![4 + 2 + 0, size_of::<LinkedList16Node::<u16>>()];
        assert_eq![4 + 4 + 0, size_of::<LinkedList16Node::<u32>>()];
        assert_eq![4 + 8 + 4, size_of::<LinkedList16Node::<u64>>()];
        assert_eq![4 + 16 + 4, size_of::<LinkedList16Node::<u128>>()];

        // the size of a list of 10 × u16:
        assert_eq![66, size_of::<LinkedList16::<u16, (), 10>>()];
        assert_eq![66, 3 * index16 + (2 * index16 + size_of::<u16>()) * 10];

        // the size of a list of 0 elements:
        assert_eq![6, size_of::<LinkedList16::<u8, (), 0>>()];
        assert_eq![6, size_of::<LinkedList16::<u16, (), 0>>()];
        assert_eq![8, size_of::<LinkedList16::<u32, (), 0>>()];
        assert_eq![8, size_of::<LinkedList16::<u64, (), 0>>()];
        assert_eq![8, size_of::<LinkedList16::<u128, (), 0>>()];

        // the size of a list of 1 element:
        assert_eq![6 + 1 + 5, size_of::<LinkedList16::<u8, (), 1>>()];
        assert_eq![6 + 2 + 4, size_of::<LinkedList16::<u16, (), 1>>()];
        assert_eq![8 + 4 + 4, size_of::<LinkedList16::<u32, (), 1>>()];
        assert_eq![8 + 8 + 8, size_of::<LinkedList16::<u64, (), 1>>()];
        assert_eq![8 + 16 + 8, size_of::<LinkedList16::<u128, (), 1>>()];

        // on the heap
        assert_eq![16, size_of::<LinkedList16::<u8, Boxed, 10>>()];
        assert_eq![16, size_of::<LinkedList16::<u128, Boxed, 10>>()];


        /* 32-bit index list */

        assert_eq!(4, size_of::<LinkedList32Index>());
        let index16 = 4;

        assert_eq!(8 + 0 + 0, size_of::<LinkedList32Node<()>>());
        assert_eq![8 + 1 + 3, size_of::<LinkedList32Node::<u8>>()];
        assert_eq![8 + 2 + 2, size_of::<LinkedList32Node::<u16>>()];
        assert_eq![8 + 4 + 0, size_of::<LinkedList32Node::<u32>>()];
        assert_eq![8 + 8 + 0, size_of::<LinkedList32Node::<u64>>()];
        assert_eq![8 + 16 + 0, size_of::<LinkedList32Node::<u128>>()];

        // the size of a list of 0 elements:
        assert_eq![12, size_of::<LinkedList32::<u8, (), 0>>()];
        assert_eq![12, size_of::<LinkedList32::<u16, (), 0>>()];
        assert_eq![12, size_of::<LinkedList32::<u32, (), 0>>()];
        assert_eq![16, size_of::<LinkedList32::<u64, (), 0>>()];
        assert_eq![16, size_of::<LinkedList32::<u128, (), 0>>()];

        // the size of a list of 1 element:
        assert_eq![12 + 1 + 11, size_of::<LinkedList32::<u8, (), 1>>()];
        assert_eq![12 + 2 + 10, size_of::<LinkedList32::<u16, (), 1>>()];
        assert_eq![12 + 4 + 8, size_of::<LinkedList32::<u32, (), 1>>()];
        assert_eq![16 + 8 + 8, size_of::<LinkedList32::<u64, (), 1>>()];
        assert_eq![16 + 16 + 8, size_of::<LinkedList32::<u128, (), 1>>()];

        // on the heap
        assert_eq![24, size_of::<LinkedList32::<u8, Boxed, 10>>()];
        assert_eq![24, size_of::<LinkedList32::<u128, Boxed, 10>>()];

        /* misc. list sizes */

        // max 8-bit len with a byte per node occupies ± 0.75 KiB
        assert_eq![765, size_of::<LinkedList8::<u8, (), { u8::MAX as usize - 1 }>>()];
        // to store one node more we need 16-bit indexes, occupping 1.5 KiB
        assert_eq![1536, size_of::<LinkedList16::<u8, (), { u8::MAX as usize }>>()];
        // max 16-bit len with a byte per node occupies ± 384 KiB)
        assert_eq![393_210, size_of::<LinkedList16::<u8, (), { u16::MAX as usize - 1 }>>()];
        // to store one node more we need 32-bit indexes, occupping 768 KiB
        assert_eq![786_432, size_of::<LinkedList32::<u8, (), { u16::MAX as usize }>>()];
        // max 32-bit len with a byte per node occupies ± 48 GiB)
        assert_eq![51_539_607_540, size_of::<LinkedList32::<u8, (), { u32::MAX as usize - 1 }>>()];
    }

    #[test]
    fn WIP() {
    }
}
