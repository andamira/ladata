// ladata::builder::lines
//
//! DataLine*s are generated here.
//

/// for defining DataLine*s
#[macro_export]
#[doc(hidden)]
macro_rules! define_line {
    (
        c: $cname:ident, t: $tname:ident, b: $bname:ident,
        size: $B:literal, $b:literal,
    ) => {
        paste::paste! {
            // DEFINE DataLine*

            // array, Copy
            #[doc = "An **array** of [`"
            [< $cname $B Byte Copy With >] "`][crate::all::" [< $cname $B Byte Copy With >] "]" ]
            #[derive(Clone, Copy, Debug)]
            pub struct [< DataLine $B Byte Copy With >]<C: DataCellsCopy, const LEN: usize> {
                cells: [[< $cname $B Byte Copy With >]<C>; LEN]
            }
            // array, Copy, non-With
            #[doc = "An **array** of [`"
            [< $cname $B Byte Copy >] "`][crate::all::" [< $cname $B Byte Copy >] "]" ]
            pub type [< DataLine $B Byte Copy >]<const LEN: usize> =
                [< DataLine $B Byte Copy With >]<NoData, LEN>;

            // array, non-Copy
            #[doc = "An **array** of [`"
            [< $cname $B Byte With >] "`][crate::all::" [< $cname $B Byte With >] "]" ]
            #[derive(Debug)]
            pub struct [< DataLine $B Byte With >]<C: DataCells, const LEN: usize> {
                cells: [[< $cname $B Byte With >]<C>; LEN]
            }
            // array, non-Copy, non-With
            #[doc = "An array of [`" [< $cname $B Byte >] "`][crate::all::" [< $cname $B Byte >] "]" ]
            pub type [< DataLine $B Byte >]<const LEN: usize> =
                [< DataLine $B Byte With >]<NoData, LEN>;

            // WIP
            // type_aliases![l: $tname, size: $B, $b, "Copy", "data **Type**", "(Copy)" ];

            // DEFINE DataLineGrow*

            // vec, Copy
            #[doc = "A **vector** of [`"
            [< $cname $B Byte Copy With >] "`][crate::all::" [< $cname $B Byte Copy With >] "]" ]
            #[derive(Clone, Debug)]
            pub struct [< DataLineGrow $B Byte Copy With >]<C: DataCellsCopy> {
                cells: Vec<[< $cname $B Byte Copy With >]<C>>
            }
            // vec, Copy, non-With
            #[doc = "A **vector** of [`" [< $cname $B Byte Copy >] "`][crate::all::" [< $cname $B Byte Copy >] "]" ]
            pub type [< DataLineGrow $B Byte Copy >] = [< DataLineGrow $B Byte Copy With >]<NoData>;

            // vec, non-Copy
            #[doc = "A **vector** of [`"
            [< $cname $B Byte Copy With >] "`][crate::all::" [< $cname $B Byte With >] "]" ]
            #[derive(Debug)]
            pub struct [< DataLineGrow $B Byte With >]<C: DataCells> {
                cells: Vec<[< $cname $B Byte With >]<C>>
            }
            // vec, non-Copy, non-With
            #[doc = "A **vector** of [`" [< $cname $B Byte>] "`][crate::all::" [< $cname $B Byte >] "]" ]
            pub type [< DataLineGrow $B Byte >] = [< DataLineGrow $B Byte With >]<NoData>;

            // DEFINE DataLineCompact*

            // compact array, Copy
            #[doc = "A dense **array** of [`"
            [< $bname $B Byte Copy >] "`][crate::all::" [< $bname $B Byte Copy >] "]\n" ]
            ///
            #[derive(Clone, Copy, Debug)]
            pub struct [< DataLineCompact $B Byte Copy >]<const LEN: usize> {
                // WIP
                _types: [[< $tname $B Byte Copy >]; LEN],
                _cells: [[< $bname $B Byte Copy >]; LEN]
            }

            // WIP impl traits

            // impl<C: DataCells> [< DataLineGrow $B Byte With >]<C> {
            //     fn new<I: Into<[<DataLineGrow $B Byte With>]<C>>>(&mut self, i: I ) -> Self {
            //         Self::from(i)
            //     }
            // }
            // impl<C, I> [< DataLineGrow $B Byte With >]<C>
            // where c: DataCells, I: Into<[<DataLineGrow $B Byte With>]<C>> {
            //     fn new(&mut self, i: I ) -> Self {
            //         Self::from(i)
            //     }
            // }


            // From/Into Array, Copy
            impl<C: DataCellsCopy, const LEN: usize> From< [<DataLine $B Byte Copy With>]<C, LEN> >
                for [[<$cname $B Byte Copy With>]<C>; LEN] {
                fn from(from: [<DataLine $B Byte Copy With>]<C, LEN>) -> Self {
                    from.cells
                }
            }
            impl<C: DataCellsCopy, const LEN: usize> From<[[<$cname $B Byte Copy With>]<C>; LEN]>
                for [<DataLine $B Byte Copy With>]<C, LEN> {
                fn from(from: [[<$cname $B Byte Copy With>]<C>; LEN] ) -> Self {
                    [<DataLine $B Byte Copy With>] {
                        cells: from
                    }
                }
            }
            // From/Into Array, non-Copy
            impl<C: DataCells, const LEN: usize> From< [<DataLine $B Byte With>]<C, LEN> >
                for [[<$cname $B Byte With>]<C>; LEN] {
                fn from(from: [<DataLine $B Byte With>]<C, LEN>) -> Self {
                    from.cells
                }
            }
            impl<C: DataCells, const LEN: usize> From<[[<$cname $B Byte With>]<C>; LEN]>
                for [<DataLine $B Byte With>]<C, LEN> {
                fn from(from: [[<$cname $B Byte With>]<C>; LEN] ) -> Self {
                    [<DataLine $B Byte With>] {
                        cells: from
                    }
                }
            }

            // From/Into Vec, Copy
            impl<C: DataCellsCopy> From< [<DataLineGrow $B Byte Copy With>]<C> > for Vec< [<$cname $B Byte Copy With>]<C> > {
                fn from(from: [<DataLineGrow $B Byte Copy With>]<C>) -> Self {
                    from.cells
                }
            }
            impl<C: DataCellsCopy> From< Vec<[<$cname $B Byte Copy With>]<C>> > for [<DataLineGrow $B Byte Copy With>]<C> {
                fn from(from: Vec< [<$cname $B Byte Copy With>]<C> > ) -> Self {
                    [<DataLineGrow $B Byte Copy With>] {
                        cells: from
                    }
                }
            }

            // From/Into Vec, non-Copy
            impl<C: DataCells> From< [<DataLineGrow $B Byte With>]<C> > for Vec< [<$cname $B Byte With>]<C> > {
                fn from(from: [<DataLineGrow $B Byte With>]<C>) -> Self {
                    from.cells
                }
            }
            impl<C: DataCells> From< Vec<[<$cname $B Byte With>]<C>> > for [<DataLineGrow $B Byte With>]<C> {
                fn from(from: Vec< [<$cname $B Byte With>]<C> > ) -> Self {
                    [<DataLineGrow $B Byte With>] {
                        cells: from
                    }
                }
            }

        }

    };
}
