// ladata::builder
//
//! Types are generated here and re-exported from several modules.
//
// # TOC
//
// - MACROS (exported & private):
//   - reexport
//   &
//   - define_all_sizes
//   - define_single_size
//   - define_type
//   - define_cell
//   - define_unsafe_cell
//   - type_aliases
//   - impl_data_types
//   - impl_data_cells
//   - impl_data_unsafe_cells
//
// - DEFINITIONS:
//   - DataType, DataCell, DataUnsafeCell @ Byte: 1, 2, 4, 8, 16, 32, 64, 128

use core::{
    // any::TypeId,
    convert::TryFrom,
    fmt,
    mem::{align_of, size_of},
};

use super::traits::{DataCells, DataCellsCopy, DataTypes, DataTypesCopy, DataUnsafeCells};
use super::NoData;

/// used for re-exporting types from public modules.
#[macro_export]
#[doc(hidden)]
macro_rules! reexport {
    // Generates `::size` sub-modules for all sizes.
    (mod_size $path:path; all_sizes) => {
        use crate::reexport;
        reexport![mod_size $path; 1, 8];
        reexport![mod_size $path; 2, 16];
        reexport![mod_size $path; 4, 32];
        reexport![mod_size $path; 8, 64];
        reexport![mod_size $path; 16, 128];
        reexport![mod_size $path; 32, 256];
        reexport![mod_size $path; 64, 512];
        reexport![mod_size $path; 128, 1024];
    };

    // Generates `::size` sub-modules for the pair of (Byte, bit) sizes.
    // - Each module is referenced as a submodule of each other.
    (mod_size $path:path; $B:literal, $b:literal ) => {
        paste::paste!{
            #[doc = $B " Byte data (== " $b " bit)" ]
            pub mod [< B $B >] {
                #[doc(inline)]
                pub use super::[< b $b >];
                crate::reexport![@cell_type super::$path; size: $B; Byte ByteWith ByteCopy ByteCopyWith ];
                crate::reexport![@unsafecell super::$path; size: $B; ByteCopy ];
            }
            #[doc = $b " bit data (== " $B " Byte)" ]
            pub mod [< b $b >] {
                #[doc(inline)]
                pub use super::[< B $B >];
                crate::reexport![@cell_type super::$path; size: $b; bit bitWith bitCopy bitCopyWith ];
                crate::reexport![@unsafecell super::$path; size: $b; bitCopy ];
            }
        }
    };

    // export Types of the given sizes in Bytes
    ($path:path; Byte: $( $B:literal )+ ) => {
        // reexport![@cells $path; Byte: $( $B )+ ];
        reexport![@cells_copy $path; Byte: $( $B )+ ];
        // reexport![@types $path; Byte: $( $B )+ ]; // TODO

    };

    // rethink
    (@gentype $path:path; $type:ty; Byte: $( $B:literal )+ ) => {
        $( paste::paste!{
            pub use $path::[< $type $B Byte With >];
            pub use $path::[< $type $B Byte >];
        } )+
    };

    (@cell_type $path:path; size: $size:literal; $( $suf:ident )+ ) => {
        crate::reexport![@multisuf $path; DataCell; size: $size ; $( $suf )+ ];
        crate::reexport![@multisuf $path; DataType; size: $size ; $( $suf )+ ];
        // crate::reexport![@multisuf $path; DataUnsafeCell; size: $size ; $( $suf )+ ]; //↓FIX
    };
    // TEMP: DataUnsafeCell can't accept non-copy (for now) so must be treated separately:
    (@unsafecell $path:path; size: $size:literal; $( $suf:ident )+ ) => {
        crate::reexport![@multisuf $path; DataUnsafeCell; size: $size ; $( $suf )+ ];
    };

    // 1 size, multiple suffixes (must include the Byte|bit part)
    (@multisuf $path:path; $type:ty; size: $size:literal; $( $suf:ident )+ ) => {
        $( paste::paste!{
            pub use $path::[< $type $size $suf >];
        } )+
    };
}

