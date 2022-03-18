// ladata::builder::lines
//
//! DataLine*s are generated here.
//

/// for defining DataLine*s
#[macro_export]
#[doc(hidden)]
macro_rules! define_line {
    (
        u: $uname:ident, t: $tname:ident, l: $lcname:ident,
        size: $B:literal, $b:literal,
    ) => {
        paste::paste! {
            // DEFINE DataLine*

            // array, Copy
            #[doc = "An array of [`"
            [< $uname $B Byte Copy With >] "`][crate::all::" [< $uname $B Byte Copy With >] "]" ]
            #[derive(Clone, Copy, Debug)]
            pub struct [< DataLine $B Byte Copy With >]<U: DataUnitsCopy, const SIZE: usize> {
                cells: [[< $uname $B Byte Copy With >]<U>; SIZE]
            }
            // array, Copy, non-With
            #[doc = "An array of [`"
            [< $uname $B Byte Copy >] "`][crate::all::" [< $uname $B Byte Copy >] "]" ]
            pub type [< DataLine $B Byte Copy >]<const SIZE: usize> =
                [< DataLine $B Byte Copy With >]<NoData, SIZE>;

            // array, non-Copy
            #[doc = "An array of [`"
            [< $uname $B Byte With >] "`][crate::all::" [< $uname $B Byte With >] "]" ]
            #[derive(Debug)]
            pub struct [< DataLine $B Byte With >]<U: DataUnits, const SIZE: usize> {
                cells: [[< $uname $B Byte With >]<U>; SIZE]
            }
            // array, non-Copy, non-With
            #[doc = "An array of [`" [< $uname $B Byte >] "`][crate::all::" [< $uname $B Byte >] "]" ]
            pub type [< DataLine $B Byte >]<const SIZE: usize> =
                [< DataLine $B Byte With >]<NoData, SIZE>;

            // DEFINE DataLineGrow*

            // vec, Copy
            #[doc = "A vector of [`"
            [< $uname $B Byte Copy With >] "`][crate::all::" [< $uname $B Byte Copy With >] "]" ]
            #[derive(Clone, Debug)]
            pub struct [< DataLineGrow $B Byte Copy With >]<U: DataUnitsCopy> {
                cells: Vec<[< $uname $B Byte Copy With >]<U>>
            }
            // vec, Copy, non-With
            #[doc = "A vector of [`" [< $uname $B Byte Copy >] "`][crate::all::" [< $uname $B Byte Copy >] "]" ]
            pub type [< DataLineGrow $B Byte Copy >] = [< DataLineGrow $B Byte Copy With >]<NoData>;

            // vec, non-Copy
            #[doc = "A vector of [`"
            [< $uname $B Byte Copy With >] "`][crate::all::" [< $uname $B Byte With >] "]" ]
            #[derive(Debug)]
            pub struct [< DataLineGrow $B Byte With >]<U: DataUnits> {
                cells: Vec<[< $uname $B Byte With >]<U>>
            }
            // vec, non-Copy, non-With
            #[doc = "A vector of [`" [< $uname $B Byte>] "`][crate::all::" [< $uname $B Byte >] "]" ]
            pub type [< DataLineGrow $B Byte >] = [< DataLineGrow $B Byte With >]<NoData>;

            // DEFINE DataLineCompact*

            // compact array, Copy
            #[doc = "A dense array of [`"
            [< $lcname $B Byte Copy >] "`][crate::all::" [< $lcname $B Byte Copy >] "]\n" ]
            ///
            #[derive(Clone, Copy, Debug)]
            pub struct [< DataLineCompact $B Byte Copy >]<const SIZE: usize> {
                // WIP
                _types: [[< $tname $B Byte Copy >]; SIZE],
                _cells: [[< $lcname $B Byte Copy >]; SIZE]
            }

            // WIP impl traits

            // impl<U: DataUnits> [< DataLineGrow $B Byte With >]<U> {
            //     fn new<I: Into<[<DataLineGrow $B Byte With>]<U>>>(&mut self, i: I ) -> Self {
            //         Self::from(i)
            //     }
            // }
            // impl<U, I> [< DataLineGrow $B Byte With >]<U>
            // where U: DataUnits, I: Into<[<DataLineGrow $B Byte With>]<U>> {
            //     fn new(&mut self, i: I ) -> Self {
            //         Self::from(i)
            //     }
            // }


            // From/Into Array, Copy
            impl<U: DataUnitsCopy, const SIZE: usize> From< [<DataLine $B Byte Copy With>]<U, SIZE> >
                for [[<$uname $B Byte Copy With>]<U>; SIZE] {
                fn from(from: [<DataLine $B Byte Copy With>]<U, SIZE>) -> Self {
                    from.cells
                }
            }
            impl<U: DataUnitsCopy, const SIZE: usize> From<[[<$uname $B Byte Copy With>]<U>; SIZE]>
                for [<DataLine $B Byte Copy With>]<U, SIZE> {
                fn from(from: [[<$uname $B Byte Copy With>]<U>; SIZE] ) -> Self {
                    [<DataLine $B Byte Copy With>] {
                        cells: from
                    }
                }
            }
            // From/Into Array, non-Copy
            impl<U: DataUnits, const SIZE: usize> From< [<DataLine $B Byte With>]<U, SIZE> >
                for [[<$uname $B Byte With>]<U>; SIZE] {
                fn from(from: [<DataLine $B Byte With>]<U, SIZE>) -> Self {
                    from.cells
                }
            }
            impl<U: DataUnits, const SIZE: usize> From<[[<$uname $B Byte With>]<U>; SIZE]>
                for [<DataLine $B Byte With>]<U, SIZE> {
                fn from(from: [[<$uname $B Byte With>]<U>; SIZE] ) -> Self {
                    [<DataLine $B Byte With>] {
                        cells: from
                    }
                }
            }

            // From/Into Vec, Copy
            impl<U: DataUnitsCopy> From< [<DataLineGrow $B Byte Copy With>]<U> > for Vec< [<$uname $B Byte Copy With>]<U> > {
                fn from(from: [<DataLineGrow $B Byte Copy With>]<U>) -> Self {
                    from.cells
                }
            }
            impl<U: DataUnitsCopy> From< Vec<[<$uname $B Byte Copy With>]<U>> > for [<DataLineGrow $B Byte Copy With>]<U> {
                fn from(from: Vec< [<$uname $B Byte Copy With>]<U> > ) -> Self {
                    [<DataLineGrow $B Byte Copy With>] {
                        cells: from
                    }
                }
            }

            // From/Into Vec, non-Copy
            impl<U: DataUnits> From< [<DataLineGrow $B Byte With>]<U> > for Vec< [<$uname $B Byte With>]<U> > {
                fn from(from: [<DataLineGrow $B Byte With>]<U>) -> Self {
                    from.cells
                }
            }
            impl<U: DataUnits> From< Vec<[<$uname $B Byte With>]<U>> > for [<DataLineGrow $B Byte With>]<U> {
                fn from(from: Vec< [<$uname $B Byte With>]<U> > ) -> Self {
                    [<DataLineGrow $B Byte With>] {
                        cells: from
                    }
                }
            }

        }

    };
}
