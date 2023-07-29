// ladata::list::link::singly::node

/// Generates a singly linked list backed by an array, with custom index size.
#[rustfmt::skip]
macro_rules! linked_list_array_node {

    // $name : name prefix. E.g.: SinglyLinked
    // $b : bit size
    ( $name:ident, $b:literal) => { devela::paste! {

        /// The singly list node.
        pub(super) struct [<$name$b Node>] <T> {
            /// The index of the next element, towards the back of the list.
            next: [<NonMaxIndex$b>],
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
        impl<T: Debug> fmt::Debug for [<$name$b Node>]<T> {
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
            //     next: [<NonMaxIndex$b>],
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
                    next: [<NonMaxIndex$b>]::none(),
                    data,
                }
            }

            //

            /// Returns this node's next index (towards the back).
            #[inline]
            pub(super) fn next(&self) -> [<NonMaxIndex$b>] {
                self.next
            }

            /// Sets this node's next index (towards the back).
            #[inline]
            pub(super) fn set_next(&mut self, index: [<NonMaxIndex$b>]) {
                self.next = index;
            }

            /// Unlinks the node, clearing the next index.
            #[inline]
            pub(super) fn unlink(&mut self) {
                self.next = [<NonMaxIndex$b>]::none();
            }

            /// Sets the `data` and unlinks the node, clearing the next index.
            #[inline]
            pub(super) fn reset(&mut self, data: T) {
                self.data = data;
                self.next = [<NonMaxIndex$b>]::none();
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
            pub(super) fn into_components(self) -> ([<NonMaxIndex$b>], T) {
                (self.next, self.data)
            }
        }
    }};
}
pub(super) use linked_list_array_node;