/// defines all sizes at the same time
macro_rules! define_all_sizes {
    (
        $tname:ident, $cname:ident, $ucname:ident,
        // 1-Byte / 8-bit
        copy_variants_1B: $( $cvdoc_1B:literal, $cvname_1B:ident, $cvtype_1B:ty ),* ,
        // noncopy_variants_1B: $( $vdoc_1B:literal, $vname_1B:ident, $vtype_1B:ty ),* ,

        // 2-Byte / 16-bit
        copy_variants_2B: $( $cvdoc_2B:literal, $cvname_2B:ident, $cvtype_2B:ty ),* ,
        // noncopy_variants_2B: $( $vdoc_2B:literal, $vname_2B:ident, $vtype_2B:ty ),* ,
        iusize_2B: $iusize_2B:meta,

        // 4-Byte / 32-bit
        copy_variants_4B: $( $cvdoc_4B:literal, $cvname_4B:ident, $cvtype_4B:ty ),* ,
        noncopy_variants_4B: $( $vdoc_4B:literal, $vname_4B:ident, $vtype_4B:ty ),* ,
        iusize_4B: $iusize_4B:meta,

        // 8-Byte / 64-bit
        copy_variants_8B: $( $cvdoc_8B:literal, $cvname_8B:ident, $cvtype_8B:ty ),* ,
        noncopy_variants_8B: $( $vdoc_8B:literal, $vname_8B:ident, $vtype_8B:ty ),* ,
        iusize_8B: $iusize_8B:meta,

        // 16-Byte / 128-bit
        copy_variants_16B: $( $cvdoc_16B:literal, $cvname_16B:ident, $cvtype_16B:ty ),* ,
        // noncopy_variants_16B: $( $vdoc_16B:literal, $vname_16B:ident, $vtype_16B:ty ),* ,

        // 32-Byte / 256-bit
        copy_variants_32B: $( $cvdoc_32B:literal, $cvname_32B:ident, $cvtype_32B:ty ),* ,
        noncopy_variants_32B: $( $vdoc_32B:literal, $vname_32B:ident, $vtype_32B:ty ),* ,
        noncopy_variants_32B_std: $( $vdoc_32B_std:literal, $vname_32B_std:ident, $vtype_32B_std:ty ),* ,

        // 64-Byte / 512-bit
        copy_variants_64B: $( $cvdoc_64B:literal, $cvname_64B:ident, $cvtype_64B:ty ),* ,
        // noncopy_variants_64B: $( $vdoc_64B:literal, $vname_64B:ident, $vtype_64B:ty ),* ,

        // 128-Byte / 1024-bit
        copy_variants_128B: $( $cvdoc_128B:literal, $cvname_128B:ident, $cvtype_128B:ty ),* ,
        noncopy_variants_128B: $( $vdoc_128B:literal, $vname_128B:ident, $vtype_128B:ty ),* ,

    ) => {
        // 1-Byte / 8-bit
        define_single_size! {
            $tname, $cname, $ucname,
            size: 1, 8,
            copy_variants: $( $cvdoc_1B, $cvname_1B, $cvtype_1B ),* ,
            noncopy_variants: ; // $( $vdoc_1B, $vname_1B, $vtype_1B ),* ,
            noncopy_variants_std: ;
            pointer:
        }
        // 2-Byte / 16-bit
        define_single_size! {
            $tname, $cname, $ucname,
            size: 2, 16,
            copy_variants:
                $( $cvdoc_1B, $cvname_1B, $cvtype_1B ),* ,
                $( $cvdoc_2B, $cvname_2B, $cvtype_2B ),* ,
            noncopy_variants: ;
                // $( $vdoc_1B, $vname_1B, $vtype_1B ),* ,
                // $( $vdoc_2B, $vname_2B, $vtype_2B ),* ,
            noncopy_variants_std: ;
            pointer: $iusize_2B
        }
        // 4-Byte / 32-bit
        define_single_size! {
            $tname, $cname, $ucname,
            size: 4, 32,
            copy_variants:
                $( $cvdoc_1B, $cvname_1B, $cvtype_1B ),* ,
                $( $cvdoc_2B, $cvname_2B, $cvtype_2B ),* ,
                $( $cvdoc_4B, $cvname_4B, $cvtype_4B ),* ,
            noncopy_variants:
                $( $vdoc_4B, $vname_4B, $vtype_4B ),* ;
            noncopy_variants_std: ;
            pointer: $iusize_2B, $iusize_4B
        }
        // 8-Byte / 32-bit
        define_single_size! {
            $tname, $cname, $ucname,
            size: 8, 64,
            copy_variants:
                $( $cvdoc_1B, $cvname_1B, $cvtype_1B ),* ,
                $( $cvdoc_2B, $cvname_2B, $cvtype_2B ),* ,
                $( $cvdoc_4B, $cvname_4B, $cvtype_4B ),* ,
                $( $cvdoc_8B, $cvname_8B, $cvtype_8B ),* ,
            noncopy_variants:
                $( $vdoc_4B, $vname_4B, $vtype_4B ),* ,
                $( $vdoc_8B, $vname_8B, $vtype_8B ),* ;
            noncopy_variants_std: ;
            pointer: $iusize_2B, $iusize_4B, $iusize_8B
        }
        // 16-Byte / 64-bit
        define_single_size! {
            $tname, $cname, $ucname,
            size: 16, 128,
            copy_variants:
                $( $cvdoc_1B, $cvname_1B, $cvtype_1B ),* ,
                $( $cvdoc_2B, $cvname_2B, $cvtype_2B ),* ,
                $( $cvdoc_4B, $cvname_4B, $cvtype_4B ),* ,
                $( $cvdoc_8B, $cvname_8B, $cvtype_8B ),* ,
                $( $cvdoc_16B, $cvname_16B, $cvtype_16B ),* ,
            noncopy_variants:
                $( $vdoc_4B, $vname_4B, $vtype_4B ),* ,
                $( $vdoc_8B, $vname_8B, $vtype_8B ),* ;
                // $( $vdoc_16B, $vname_16B, $vtype_16B ),* ;
            noncopy_variants_std: ;
            pointer: $iusize_2B, $iusize_4B, $iusize_8B
        }
        // 32-Byte / 128-bit
        define_single_size! {
            $tname, $cname, $ucname,
            size: 32, 256,
            copy_variants:
                $( $cvdoc_1B, $cvname_1B, $cvtype_1B ),* ,
                $( $cvdoc_2B, $cvname_2B, $cvtype_2B ),* ,
                $( $cvdoc_4B, $cvname_4B, $cvtype_4B ),* ,
                $( $cvdoc_8B, $cvname_8B, $cvtype_8B ),* ,
                $( $cvdoc_16B, $cvname_16B, $cvtype_16B ),* ,
                $( $cvdoc_32B, $cvname_32B, $cvtype_32B ),* ,
            noncopy_variants:
                $( $vdoc_4B, $vname_4B, $vtype_4B ),* ,
                $( $vdoc_8B, $vname_8B, $vtype_8B ),* ,
                // $( $vdoc_16B, $vname_16B, $vtype_16B ),* ,
                $( $vdoc_32B, $vname_32B, $vtype_32B ),* ;
            noncopy_variants_std:
                $( $vdoc_32B_std, $vname_32B_std, $vtype_32B_std ),* ;
            pointer: $iusize_2B, $iusize_4B, $iusize_8B
        }
        // 64-Byte / 512-bit
        define_single_size! {
            $tname, $cname, $ucname,
            size: 64, 512,
            copy_variants:
                $( $cvdoc_1B, $cvname_1B, $cvtype_1B ),* ,
                $( $cvdoc_2B, $cvname_2B, $cvtype_2B ),* ,
                $( $cvdoc_4B, $cvname_4B, $cvtype_4B ),* ,
                $( $cvdoc_8B, $cvname_8B, $cvtype_8B ),* ,
                $( $cvdoc_16B, $cvname_16B, $cvtype_16B ),* ,
                $( $cvdoc_32B, $cvname_32B, $cvtype_32B ),* ,
                $( $cvdoc_64B, $cvname_64B, $cvtype_64B ),* ,
            noncopy_variants:
                $( $vdoc_4B, $vname_4B, $vtype_4B ),* ,
                $( $vdoc_8B, $vname_8B, $vtype_8B ),* ,
                // $( $vdoc_16B, $vname_16B, $vtype_16B ),* ,
                $( $vdoc_32B, $vname_32B, $vtype_32B ),* ;
                // $( $vdoc_64B, $vname_64B, $vtype_64B ),* ;
            noncopy_variants_std:
                $( $vdoc_32B_std, $vname_32B_std, $vtype_32B_std ),* ;
            pointer: $iusize_2B, $iusize_4B, $iusize_8B
        }
        // 128-Byte / 1024-bit
        define_single_size! {
            $tname, $cname, $ucname,
            size: 128, 1024,
            copy_variants:
                $( $cvdoc_1B, $cvname_1B, $cvtype_1B ),* ,
                $( $cvdoc_2B, $cvname_2B, $cvtype_2B ),* ,
                $( $cvdoc_4B, $cvname_4B, $cvtype_4B ),* ,
                $( $cvdoc_8B, $cvname_8B, $cvtype_8B ),* ,
                $( $cvdoc_16B, $cvname_16B, $cvtype_16B ),* ,
                $( $cvdoc_32B, $cvname_32B, $cvtype_32B ),* ,
                $( $cvdoc_64B, $cvname_64B, $cvtype_64B ),* ,
                $( $cvdoc_128B, $cvname_128B, $cvtype_128B ),* ,
            noncopy_variants:
                $( $vdoc_4B, $vname_4B, $vtype_4B ),* ,
                $( $vdoc_8B, $vname_8B, $vtype_8B ),* ,
                // $( $vdoc_16B, $vname_16B, $vtype_16B ),* ,
                $( $vdoc_32B, $vname_32B, $vtype_32B ),* ,
                // $( $vdoc_64B, $vname_64B, $vtype_64B ),* ,
                $( $vdoc_128B, $vname_128B, $vtype_128B ),* ;
            noncopy_variants_std:
                $( $vdoc_32B_std, $vname_32B_std, $vtype_32B_std ),* ;
            pointer: $iusize_2B, $iusize_4B, $iusize_8B
        }
    };
}

