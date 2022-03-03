// ladata::macro_builder
//
//! Types are generated here and re-exported multiple times from public modules.
//
// # TOC
//
// - MACROS (exported & private):
//   - reexport
//   &
//   - define_all
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
    any::TypeId,
    mem::{align_of, size_of},
};

use super::traits::{DataCells, DataCellsCopy, DataTypes, DataTypesCopy};
use super::NoData;

/// macro useful for re-exporting types
//
// WIP
#[macro_export]
#[doc(hidden)]
macro_rules! reexport {
    // Generates `super::size` sub-modules
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

/// define at the same type: DataType*, DataCell* & DataUnsafeCell*
// depends on:
// - define_type!
// - define_cell!
// - define_unsafe_cell
macro_rules! define_all {
    // receive only Copy variants
    ( $tname:ident, $cname:ident, $ucname:ident,
      size: $B:literal, $b:literal,
      copy_variants: $( $cvdoc:literal, $cvname:ident, $cvtype:ty ),+ // no ending comma
    ) => {
        define_type!{
            $tname, size: $B, $b,
            copy_variants: $( $cvdoc, $cvname, $cvtype ),+
        }
        define_cell!{
            $cname, size: $B, $b,
            copy_variants: $( $cvdoc, $cvname, $cvtype ),+
        }
        define_unsafe_cell!{
            $ucname, size: $B, $b,
            copy_variants: $( $cvdoc, $cvname, $cvtype ),+
        }
    };
    // receive both Copy and non-Copy variants
    (
        $tname:ident, $cname:ident, $ucname:ident,
        size: $B:literal, $b:literal,
        copy_variants: $( $cvdoc:literal, $cvname:ident, $cvtype:ty ),+ ; // no ending comma
        noncopy_variants: $( $vdoc:literal, $vname:ident, $vtype:ty ),+   // "
    ) => {
        define_type!{
            $tname, size: $B, $b,
            copy_variants: $( $cvdoc, $cvname, $cvtype ),+ ;
            noncopy_variants: $( $vdoc, $vname, $vtype ),+
        }
        define_cell!{
            $cname, size: $B, $b,
            copy_variants: $( $cvdoc, $cvname, $cvtype ),+ ;
            noncopy_variants: $( $vdoc, $vname, $vtype ),+
        }
        // NOTE: only copy variants, for now
        define_unsafe_cell!{
            $ucname, size: $B, $b,
            copy_variants: $( $cvdoc, $cvname, $cvtype ),+
        }
    };
}
/// define: enum DataType*
macro_rules! define_type {
    // # receive only Copy variants (DataType)
    ( $tname:ident,
      size: $B:literal, $b:literal,
      copy_variants: $( $cvdoc:literal, $cvname:ident, $cvtype:ty ),+ // no ending comma
    ) => {
        paste::paste!{
            // ## copy version (DataType)
            #[doc = $B "-Byte / " $b "-bit " "data **Type** (extendable) (Copy)"]
            ///
            /// See also:
            #[doc = "- [" [<$tname $B ByteWith>] "][" [<$tname $B ByteWith>] "] -Copy" ]
            #[doc = "- [" [<$tname $B ByteCopy>] "][" [<$tname $B ByteCopy>] "] -With" ]
            #[doc = "- [" [<$tname $B Byte>]  "][" [<$tname $B Byte>] "] -Copy -With" ]
            #[derive(Clone, Copy)]
            pub enum [<$tname $B Byte Copy With>]<T: DataTypesCopy> {
                /// Represents the abscence of *data*.
                None,
                /// A custom *data cell* extension.
                With(T),
                $(
                    #[doc = $cvdoc]
                    $cvname,
                )*
            }
            impl_data_types![ [< $tname $B ByteCopyWith >], DataTypesCopy,
                is_copy: true,
                variants: $( $cvname, $cvtype ),+ ];
            impl<T: DataTypesCopy> DataTypesCopy for [< $tname $B ByteCopyWith >]<T> { }

            type_aliases![$tname, size: $B, $b, "Copy", "data **Type**", "(Copy)"];

            // ## non-copy version (DataType)
            #[doc = $B "-Byte / " $b "-bit " "data **Type** (extendable)"]
            ///
            /// See also:
            #[doc = "- [" [<$tname $B ByteCopyWith>] "][" [<$tname $B ByteCopyWith>] "] +Copy" ]
            #[doc = "- [" [<$tname $B ByteCopy>]  "][" [<$tname $B ByteCopy>] "] +Copy -With" ]
            #[doc = "- [" [<$tname $B Byte >] "][" [<$tname $B Byte >] "] -With" ]
            #[derive(Clone, Copy)]
            pub enum [<$tname $B Byte With>]<T: DataTypes> {
                /// Represents the abscence of *data*.
                None,
                /// A custom *data cell* extension.
                With(T),
                $(
                    #[doc = $cvdoc]
                    $cvname,
                )*
            }
            impl_data_types![ [< $tname $B Byte With >], DataTypes,
                is_copy: false,
                variants: $( $cvname, $cvtype ),+ ];

            type_aliases![$tname, size: $B, $b, "", "data **Type**", ""];
        }
    };
    // # receive both Copy and non-Copy variants (DataType)
    (
        $tname:ident,
        size: $B:literal, $b:literal,
        copy_variants: $( $cvdoc:literal, $cvname:ident, $cvtype:ty ),+ ;  // no ending comma
        noncopy_variants: $( $vdoc:literal, $vname:ident, $vtype:ty ),+    // "
    ) =>  {
        paste::paste!{
            // ## copy version (DataType)
            #[doc = $B "-Byte / " $b "-bit " "data **Type** (extendable) (Copy)"]
            ///
            /// See also:
            #[doc = "- [" [<$tname $B ByteWith>]  "][" [<$tname $B ByteWith>] "] -Copy" ]
            #[doc = "- [" [<$tname $B ByteCopy>] "][" [<$tname $B ByteCopy>] "] -With" ]
            #[doc = "- [" [<$tname $B Byte >] "][" [<$tname $B Byte >] "] -Copy -With" ]
            #[derive(Clone, Copy)]
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
            type_aliases![$tname, size: $B, $b, "Copy", "data **Type**", "(Copy)" ];
            impl_data_types![ [< $tname $B Byte Copy With >], DataTypesCopy,
                is_copy: true,
                variants: $( $cvname, $cvtype ),+ ];
            impl<T: DataTypesCopy> DataTypesCopy for [< $tname $B Byte Copy With >]<T> { }

            // ## non-copy version (DataType)
            #[doc = $B "-Byte / " $b "-bit " "data **Type** (extendable)"]
            ///
            /// See also:
            #[doc = "- [" [<$tname $B ByteCopyWith>] "][" [<$tname $B ByteCopyWith>] "] +Copy" ]
            #[doc = "- [" [<$tname $B ByteCopy>]  "][" [<$tname $B ByteCopy>] "] +Copy -With" ]
            #[doc = "- [" [<$tname $B Byte >] "][" [<$tname $B Byte >] "] -With" ]
            #[derive(Clone, Copy)]
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
            type_aliases![$tname, size: $B, $b, "", "data **Type**", ""];
            impl_data_types![ [< $tname $B Byte With >], DataTypes,
                is_copy: false,
                variants: $( $cvname, $cvtype ),+ , $( $vname, $vtype ),+ ];
        }
    };
}
/// define: enum DataCell*
macro_rules! define_cell {
    // # receive only Copy variants (DataCell)
    ( $cname:ident,
      size: $B:literal, $b:literal,
      copy_variants: $( $cvdoc:literal, $cvname:ident, $cvtype:ty ),+ // no ending comma
    ) => {
        paste::paste!{
            // ## copy version (DataCell)
            #[doc = $B "-Byte / " $b "-bit " "data **Cell** (extendable) (Copy)"]
            ///
            /// See also:
            #[doc = "- [" [<$cname $B ByteWith>] "][" [<$cname $B ByteWith>] "] -Copy" ]
            #[doc = "- [" [<$cname $B ByteCopy>] "][" [<$cname $B ByteCopy>] "] -With" ]
            #[doc = "- [" [<$cname $B Byte>]  "][" [<$cname $B Byte>] "] -Copy -With" ]
            #[derive(Clone, Copy)]
            pub enum [<$cname $B Byte Copy With>]<T: DataCells> {
                /// Represents the abscence of *data*.
                None,
                /// A custom *data cell* extension.
                With(T),
                $(
                    #[doc = $cvdoc]
                    $cvname($cvtype),
                )*
            }
            type_aliases![$cname, size: $B, $b, "Copy", "data **Cell**", "(Copy)"];
            impl_data_cells![ [< $cname $B Byte Copy With >], DataCellsCopy,
                is_copy: true,
                variants: $( $cvname, $cvtype ),+ ];
            impl<T: DataCellsCopy> DataCellsCopy for [< $cname $B Byte Copy With >]<T> { }

            // ## non-copy version (DataCell)
            #[doc = $B "-Byte / " $b "-bit " "data **Cell** (extendable)"]
            ///
            /// See also:
            #[doc = "- [" [<$cname $B ByteCopyWith>] "][" [<$cname $B ByteCopyWith>] "] +Copy" ]
            #[doc = "- [" [<$cname $B ByteCopy>]  "][" [<$cname $B ByteCopy>] "] +Copy -With" ]
            #[doc = "- [" [<$cname $B Byte >] "][" [<$cname $B Byte >] "] -With" ]
            // #[derive()]
            pub enum [<$cname $B Byte With>]<T: DataCells> {
                /// Represents the abscence of *data*.
                None,
                /// A custom *data cell* extension.
                With(T),
                $(
                    #[doc = $cvdoc]
                    $cvname($cvtype),
                )*
            }
            type_aliases![$cname, size: $B, $b, "", "data **Cell**", ""];
            impl_data_cells![ [< $cname $B Byte With >], DataCells,
                is_copy: false,
                variants: $( $cvname, $cvtype ),+ ];
        }
    };
    // # receive both Copy and non-Copy variants (DataCell)
    ( $cname:ident,
      size: $B:literal, $b:literal,
      copy_variants: $( $cvdoc:literal, $cvname:ident, $cvtype:ty ),+ ; // no ending comma
      noncopy_variants: $( $vdoc:literal, $vname:ident, $vtype:ty ),+   // "
    ) => {
        paste::paste!{
            // ## copy version (DataCell)
            #[doc = $B "-Byte/" $b "-bit " "data **Cell** (extendable) (Copy)"]
            ///
            /// See also:
            #[doc = "- [" [<$cname $B ByteWith>] "][" [<$cname $B ByteWith>] "] -Copy" ]
            #[doc = "- [" [<$cname $B ByteCopy>] "][" [<$cname $B ByteCopy>] "] -With" ]
            #[doc = "- [" [<$cname $B Byte>]  "][" [<$cname $B Byte>] "] -Copy -With" ]
            #[derive(Clone, Copy)]
            pub enum [<$cname $B Byte Copy With>]<T: DataCellsCopy> {
                /// Represents the abscence of *data*.
                None,
                /// A custom *data cell* extension.
                With(T),
                $(
                    #[doc = $cvdoc]
                    $cvname($cvtype),
                )*
            }
            type_aliases![$cname, size: $B, $b, "Copy", "data **Cell**", "(Copy)"];
            impl_data_cells![ [< $cname $B Byte Copy With >], DataCellsCopy,
                is_copy: true,
                variants: $( $cvname, $cvtype ),+ ];
            impl<T: DataCellsCopy> DataCellsCopy for [< $cname $B Byte Copy With >]<T> { }

            // ## non-copy version (DataCell)
            #[doc = $B "-Byte/" $b "-bit " "data **Cell** (extendable)"]
            ///
            /// See also:
            #[doc = "- [" [<$cname $B ByteCopyWith>] "][" [<$cname $B ByteCopyWith>] "] +Copy" ]
            #[doc = "- [" [<$cname $B ByteCopy>]  "][" [<$cname $B ByteCopy>] "] +Copy -With" ]
            #[doc = "- [" [<$cname $B Byte >] "][" [<$cname $B Byte >] "] -With" ]
            // #[derive()]
            pub enum [<$cname $B Byte With>]<T: DataCells> {
                /// Represents the abscence of *data*.
                None,
                /// A custom *data cell* extension.
                With(T),
                $(
                    #[doc = $cvdoc]
                    $cvname($cvtype),
                )*
                $(
                    #[doc = $vdoc]
                    $vname($vtype),
                )*
            }
            type_aliases![$cname, size: $B, $b, "", "data **Cell**", ""];
            impl_data_cells![ [< $cname $B Byte With >], DataCells,
                is_copy: false,
                variants: $( $cvname, $cvtype ),+ , $( $vname, $vtype ),+ ];
        }
    };
}
/// define: union DataUnsafeCell*
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
      copy_variants: $( $cvdoc:literal, $cvname:ident, $cvtype:ty ),+ // no ending comma
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
            // type_aliases![$cname, size: $B, $b, "Copy", "data cell", "(Copy)"];
        }
    };
}

