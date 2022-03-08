// ladata::macro_builder
//
//! Types are generated here and re-exported multiple times from public modules.
//
// # TOC
//
// - MACROS (exported & private):
//   - reexport
//   &
//   - define_single_size
//   - define_type
//   - define_cell
//   - define_unsafe_cell
//   - type_aliases
//   - impl_data_types
//   - impl_data_cells
//
// - DEFINITIONS:
//   - DataType, DataCell, DataUnsafeCell @ Byte: 1, 2, 4, 8, 16, 32, 64, 128

use core::{
    // any::TypeId,
    convert::TryFrom,
    mem::{align_of, size_of},
};

use super::traits::{DataCells, DataCellsCopy, DataTypes, DataTypesCopy};
use super::NoData;

/// for re-exporting types from public modules
//
// WIP
#[macro_export]
#[doc(hidden)]
macro_rules! reexport {
    // Generates `::size` sub-modules for all sizes.
    (mod_size $path:path; all_sizes) => {
        use crate::reexport;
        reexport![mod_size $path; 1, 8];
        // WIP
        reexport![mod_size $path; 2, 16];
        reexport![mod_size $path; 4, 32];
        reexport![mod_size $path; 8, 64];
        reexport![mod_size $path; 16, 128];
        reexport![mod_size $path; 32, 256];
        // reexport![mod_size $path; 64, 512]; // WIP
        // reexport![mod_size $path; 128, 1024]; // WIP
    };

    // Generates `::size` sub-modules for the pair of (Byte, bit) sizes.
    // - Each module is referenced as a submodule of each other.
    (mod_size $path:path; $B:literal, $b:literal ) => {
        paste::paste!{
            #[doc = $B " Byte data (== " $b " bit)" ]
            pub mod [< B $B >] {
                #[doc(inline)]
                pub use super::[< b $b >];
                crate::reexport![@cell_type super::$path; size: $B; Byte ByteCopy ByteCopyWith ];
            }
            #[doc = $b " bit data (== " $B " Byte)" ]
            pub mod [< b $b >] {
                #[doc(inline)]
                pub use super::[< B $B >];
                crate::reexport![@cell_type super::$path; size: $b; bit bitCopy bitCopyWith ];
            }
        }
    };

    //
    // TODO:WIP: single-type in all the Byte/bit sizes
    // ::reexport![DataType, super::builder; all_sizes];
    // ($tu$path

    // TODO: export Types of the given sizes in bits
    // ($path:path; bit: $( $b:literal )+ ) => {
    // };

    ($type:ty, $path:path; bits: $( $b:literal )+ ) => {
        reexport![@gentype $path; $type; bit: $( $b )+ ];
    };
    ($type:ty, $path:path; Bytes: $( $B:literal )+ ) => {
        reexport![@gentype $path; $type; Byte: $( $B )+ ];
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
    // unused (unify?)
    // (@gentype_copy $path:path; $type:ty; Byte: $( $B:literal )+ ) => {
    //     $( paste::paste!{
    //         pub use $path::[< $type $B Byte Copy With >];
    //         pub use $path::[< $type $B Byte Copy >];
    //     } )+
    // };


    (@cell_type $path:path; size: $size:literal; $( $suf:ident )+ ) => {
        crate::reexport![@multisuf $path; DataCell; size: $size ; $( $suf )+ ];
        crate::reexport![@multisuf $path; DataType; size: $size ; $( $suf )+ ];
        // crate::reexport![@multisuf $path; DataUnsafeCell; size: $size ; $( $suf )+ ];
    };
    // DataUnsafeCell can't accept non-copy (for now) so must be treated separately:
    // (@unsafe_cell $path:path; size: $size:literal; $( $suf:ident )+ ) => {
    //     crate::reexport![@multisuf $path; DataCell; size: $size ; $( $suf )+ ];
    //     crate::reexport![@multisuf $path; DataType; size: $size ; $( $suf )+ ];
    //     // crate::reexport![@multisuf $path; DataUnsafeCell; size: $size ; $( $suf )+ ];
    // };

    // 1 size, multiple suffixes (must include the Byte|bit part)
    (@multisuf $path:path; $type:ty; size: $size:literal; $( $suf:ident )+ ) => {
        $( paste::paste!{
            pub use $path::[< $type $size $suf >];
        } )+
    };

    // MAYBE: multiple sizes, 1 suffix
    // (@multisize $path:path; $type:ty; size: $( $size:literal )+ ; $suffix:ident ) => {
    //     $( paste::paste!{
    //         pub use $path::[< $type $size $suffix >];
    //     } )+
    // };
}

/// defines all sizes at the same time
macro_rules! define_all_sizes {
    (
        $tname:ident, $cname:ident, $ucname:ident,
        // 1-Byte / 8-bit
        copy_variants_1B: $( $cvdoc_1B:literal, $cvname_1B:ident, $cvtype_1B:ty ),* ,
        noncopy_variants_1B: $( $vdoc_1B:literal, $vname_1B:ident, $vtype_1B:ty ),* ,
        // 2-Byte / 16-bit
        copy_variants_2B: $( $cvdoc_2B:literal, $cvname_2B:ident, $cvtype_2B:ty ),* ,
        noncopy_variants_2B: $( $vdoc_2B:literal, $vname_2B:ident, $vtype_2B:ty ),* ,
        // 4-Byte / 32-bit
        copy_variants_4B: $( $cvdoc_4B:literal, $cvname_4B:ident, $cvtype_4B:ty ),* ,
        noncopy_variants_4B: $( $vdoc_4B:literal, $vname_4B:ident, $vtype_4B:ty ),* ,
        // 8-Byte / 64-bit
        copy_variants_8B: $( $cvdoc_8B:literal, $cvname_8B:ident, $cvtype_8B:ty ),* ,
        // noncopy_variants_8B: $( $vdoc_8B:literal, $vname_8B:ident, $vtype_8B:ty ),* , // WIP
        // 16-Byte / 128-bit
        copy_variants_16B: $( $cvdoc_16B:literal, $cvname_16B:ident, $cvtype_16B:ty ),* ,
        // noncopy_variants_16B: $( $vdoc_16B:literal, $vname_16B:ident, $vtype_16B:ty ),* , // WIP
        // 32-Byte / 256-bit
        copy_variants_32B: $( $cvdoc_32B:literal, $cvname_32B:ident, $cvtype_32B:ty ),* ,
        noncopy_variants_32B: $( $vdoc_32B:literal, $vname_32B:ident, $vtype_32B:ty ),* ,
    ) => {
        // 1-Byte / 8-bit
        define_single_size! {
            $tname, $cname, $ucname,
            size: 1, 8,
            copy_variants: $( $cvdoc_1B, $cvname_1B, $cvtype_1B ),* ,
            noncopy_variants: $( $vdoc_1B, $vname_1B, $vtype_1B ),* ,
        }
        // 2-Byte / 16-bit
        define_single_size! {
            $tname, $cname, $ucname,
            size: 2, 16,
            copy_variants:
                $( $cvdoc_1B, $cvname_1B, $cvtype_1B ),* ,
                $( $cvdoc_2B, $cvname_2B, $cvtype_2B ),* ,
            noncopy_variants:
                $( $vdoc_1B, $vname_1B, $vtype_1B ),* ,
                $( $vdoc_2B, $vname_2B, $vtype_2B ),* ,
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
                $( $vdoc_1B, $vname_1B, $vtype_1B ),* ,
                $( $vdoc_2B, $vname_2B, $vtype_2B ),* ,
                $( $vdoc_4B, $vname_4B, $vtype_4B ),* ,
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
                $( $vdoc_1B, $vname_1B, $vtype_1B ),* ,
                $( $vdoc_2B, $vname_2B, $vtype_2B ),* ,
                $( $vdoc_4B, $vname_4B, $vtype_4B ),* ,
                // $( $vdoc_8B, $vname_8B, $vtype_8B ),* , // WIP
        }
        // 16-Byte / 128-bit
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
                $( $vdoc_1B, $vname_1B, $vtype_1B ),* ,
                $( $vdoc_2B, $vname_2B, $vtype_2B ),* ,
                $( $vdoc_4B, $vname_4B, $vtype_4B ),* ,
                // $( $vdoc_8B, $vname_8B, $vtype_8B ),* , // WIP
                // $( $vdoc_16B, $vname_16B, $vtype_16B ),* , // WIP
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
                $( $cvdoc_32B, $cvname_32B, $cvtype_32B ),* ,
            noncopy_variants:
                $( $vdoc_1B, $vname_1B, $vtype_1B ),* ,
                $( $vdoc_2B, $vname_2B, $vtype_2B ),* ,
                $( $vdoc_4B, $vname_4B, $vtype_4B ),* ,
                $( $cvdoc_16B, $cvname_16B, $cvtype_16B ),* ,
                // $( $vdoc_8B, $vname_8B, $vtype_8B ),* // WIP
                // $( $vdoc_16B, $vname_16B, $vtype_16B ),* // WIP
                $( $vdoc_32B, $vname_32B, $vtype_32B ),* ,
        }
    };
}