/// for defining in one pass: DataType*, DataCell* & DataUnsafeCell*
macro_rules! define_single_size {
    (
    $tname:ident, $cname:ident, $ucname:ident,
    size: $B:literal, $b:literal,
    copy_variants: $( $cvdoc:literal, $cvname:ident, $cvtype:ty ),* ,
    noncopy_variants: $( $vdoc:literal, $vname:ident, $vtype:ty ),* ;
    noncopy_variants_std: $( $vdoc_std:literal, $vname_std:ident, $vtype_std:ty ),* ;
    pointer: $( $iusize:meta ),*
    ) => {
        define_type!{
            $tname, size: $B, $b,
            copy_variants: $( $cvdoc, $cvname, $cvtype ),* ,
            noncopy_variants: $( $vdoc, $vname, $vtype ),* ;
            noncopy_variants_std: $( $vdoc_std, $vname_std, $vtype_std ),* ;
            pointer: $( $iusize ),* ;
        }
        define_cell!{
            c: $cname, t:$tname, u:$ucname, size: $B, $b,
            copy_variants: $( $cvdoc, $cvname, $cvtype ),* ,
            noncopy_variants: $( $vdoc, $vname, $vtype ),* ;
            noncopy_variants_std: $( $vdoc_std, $vname_std, $vtype_std ),* ;
            pointer: $( $iusize ),* ;
        }
        define_unsafe_cell!{
            $ucname, size: $B, $b,
            copy_variants: $( $cvdoc, $cvname, $cvtype ),* ,
            pointer: $( $iusize ),* ;
        }
    };
}
/// for defining enum DataType*
macro_rules! define_type {
    (
        $tname:ident,
        size: $B:literal, $b:literal,
        copy_variants: $( $cvdoc:literal, $cvname:ident, $cvtype:ty ),* ,
        noncopy_variants: $( $vdoc:literal, $vname:ident, $vtype:ty ),* ;
        noncopy_variants_std: $( $vdoc_std:literal, $vname_std:ident, $vtype_std:ty ),* ;
        pointer: $( $iusize:meta ),* ;
    ) =>  {
        paste::paste!{
            // ## copy version (DataType)
            #[doc = $B "-Byte / " $b "-bit " "data **Type** (extendable) (Copy)"]
            ///
            /// See also:
            #[doc = "- [" [<$tname $B ByteWith>]  "][" [<$tname $B ByteWith>] "] -Copy" ]
            #[doc = "- [" [<$tname $B ByteCopy>] "][" [<$tname $B ByteCopy>] "] -With" ]
            #[doc = "- [" [<$tname $B Byte >] "][" [<$tname $B Byte >] "] -Copy -With" ]
            #[derive(Clone, Copy, Debug)]
            pub enum [< $tname $B Byte Copy With >]<T: DataTypesCopy> {
                /// Represents the absence of *data type*.
                None,
                /// A custom *data type* extension.
                With(T),

                #[cfg(any( $( $iusize ),* ))]
                #[doc = $b "-bit usize"]
                Usize,
                #[cfg(any( $( $iusize ),* ))]
                #[doc = $b "-bit isize"]
                Isize,

                $(
                    #[doc = $cvdoc ]
                    $cvname,
                )*
            }
            type_aliases![t: $tname, size: $B, $b, "Copy", "data **Type**", "(Copy)" ];
            impl_data_types![ [< $tname $B Byte Copy With >], DataTypesCopy,
                is_copy: true,
                copy_variants: $( $cvname, $cvtype ),* ;
                noncopy_variants: ;
                noncopy_variants_std: ;
                pointer: $( $iusize ),* ;
            ];
            impl<T: DataTypesCopy> DataTypesCopy for [< $tname $B Byte Copy With >]<T> { }

            // ## non-copy version (DataType)
            #[doc = $B "-Byte / " $b "-bit " "data **Type** (extendable)"]
            ///
            /// See also:
            #[doc = "- [" [<$tname $B ByteCopyWith>] "][" [<$tname $B ByteCopyWith>] "] +Copy" ]
            #[doc = "- [" [<$tname $B ByteCopy>]  "][" [<$tname $B ByteCopy>] "] +Copy -With" ]
            #[doc = "- [" [<$tname $B Byte >] "][" [<$tname $B Byte >] "] -With" ]
            #[derive(Clone, Copy, Debug)]
            pub enum [< $tname $B Byte With >]<T: DataTypes> {
                /// Represents the absence of *data type*.
                None,
                /// A custom *data type* extension.
                With(T),

                #[cfg(any( $( $iusize ),* ))]
                #[doc = $b "-bit usize"]
                Usize,
                #[cfg(any( $( $iusize ),* ))]
                #[doc = $b "-bit isize"]
                Isize,

                $(
                    #[cfg(feature = "std")]
                    #[doc = $vdoc_std ]
                    $vname_std,
                ),*

                $(
                    #[doc = $cvdoc ]
                    $cvname,
                )*
                $(
                    #[doc = $vdoc ]
                    $vname,
                )*
            }
            type_aliases![t: $tname, size: $B, $b, "", "data **Type**", ""];
            impl_data_types![ [< $tname $B Byte With >], DataTypes,
                is_copy: false,
                copy_variants: $( $cvname, $cvtype ),* ;
                noncopy_variants: $($vname, $vtype ),* ;
                noncopy_variants_std: $($vname_std, $vtype_std ),* ;
                pointer: $( $iusize ),* ;
            ];

            // DESIGN: nested loops :S
            // $(
            //     impl_From_variants![
            //         from: [< $tname $B Byte With>],
            //         for_: [<$tname $forBlist Byte With>]
            //         …WIP... (→ next macro)
            //     ];
            // )*
        }
    };
}