// -----------------------------------------------------------------------------

/// define: types aliases
macro_rules! type_aliases {
    ( $name:ident, size: $B:literal, $b:literal,
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
      variants: $( $vname:ident, $vtype:ty ),+
    ) => {
        paste::paste!{
            impl<T: $tbound> DataTypes for $tname<T> {
                fn type_align(&self) -> usize {
                    use $tname::*;
                    match self {
                        None => align_of::<NoData>(),
                        With(o) => o.type_align(),
                        $( $vname => align_of::<$vtype>(), )*
                    }
                }
                fn type_size(&self) -> usize {
                    use $tname::*;
                    match self {
                        None => size_of::<NoData>(),
                        With(o) => o.type_size(),
                        $( $vname => size_of::<$vtype>(), )*
                    }
                }
                fn type_id(&self) -> TypeId {
                    use $tname::*;
                    match self {
                        None => TypeId::of::<NoData>(),
                        With(o) => o.type_id(),
                        $( $vname => TypeId::of::<$vtype>(), )*
                    }
                }
                fn is_copy(&self) -> bool { $is_copy }
            }
        }
    };
}

/// implement: DataCells trait
macro_rules! impl_data_cells {
    ( $tname:ident, $tbound:ident,
      is_copy: $is_copy:stmt,
      variants: $( $vname:ident, $vtype:ty ),+
    ) => {
        paste::paste! {
            impl<T: $tbound> DataCells for $tname<T> {
                fn is_copy(&self) -> bool { $is_copy }
            }
        }
    };
}