/// for defining in one pass: DataType*, DataCell* & DataUnsafeCell*
macro_rules! define_single_size {
    (
        $tname:ident, $cname:ident, $ucname:ident,
        size: $B:literal, $b:literal,
        copy_variants: $( $cvdoc:literal, $cvname:ident, $cvtype:ty ),* ,
        noncopy_variants: $( $vdoc:literal, $vname:ident, $vtype:ty ),* ,
    ) => {
        define_type!{
            $tname, size: $B, $b,
            copy_variants: $( $cvdoc, $cvname, $cvtype ),* ,
            noncopy_variants: $( $vdoc, $vname, $vtype ),* ,
        }
        define_cell!{
            c: $cname, t:$tname, size: $B, $b,
            copy_variants: $( $cvdoc, $cvname, $cvtype ),* ,
            noncopy_variants: $( $vdoc, $vname, $vtype ),* ,
        }
        define_unsafe_cell!{
            $ucname, size: $B, $b,
            copy_variants: $( $cvdoc, $cvname, $cvtype ),* ,
            // noncopy_variants: $( $vdoc, $vname, $vtype ),* , // NOTE: only copy variants, for now
        }
    };
}
/// for defining enum DataType*
macro_rules! define_type {
    (
        $tname:ident,
        size: $B:literal, $b:literal,
        copy_variants: $( $cvdoc:literal, $cvname:ident, $cvtype:ty ),* ,
        noncopy_variants: $( $vdoc:literal, $vname:ident, $vtype:ty ),* ,
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
                /// Represents the abscence of *data type*.
                None,
                /// A custom *data type* extension.
                With(T),
                $(
                    #[doc = $cvdoc ]
                    $cvname,
                )*
            }
            type_aliases![t: $tname, size: $B, $b, "Copy", "data **Type**", "(Copy)" ];
            impl_data_types![ [< $tname $B Byte Copy With >], DataTypesCopy,
                is_copy: true,
                copy_variants: $( $cvname, $cvtype ),* ;
                noncopy_variants: ];
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
                /// Represents the abscence of *data type*.
                None,
                /// A custom *data type* extension.
                With(T),
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
                noncopy_variants: $($vname, $vtype ),* ];
        }
    };
}
//WIP: From<current type> for bigger types (loop1)
//  - (loops) current type variant_names => bigger type variant_names
//
// this is a WIP from one size to bigger sizes. this will always work
// since bigger cell sizes include all smaller's variants
//
// ($forBlist arg is received as a list of bigger bytes at callsite) // TO
//
macro_rules! impl_From_variants {
    ( from:$from:ident,
      for_:$for:ident,
        )
        => {
            impl From<$from> for  {
                fn from(t: $from) -> Self {
                    match t {
                        $(
                            [< $tname $B Byte With >]::$cvname == Self::$cvname,
                        )*
                        $(
                            [< $tname $B Byte With >]::$vname == Self::$vname,
                        )*
                    }
                }
            }
    };
}

