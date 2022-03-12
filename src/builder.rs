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
        copy_variants_1B_dep:
        $( $cvdoc_1B_dep:literal, $cvname_1B_dep:ident, $cvtype_1B_dep:ty, $cvfeature_1B_dep:literal ),* ,
        // noncopy_variants_1B: $( $vdoc_1B:literal, $vname_1B:ident, $vtype_1B:ty ),* ,

        // 2-Byte / 16-bit
        copy_variants_2B: $( $cvdoc_2B:literal, $cvname_2B:ident, $cvtype_2B:ty ),* ,
        copy_variants_2B_dep:
        $( $cvdoc_2B_dep:literal, $cvname_2B_dep:ident, $cvtype_2B_dep:ty, $cvfeature_2B_dep:literal ),* ,
        // noncopy_variants_2B: $( $vdoc_2B:literal, $vname_2B:ident, $vtype_2B:ty ),* ,
        iusize_2B: $iusize_2B:meta,

        // 4-Byte / 32-bit
        copy_variants_4B: $( $cvdoc_4B:literal, $cvname_4B:ident, $cvtype_4B:ty ),* ,
        copy_variants_4B_dep:
        $( $cvdoc_4B_dep:literal, $cvname_4B_dep:ident, $cvtype_4B_dep:ty, $cvfeature_4B_dep:literal ),* ,
        // noncopy_variants_4B: $( $vdoc_4B:literal, $vname_4B:ident, $vtype_4B:ty ),* ,
        noncopy_variants_4B_dep:
        $( $vdoc_4B_dep:literal, $vname_4B_dep:ident, $vtype_4B_dep:ty, $vfeature_4B_dep:literal ),* ,
        iusize_4B: $iusize_4B:meta,

        // 8-Byte / 64-bit
        copy_variants_8B: $( $cvdoc_8B:literal, $cvname_8B:ident, $cvtype_8B:ty ),* ,
        copy_variants_8B_dep:
        $( $cvdoc_8B_dep:literal, $cvname_8B_dep:ident, $cvtype_8B_dep:ty, $cvfeature_8B_dep:literal ),* ,
        // noncopy_variants_8B: $( $vdoc_8B:literal, $vname_8B:ident, $vtype_8B:ty ),* ,
        noncopy_variants_8B_dep:
        $( $vdoc_8B_dep:literal, $vname_8B_dep:ident, $vtype_8B_dep:ty, $vfeature_8B_dep:literal ),* ,
        iusize_8B: $iusize_8B:meta,

        // 16-Byte / 128-bit
        copy_variants_16B: $( $cvdoc_16B:literal, $cvname_16B:ident, $cvtype_16B:ty ),* ,
        copy_variants_16B_dep:
        $( $cvdoc_16B_dep:literal, $cvname_16B_dep:ident, $cvtype_16B_dep:ty, $cvfeature_16B_dep:literal ),* ,
        copy_variants_16B_std:
        $( $cvdoc_16B_std:literal, $cvname_16B_std:ident, $cvtype_16B_std:ty, $cvfeature_16B_std:literal ),* ,
        // noncopy_variants_16B: $( $vdoc_16B:literal, $vname_16B:ident, $vtype_16B:ty ),* ,

        // 32-Byte / 256-bit
        copy_variants_32B: $( $cvdoc_32B:literal, $cvname_32B:ident, $cvtype_32B:ty ),* ,
        copy_variants_32B_dep:
        $( $cvdoc_32B_dep:literal, $cvname_32B_dep:ident, $cvtype_32B_dep:ty, $cvfeature_32B_dep:literal ),* ,
        // noncopy_variants_32B: $( $vdoc_32B:literal, $vname_32B:ident, $vtype_32B:ty ),* ,
        noncopy_variants_32B_dep:
        $( $vdoc_32B_dep:literal, $vname_32B_dep:ident, $vtype_32B_dep:ty, $vfeature_32B_dep:literal ),* ,
        noncopy_variants_32B_std:
        $( $vdoc_32B_std:literal, $vname_32B_std:ident, $vtype_32B_std:ty, $vfeature_32B_std:literal ),* ,

        // 64-Byte / 512-bit
        copy_variants_64B: $( $cvdoc_64B:literal, $cvname_64B:ident, $cvtype_64B:ty ),* ,
        copy_variants_64B_dep:
        $( $cvdoc_64B_dep:literal, $cvname_64B_dep:ident, $cvtype_64B_dep:ty, $cvfeature_64B_dep:literal ),* ,
        // noncopy_variants_64B: $( $vdoc_64B:literal, $vname_64B:ident, $vtype_64B:ty ),* ,
        noncopy_variants_64B_dep:
        $( $vdoc_64B_dep:literal, $vname_64B_dep:ident, $vtype_64B_dep:ty, $vfeature_64B_dep:literal ),* ,

        // 128-Byte / 1024-bit
        copy_variants_128B: $( $cvdoc_128B:literal, $cvname_128B:ident, $cvtype_128B:ty ),* ,
        copy_variants_128B_dep:
        $( $cvdoc_128B_dep:literal, $cvname_128B_dep:ident, $cvtype_128B_dep:ty, $cvfeature_128B_dep:literal ),* ,
        noncopy_variants_128B: $( $vdoc_128B:literal, $vname_128B:ident, $vtype_128B:ty ),* ,

    ) => {
        // 1-Byte / 8-bit
        define_single_size! {
            $tname, $cname, $ucname,
            size: 1, 8,
            copy_variants: $( $cvdoc_1B, $cvname_1B, $cvtype_1B ),* ,
            copy_variants_dep:
                $( $cvdoc_1B_dep, $cvname_1B_dep, $cvtype_1B_dep, $cvfeature_1B_dep ),* ;
            copy_variants_std: ;
            noncopy_variants:
                // $( $vdoc_1B, $vname_1B, $vtype_1B ),* ,
            ;
            noncopy_variants_dep:
                // $( $vdoc_1B_dep, $vname_1B_dep, $vtype_1B_dep, $vfeature_1B_dep ),* ;
            ;
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
            copy_variants_dep:
                $( $cvdoc_1B_dep, $cvname_1B_dep, $cvtype_1B_dep, $cvfeature_1B_dep ),* ,
                $( $cvdoc_2B_dep, $cvname_2B_dep, $cvtype_2B_dep, $cvfeature_2B_dep ),* ;
            copy_variants_std: ;
            noncopy_variants: ;
                // $( $vdoc_1B, $vname_1B, $vtype_1B ),* ,
                // $( $vdoc_2B, $vname_2B, $vtype_2B ),* ,
            noncopy_variants_dep:
                // $( $vdoc_1B_dep, $vname_1B_dep, $vtype_1B_dep, $vfeature_1B_dep ),* ,
                // $( $vdoc_2B_dep, $vname_2B_dep, $vtype_2B_dep, $vfeature_2B_dep ),* ;
            ;
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
            copy_variants_dep:
                $( $cvdoc_1B_dep, $cvname_1B_dep, $cvtype_1B_dep, $cvfeature_1B_dep ),* ,
                $( $cvdoc_2B_dep, $cvname_2B_dep, $cvtype_2B_dep, $cvfeature_2B_dep ),* ,
                $( $cvdoc_4B_dep, $cvname_4B_dep, $cvtype_4B_dep, $cvfeature_4B_dep ),* ;
            copy_variants_std: ;
            noncopy_variants: ;
                // $( $vdoc_1B, $vname_1B, $vtype_1B ),* ,
                // $( $vdoc_2B, $vname_2B, $vtype_2B ),* ,
                // $( $vdoc_4B, $vname_4B, $vtype_4B ),* ;
            noncopy_variants_dep:
                // $( $vdoc_1B_dep, $vname_1B_dep, $vtype_1B_dep, $vfeature_1B_dep ),* ,
                // $( $vdoc_2B_dep, $vname_2B_dep, $vtype_2B_dep, $vfeature_2B_dep ),* ,
                $( $vdoc_4B_dep, $vname_4B_dep, $vtype_4B_dep, $vfeature_4B_dep ),* ;
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
            copy_variants_dep:
                $( $cvdoc_1B_dep, $cvname_1B_dep, $cvtype_1B_dep, $cvfeature_1B_dep ),* ,
                $( $cvdoc_2B_dep, $cvname_2B_dep, $cvtype_2B_dep, $cvfeature_2B_dep ),* ,
                $( $cvdoc_4B_dep, $cvname_4B_dep, $cvtype_4B_dep, $cvfeature_4B_dep ),* ,
                $( $cvdoc_8B_dep, $cvname_8B_dep, $cvtype_8B_dep, $cvfeature_8B_dep ),* ;
            copy_variants_std: ;
            noncopy_variants: ;
                // $( $vdoc_1B, $vname_1B, $vtype_1B ),* ,
                // $( $vdoc_2B, $vname_2B, $vtype_2B ),* ,
                // $( $vdoc_4B, $vname_4B, $vtype_4B ),* ,
                // $( $vdoc_8B, $vname_8B, $vtype_8B ),* ;
            noncopy_variants_dep:
                // $( $vdoc_1B_dep, $vname_1B_dep, $vtype_1B_dep, $vfeature_1B_dep ),* ,
                // $( $vdoc_2B_dep, $vname_2B_dep, $vtype_2B_dep, $vfeature_2B_dep ),* ,
                $( $vdoc_4B_dep, $vname_4B_dep, $vtype_4B_dep, $vfeature_4B_dep ),* ,
                $( $vdoc_8B_dep, $vname_8B_dep, $vtype_8B_dep, $vfeature_8B_dep ),* ;
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
            copy_variants_dep:
                $( $cvdoc_1B_dep, $cvname_1B_dep, $cvtype_1B_dep, $cvfeature_1B_dep ),* ,
                $( $cvdoc_2B_dep, $cvname_2B_dep, $cvtype_2B_dep, $cvfeature_2B_dep ),* ,
                $( $cvdoc_4B_dep, $cvname_4B_dep, $cvtype_4B_dep, $cvfeature_4B_dep ),* ,
                $( $cvdoc_8B_dep, $cvname_8B_dep, $cvtype_8B_dep, $cvfeature_8B_dep ),* ,
                $( $cvdoc_16B_dep, $cvname_16B_dep, $cvtype_16B_dep, $cvfeature_16B_dep ),* ;
            copy_variants_std:
                $( $cvdoc_16B_std, $cvname_16B_std, $cvtype_16B_std, $cvfeature_16B_std ),* ;
            noncopy_variants: ;
                // $( $vdoc_1B, $vname_1B, $vtype_1B ),* ,
                // $( $vdoc_2B, $vname_2B, $vtype_2B ),* ,
                // $( $vdoc_4B, $vname_4B, $vtype_4B ),* ,
                // $( $vdoc_8B, $vname_8B, $vtype_8B ),* ;
                // $( $vdoc_16B, $vname_16B, $vtype_16B ),* ;
            noncopy_variants_dep:
                // $( $vdoc_1B_dep, $vname_1B_dep, $vtype_1B_dep, $vfeature_1B_dep ),* ,
                // $( $vdoc_2B_dep, $vname_2B_dep, $vtype_2B_dep, $vfeature_2B_dep ),* ,
                $( $vdoc_4B_dep, $vname_4B_dep, $vtype_4B_dep, $vfeature_4B_dep ),* ,
                $( $vdoc_8B_dep, $vname_8B_dep, $vtype_8B_dep, $vfeature_8B_dep ),* ;
                // $( $vdoc_16B_dep, $vname_16B_dep, $vtype_16B_dep, $vfeature_16B_dep ),* ;
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
            copy_variants_dep:
                $( $cvdoc_1B_dep, $cvname_1B_dep, $cvtype_1B_dep, $cvfeature_1B_dep ),* ,
                $( $cvdoc_2B_dep, $cvname_2B_dep, $cvtype_2B_dep, $cvfeature_2B_dep ),* ,
                $( $cvdoc_4B_dep, $cvname_4B_dep, $cvtype_4B_dep, $cvfeature_4B_dep ),* ,
                $( $cvdoc_8B_dep, $cvname_8B_dep, $cvtype_8B_dep, $cvfeature_8B_dep ),* ,
                $( $cvdoc_16B_dep, $cvname_16B_dep, $cvtype_16B_dep, $cvfeature_16B_dep ),* ,
                $( $cvdoc_32B_dep, $cvname_32B_dep, $cvtype_32B_dep, $cvfeature_32B_dep ),* ;
            copy_variants_std:
                $( $cvdoc_16B_std, $cvname_16B_std, $cvtype_16B_std, $cvfeature_16B_std ),* ;
            noncopy_variants:
                // $( $vdoc_1B, $vname_1B, $vtype_1B ),* ,
                // $( $vdoc_2B, $vname_2B, $vtype_2B ),* ,
                // $( $vdoc_4B, $vname_4B, $vtype_4B ),* ,
                // $( $vdoc_8B, $vname_8B, $vtype_8B ),* ,
                // $( $vdoc_16B, $vname_16B, $vtype_16B ),* ,
                // $( $vdoc_32B, $vname_32B, $vtype_32B ),* ;
                ;
            noncopy_variants_dep:
                // $( $vdoc_1B_dep, $vname_1B_dep, $vtype_1B_dep, $vfeature_1B_dep ),* ,
                // $( $vdoc_2B_dep, $vname_2B_dep, $vtype_2B_dep, $vfeature_2B_dep ),* ,
                $( $vdoc_4B_dep, $vname_4B_dep, $vtype_4B_dep, $vfeature_4B_dep ),* ,
                $( $vdoc_8B_dep, $vname_8B_dep, $vtype_8B_dep, $vfeature_8B_dep ),* ,
                // $( $vdoc_16B_dep, $vname_16B_dep, $vtype_16B_dep, $vfeature_16B_dep ),* ,
                $( $vdoc_32B_dep, $vname_32B_dep, $vtype_32B_dep, $vfeature_32B_dep ),* ;
            noncopy_variants_std:
                $( $vdoc_32B_std, $vname_32B_std, $vtype_32B_std, $vfeature_32B_std ),* ;
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
            copy_variants_dep:
                $( $cvdoc_1B_dep, $cvname_1B_dep, $cvtype_1B_dep, $cvfeature_1B_dep ),* ,
                $( $cvdoc_2B_dep, $cvname_2B_dep, $cvtype_2B_dep, $cvfeature_2B_dep ),* ,
                $( $cvdoc_4B_dep, $cvname_4B_dep, $cvtype_4B_dep, $cvfeature_4B_dep ),* ,
                $( $cvdoc_8B_dep, $cvname_8B_dep, $cvtype_8B_dep, $cvfeature_8B_dep ),* ,
                $( $cvdoc_16B_dep, $cvname_16B_dep, $cvtype_16B_dep, $cvfeature_16B_dep ),* ,
                $( $cvdoc_32B_dep, $cvname_32B_dep, $cvtype_32B_dep, $cvfeature_32B_dep ),* ,
                $( $cvdoc_64B_dep, $cvname_64B_dep, $cvtype_64B_dep, $cvfeature_64B_dep ),* ;
            copy_variants_std:
                $( $cvdoc_16B_std, $cvname_16B_std, $cvtype_16B_std, $cvfeature_16B_std ),* ;
            noncopy_variants:
                // $( $vdoc_1B, $vname_1B, $vtype_1B ),* ,
                // $( $vdoc_2B, $vname_2B, $vtype_2B ),* ,
                // $( $vdoc_4B, $vname_4B, $vtype_4B ),* ,
                // $( $vdoc_8B, $vname_8B, $vtype_8B ),* ,
                // $( $vdoc_16B, $vname_16B, $vtype_16B ),* ,
                // $( $vdoc_32B, $vname_32B, $vtype_32B ),* ;
                // $( $vdoc_64B, $vname_64B, $vtype_64B ),* ;
                ;
            noncopy_variants_dep:
                // $( $vdoc_1B_dep, $vname_1B_dep, $vtype_1B_dep, $vfeature_1B_dep ),* ,
                // $( $vdoc_2B_dep, $vname_2B_dep, $vtype_2B_dep, $vfeature_2B_dep ),* ,
                $( $vdoc_4B_dep, $vname_4B_dep, $vtype_4B_dep, $vfeature_4B_dep ),* ,
                $( $vdoc_8B_dep, $vname_8B_dep, $vtype_8B_dep, $vfeature_8B_dep ),* ,
                // $( $vdoc_16B_dep, $vname_16B_dep, $vtype_16B_dep, $vfeature_16B_dep ),* ,
                $( $vdoc_32B_dep, $vname_32B_dep, $vtype_32B_dep, $vfeature_32B_dep ),* ,
                $( $vdoc_64B_dep, $vname_64B_dep, $vtype_64B_dep, $vfeature_64B_dep ),* ;
            noncopy_variants_std:
                $( $vdoc_32B_std, $vname_32B_std, $vtype_32B_std, $vfeature_32B_std ),* ;
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
            copy_variants_dep:
                $( $cvdoc_1B_dep, $cvname_1B_dep, $cvtype_1B_dep, $cvfeature_1B_dep ),* ,
                $( $cvdoc_2B_dep, $cvname_2B_dep, $cvtype_2B_dep, $cvfeature_2B_dep ),* ,
                $( $cvdoc_4B_dep, $cvname_4B_dep, $cvtype_4B_dep, $cvfeature_4B_dep ),* ,
                $( $cvdoc_8B_dep, $cvname_8B_dep, $cvtype_8B_dep, $cvfeature_8B_dep ),* ,
                $( $cvdoc_16B_dep, $cvname_16B_dep, $cvtype_16B_dep, $cvfeature_16B_dep ),* ,
                $( $cvdoc_32B_dep, $cvname_32B_dep, $cvtype_32B_dep, $cvfeature_32B_dep ),* ,
                $( $cvdoc_64B_dep, $cvname_64B_dep, $cvtype_64B_dep, $cvfeature_64B_dep ),* ,
                $( $cvdoc_128B_dep, $cvname_128B_dep, $cvtype_128B_dep, $cvfeature_128B_dep ),* ;
            copy_variants_std:
                $( $cvdoc_16B_std, $cvname_16B_std, $cvtype_16B_std, $cvfeature_16B_std ),* ;
            noncopy_variants:
                // $( $vdoc_4B, $vname_4B, $vtype_4B ),* ,
                // $( $vdoc_8B, $vname_8B, $vtype_8B ),* ,
                // $( $vdoc_16B, $vname_16B, $vtype_16B ),* ,
                // $( $vdoc_32B, $vname_32B, $vtype_32B ),* ;
                // $( $vdoc_64B, $vname_64B, $vtype_64B ),* ;
                // $( $vdoc_128B, $vname_128B, $vtype_128B ),*
                ;
            noncopy_variants_dep:
                // $( $vdoc_1B_dep, $vname_1B_dep, $vtype_1B_dep, $vfeature_1B_dep ),* ,
                // $( $vdoc_2B_dep, $vname_2B_dep, $vtype_2B_dep, $vfeature_2B_dep ),* ,
                $( $vdoc_4B_dep, $vname_4B_dep, $vtype_4B_dep, $vfeature_4B_dep ),* ,
                $( $vdoc_8B_dep, $vname_8B_dep, $vtype_8B_dep, $vfeature_8B_dep ),* ,
                // $( $vdoc_16B_dep, $vname_16B_dep, $vtype_16B_dep, $vfeature_16B_dep ),* ,
                $( $vdoc_32B_dep, $vname_32B_dep, $vtype_32B_dep, $vfeature_32B_dep ),* ,
                $( $vdoc_64B_dep, $vname_64B_dep, $vtype_64B_dep, $vfeature_64B_dep ),* ;
                // $( $vdoc_128B_dep, $vname_128B_dep, $vtype_128B_dep, $vfeature_128B_dep ),* ;
            noncopy_variants_std:
                $( $vdoc_32B_std, $vname_32B_std, $vtype_32B_std, $vfeature_32B_std ),* ;
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
        copy_variants_dep: $( $cvdoc_dep:literal, $cvname_dep:ident, $cvtype_dep:ty, $cvfeature_dep:literal ),* ;
        copy_variants_std: $( $cvdoc_std:literal, $cvname_std:ident, $cvtype_std:ty, $cvfeature_std:literal ),* ;
        noncopy_variants: $( $vdoc:literal, $vname:ident, $vtype:ty ),* ;
        noncopy_variants_dep: $( $vdoc_dep:literal, $vname_dep:ident, $vtype_dep:ty, $vfeature_dep:literal ),* ;
        noncopy_variants_std: $( $vdoc_std:literal, $vname_std:ident, $vtype_std:ty, $vfeature_std:literal ),* ;
        pointer: $( $iusize:meta ),*
    ) => {
        define_type!{
            $tname, size: $B, $b,
            copy_variants: $( $cvdoc, $cvname, $cvtype ),* ,
            copy_variants_dep: $( $cvdoc_dep, $cvname_dep, $cvtype_dep, $cvfeature_dep ),* ;
            copy_variants_std: $( $cvdoc_std, $cvname_std, $cvtype_std, $cvfeature_std ),* ;
            noncopy_variants: $( $vdoc, $vname, $vtype ),* ;
            noncopy_variants_dep: $( $vdoc_dep, $vname_dep, $vtype_dep, $vfeature_dep ),* ;
            noncopy_variants_std: $( $vdoc_std, $vname_std, $vtype_std, $vfeature_std ),* ;
            pointer: $( $iusize ),* ;
        }
        define_cell!{
            c: $cname, t:$tname, u:$ucname, size: $B, $b,
            copy_variants: $( $cvdoc, $cvname, $cvtype ),* ,
            copy_variants_dep: $( $cvdoc_dep, $cvname_dep, $cvtype_dep, $cvfeature_dep ),* ;
            copy_variants_std: $( $cvdoc_std, $cvname_std, $cvtype_std, $cvfeature_std ),* ;
            noncopy_variants: $( $vdoc, $vname, $vtype ),* ;
            noncopy_variants_dep: $( $vdoc_dep, $vname_dep, $vtype_dep, $vfeature_dep ),* ;
            noncopy_variants_std: $( $vdoc_std, $vname_std, $vtype_std, $vfeature_std ),* ;
            pointer: $( $iusize ),* ;
        }
        define_unsafe_cell!{
            $ucname, size: $B, $b,
            copy_variants: $( $cvdoc, $cvname, $cvtype ),* ,
            copy_variants_dep: $( $cvdoc_dep, $cvname_dep, $cvtype_dep, $cvfeature_dep ),* ;
            copy_variants_std: $( $cvdoc_std, $cvname_std, $cvtype_std, $cvfeature_std ),* ;
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
        copy_variants_dep: $( $cvdoc_dep:literal, $cvname_dep:ident, $cvtype_dep:ty, $cvfeature_dep:literal ),* ;
        copy_variants_std: $( $cvdoc_std:literal, $cvname_std:ident, $cvtype_std:ty, $cvfeature_std:literal ),* ;
        noncopy_variants: $( $vdoc:literal, $vname:ident, $vtype:ty ),* ;
        noncopy_variants_dep: $( $vdoc_dep:literal, $vname_dep:ident, $vtype_dep:ty, $vfeature_dep:literal ),* ;
        noncopy_variants_std: $( $vdoc_std:literal, $vname_std:ident, $vtype_std:ty, $vfeature_std:literal ),* ;
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

                // feature-gated dependencies
                $(
                    #[cfg(feature = $cvfeature_dep )]
                    #[doc = $cvdoc_dep]
                    $cvname_dep,
                )*

                // std dependencies (+ optionally distinct feature)
                $(
                    #[cfg(all(feature = "std", feature = $cvfeature_std))]
                    #[doc = $cvdoc_std]
                    $cvname_std,
                )*

                $(
                    #[doc = $cvdoc ]
                    $cvname,
                )*
            }
            type_aliases![t: $tname, size: $B, $b, "Copy", "data **Type**", "(Copy)" ];
            impl_data_types![ [< $tname $B Byte Copy With >], DataTypesCopy,
                is_copy: true,
                copy_variants: $( $cvname, $cvtype ),* ;
                copy_variants_dep: $( $cvname_dep, $cvtype_dep, $cvfeature_dep ),* ;
                copy_variants_std: $( $cvname_std, $cvtype_std, $cvfeature_std ),* ;
                noncopy_variants: ;
                noncopy_variants_dep: ;
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

                // feature-gated dependencies
                $(
                    #[cfg(feature = $cvfeature_dep )]
                    #[doc = $cvdoc_dep]
                    $cvname_dep,
                )*
                $(
                    #[cfg(feature = $vfeature_dep )]
                    #[doc = $vdoc_dep]
                    $vname_dep,
                )*

                // std dependencies (+ optionally distinct feature)
                $(
                    #[cfg(all(feature = "std", feature = $cvfeature_std))]
                    #[doc = $cvdoc_std]
                    $cvname_std,
                )*
                $(
                    #[cfg(all(feature = "std", feature = $vfeature_std))]
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
                copy_variants_dep: $( $cvname_dep, $cvtype_dep, $cvfeature_dep ),* ;
                copy_variants_std: $( $cvname_std, $cvtype_std, $cvfeature_std ),* ;
                noncopy_variants: $($vname, $vtype ),* ;
                noncopy_variants_dep: $( $vname_dep, $vtype_dep, $vfeature_dep ),* ;
                noncopy_variants_std: $($vname_std, $vtype_std, $vfeature_std ),* ;
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
        copy_variants_dep: $( $cvdoc_dep:literal, $cvname_dep:ident, $cvtype_dep:ty, $cvfeature_dep:literal ),* ;
        copy_variants_std: $( $cvdoc_std:literal, $cvname_std:ident, $cvtype_std:ty, $cvfeature_std:literal ),* ;
        noncopy_variants: $( $vdoc:literal, $vname:ident, $vtype:ty ),* ;
        noncopy_variants_dep: $( $vdoc_dep:literal, $vname_dep:ident, $vtype_dep:ty, $vfeature_dep:literal ),* ;
        noncopy_variants_std: $( $vdoc_std:literal, $vname_std:ident, $vtype_std:ty, $vfeature_std:literal ),* ;
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

                // feature-gated dependencies
                $(
                    #[cfg(feature = $cvfeature_dep )]
                    #[doc = $cvdoc_dep]
                    $cvname_dep($cvtype_dep),
                )*

                // std dependencies (+ optionally distinct feature)
                $(
                    #[cfg(all(feature = "std", feature = $cvfeature_std))]
                    #[doc = $cvdoc_std]
                    $cvname_std($cvtype_std),
                )*

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
                copy_variants_dep: $( $cvname_dep, $cvtype_dep, $cvfeature_dep ),* ;
                copy_variants_std: $( $cvname_std, $cvtype_std, $cvfeature_std ),* ;
                noncopy_variants: ;
                noncopy_variants_dep: ;
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

                // feature-gated dependencies
                $(
                    #[cfg(feature = $cvfeature_dep )]
                    #[doc = $cvdoc_dep]
                    $cvname_dep($cvtype_dep),
                )*
                $(
                    #[cfg(feature = $vfeature_dep )]
                    #[doc = $vdoc_dep]
                    $vname_dep($vtype_dep),
                )*

                // std dependencies (+ optionally distinct feature)
                $(
                    #[cfg(all(feature = "std", feature = $cvfeature_std))]
                    #[doc = $cvdoc_std]
                    $cvname_std($cvtype_std),
                )*
                $(
                    #[cfg(all(feature = "std", feature = $vfeature_std))]
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
                copy_variants_dep: $( $cvname_dep, $cvtype_dep, $cvfeature_dep ),* ;
                copy_variants_std: $( $cvname_std, $cvtype_std, $cvfeature_std ),* ;
                noncopy_variants: $($vname, $vtype ),* ;
                noncopy_variants_dep: $( $vname_dep, $vtype_dep, $vfeature_dep ),* ;
                noncopy_variants_std: $( $vname_std, $vtype_std, $vfeature_std ),* ;
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

                        // feature-gated dependencies
                        $(
                            #[cfg(feature = $cvfeature_dep )]
                            [<$cname $B Byte Copy With>]::$cvname_dep(v) => Self { $cvname_dep: v },
                        )*

                        // std dependencies (+ optionally distinct feature)
                        $(
                            #[cfg(all(feature = "std", feature = $cvfeature_std))]
                            [<$cname $B Byte Copy With>]::$cvname_std(v) => Self { $cvname_std: v },
                        )*

                        $(
                            [<$cname $B Byte Copy With>]::$cvname(v) => Self { $cvname: v },
                        )*

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
        copy_variants_dep: $( $cvdoc_dep:literal, $cvname_dep:ident, $cvtype_dep:ty, $cvfeature_dep:literal ),* ;
        copy_variants_std: $( $cvdoc_std:literal, $cvname_std:ident, $cvtype_std:ty, $cvfeature_std:literal ),* ;
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

                // feature-gated dependencies
                $(
                    #[cfg(feature = $cvfeature_dep )]
                    #[doc = $cvdoc_dep]
                    pub $cvname_dep: $cvtype_dep,
                )*

                // std dependencies (+ optionally distinct feature)
                $(
                    #[cfg(all(feature = "std", feature = $cvfeature_std))]
                    #[doc = $cvdoc_std]
                    pub $cvname_std: $cvtype_std,
                )*

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
        copy_variants_dep: $( $cvname_dep:ident, $cvtype_dep:ty, $cvfeature_dep:literal ),* ;
        copy_variants_std: $( $cvname_std:ident, $cvtype_std:ty, $cvfeature_std:literal ),* ;
        noncopy_variants: $( $vname:ident, $vtype:ty ),* ;
        noncopy_variants_dep: $( $vname_dep:ident, $vtype_dep:ty, $vfeature_dep:literal ),* ;
        noncopy_variants_std: $( $vname_std:ident, $vtype_std:ty, $vfeature_std:literal ),* ;
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

                        // feature-gated dependencies
                        $(
                            #[cfg(feature = $cvfeature_dep )]
                            $cvname_dep => align_of::<$cvtype_dep>(),
                        )*
                        $(
                            #[cfg(feature = $vfeature_dep )]
                            $vname_dep => align_of::<$vtype_dep>(),
                        )*

                        // std dependencies (+ optionally distinct feature)
                        $(
                            #[cfg(all(feature = "std", feature = $cvfeature_std))]
                            $cvname_std => align_of::<$cvtype_std>(),
                        )*
                        $(
                            #[cfg(all(feature = "std", feature = $vfeature_std))]
                            $vname_std => align_of::<$vtype_std>(),
                        )*

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

                        // feature-gated dependencies
                        $(
                            #[cfg(feature = $cvfeature_dep )]
                            $cvname_dep => size_of::<$cvtype_dep>(),
                        )*
                        $(
                            #[cfg(feature = $vfeature_dep )]
                            $vname_dep => size_of::<$vtype_dep>(),
                        )*

                        // std dependencies (+ optionally distinct feature)
                        $(
                            #[cfg(all(feature = "std", feature = $cvfeature_std))]
                            $cvname_std => size_of::<$cvtype_std>(),
                        )*
                        $(
                            #[cfg(all(feature = "std", feature = $vfeature_std))]
                            $vname_std => size_of::<$vtype_std>(),
                        )*

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
        copy_variants_dep: $( $cvname_dep:ident, $cvtype_dep:ty, $cvfeature_dep:literal ),* ;
        copy_variants_std: $( $cvname_std:ident, $cvtype_std:ty, $cvfeature_std:literal ),* ;
        noncopy_variants: $( $vname:ident, $vtype:ty ),* ;
        noncopy_variants_dep: $( $vname_dep:ident, $vtype_dep:ty, $vfeature_dep:literal ),* ;
        noncopy_variants_std: $( $vname_std:ident, $vtype_std:ty, $vfeature_std:literal ),* ;
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

// MOCKUP IMPORTS OF OPTIONAL EXTERNAL DEPENDENCIES
// -------------------------------------------------------------------------
// so that `define_all_sizes` macro invocation works

// "external_continuous"

#[cfg(not(feature = "half"))]
mod half {
    #![allow(dead_code)]
    pub struct f16;
    pub struct bf16;
}
#[cfg(not(feature = "softposit"))]
mod softposit {
    #![allow(dead_code)]
    pub struct P8;
    pub struct P16;
    pub struct P32;
    pub struct Q8;
    pub struct Q16;
    pub struct Q32;
}
#[cfg(not(feature = "twofloat"))]
mod twofloat {
    #![allow(dead_code)]
    pub struct TwoFloat;
}

// "external_discrete"

#[cfg(not(feature = "num-rational"))]
mod num_rational {
    #![allow(dead_code)]
    pub struct Ratio;
}

#[cfg(not(feature = "num-bigint"))]
mod num_bigint {
    #![allow(dead_code)]
    pub struct BigInt;
}
#[cfg(not(feature = "rust_decimal"))]
mod rust_decimal {
    #![allow(dead_code)]
    pub struct Decimal;
}

// "external_string"

#[cfg(not(feature = "arraystring"))]
mod arraystring {
    #![allow(dead_code)]
    pub struct ArrayString<T> {
        _t: T,
    }
}
#[cfg(feature = "arraystring")]
use arraystring::{typenum, ArrayString};

// "external_time"

#[cfg(not(feature = "fugit"))]
mod fugit {
    #![allow(dead_code)]
    pub struct Instant<T, const A: usize, B: Into<usize>> {
        _t: T,
        _b: B,
    }
    pub struct Duration<T, const A: usize, B: Into<usize>> {
        _t: T,
        _b: B,
    }
}
#[cfg(not(feature = "time"))]
mod time {
    #![allow(dead_code)]
    pub struct Date;
    pub struct Time;
    pub struct Instant;
    pub struct UtcOffset;
    pub struct OffsetDateTime;
    pub struct PrimitiveDateTime;
    pub struct Duration;
}

// DEFINITIONS
// -------------------------------------------------------------------------

define_all_sizes! {
    DataType, DataCell, DataUnsafeCell,

    // ------------------------------------------------------------------------- 1-B / 8-b
    copy_variants_1B:
    "8-bit unsigned integer ", U8, u8,
    "8-bit signed integer", I8, i8,
    "1-Byte array of bytes", ArrayBytes1, [u8; 1],
    "Boolean value", Bool, bool,
    copy_variants_1B_dep:
    "8-bit [`softposit`](https://crates.io/crates/softposit)'s `Posit` with exp=0", P8, softposit::P8, "softposit",
    // noncopy_variants_1B:
    // noncopy_variants_1B_dep:

    // ------------------------------------------------------------------------- 2-B / 16-b
    copy_variants_2B:
    "16-bit unsigned integer ", U16, u16,
    "16-bit signed integer", I16, i16,
    "2-Byte array of bytes", ArrayBytes2, [u8; 2],
    copy_variants_2B_dep:
    "16-bit [`half`](https://crates.io/crates/half)'s `binary16` floating-point number", F16, half::f16, "half",
    "16-bit [`half`](https://crates.io/crates/half)'s `bfloat16` floating-point number", BF16, half::bf16, "half",
    "16-bit [`softposit`](https://crates.io/crates/softposit)'s `Posit` with exp=1", P16, softposit::P16, "softposit",
    "2-Byte [`arraystring`](https://crates.io/crates/arraystring)'s ArrayString of len 1",
        ArrayString1, ArrayString<typenum::U1>, "arraystring",
    // noncopy_variants_2B:
    // noncopy_variants_2B_dep:
    iusize_2B: target_pointer_width = "16",

    // ------------------------------------------------------------------------- 4-B / 32-b
    copy_variants_4B:
    "32-bit unsigned integer ", U32, u32,
    "32-bit signed integer", I32, i32,
    "32-bit floating-point number", F32, f32,
    "4-Byte array of bytes", ArrayBytes4, [u8; 4],
    "4-Byte char ", Char, char,
    copy_variants_4B_dep:
    "32-bit [`softposit`](https://crates.io/crates/softposit)'s `Posit` with exp=2", P32, softposit::P32, "softposit",
    "4-Byte [`arraystring`](https://crates.io/crates/arraystring)'s ArrayString of len 3",
        ArrayString3, ArrayString<typenum::U3>, "arraystring",
    "32-bit [`time`](https://crates.io/crates/time)'s `Date`", TDate, time::Date, "time",
    "32-bit [`time`](https://crates.io/crates/time)'s `UtcOffset`", TUtcOffset, time::UtcOffset, "time",
    "32-bit [`fugit`](https://crates.io/crates/fugit)'s `Duration` in hours",
        FugitDuration32Hours, fugit::Duration<u32, 3_600, 1>, "fugit",
    "32-bit [`fugit`](https://crates.io/crates/fugit)'s `Duration` in minutes",
        FugitDuration32Minutes, fugit::Duration<u32, 60, 1>, "fugit",
    "32-bit [`fugit`](https://crates.io/crates/fugit)'s `Duration` in seconds",
        FugitDuration32Seconds, fugit::Duration<u32, 1, 1>, "fugit",
    "32-bit [`fugit`](https://crates.io/crates/fugit)'s `Duration` in milliseconds",
        FugitDuration32Millis, fugit::Duration<u32, 1, 1_000>, "fugit",
    "32-bit [`fugit`](https://crates.io/crates/fugit)'s `Duration` in nanoseconds",
        FugitDuration32Nanos, fugit::Duration<u32, 1, 1_000_000>, "fugit",
    "32-bit [`fugit`](https://crates.io/crates/fugit)'s `Instant` in hours",
        FugitInstant32Hours, fugit::Instant<u32, 3_600, 1>, "fugit",
    "32-bit [`fugit`](https://crates.io/crates/fugit)'s `Instant` in minutes",
        FugitInstant32Minutes, fugit::Instant<u32, 60, 1>, "fugit",
    "32-bit [`fugit`](https://crates.io/crates/fugit)'s `Instant` in seconds",
        FugitInstant32Seconds, fugit::Instant<u32, 1, 1>, "fugit",
    "32-bit [`fugit`](https://crates.io/crates/fugit)'s `Instant` in milliseconds",
        FugitInstant32Millis, fugit::Instant<u32, 1, 1_000>, "fugit",
    "32-bit [`fugit`](https://crates.io/crates/fugit)'s `Instant` in nanoseconds",
        FugitInstant32Nanos, fugit::Instant<u32, 1, 1_000_000>, "fugit",
    // noncopy_variants_4B:
    noncopy_variants_4B_dep:
    "8-bit [`softposit`](https://crates.io/crates/softposit)'s `Quire` with exp=0", Q8, softposit::Q8, "softposit",
    iusize_4B: target_pointer_width = "32",

    // ------------------------------------------------------------------------- 8-B / 64-b
    copy_variants_8B:
    "64-bit unsigned integer ", U64, u64,
    "64-bit signed integer", I64, i64,
    "64-bit floating-point number", F64, f64,
    "8-Byte array of bytes", ArrayBytes8, [u8; 8],
    copy_variants_8B_dep:
    "32-bit [`num_rational`](https://crates.io/crates/num_rational)'s `Ratio` rational number",
        R32, num_rational::Ratio<i32>, "num-rational",
    "8-Byte [`arraystring`](https://crates.io/crates/arraystring)'s ArrayString of len 7",
        ArrayString7, ArrayString<typenum::U7>, "arraystring",
    "64-bit [`time`](https://crates.io/crates/time)'s `Time`", TTime, time::Time, "time",
    "64-bit [`fugit`](https://crates.io/crates/fugit)'s `Duration` in hours",
        FugitDuration64Hours, fugit::Duration<u64, 3_600, 1>, "fugit",
    "64-bit [`fugit`](https://crates.io/crates/fugit)'s `Duration` in minutes",
        FugitDuration64Minutes, fugit::Duration<u64, 60, 1>, "fugit",
    "64-bit [`fugit`](https://crates.io/crates/fugit)'s `Duration` in seconds",
        FugitDuration64Seconds, fugit::Duration<u64, 1, 1>, "fugit",
    "64-bit [`fugit`](https://crates.io/crates/fugit)'s `Duration` in milliseconds",
        FugitDuration64Millis, fugit::Duration<u64, 1, 1_000>, "fugit",
    "64-bit [`fugit`](https://crates.io/crates/fugit)'s `Duration` in nanoseconds",
        FugitDuration64Nanos, fugit::Duration<u64, 1, 1_000_000>, "fugit",
    "64-bit [`fugit`](https://crates.io/crates/fugit)'s `Instant` in hours",
        FugitInstant64Hours, fugit::Instant<u64, 3_600, 1>, "fugit",
    "64-bit [`fugit`](https://crates.io/crates/fugit)'s `Instant` in minutes",
        FugitInstant64Minutes, fugit::Instant<u64, 60, 1>, "fugit",
    "64-bit [`fugit`](https://crates.io/crates/fugit)'s `Instant` in seconds",
        FugitInstant64Seconds, fugit::Instant<u64, 1, 1>, "fugit",
    "64-bit [`fugit`](https://crates.io/crates/fugit)'s `Instant` in milliseconds",
        FugitInstant64Millis, fugit::Instant<u64, 1, 1_000>, "fugit",
    "64-bit [`fugit`](https://crates.io/crates/fugit)'s `Instant` in nanoseconds",
        FugitInstant64Nanos, fugit::Instant<u64, 1, 1_000_000>, "fugit",
    // noncopy_variants_8B:
    noncopy_variants_8B_dep:
    "16-bit [`softposit`](https://crates.io/crates/softposit)'s `Quire` with exp=1", Q16, softposit::Q16, "softposit",
    iusize_8B: target_pointer_width = "64",

    // ------------------------------------------------------------------------- 16-B /128-b
    copy_variants_16B:
    "128-bit unsigned integer ", U128, u128,
    "128-bit signed integer", I128, i128,
    "16-Byte array of bytes", ArrayBytes16, [u8; 16],
    "128-bit Duration", Duration, core::time::Duration,
    copy_variants_16B_dep:
    "128-bit floating point number", F128, twofloat::TwoFloat, "twofloat",
    "64-bit [`num_rational`](https://crates.io/crates/num_rational)'s `Ratio` rational number",
        R64, num_rational::Ratio<i64>, "num-rational",
    "16-Byte [rust_decimal] Decimal number", Decimal, rust_decimal::Decimal, "rust_decimal",
    "16-Byte [`arraystring`](https://crates.io/crates/arraystring)'s ArrayString of len 15",
        ArrayString15, ArrayString<typenum::U15>, "arraystring",
    "128-bit [`time`](https://crates.io/crates/time)'s `Duration`", TDuration, time::Duration, "time",
    "128-bit [`time`](https://crates.io/crates/time)'s `PrimitiveDateTime`",
        TDateTime, time::PrimitiveDateTime, "time",
    "128-bit [`time`](https://crates.io/crates/time)'s `OffsetDateTime`",
        TOffsetDateTime, time::OffsetDateTime, "time",
    copy_variants_16B_std:
    "128-bit Instant", Instant, std::time::Instant, "std",
    "128-bit SystemTime", SystemTime, std::time::SystemTime, "std",
    "128-bit [`time`](https://crates.io/crates/time)'s `Instant`", TInstant, time::Instant, "time",
    // noncopy_variants_16B:

    // ------------------------------------------------------------------------- 32-B / 256-b
    copy_variants_32B:
    "32-Byte array of bytes", ArrayBytes32, [u8; 32],
    copy_variants_32B_dep:
    "128-bit rational number", R128, num_rational::Ratio<i128>, "num-rational",
    "32-Byte [`arraystring`](https://crates.io/crates/arraystring)'s ArrayString of len 31",
        ArrayString31, ArrayString<typenum::U31>, "arraystring",
    // noncopy_variants_32B:
    noncopy_variants_32B_dep:
    "Big Integer", BigInt, num_bigint::BigInt, "num-bigint",
    noncopy_variants_32B_std:
    "string", String, std::string::String, "std",

    // ------------------------------------------------------------------------- 64 B
    copy_variants_64B:
    "64-Byte array of bytes", ArrayBytes64, [u8; 64],
    copy_variants_64B_dep:
    "64-Byte [`arraystring`](https://crates.io/crates/arraystring)'s ArrayString of len 63",
        ArrayString63, ArrayString<typenum::U63>, "arraystring",
    // noncopy_variants_64B:
    noncopy_variants_64B_dep:
    "32-bit [`softposit`](https://crates.io/crates/softposit)'s `Quire` with exp=2", Q32, softposit::Q32, "softposit",

    // ------------------------------------------------------------------------- 128-B / 1024-b
    copy_variants_128B:
    "128-Byte array of bytes", ArrayBytes128, [u8; 128],
    copy_variants_128B_dep:
    "127-Byte [`arraystring`](https://crates.io/crates/arraystring)'s ArrayString of len 127",
        ArrayString127, ArrayString<typenum::U127>, "arraystring",
    noncopy_variants_128B: ,
    //
}