/// for defining enum DataCell*
macro_rules! define_cell {
    (
        c: $cname:ident, t: $tname:ident, u: $ucname:ident,
        size: $B:literal, $b:literal,
        copy_variants: $( $cvdoc:literal, $cvname:ident, $cvtype:ty ),* ,
        noncopy_variants: $( $vdoc:literal, $vname:ident, $vtype:ty ),* ;
        noncopy_variants_std: $( $vdoc_std:literal, $vname_std:ident, $vtype_std:ty ),* ;
        pointer: $( $iusize:meta ),* ;
    ) => {
        paste::paste!{
            // ## copy version (DataCell)
            #[doc = $B "-Byte/" $b "-bit " "data **Cell** (extendable) (Copy)"]
            ///
            /// See also:
            #[doc = "- [" [<$cname $B ByteWith>] "][" [<$cname $B ByteWith>] "] -Copy" ]
            #[doc = "- [" [<$cname $B ByteCopy>] "][" [<$cname $B ByteCopy>] "] -With" ]
            #[doc = "- [" [<$cname $B Byte>]  "][" [<$cname $B Byte>] "] -Copy -With" ]
            #[derive(Clone, Copy, Debug)]
            // pub enum [<$cname $B Byte Copy With>]<C: DataCellsCopy, T: DataTypesCopy> {
            pub enum [<$cname $B Byte Copy With>]<C: DataCellsCopy> {
                /// Represents the absence of *data*.
                None,
                /// A custom *data cell* extension.
                With(C),
                // _data_type(core::marker::PhantomData<*const T>), // WIP

                #[cfg(any($($iusize),*))]
                Usize(usize),
                #[cfg(any($($iusize),*))]
                Isize(isize),

                $(
                    #[doc = $cvdoc]
                    $cvname($cvtype),
                )*
            }
            type_aliases![c: $cname, size: $B, $b, "Copy", "data **Cell**", "(Copy)"];
            impl_data_cells![
                c: [< $cname $B Byte Copy With >], DataCellsCopy,
                t: [< $tname $B Byte Copy With >], DataTypesCopy,
                is_copy: true,
                copy_variants: $( $cvname, $cvtype ),* ;
                noncopy_variants: ;
                noncopy_variants_std: ;
                pointer: $( $iusize:meta ),* ;
            ];
            // impl<C: DataCellsCopy, T: DataTypesCopy> DataCellsCopy for [< $cname $B Byte Copy With >]<C, T> { }
            impl<C: DataCellsCopy> DataCellsCopy for [< $cname $B Byte Copy With >]<C> { }

            // ## non-copy version (DataCell)
            #[doc = $B "-Byte/" $b "-bit " "data **Cell** (extendable)"]
            ///
            /// See also:
            #[doc = "- [" [<$cname $B ByteCopyWith>] "][" [<$cname $B ByteCopyWith>] "] +Copy" ]
            #[doc = "- [" [<$cname $B ByteCopy>]  "][" [<$cname $B ByteCopy>] "] +Copy -With" ]
            #[doc = "- [" [<$cname $B Byte >] "][" [<$cname $B Byte >] "] -With" ]
            #[derive(Debug)]
            // pub enum [<$cname $B Byte With>]<C: DataCells, T: DataTypes> {
            pub enum [<$cname $B Byte With>]<C: DataCells> {
                /// Represents the absence of *data*.
                None,
                /// A custom *data cell* extension.
                With(C),
                // _data_type(core::marker::PhantomData<*const T>), // WIP

                #[cfg(any($($iusize),*))]
                Usize(usize),
                #[cfg(any($($iusize),*))]
                Isize(isize),

                $(
                    #[cfg(feature = "std")]
                    #[doc = $vdoc_std ]
                    $vname_std($vtype_std),
                ),*

                $(
                    #[doc = $cvdoc]
                    $cvname($cvtype),
                )*
                $(
                    #[doc = $vdoc]
                    $vname($vtype),
                )*
            }
            type_aliases![c: $cname, size: $B, $b, "", "data **Cell**", ""];
            impl_data_cells![
                c: [< $cname $B Byte With >], DataCells,
                t: [< $tname $B Byte With >], DataTypes,
                is_copy: false,
                copy_variants: $( $cvname, $cvtype ),* ;
                noncopy_variants: $($vname, $vtype ),* ;
                noncopy_variants_std: $($vname_std, $vtype_std ),* ;
                pointer: $( $iusize:meta ),* ;
            ];

            // From DataCell to contained value
            $( // Copy
                impl<C: DataCellsCopy> TryFrom<[<$cname $B Byte Copy With>]<C>> for $cvtype {
                    type Error = ();
                    fn try_from(c: [<$cname $B Byte Copy With>]<C>) -> Result<Self, Self::Error> {
                        match c {
                            [<$cname $B Byte Copy With>]::$cvname(c) => Ok(c),
                            _ => Err(()),
                        }
                    }

                }
            )*
            $( // non-Copy
                impl<C: DataCells> TryFrom<[<$cname $B Byte With>]<C>> for $vtype {
                    type Error = ();
                    fn try_from(c: [<$cname $B Byte With>]<C>) -> Result<Self, Self::Error> {
                        match c {
                            [<$cname $B Byte With>]::$vname(c) => Ok(c),
                            _ => Err(()),
                        }
                    }

                }
            )*
            $( // non-Copy (std)
                #[cfg(feature = "std")]
                impl<C: DataCells> TryFrom<[<$cname $B Byte With>]<C>> for $vtype_std {
                    type Error = ();
                    fn try_from(c: [<$cname $B Byte With>]<C>) -> Result<Self, Self::Error> {
                        match c {
                            [<$cname $B Byte With>]::$vname_std(c) => Ok(c),
                            _ => Err(()),
                        }
                    }

                }
            )*

            // from to-be-contained value to DataCell
            $( // Copy
                impl<C: DataCellsCopy> From<$cvtype> for [<$cname $B Byte Copy With>]<C> {
                    fn from(v: $cvtype) -> Self {
                        [<$cname $B Byte Copy With>]::$cvname(v)
                    }

                }
            )*
            $( // non-Copy
                impl<C: DataCells> From<$vtype> for [<$cname $B Byte With>]<C> {
                    fn from(v: $vtype) -> Self {
                        [<$cname $B Byte With>]::$vname(v)
                    }

                }
            )*
            $( // non-Copy (std)
                #[cfg(feature = "std")]
                impl<C: DataCells> From<$vtype_std> for [<$cname $B Byte With>]<C> {
                    fn from(v: $vtype_std) -> Self {
                        [<$cname $B Byte With>]::$vname_std(v)
                    }

                }
            )*

            // from DataCell to DataUnsafeCell
            impl<C: DataCellsCopy> From<[<$cname $B Byte Copy With>]<C>> for [<$ucname $B Byte Copy>] {
                fn from(cell: [<$cname $B Byte Copy With>]<C>) -> Self {
                    match cell {
                        [<$cname $B Byte Copy With>]::None => Self { None: NoData },
                        [<$cname $B Byte Copy With>]::With(_) => Self { None: NoData },

                        #[cfg(any($($iusize),*))]
                        [<$cname $B Byte Copy With>]::Usize(u) => Self { Usize: u },
                        #[cfg(any($($iusize),*))]
                        [<$cname $B Byte Copy With>]::Isize(i) => Self { Isize: i },

                        $(
                            [<$cname $B Byte Copy With>]::$cvname(v) => Self { $cvname: v }
                        ),*

                    }
                }

            }
        }
    };
}