// DEFINITIONS
// -------------------------------------------------------------------------

define_all! {
    DataType, DataCell, DataUnsafeCell,
    size: 1, 8,
    copy_variants:
    "8-bit unsigned integer ", U8, u8,
    "8-bit signed integer", I8, i8,
    "Boolean value", Bool, bool
}

define_all! {
    DataType, DataCell, DataUnsafeCell,
    size: 2, 16,
    copy_variants:
    "8-bit unsigned integer ", U8, u8,
    "8-bit signed integer", I8, i8,
    "Boolean value", Bool, bool,
    "16-bit unsigned integer ", U16, u16,
    "16-bit signed integer", I16, i16,
}

define_all! {
    DataType, DataCell, DataUnsafeCell,
    size: 4, 32,
    copy_variants:
    "8-bit unsigned integer ", U8, u8,
    "8-bit signed integer", I8, i8,
    "Boolean value", Bool, bool,
    "16-bit unsigned integer ", U16, u16,
    "16-bit signed integer", I16, i16,
    "32-bit unsigned integer ", U32, u32,
    "32-bit signed integer", I32, i32,
    "32-bit floating-point number", F32, f32
}
define_all! {
    DataType, DataCell, DataUnsafeCell,
    size: 8, 64,
    copy_variants:
    "8-bit unsigned integer ", U8, u8,
    "8-bit signed integer", I8, i8,
    "Boolean value", Bool, bool,
    "16-bit unsigned integer ", U16, u16,
    "16-bit signed integer", I16, i16,
    "32-bit unsigned integer ", U32, u32,
    "32-bit signed integer", I32, i32,
    "32-bit floating-point number", F32, f32,
    "64-bit unsigned integer ", U64, u64,
    "64-bit signed integer", I64, i64,
    "64-bit floating-point number", F64, f64
}
define_all! {
    DataType, DataCell, DataUnsafeCell,
    size: 16, 128,
    copy_variants:
    "8-bit unsigned integer ", U8, u8,
    "8-bit signed integer", I8, i8,
    "Boolean value", Bool, bool,
    "16-bit unsigned integer ", U16, u16,
    "16-bit signed integer", I16, i16,
    "32-bit unsigned integer ", U32, u32,
    "32-bit signed integer", I32, i32,
    "32-bit floating-point number", F32, f32,
    "64-bit unsigned integer ", U64, u64,
    "64-bit signed integer", I64, i64,
    "64-bit floating-point number", F64, f64,
    "128-bit unsigned integer ", U128, u128,
    "128-bit signed integer", I128, i128
}

