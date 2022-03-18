// ladata::builder::lines
//
//! DataLine*s are generated here.
//

/// for defining DataLine*s
#[macro_export]
#[doc(hidden)]
macro_rules! define_line {
    (
        c: $cname:ident, t: $tname:ident, u: $ucname:ident,
        size: $B:literal, $b:literal,
    ) => {
        paste::paste! {
            // DEFINE DataArray*

            // array, Copy
            #[doc = "An array of [`"
            [< $cname $B Byte Copy With >] "`][crate::all::" [< $cname $B Byte Copy With >] "]" ]
            #[derive(Clone, Copy, Debug)]
            pub struct [< DataArray $B Byte Copy With >]<C: DataCellsCopy, const SIZE: usize> {
                cells: [[< $cname $B Byte Copy With >]<C>; SIZE]
            }
            // array, Copy, non-With
            #[doc = "An array of [`"
            [< $cname $B Byte Copy >] "`][crate::all::" [< $cname $B Byte Copy >] "]" ]
            pub type [< DataArray $B Byte Copy >]<const SIZE: usize> =
                [< DataArray $B Byte Copy With >]<NoData, SIZE>;

            // array, non-Copy
            #[doc = "An array of [`"
            [< $cname $B Byte With >] "`][crate::all::" [< $cname $B Byte With >] "]" ]
            #[derive(Debug)]
            pub struct [< DataArray $B Byte With >]<C: DataCells, const SIZE: usize> {
                cells: [[< $cname $B Byte With >]<C>; SIZE]
            }
            // array, non-Copy, non-With
            #[doc = "An array of [`" [< $cname $B Byte >] "`][crate::all::" [< $cname $B Byte >] "]" ]
            pub type [< DataArray $B Byte >]<const SIZE: usize> =
                [< DataArray $B Byte With >]<NoData, SIZE>;

            // DEFINE DataVec*

            // vec, Copy
            #[doc = "A vector of [`"
            [< $cname $B Byte Copy With >] "`][crate::all::" [< $cname $B Byte Copy With >] "]" ]
            #[derive(Clone, Debug)]
            pub struct [< DataVec $B Byte Copy With >]<C: DataCellsCopy> {
                cells: Vec<[< $cname $B Byte Copy With >]<C>>
            }
            // vec, Copy, non-With
            #[doc = "A vector of [`" [< $cname $B Byte Copy >] "`][crate::all::" [< $cname $B Byte Copy >] "]" ]
            pub type [< DataVec $B Byte Copy >] = [< DataVec $B Byte Copy With >]<NoData>;

            // vec, non-Copy
            #[doc = "A vector of [`"
            [< $cname $B Byte Copy With >] "`][crate::all::" [< $cname $B Byte With >] "]" ]
            #[derive(Debug)]
            pub struct [< DataVec $B Byte With >]<C: DataCells> {
                cells: Vec<[< $cname $B Byte With >]<C>>
            }
            // vec, non-Copy, non-With
            #[doc = "A vector of [`" [< $cname $B Byte>] "`][crate::all::" [< $cname $B Byte >] "]" ]
            pub type [< DataVec $B Byte >] = [< DataVec $B Byte With >]<NoData>;


            // DEFINE DataMiniArray

            // mini array, Copy
            #[doc = "A dense array of [`"
            [< $ucname $B Byte Copy >] "`][crate::all::" [< $ucname $B Byte Copy >] "]\n" ]
            ///
            #[derive(Clone, Copy, Debug)]
            pub struct [< DataMiniArray $B Byte Copy >]<const SIZE: usize> {
                types: [[< $tname $B Byte Copy >]; SIZE],
                cells: [[< $ucname $B Byte Copy >]; SIZE]
            }

            // WIP impl traits

            // impl<C: DataCells> [< DataVec $B Byte With >]<C> {
            //     fn new<I: Into<[<DataVec $B Byte With>]<C>>>(&mut self, i: I ) -> Self {
            //         Self::from(i)
            //     }
            // }
            // impl<C, I> [< DataVec $B Byte With >]<C>
            // where C: DataCells, I: Into<[<DataVec $B Byte With>]<C>> {
            //     fn new(&mut self, i: I ) -> Self {
            //         Self::from(i)
            //     }
            // }


            // From/Into Array, Copy
            impl<C: DataCellsCopy, const SIZE: usize> From< [<DataArray $B Byte Copy With>]<C, SIZE> >
                for [[<$cname $B Byte Copy With>]<C>; SIZE] {
                fn from(from: [<DataArray $B Byte Copy With>]<C, SIZE>) -> Self {
                    from.cells
                }
            }
            impl<C: DataCellsCopy, const SIZE: usize> From<[[<$cname $B Byte Copy With>]<C>; SIZE]>
                for [<DataArray $B Byte Copy With>]<C, SIZE> {
                fn from(from: [[<$cname $B Byte Copy With>]<C>; SIZE] ) -> Self {
                    [<DataArray $B Byte Copy With>] {
                        cells: from
                    }
                }
            }
            // From/Into Array, non-Copy
            impl<C: DataCells, const SIZE: usize> From< [<DataArray $B Byte With>]<C, SIZE> >
                for [[<$cname $B Byte With>]<C>; SIZE] {
                fn from(from: [<DataArray $B Byte With>]<C, SIZE>) -> Self {
                    from.cells
                }
            }
            impl<C: DataCells, const SIZE: usize> From<[[<$cname $B Byte With>]<C>; SIZE]>
                for [<DataArray $B Byte With>]<C, SIZE> {
                fn from(from: [[<$cname $B Byte With>]<C>; SIZE] ) -> Self {
                    [<DataArray $B Byte With>] {
                        cells: from
                    }
                }
            }

            // From/Into Vec, Copy
            impl<C: DataCellsCopy> From< [<DataVec $B Byte Copy With>]<C> > for Vec< [<$cname $B Byte Copy With>]<C> > {
                fn from(from: [<DataVec $B Byte Copy With>]<C>) -> Self {
                    from.cells
                }
            }
            impl<C: DataCellsCopy> From< Vec<[<$cname $B Byte Copy With>]<C>> > for [<DataVec $B Byte Copy With>]<C> {
                fn from(from: Vec< [<$cname $B Byte Copy With>]<C> > ) -> Self {
                    [<DataVec $B Byte Copy With>] {
                        cells: from
                    }
                }
            }

            // From/Into Vec, non-Copy
            impl<C: DataCells> From< [<DataVec $B Byte With>]<C> > for Vec< [<$cname $B Byte With>]<C> > {
                fn from(from: [<DataVec $B Byte With>]<C>) -> Self {
                    from.cells
                }
            }
            impl<C: DataCells> From< Vec<[<$cname $B Byte With>]<C>> > for [<DataVec $B Byte With>]<C> {
                fn from(from: Vec< [<$cname $B Byte With>]<C> > ) -> Self {
                    [<DataVec $B Byte With>] {
                        cells: from
                    }
                }
            }

        }

    };
}