/// for defining union DataUnsafeCell*
macro_rules! define_unsafe_cell {
    // # receive only Copy variants (DataUnsafeCell)
    (
        $ucname:ident,
        size: $B:literal, $b:literal,
        copy_variants: $( $cvdoc:literal, $cvname:ident, $cvtype:ty ),* ,
        // noncopy_variants: $( $cvdoc:literal, $cvname:ident, $cvtype:ty ),* ;
        pointer: $( $iusize:meta ),* ;
    ) => {
        paste::paste!{
            #[repr(C)]
            #[doc = $B "Byte / " $b "bit " "data *unsafe* **Cell**"]
            #[derive(Copy, Clone)]
            pub union [<$ucname $B Byte Copy>] {
                /// Represents the absence of *data*.
                pub None: NoData,

                #[cfg(any($($iusize),*))]
                pub Usize: usize,
                #[cfg(any($($iusize),*))]
                pub Isize: isize,

                $(
                    #[doc = $cvdoc]
                    pub $cvname: $cvtype,
                )*
            }
            // type aliases:
            #[doc = $B "Byte / " $b "bit " "data *unsafe* **Cell**"]
            pub type [< $ucname $b bit Copy >] = [< $ucname $B Byte Copy >];
            // TODO: unify with type_aliases
            // type_aliases![c: $ucname, size: $B, $b, "Copy", "data cell", "(Copy)"];

            // Debug
            impl fmt::Debug for [<$ucname $B Byte Copy>] {
                fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    write!(f, "{} {{...}}", stringify!{[< $ucname $B Byte Copy >]})
                }
            }

            impl_data_unsafe_cells![
                u: [< $ucname $B Byte Copy >],
            ];
        }
    };
}