/// for defining enum DataCell*
macro_rules! define_cell {
    ( c: $cname:ident, t: $tname:ident,
      size: $B:literal, $b:literal,
      copy_variants: $( $cvdoc:literal, $cvname:ident, $cvtype:ty ),* ,
      noncopy_variants: $( $vdoc:literal, $vname:ident, $vtype:ty ),* ,
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
                /// Represents the abscence of *data*.
                None,
                /// A custom *data cell* extension.
                With(C),
                // _data_type(core::marker::PhantomData<*const T>), // WIP
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
                noncopy_variants:
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
                /// Represents the abscence of *data*.
                None,
                /// A custom *data cell* extension.
                With(C),
                // _data_type(core::marker::PhantomData<*const T>), // WIP
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
                noncopy_variants: $($vname, $vtype ),*
            ];

            // From DataCell to contained value
            $( // Copy
                impl<C: DataCells> TryFrom<[<$cname $B Byte With>]<C>> for $cvtype {
                    type Error = ();
                    fn try_from(c: [<$cname $B Byte With>]<C>) -> Result<Self, Self::Error> {
                        match c {
                            [<$cname $B Byte With>]::$cvname(c) => Ok(c),
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
            // from to-be-contained value to DataCell
            $( // Copy
                impl<C: DataCells> From<$cvtype> for [<$cname $B Byte With>]<C> {
                    fn from(v: $cvtype) -> Self {
                        [<$cname $B Byte With>]::$cvname(v)
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

        }
    };
}
/// for defining union DataUnsafeCell*
//
// pros:
// - doesn't use more size than required
// cons:
// - unsafe read (current `variant` is not stored)
// - can only be Copy (no drop managment)
// - there's no `With` variant. (MAYBE IMPROVE)
macro_rules! define_unsafe_cell {
    // # receive only Copy variants (DataUnsafeCell)
    ( $cname:ident,
      size: $B:literal, $b:literal,
      copy_variants: $( $cvdoc:literal, $cvname:ident, $cvtype:ty ),* ,
    ) => {
        paste::paste!{
            #[repr(C)]
            #[doc = $B "Byte/" $b "bit " "data *unsafe* **Cell**"]
            #[derive(Copy, Clone)]
            pub union [<$cname $B Byte Copy>] {
                /// Represents the abscence of *data*.
                pub none: NoData,
                $(
                    #[doc = $cvdoc]
                    pub $cvname: $cvtype,
                )*
            }
            // type aliases:
            #[doc = $B "Byte / " $b "bit " "data *unsafe* **Cell**"]
            pub type [< $cname $b bit Copy >] = [< $cname $B Byte Copy >];
            // TODO: unify with type_aliases
            // type_aliases![c: $cname, size: $B, $b, "Copy", "data cell", "(Copy)"];
        }
    };
}

// -----------------------------------------------------------------------------

/// define: types aliases
macro_rules! type_aliases {
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
    ( $tname:ident, $tbound:ident,
      is_copy: $is_copy:stmt,
      copy_variants: $( $cvname:ident, $cvtype:ty ),* ;
      noncopy_variants: $( $vname:ident, $vtype:ty ),*
    ) => {
        paste::paste!{
            impl<T: $tbound> DataTypes for $tname<T> {
                fn data_align(&self) -> usize {
                    use $tname::*;
                    match self {
                        None => align_of::<NoData>(),
                        With(o) => o.data_align(),
                        $( $cvname => align_of::<$cvtype>(), )*
                        $( $vname => align_of::<$vtype>(), )*
                    }
                }
                fn data_size(&self) -> usize {
                    use $tname::*;
                    match self {
                        None => size_of::<NoData>(),
                        With(o) => o.data_size(),
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
// WIP
macro_rules! impl_data_cells {
    (
      c: $cname:ident, $cbound:ident,
      t: $tname:ident, $tbound:ident,
      is_copy: $is_copy:stmt,
      copy_variants: $( $cvname:ident, $cvtype:ty ),* ;
      noncopy_variants: $( $vname:ident, $vtype:ty ),*
    ) => {
        paste::paste! {
            // impl<C: $cbound, T: $tbound> DataCells for $cname<C, T> {
            impl<C: $cbound> DataCells for $cname<C> {

                fn is_copy(&self) -> bool { $is_copy }
            }
        }
    };
}

/// implement From<DataCell*> for DataType*
// WIP: RETHINK
macro_rules! impl_from_cell_to_type {
    (c: $datacell:ident, $withcell:ident
     t: $datatype:ident, $withtype:ident
     copy_variants: $( $vname:ident, $vtype:ty )+
     ) => {
        /*
        impl<C: $withcell, T: $withtype> From< $datacell<C> >
            for $datatype<T> {
                fn from(cell: $cellname<C>) -> Self {
                    match cell {
                        $cname::None => Self::None,

                        // WIP
                        // $cname::With(c) => Self::With(c), // FIX
                        // $cname::With(_) => Self::None, // TEMP OK
                        //
                        // FIX: need to convert from C to T
                        // $cname::With(c) => Self::With(c.into()),
                        //
                        // TODO: implement a `map` method
                        //

                        $( $cname::$vname(_) => $tname::$vname, )+
                    }
                }
        }
        */
    };
}

// DEFINITIONS
// -------------------------------------------------------------------------
//
// TODO:
// - add usize
//   target_pointer_width
//   #[cfg(target_pointer_width = "16")]
//   #[cfg(target_pointer_width = "32")]
//   #[cfg(target_pointer_width = "64")]


define_all_sizes! {
    DataType, DataCell, DataUnsafeCell,
    copy_variants_1B:
    "8-bit unsigned integer ", U8, u8,
    "8-bit signed integer", I8, i8,
    "8-bit [softposit](https://crates.io/crates/softposit) without exponent bits", P8, softposit::P8,
    "1-byte array of bytes", ByteArray1, [u8; 1],
    "Boolean value", Bool, bool,
    // "8-bit [f8](https://crates.io/crates/f8) floating-point number constrained // MAYBE? (Debug)
    //     to a value within the inclusive range of [0, 1]", F8, f8::f8,
    noncopy_variants_1B:
    "8-bit [softposit](https://crates.io/crates/softposit) quire without exponent bits", Q8, softposit::Q8,

    copy_variants_2B:
    "16-bit unsigned integer ", U16, u16,
    "16-bit signed integer", I16, i16,
    "16-bit [half](https://crates.io/crates/half) floating-point number (binary16)", F16, half::f16,
    "16-bit [half](https://crates.io/crates/half) floating-point number (bfloat16)", BF16, half::bf16,
    "16-bit [softposit](https://crates.io/crates/softposit) posit with exponent 1", P16, softposit::P16,
    "2-byte array of bytes", ByteArray2, [u8; 2],
    // iusize_variants!(16) // TODO
    noncopy_variants_2B:
    "16-bit [softposit](https://crates.io/crates/softposit) quire without exponent bits", Q16, softposit::Q16,

    copy_variants_4B:
    "32-bit unsigned integer ", U32, u32,
    "32-bit signed integer", I32, i32,
    "32-bit floating-point number", F32, f32,
    "4-byte array of bytes", ByteArray4, [u8; 4],
    noncopy_variants_4B:
    "32-bit [softposit](https://crates.io/crates/softposit) quire with exponent 2", Q32, softposit::Q32,

    copy_variants_8B:
    "64-bit unsigned integer ", U64, u64,
    "64-bit signed integer", I64, i64,
    "64-bit floating-point number", F64, f64,
    "8-byte array of bytes", ByteArray8, [u8; 8],
    "32-bit rational number", R32, num_rational::Ratio<i32>,
    // ; noncopy_variants_8B: // WIP

    copy_variants_16B:
    "128-bit unsigned integer ", U128, u128,
    "128-bit signed integer", I128, i128,
    "128-bit floating point number", F128, twofloat::TwoFloat,
    "16-byte array of bytes", ByteArray16, [u8; 16],
    "16-byte [rust_decimal] Decimal number", Decimal, rust_decimal::Decimal, //
    "64-bit rational number", R64, num_rational::Ratio<i64>, //
    // ; noncopy_variants_16B: // WIP

    copy_variants_32B:
    "32-byte array of bytes", ByteArray32, [u8; 32],
    "128-bit rational number", R128, num_rational::Ratio<i128>, //
    noncopy_variants_32B:
    "string", String, std::string::String,
    "Big Integer", BigInt, num_bigint::BigInt,

    // WIP
    // copy_variants_64B:
    // "64-byte array", Array64B, [u8; 64],
    // noncopy_variants_64B:
    // //
    //
    // copy_variants_128B:
    // "128-byte array", Array128B, [u8; 128],
    // noncopy_variants_128B:
    // //
}