define_all! {
    DataType, DataCell, DataUnsafeCell,
    size: 32, 256,
    copy_variants:
    "8-bit unsigned integer ", U8, u8,
    "8-bit signed integer", I8, i8,
    "Boolean value", Bool, bool,
    "16-bit unsigned integer ", U16, u16,
    "16-bit signed integer", I16, i16,
    "32-bit unsigned integer ", U32, u32,
    "32-bit signed integer", I32, i32,
    "32-bit floating-point number", F32, f32,
    "64-bit unsigned integer ", U64, u64,
    "64-bit signed integer", I64, i64,
    "64-bit floating-point number", F64, f64,
    "128-bit unsigned integer ", U128, u128,
    "128-bit signed integer", I128, i128
    ; noncopy_variants:
    "string", String, std::string::String
}

define_all! {
    DataType, DataCell, DataUnsafeCell,
    size: 64, 512,
    copy_variants:
    "8-bit unsigned integer ", U8, u8,
    "8-bit signed integer", I8, i8,
    "Boolean value", Bool, bool,
    "16-bit unsigned integer ", U16, u16,
    "16-bit signed integer", I16, i16,
    "32-bit unsigned integer ", U32, u32,
    "32-bit signed integer", I32, i32,
    "32-bit floating-point number", F32, f32,
    "64-bit unsigned integer ", U64, u64,
    "64-bit signed integer", I64, i64,
    "64-bit floating-point number", F64, f64,
    "128-bit unsigned integer ", U128, u128,
    "128-bit signed integer", I128, i128
    ; noncopy_variants:
    "string", String, std::string::String
}

define_all! {
    DataType, DataCell, DataUnsafeCell,
    size: 128, 1024,
    copy_variants:
    "8-bit unsigned integer ", U8, u8,
    "8-bit signed integer", I8, i8,
    "Boolean value", Bool, bool,
    "16-bit unsigned integer ", U16, u16,
    "16-bit signed integer", I16, i16,
    "32-bit unsigned integer ", U32, u32,
    "32-bit signed integer", I32, i32,
    "32-bit floating-point number", F32, f32,
    "64-bit unsigned integer ", U64, u64,
    "64-bit signed integer", I64, i64,
    "64-bit floating-point number", F64, f64,
    "128-bit unsigned integer ", U128, u128,
    "128-bit signed integer", I128, i128
    ; noncopy_variants:
    "string", String, std::string::String
}