// -----------------------------------------------------------------------------

/// define: types aliases
macro_rules! type_aliases {
    // DataCell aliases
    ( c: $name:ident, size: $B:literal, $b:literal,
      $nsuf:literal, $dsuf1:literal, $dsuf2:literal // name & doc suffixes
    ) => {
        paste::paste!{
            // without `With` byte version:
            #[doc = $B "-Byte / " $b "-bit " $dsuf1 " " $dsuf2 ]
            ///
            /// See also:
            #[doc = "- [" [<$name $B Byte $nsuf With>] "][" [<$name $B Byte $nsuf With>] "] +With" ]
            #[doc = "- [" [<$name $b bit $nsuf With>] "][" [<$name $b bit $nsuf With>] "] as bit +With" ]
            #[doc = "- [" [<$name $b bit $nsuf>] "][" [<$name $b bit $nsuf>] "] as bit" ]
            // pub type [< $name $B Byte $nsuf >] = [< $name $B Byte $nsuf With >]<NoData, NoData>;
            pub type [< $name $B Byte $nsuf >] = [< $name $B Byte $nsuf With >]<NoData>;

            // without `With` bit version
            #[doc = $B "-Byte / " $b "-bit " $dsuf1 " " $dsuf2 ]
            ///
            /// See also:
            #[doc = "- [" [<$name $b bit $nsuf With>] "][" [<$name $b bit $nsuf With>] "] +With" ]
            #[doc = "- [" [<$name $B Byte $nsuf>] "][" [<$name $B Byte $nsuf>] "] as Byte" ]
            #[doc = "- [" [<$name $B Byte $nsuf With>] "][" [<$name $B Byte $nsuf With>] "] as Byte +With" ]
            // pub type [< $name $b bit $nsuf >] = [< $name $B Byte $nsuf With >]<NoData, NoData>;
            pub type [< $name $b bit $nsuf >] = [< $name $B Byte $nsuf With >]<NoData>;

            // bit version:
            #[doc = $B "-Byte / " $b "-bit " $dsuf1 " (extendable) " $dsuf2 ]
            ///
            /// See also:
            #[doc = "- [" [<$name $b bit $nsuf>] "][" [<$name $b bit $nsuf>] "] -With" ]
            #[doc = "- [" [<$name $B Byte $nsuf With>] "][" [<$name $B Byte $nsuf With>] "] as Byte" ]
            #[doc = "- [" [<$name $B Byte $nsuf>] "][" [<$name $B Byte $nsuf>] "] as Byte -With" ]
            // pub type [< $name $b bit $nsuf With >]<C, T> = [< $name $B Byte $nsuf With >]<C, T>;
            pub type [< $name $b bit $nsuf With >]<C> = [< $name $B Byte $nsuf With >]<C>;
        }
    };
    // DataType aliases
    ( t: $name:ident, size: $B:literal, $b:literal,
      $nsuf:literal, $dsuf1:literal, $dsuf2:literal // name & doc suffixes
    ) => {
        paste::paste!{
            // without `With` byte version:
            #[doc = $B "-Byte / " $b "-bit " $dsuf1 " " $dsuf2 ]
            ///
            /// See also:
            #[doc = "- [" [<$name $B Byte $nsuf With>] "][" [<$name $B Byte $nsuf With>] "] +With" ]
            #[doc = "- [" [<$name $b bit $nsuf With>] "][" [<$name $b bit $nsuf With>] "] as bit +With" ]
            #[doc = "- [" [<$name $b bit $nsuf>] "][" [<$name $b bit $nsuf>] "] as bit" ]
            pub type [< $name $B Byte $nsuf >] = [< $name $B Byte $nsuf With >]<NoData>;

            // without `With` bit version
            #[doc = $B "-Byte / " $b "-bit " $dsuf1 " " $dsuf2 ]
            ///
            /// See also:
            #[doc = "- [" [<$name $b bit $nsuf With>] "][" [<$name $b bit $nsuf With>] "] +With" ]
            #[doc = "- [" [<$name $B Byte $nsuf>] "][" [<$name $B Byte $nsuf>] "] as Byte" ]
            #[doc = "- [" [<$name $B Byte $nsuf With>] "][" [<$name $B Byte $nsuf With>] "] as Byte +With" ]
            pub type [< $name $b bit $nsuf >] = [< $name $B Byte $nsuf With >]<NoData>;

            // bit version:
            #[doc = $B "-Byte / " $b "-bit " $dsuf1 " (extendable) " $dsuf2 ]
            ///
            /// See also:
            #[doc = "- [" [<$name $b bit $nsuf>] "][" [<$name $b bit $nsuf>] "] -With" ]
            #[doc = "- [" [<$name $B Byte $nsuf With>] "][" [<$name $B Byte $nsuf With>] "] as Byte" ]
            #[doc = "- [" [<$name $B Byte $nsuf>] "][" [<$name $B Byte $nsuf>] "] as Byte -With" ]
            pub type [< $name $b bit $nsuf With >]<T> = [< $name $B Byte $nsuf With >]<T>;
        }
    };
}

/// implement: DataTypes trait
macro_rules! impl_data_types {
    (
        $tname:ident, $tbound:ident,
        is_copy: $is_copy:stmt,
        copy_variants: $( $cvname:ident, $cvtype:ty ),* ;
        noncopy_variants: $( $vname:ident, $vtype:ty ),* ;
        noncopy_variants_std: $($vname_std:ident, $vtype_std:ty ),* ;
        pointer: $( $iusize:meta ),* ;
    ) => {
        paste::paste!{
            impl<T: $tbound> DataTypes for $tname<T> {
                fn data_align(&self) -> usize {
                    use $tname::*;
                    match self {
                        None => align_of::<NoData>(),
                        With(o) => o.data_align(),

                        #[cfg(any( $( $iusize ),* ))]
                        Usize => align_of::<usize>(),
                        #[cfg(any( $( $iusize ),* ))]
                        Isize => align_of::<isize>(),

                        $(
                            #[cfg(feature = "std")]
                            $vname_std => align_of::<$vtype_std>(),
                        ),*

                        $( $cvname => align_of::<$cvtype>(), )*
                        $( $vname => align_of::<$vtype>(), )*
                    }
                }
                fn data_size(&self) -> usize {
                    use $tname::*;
                    match self {
                        None => size_of::<NoData>(),
                        With(o) => o.data_size(),

                        #[cfg(any( $( $iusize ),* ))]
                        Usize => size_of::<usize>(),
                        #[cfg(any( $( $iusize ),* ))]
                        Isize => size_of::<isize>(),

                        $(
                            #[cfg(feature = "std")]
                            $vname_std => size_of::<$vtype_std>(),
                        ),*

                        $( $cvname => align_of::<$cvtype>(), )*
                        $( $vname => size_of::<$vtype>(), )*
                    }
                }
                fn is_copy(&self) -> bool { $is_copy }
            }
        }
    };
}

/// implement: DataCells trait
macro_rules! impl_data_cells {
    (
        c: $cname:ident, $cbound:ident,
        t: $tname:ident, $tbound:ident,
        is_copy: $is_copy:stmt,
        copy_variants: $( $cvname:ident, $cvtype:ty ),* ;
        noncopy_variants: $( $vname:ident, $vtype:ty ),* ;
        noncopy_variants_std: $($vname_std:ident, $vtype_std:ty ),* ;
        pointer: $( $iusize:meta ),* ;
    ) => {
        paste::paste! {
            // impl<C: $cbound, T: $tbound> DataCells for $cname<C, T> {
            impl<C: $cbound> DataCells for $cname<C> {

                fn is_copy(&self) -> bool { $is_copy }
            }
        }
    };
}
/// implement: DataUnsafeCells trait
macro_rules! impl_data_unsafe_cells {
    (
      u: $ucname:ident,
    ) => {
        // impl DataCells for $ucname {
        //     fn is_copy(&self) -> bool { true }
        // }
        // impl DataCellsCopy for $ucname {}
        unsafe impl DataUnsafeCells for $ucname {}
    };
}

// DEFINITIONS
// -------------------------------------------------------------------------

define_all_sizes! {
    DataType, DataCell, DataUnsafeCell,
    copy_variants_1B:
    "8-bit unsigned integer ", U8, u8,
    "8-bit signed integer", I8, i8,
    "8-bit [softposit](https://crates.io/crates/softposit)'s Posit without exponent", P8, softposit::P8,
    "1-byte array of bytes", ArrayByte1, [u8; 1],
    "Boolean value", Bool, bool,
    // noncopy_variants_1B:

    copy_variants_2B:
    "16-bit unsigned integer ", U16, u16,
    "16-bit signed integer", I16, i16,
    "16-bit [half](https://crates.io/crates/half) floating-point number (binary16)", F16, half::f16,
    "16-bit [half](https://crates.io/crates/half) floating-point number (bfloat16)", BF16, half::bf16,
    "16-bit [softposit](https://crates.io/crates/softposit)'s Posit with exp=1", P16, softposit::P16,
    "2-byte array of bytes", ArrayByte2, [u8; 2],
    // noncopy_variants_2B:
    iusize_2B: target_pointer_width = "16",

    copy_variants_4B:
    "32-bit unsigned integer ", U32, u32,
    "32-bit signed integer", I32, i32,
    "32-bit floating-point number", F32, f32,
    "4-byte array of bytes", ArrayByte4, [u8; 4],
    "4-byte char ", Char, char,
    noncopy_variants_4B:
    "8-bit [softposit](https://crates.io/crates/softposit)'s Quire without exponent", Q8, softposit::Q8,
    iusize_4B: target_pointer_width = "32",

    copy_variants_8B:
    "64-bit unsigned integer ", U64, u64,
    "64-bit signed integer", I64, i64,
    "64-bit floating-point number", F64, f64,
    "8-byte array of bytes", ArrayByte8, [u8; 8],
    "32-bit rational number", R32, num_rational::Ratio<i32>,
    "8-byte [arrayvec](https://crates.io/crates/arrayvec)'s ArrayString of len 4",
        ArrayString4, arrayvec::ArrayString<4>,
    noncopy_variants_8B:
    "16-bit [softposit](https://crates.io/crates/softposit)'s Quire with exp=1", Q16, softposit::Q16,
    iusize_8B: target_pointer_width = "64",

    copy_variants_16B:
    "128-bit unsigned integer ", U128, u128,
    "128-bit signed integer", I128, i128,
    "128-bit floating point number", F128, twofloat::TwoFloat,
    "16-byte array of bytes", ArrayByte16, [u8; 16],
    "16-byte [rust_decimal] Decimal number", Decimal, rust_decimal::Decimal, //
    "64-bit rational number", R64, num_rational::Ratio<i64>, //
    "16-byte [arrayvec](https://crates.io/crates/arrayvec)'s ArrayString of len 12",
        ArrayString12, arrayvec::ArrayString<12>,
    // noncopy_variants_16B:

    copy_variants_32B:
    "32-byte array of bytes", ArrayByte32, [u8; 32],
    "128-bit rational number", R128, num_rational::Ratio<i128>, //
    noncopy_variants_32B:
    "Big Integer", BigInt, num_bigint::BigInt,
    noncopy_variants_32B_std:
    "string", String, std::string::String,

    copy_variants_64B:
    "64-byte array", Array64B, [u8; 64],
    // noncopy_variants_64B:

    copy_variants_128B:
    "128-byte array", Array128B, [u8; 128],
    noncopy_variants_128B:
    "32-bit [softposit](https://crates.io/crates/softposit)'s Quire with exp=2", Q32, softposit::Q32,
    //
}
