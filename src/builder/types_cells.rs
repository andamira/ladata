// ladata::builder::types_cells
//
//! DataType*s and DataCell*s are generated here.
//
// # TOC
//
// - MACROS:
//   - define_all_sizes
//   - define_single_size
//   - define_type
//   - define_cell
//   - define_unsafe
//   - type_aliases
//   - impl_data_types
//   - impl_data_cells
//   - impl_data_unsafes
//
// - MOCKUPS OF UNUSED DEPENDENCIES
//
// - DEFINITIONS
//
//   - DataType, DataCell, DataUnsafe @ Byte: 1, 2, 4, 8, 16, 32, 64, 128

use core::{
    // any::TypeId,
    convert::TryFrom,
    fmt,
    mem::{align_of, size_of},
};

use crate::special::NoData;
use crate::traits::{DataCells, DataCellsCopy, DataTypes, DataTypesCopy, DataUnsafes};
use crate::define_line;

/// defines all sizes at the same time
///
/// This single-branch macro receives all the types ordered by size,
/// already classified according to the following grouping:
///
/// - copy_variants_nB:              (name, type),*
/// - copy_variants_nB_dep:          (doc, name, type, dep1, dep2),*
/// - copy_variants_nB_psize:        (doc, name, type, psize),*
/// - copy_variants_nB_psize_dep:    (doc, name, type, psize, dep1, dep2),*
/// - noncopy_variants_nB:           (doc, name, type),*
/// - noncopy_variants_nB_dep:       (doc, name, type, dep1, dep2),*
/// - noncopy_variants_nB_psize:     (doc, name, type, psize),*
/// - noncopy_variants_nB_psize_dep: (doc, name, type, psize, dep1, dep2),*
///
/// where:
/// - the `copy_` prefix indicates the types are `Copy`,
///   otherwise the `noncopy` prefix is used.
/// - `nB` indicates the number of bytes of the types in the current group.
/// - the `_dep` suffix indicates a dependency on 2 features (dep1 & dep2)
///   (pass the same dependency twice in order to only depend on one).
/// - the `_psize` suffix indicates a dependency on a specific pointer size.
///
/// The `define_single_size!` macro is called making sure each size contains
/// all variants with a size less than or equal to the current size.
macro_rules! define_all_sizes {
    (
        $tname:ident, $cname:ident, $ucname:ident,

        // 1-Byte / 8-bit
        copy_variants_1B: $( $cvdoc_1B:literal, $cvname_1B:ident, $cvtype_1B:ty ),* ,
        copy_variants_1B_dep: $( $cvdoc_1B_dep:literal, $cvname_1B_dep:ident, $cvtype_1B_dep:ty,
            $cvdep1_1B_dep:literal, $cvdep2_1B_dep:literal  ),* ,
        copy_variants_1B_psize:
        $( $cvdoc_1B_psize:literal, $cvname_1B_psize:ident, $cvtype_1B_psize:ty, $cvpsize_1B_psize:meta ),* ,
        // noncopy_variants_1B: $( $vdoc_1B:literal, $vname_1B:ident, $vtype_1B:ty ),* ,
        // noncopy_variants_1B_dep:
        // $( $vdoc_1B_dep:literal, $vname_1B_dep:ident, $vtype_1B_dep:ty,
        // noncopy_variants_1B_psize:
        // $( $vdoc_1B_psize:literal, $vname_1B_psize:ident, $vtype_1B_psize:ty, $vpsize_1B_psize:meta ),* ,
            // $vdep1_1B_dep:literal, $vdep2_1B_dep:literal ),* ,

        // 2-Byte / 16-bit
        copy_variants_2B: $( $cvdoc_2B:literal, $cvname_2B:ident, $cvtype_2B:ty ),* ,
        copy_variants_2B_dep: $( $cvdoc_2B_dep:literal, $cvname_2B_dep:ident, $cvtype_2B_dep:ty,
            $cvdep1_2B_dep:literal, $cvdep2_2B_dep:literal ),* ,
        copy_variants_2B_psize:
        $( $cvdoc_2B_psize:literal, $cvname_2B_psize:ident, $cvtype_2B_psize:ty, $cvpsize_2B_psize:meta ),* ,
        // noncopy_variants_2B: $( $vdoc_2B:literal, $vname_2B:ident, $vtype_2B:ty ),* ,
        // noncopy_variants_2B_dep: $( $vdoc_2B_dep:literal, $vname_2B_dep:ident, $vtype_2B_dep:ty,
            // $vdep1_2B_dep:literal ),* ,
        // noncopy_variants_2B_psize:
        // $( $vdoc_2B_psize:literal, $vname_2B_psize:ident, $vtype_2B_psize:ty, $vpsize_2B_psize:meta ),* ,

        // 4-Byte / 32-bit
        copy_variants_4B: $( $cvdoc_4B:literal, $cvname_4B:ident, $cvtype_4B:ty ),* ,
        copy_variants_4B_dep: $( $cvdoc_4B_dep:literal, $cvname_4B_dep:ident, $cvtype_4B_dep:ty,
            $cvdep1_4B_dep:literal, $cvdep2_4B_dep:literal ),* ,
        copy_variants_4B_psize:
        $( $cvdoc_4B_psize:literal, $cvname_4B_psize:ident, $cvtype_4B_psize:ty, $cvpsize_4B_psize:meta ),* ,
        // noncopy_variants_4B: $( $vdoc_4B:literal, $vname_4B:ident, $vtype_4B:ty ),* ,
        noncopy_variants_4B_dep: $( $vdoc_4B_dep:literal, $vname_4B_dep:ident, $vtype_4B_dep:ty,
            $vdep1_4B_dep:literal, $vdep2_4B_dep:literal ),* ,
        // noncopy_variants_4B_psize:
        // $( $vdoc_4B_psize:literal, $vname_4B_psize:ident, $vtype_4B_psize:ty, $vpsize_4B_psize:meta ),* ,

        // 8-Byte / 64-bit
        copy_variants_8B: $( $cvdoc_8B:literal, $cvname_8B:ident, $cvtype_8B:ty ),* ,
        copy_variants_8B_dep: $( $cvdoc_8B_dep:literal, $cvname_8B_dep:ident, $cvtype_8B_dep:ty,
            $cvdep1_8B_dep:literal, $cvdep2_8B_dep:literal ),* ,
        copy_variants_8B_psize:
        $( $cvdoc_8B_psize:literal, $cvname_8B_psize:ident, $cvtype_8B_psize:ty, $cvpsize_8B_psize:meta ),* ,
        // noncopy_variants_8B: $( $vdoc_8B:literal, $vname_8B:ident, $vtype_8B:ty ),* ,
        noncopy_variants_8B_dep: $( $vdoc_8B_dep:literal, $vname_8B_dep:ident, $vtype_8B_dep:ty,
            $vdep1_8B_dep:literal, $vdep2_8B_dep:literal ),* ,
        // noncopy_variants_8B_psize:
        // $( $vdoc_8B_psize:literal, $vname_8B_psize:ident, $vtype_8B_psize:ty, $vpsize_8B_psize:meta ),* ,
        noncopy_variants_8B_psize_dep:
        $( $vdoc_8B_psize_dep:literal, $vname_8B_psize_dep:ident, $vtype_8B_psize_dep:ty,
           $vpsize_8B_psize_dep:meta, $vdep1_8B_psize_dep:literal, $vdep2_8B_psize_dep:literal ),* ,

        // 16-Byte / 128-bit
        copy_variants_16B: $( $cvdoc_16B:literal, $cvname_16B:ident, $cvtype_16B:ty ),* ,
        copy_variants_16B_dep: $( $cvdoc_16B_dep:literal, $cvname_16B_dep:ident, $cvtype_16B_dep:ty,
            $cvdep1_16B_dep:literal, $cvdep2_16B_dep:literal ),* ,
        copy_variants_16B_psize:
        $( $cvdoc_16B_psize:literal, $cvname_16B_psize:ident, $cvtype_16B_psize:ty, $cvpsize_16B_psize:meta ),* ,
        // noncopy_variants_16B: $( $vdoc_16B:literal, $vname_16B:ident, $vtype_16B:ty ),* ,
        // noncopy_variants_16B_dep: $( $vdoc_16B_dep:literal, $vname_16B_dep:ident, $vtype_16B_dep:ty,
        //     $vdep1_16B_dep:literal, $vdep2_16B_dep:literal ),* ,
        // noncopy_variants_16B_psize:
        // $( $vdoc_16B_psize:literal, $vname_16B_psize:ident, $vtype_16B_psize:ty, $vpsize_16B_psize:meta ),* ,
        noncopy_variants_16B_psize_dep:
        $( $vdoc_16B_psize_dep:literal, $vname_16B_psize_dep:ident, $vtype_16B_psize_dep:ty,
           $vpsize_16B_psize_dep:meta, $vdep1_16B_psize_dep:literal, $vdep2_16B_psize_dep:literal ),* ,

        // 32-Byte / 256-bit
        copy_variants_32B: $( $cvdoc_32B:literal, $cvname_32B:ident, $cvtype_32B:ty ),* ,
        copy_variants_32B_dep: $( $cvdoc_32B_dep:literal, $cvname_32B_dep:ident, $cvtype_32B_dep:ty,
            $cvdep1_32B_dep:literal, $cvdep2_32B_dep:literal ),* ,
        // noncopy_variants_32B: $( $vdoc_32B:literal, $vname_32B:ident, $vtype_32B:ty ),* ,
        noncopy_variants_32B_dep: $( $vdoc_32B_dep:literal, $vname_32B_dep:ident, $vtype_32B_dep:ty,
            $vdep1_32B_dep:literal, $vdep2_32B_dep:literal ),* ,
        // noncopy_variants_32B_psize:
        // $( $vdoc_32B_psize:literal, $vname_32B_psize:ident, $vtype_32B_psize:ty, $vpsize_32B_psize:meta ),* ,
        noncopy_variants_32B_psize_dep:
        $( $vdoc_32B_psize_dep:literal, $vname_32B_psize_dep:ident, $vtype_32B_psize_dep:ty,
           $vpsize_32B_psize_dep:meta, $vdep1_32B_psize_dep:literal, $vdep2_32B_psize_dep:literal ),* ,

        // 64-Byte / 512-bit
        copy_variants_64B: $( $cvdoc_64B:literal, $cvname_64B:ident, $cvtype_64B:ty ),* ,
        copy_variants_64B_dep: $( $cvdoc_64B_dep:literal, $cvname_64B_dep:ident, $cvtype_64B_dep:ty,
            $cvdep1_64B_dep:literal, $cvdep2_64B_dep:literal ),* ,
        // noncopy_variants_64B: $( $vdoc_64B:literal, $vname_64B:ident, $vtype_64B:ty ),* ,
        noncopy_variants_64B_dep: $( $vdoc_64B_dep:literal, $vname_64B_dep:ident, $vtype_64B_dep:ty,
            $vdep1_64B_dep:literal, $vdep2_64B_dep:literal ),* ,
        // noncopy_variants_64B_psize:
        // $( $vdoc_64B_psize:literal, $vname_64B_psize:ident, $vtype_64B_psize:ty, $vpsize_64B_psize:meta ),* ,
        noncopy_variants_64B_psize_dep:
        $( $vdoc_64B_psize_dep:literal, $vname_64B_psize_dep:ident, $vtype_64B_psize_dep:ty,
           $vpsize_64B_psize_dep:meta, $vdep1_64B_psize_dep:literal, $vdep2_64B_psize_dep:literal ),* ,

        // 128-Byte / 1024-bit
        copy_variants_128B: $( $cvdoc_128B:literal, $cvname_128B:ident, $cvtype_128B:ty ),* ,
        copy_variants_128B_dep: $( $cvdoc_128B_dep:literal, $cvname_128B_dep:ident, $cvtype_128B_dep:ty,
            $cvdep1_128B_dep:literal, $cvdep2_128B_dep:literal ),* ,
        // noncopy_variants_128B: $( $vdoc_128B:literal, $vname_128B:ident, $vtype_128B:ty ),* ,
        // noncopy_variants_128B_dep: $( $vdoc_128B_dep:literal, $vname_128B_dep:ident, $vtype_128B_dep:ty,
        //     $vdep1_128B_dep:literal, $vdep2_128B_dep:literal ),* ,
        // noncopy_variants_128B_psize:
        // $( $vdoc_128B_psize:literal, $vname_128B_psize:ident, $vtype_128B_psize:ty, $vpsize_128B_psize:meta ),* ,
        // noncopy_variants_128B_psize_dep:
        // $( $vdoc_128B_psize_dep:literal, $vname_128B_psize_dep:ident, $vtype_128B_psize_dep:ty,
        //    $vpsize_128B_psize_dep:meta, $vdep1_128B_psize_dep:literal, $vdep2_128B_psize_dep:literal ),* ,

    ) => {
        // 1-Byte / 8-bit
        define_single_size! {
            $tname, $cname, $ucname,
            size: 1, 8,
            copy_variants: $( $cvdoc_1B, $cvname_1B, $cvtype_1B ),* ,
            copy_variants_dep:
                $( $cvdoc_1B_dep, $cvname_1B_dep, $cvtype_1B_dep, $cvdep1_1B_dep, $cvdep2_1B_dep ),* ;
            copy_variants_psize:
                $( $cvdoc_1B_psize, $cvname_1B_psize, $cvtype_1B_psize, $cvpsize_1B_psize ),* ;
            copy_variants_psize_dep: ;
            noncopy_variants: ;
                // $( $vdoc_1B, $vname_1B, $vtype_1B ),* ,
            noncopy_variants_dep: ;
                // $( $vdoc_1B_dep, $vname_1B_dep, $vtype_1B_dep, $vdep1_1B_dep, $vdep2_1B_dep ),* ;
            noncopy_variants_psize: ;
            noncopy_variants_psize_dep: ;
        }

        // 2-Byte / 16-bit
        define_single_size! {
            $tname, $cname, $ucname,
            size: 2, 16,
            copy_variants:
                $( $cvdoc_1B, $cvname_1B, $cvtype_1B ),* ,
                $( $cvdoc_2B, $cvname_2B, $cvtype_2B ),* ,
            copy_variants_dep:
                $( $cvdoc_1B_dep, $cvname_1B_dep, $cvtype_1B_dep, $cvdep1_1B_dep, $cvdep2_1B_dep ),* ,
                $( $cvdoc_2B_dep, $cvname_2B_dep, $cvtype_2B_dep, $cvdep1_2B_dep, $cvdep2_2B_dep ),* ;
            copy_variants_psize:
                $( $cvdoc_1B_psize, $cvname_1B_psize, $cvtype_1B_psize, $cvpsize_1B_psize ),* ,
                $( $cvdoc_2B_psize, $cvname_2B_psize, $cvtype_2B_psize, $cvpsize_2B_psize ),* ;
            copy_variants_psize_dep: ;
            noncopy_variants: ;
                // $( $vdoc_1B, $vname_1B, $vtype_1B ),* ,
                // $( $vdoc_2B, $vname_2B, $vtype_2B ),* ,
            noncopy_variants_dep: ;
                // $( $vdoc_1B_dep, $vname_1B_dep, $vtype_1B_dep, $vdep1_1B_dep, $vdep2_1B_dep ),* ,
                // $( $vdoc_2B_dep, $vname_2B_dep, $vtype_2B_dep, $vdep1_2B_dep, $vdep2_2B_dep ),* ;
            noncopy_variants_psize: ;
            noncopy_variants_psize_dep: ;
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
                $( $cvdoc_1B_dep, $cvname_1B_dep, $cvtype_1B_dep, $cvdep1_1B_dep, $cvdep2_1B_dep ),* ,
                $( $cvdoc_2B_dep, $cvname_2B_dep, $cvtype_2B_dep, $cvdep1_2B_dep, $cvdep2_2B_dep ),* ,
                $( $cvdoc_4B_dep, $cvname_4B_dep, $cvtype_4B_dep, $cvdep1_4B_dep, $cvdep2_4B_dep ),* ;
            copy_variants_psize:
                $( $cvdoc_1B_psize, $cvname_1B_psize, $cvtype_1B_psize, $cvpsize_1B_psize ),* ,
                $( $cvdoc_2B_psize, $cvname_2B_psize, $cvtype_2B_psize, $cvpsize_2B_psize ),* ,
                $( $cvdoc_4B_psize, $cvname_4B_psize, $cvtype_4B_psize, $cvpsize_4B_psize ),* ;
            copy_variants_psize_dep: ;
            noncopy_variants: ;
                // $( $vdoc_1B, $vname_1B, $vtype_1B ),* ,
                // $( $vdoc_2B, $vname_2B, $vtype_2B ),* ,
                // $( $vdoc_4B, $vname_4B, $vtype_4B ),* ;
            noncopy_variants_dep:
                // $( $vdoc_1B_dep, $vname_1B_dep, $vtype_1B_dep, $vdep1_1B_dep, $vdep2_1B_dep ),* ,
                // $( $vdoc_2B_dep, $vname_2B_dep, $vtype_2B_dep, $vdep1_2B_dep, $vdep2_2B_dep ),* ,
                $( $vdoc_4B_dep, $vname_4B_dep, $vtype_4B_dep, $vdep1_4B_dep, $vdep2_4B_dep ),* ;
            noncopy_variants_psize: ;
            noncopy_variants_psize_dep: ;
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
                $( $cvdoc_1B_dep, $cvname_1B_dep, $cvtype_1B_dep, $cvdep1_1B_dep, $cvdep2_1B_dep ),* ,
                $( $cvdoc_2B_dep, $cvname_2B_dep, $cvtype_2B_dep, $cvdep1_2B_dep, $cvdep2_2B_dep ),* ,
                $( $cvdoc_4B_dep, $cvname_4B_dep, $cvtype_4B_dep, $cvdep1_4B_dep, $cvdep2_4B_dep ),* ,
                $( $cvdoc_8B_dep, $cvname_8B_dep, $cvtype_8B_dep, $cvdep1_8B_dep, $cvdep2_8B_dep ),* ;
            copy_variants_psize:
                $( $cvdoc_1B_psize, $cvname_1B_psize, $cvtype_1B_psize, $cvpsize_1B_psize ),* ,
                $( $cvdoc_2B_psize, $cvname_2B_psize, $cvtype_2B_psize, $cvpsize_2B_psize ),* ,
                $( $cvdoc_4B_psize, $cvname_4B_psize, $cvtype_4B_psize, $cvpsize_4B_psize ),* ,
                $( $cvdoc_8B_psize, $cvname_8B_psize, $cvtype_8B_psize, $cvpsize_8B_psize ),* ;
            copy_variants_psize_dep: ;
            noncopy_variants: ;
                // $( $vdoc_1B, $vname_1B, $vtype_1B ),* ,
                // $( $vdoc_2B, $vname_2B, $vtype_2B ),* ,
                // $( $vdoc_4B, $vname_4B, $vtype_4B ),* ,
                // $( $vdoc_8B, $vname_8B, $vtype_8B ),* ;
            noncopy_variants_dep:
                // $( $vdoc_1B_dep, $vname_1B_dep, $vtype_1B_dep, $vdep1_1B_dep, $vdep2_1B_dep ),* ,
                // $( $vdoc_2B_dep, $vname_2B_dep, $vtype_2B_dep, $vdep1_2B_dep, $vdep2_2B_dep ),* ,
                $( $vdoc_4B_dep, $vname_4B_dep, $vtype_4B_dep, $vdep1_4B_dep, $vdep2_4B_dep ),* ,
                $( $vdoc_8B_dep, $vname_8B_dep, $vtype_8B_dep, $vdep1_8B_dep, $vdep2_8B_dep ),* ;
            noncopy_variants_psize: ;
            noncopy_variants_psize_dep:
                $( $vdoc_8B_psize_dep, $vname_8B_psize_dep, $vtype_8B_psize_dep, $vpsize_8B_psize_dep,
                   $vdep1_8B_psize_dep, $vdep2_8B_psize_dep ),* ;
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
                $( $cvdoc_1B_dep, $cvname_1B_dep, $cvtype_1B_dep, $cvdep1_1B_dep, $cvdep2_1B_dep ),* ,
                $( $cvdoc_2B_dep, $cvname_2B_dep, $cvtype_2B_dep, $cvdep1_2B_dep, $cvdep2_2B_dep ),* ,
                $( $cvdoc_4B_dep, $cvname_4B_dep, $cvtype_4B_dep, $cvdep1_4B_dep, $cvdep2_4B_dep ),* ,
                $( $cvdoc_8B_dep, $cvname_8B_dep, $cvtype_8B_dep, $cvdep1_8B_dep, $cvdep2_8B_dep ),* ,
                $( $cvdoc_16B_dep, $cvname_16B_dep, $cvtype_16B_dep, $cvdep1_16B_dep, $cvdep2_16B_dep ),* ;
            copy_variants_psize:
                $( $cvdoc_1B_psize, $cvname_1B_psize, $cvtype_1B_psize, $cvpsize_1B_psize ),* ,
                $( $cvdoc_2B_psize, $cvname_2B_psize, $cvtype_2B_psize, $cvpsize_2B_psize ),* ,
                $( $cvdoc_4B_psize, $cvname_4B_psize, $cvtype_4B_psize, $cvpsize_4B_psize ),* ,
                $( $cvdoc_8B_psize, $cvname_8B_psize, $cvtype_8B_psize, $cvpsize_8B_psize ),* ,
                $( $cvdoc_16B_psize, $cvname_16B_psize, $cvtype_16B_psize, $cvpsize_16B_psize ),* ;
            copy_variants_psize_dep: ;
            noncopy_variants: ;
                // $( $vdoc_1B, $vname_1B, $vtype_1B ),* ,
                // $( $vdoc_2B, $vname_2B, $vtype_2B ),* ,
                // $( $vdoc_4B, $vname_4B, $vtype_4B ),* ,
                // $( $vdoc_8B, $vname_8B, $vtype_8B ),* ;
                // $( $vdoc_16B, $vname_16B, $vtype_16B ),* ;
            noncopy_variants_dep:
                // $( $vdoc_1B_dep, $vname_1B_dep, $vtype_1B_dep, $vdep1_1B_dep, $vdep2_1B_dep ),* ,
                // $( $vdoc_2B_dep, $vname_2B_dep, $vtype_2B_dep, $vdep1_2B_dep, $vdep2_2B_dep ),* ,
                $( $vdoc_4B_dep, $vname_4B_dep, $vtype_4B_dep, $vdep1_4B_dep, $vdep2_4B_dep ),* ,
                $( $vdoc_8B_dep, $vname_8B_dep, $vtype_8B_dep, $vdep1_8B_dep, $vdep2_8B_dep ),* ;
                // $( $vdoc_16B_dep, $vname_16B_dep, $vtype_16B_dep, $vdep1_16B_dep, $vdep2_16B_dep ),* ;
            noncopy_variants_psize: ;
            noncopy_variants_psize_dep:
                $( $vdoc_8B_psize_dep, $vname_8B_psize_dep, $vtype_8B_psize_dep, $vpsize_8B_psize_dep,
                   $vdep1_8B_psize_dep, $vdep2_8B_psize_dep ),* ,
                $( $vdoc_16B_psize_dep, $vname_16B_psize_dep, $vtype_16B_psize_dep, $vpsize_16B_psize_dep,
                   $vdep1_16B_psize_dep, $vdep2_16B_psize_dep ),* ;
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
                $( $cvdoc_1B_dep, $cvname_1B_dep, $cvtype_1B_dep, $cvdep1_1B_dep, $cvdep2_1B_dep ),* ,
                $( $cvdoc_2B_dep, $cvname_2B_dep, $cvtype_2B_dep, $cvdep1_2B_dep, $cvdep2_2B_dep ),* ,
                $( $cvdoc_4B_dep, $cvname_4B_dep, $cvtype_4B_dep, $cvdep1_4B_dep, $cvdep2_4B_dep ),* ,
                $( $cvdoc_8B_dep, $cvname_8B_dep, $cvtype_8B_dep, $cvdep1_8B_dep, $cvdep2_8B_dep ),* ,
                $( $cvdoc_16B_dep, $cvname_16B_dep, $cvtype_16B_dep, $cvdep1_16B_dep, $cvdep2_16B_dep ),* ,
                $( $cvdoc_32B_dep, $cvname_32B_dep, $cvtype_32B_dep, $cvdep1_32B_dep, $cvdep2_32B_dep ),* ;
            copy_variants_psize:
                $( $cvdoc_1B_psize, $cvname_1B_psize, $cvtype_1B_psize, $cvpsize_1B_psize ),* ,
                $( $cvdoc_2B_psize, $cvname_2B_psize, $cvtype_2B_psize, $cvpsize_2B_psize ),* ,
                $( $cvdoc_4B_psize, $cvname_4B_psize, $cvtype_4B_psize, $cvpsize_4B_psize ),* ,
                $( $cvdoc_8B_psize, $cvname_8B_psize, $cvtype_8B_psize, $cvpsize_8B_psize ),* ,
                $( $cvdoc_16B_psize, $cvname_16B_psize, $cvtype_16B_psize, $cvpsize_16B_psize ),* ;
                // $( $cvdoc_32B_psize, $cvname_32B_psize, $cvtype_32B_psize, $cvpsize_32B_psize ),* ;
            copy_variants_psize_dep: ;
            noncopy_variants:
                // $( $vdoc_1B, $vname_1B, $vtype_1B ),* ,
                // $( $vdoc_2B, $vname_2B, $vtype_2B ),* ,
                // $( $vdoc_4B, $vname_4B, $vtype_4B ),* ,
                // $( $vdoc_8B, $vname_8B, $vtype_8B ),* ,
                // $( $vdoc_16B, $vname_16B, $vtype_16B ),* ,
                // $( $vdoc_32B, $vname_32B, $vtype_32B ),* ;
                ;
            noncopy_variants_dep:
                // $( $vdoc_1B_dep, $vname_1B_dep, $vtype_1B_dep, $vdep1_1B_dep, $vdep2_1B_dep ),* ,
                // $( $vdoc_2B_dep, $vname_2B_dep, $vtype_2B_dep, $vdep1_2B_dep, $vdep2_2B_dep ),* ,
                $( $vdoc_4B_dep, $vname_4B_dep, $vtype_4B_dep, $vdep1_4B_dep, $vdep2_4B_dep ),* ,
                $( $vdoc_8B_dep, $vname_8B_dep, $vtype_8B_dep, $vdep1_8B_dep, $vdep2_8B_dep ),* ,
                // $( $vdoc_16B_dep, $vname_16B_dep, $vtype_16B_dep, $vdep1_16B_dep, $vdep2_16B_dep ),* ,
                $( $vdoc_32B_dep, $vname_32B_dep, $vtype_32B_dep, $vdep1_32B_dep, $vdep2_32B_dep ),* ;
            noncopy_variants_psize: ;
            noncopy_variants_psize_dep:
                $( $vdoc_8B_psize_dep, $vname_8B_psize_dep, $vtype_8B_psize_dep, $vpsize_8B_psize_dep,
                   $vdep1_8B_psize_dep, $vdep2_8B_psize_dep ),* ,
                $( $vdoc_16B_psize_dep, $vname_16B_psize_dep, $vtype_16B_psize_dep, $vpsize_16B_psize_dep,
                   $vdep1_16B_psize_dep, $vdep2_16B_psize_dep ),* ,
                $( $vdoc_32B_psize_dep, $vname_32B_psize_dep, $vtype_32B_psize_dep, $vpsize_32B_psize_dep,
                   $vdep1_32B_psize_dep, $vdep2_32B_psize_dep ),* ;
        }

        define_single_size! {
            $tname, $cname, $ucname,
            size: 64, 512, // 64-Byte / 512-bit
            copy_variants:
                $( $cvdoc_1B, $cvname_1B, $cvtype_1B ),* ,
                $( $cvdoc_2B, $cvname_2B, $cvtype_2B ),* ,
                $( $cvdoc_4B, $cvname_4B, $cvtype_4B ),* ,
                $( $cvdoc_8B, $cvname_8B, $cvtype_8B ),* ,
                $( $cvdoc_16B, $cvname_16B, $cvtype_16B ),* ,
                $( $cvdoc_32B, $cvname_32B, $cvtype_32B ),* ,
                $( $cvdoc_64B, $cvname_64B, $cvtype_64B ),* ,
            copy_variants_dep:
                $( $cvdoc_1B_dep, $cvname_1B_dep, $cvtype_1B_dep, $cvdep1_1B_dep, $cvdep2_1B_dep ),* ,
                $( $cvdoc_2B_dep, $cvname_2B_dep, $cvtype_2B_dep, $cvdep1_2B_dep, $cvdep2_2B_dep ),* ,
                $( $cvdoc_4B_dep, $cvname_4B_dep, $cvtype_4B_dep, $cvdep1_4B_dep, $cvdep2_4B_dep ),* ,
                $( $cvdoc_8B_dep, $cvname_8B_dep, $cvtype_8B_dep, $cvdep1_8B_dep, $cvdep2_8B_dep ),* ,
                $( $cvdoc_16B_dep, $cvname_16B_dep, $cvtype_16B_dep, $cvdep1_16B_dep, $cvdep2_16B_dep ),* ,
                $( $cvdoc_32B_dep, $cvname_32B_dep, $cvtype_32B_dep, $cvdep1_32B_dep, $cvdep2_32B_dep ),* ,
                $( $cvdoc_64B_dep, $cvname_64B_dep, $cvtype_64B_dep, $cvdep1_64B_dep, $cvdep2_64B_dep ),* ;
            copy_variants_psize:
                $( $cvdoc_1B_psize, $cvname_1B_psize, $cvtype_1B_psize, $cvpsize_1B_psize ),* ,
                $( $cvdoc_2B_psize, $cvname_2B_psize, $cvtype_2B_psize, $cvpsize_2B_psize ),* ,
                $( $cvdoc_4B_psize, $cvname_4B_psize, $cvtype_4B_psize, $cvpsize_4B_psize ),* ,
                $( $cvdoc_8B_psize, $cvname_8B_psize, $cvtype_8B_psize, $cvpsize_8B_psize ),* ,
                $( $cvdoc_16B_psize, $cvname_16B_psize, $cvtype_16B_psize, $cvpsize_16B_psize ),* ;
                // $( $cvdoc_32B_psize, $cvname_32B_psize, $cvtype_32B_psize, $cvpsize_32B_psize ),* ,
                // $( $cvdoc_64B_psize, $cvname_64B_psize, $cvtype_64B_psize, $cvpsize_64B_psize ),* ;
            copy_variants_psize_dep: ;
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
                // $( $vdoc_1B_dep, $vname_1B_dep, $vtype_1B_dep, $vdep1_1B_dep, $vdep2_1B_dep ),* ,
                // $( $vdoc_2B_dep, $vname_2B_dep, $vtype_2B_dep, $vdep1_2B_dep, $vdep2_2B_dep ),* ,
                $( $vdoc_4B_dep, $vname_4B_dep, $vtype_4B_dep, $vdep1_4B_dep, $vdep2_4B_dep ),* ,
                $( $vdoc_8B_dep, $vname_8B_dep, $vtype_8B_dep, $vdep1_8B_dep, $vdep2_8B_dep ),* ,
                // $( $vdoc_16B_dep, $vname_16B_dep, $vtype_16B_dep, $vdep1_16B_dep, $vdep2_16B_dep ),* ,
                $( $vdoc_32B_dep, $vname_32B_dep, $vtype_32B_dep, $vdep1_32B_dep, $vdep2_32B_dep ),* ,
                $( $vdoc_64B_dep, $vname_64B_dep, $vtype_64B_dep, $vdep1_64B_dep, $vdep2_64B_dep ),* ;
            noncopy_variants_psize: ;
            noncopy_variants_psize_dep:
                $( $vdoc_8B_psize_dep, $vname_8B_psize_dep, $vtype_8B_psize_dep, $vpsize_8B_psize_dep,
                   $vdep1_8B_psize_dep, $vdep2_8B_psize_dep ),* ,
                $( $vdoc_16B_psize_dep, $vname_16B_psize_dep, $vtype_16B_psize_dep, $vpsize_16B_psize_dep,
                   $vdep1_16B_psize_dep, $vdep2_16B_psize_dep ),* ,
                $( $vdoc_32B_psize_dep, $vname_32B_psize_dep, $vtype_32B_psize_dep, $vpsize_32B_psize_dep,
                   $vdep1_32B_psize_dep, $vdep2_32B_psize_dep ),* ,
                $( $vdoc_64B_psize_dep, $vname_64B_psize_dep, $vtype_64B_psize_dep, $vpsize_64B_psize_dep,
                   $vdep1_64B_psize_dep, $vdep2_64B_psize_dep ),* ;
        }

        define_single_size! {
            $tname, $cname, $ucname,
            size: 128, 1024, // 128-Byte / 1024-bit
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
                $( $cvdoc_1B_dep, $cvname_1B_dep, $cvtype_1B_dep, $cvdep1_1B_dep, $cvdep2_1B_dep ),* ,
                $( $cvdoc_2B_dep, $cvname_2B_dep, $cvtype_2B_dep, $cvdep1_2B_dep, $cvdep2_2B_dep ),* ,
                $( $cvdoc_4B_dep, $cvname_4B_dep, $cvtype_4B_dep, $cvdep1_4B_dep, $cvdep2_4B_dep ),* ,
                $( $cvdoc_8B_dep, $cvname_8B_dep, $cvtype_8B_dep, $cvdep1_8B_dep, $cvdep2_8B_dep ),* ,
                $( $cvdoc_16B_dep, $cvname_16B_dep, $cvtype_16B_dep, $cvdep1_16B_dep, $cvdep2_16B_dep ),* ,
                $( $cvdoc_32B_dep, $cvname_32B_dep, $cvtype_32B_dep, $cvdep1_32B_dep, $cvdep2_32B_dep ),* ,
                $( $cvdoc_64B_dep, $cvname_64B_dep, $cvtype_64B_dep, $cvdep1_64B_dep, $cvdep2_64B_dep ),* ,
                $( $cvdoc_128B_dep, $cvname_128B_dep, $cvtype_128B_dep, $cvdep1_128B_dep, $cvdep2_128B_dep ),* ;
            copy_variants_psize:
                $( $cvdoc_1B_psize, $cvname_1B_psize, $cvtype_1B_psize, $cvpsize_1B_psize ),* ,
                $( $cvdoc_2B_psize, $cvname_2B_psize, $cvtype_2B_psize, $cvpsize_2B_psize ),* ,
                $( $cvdoc_4B_psize, $cvname_4B_psize, $cvtype_4B_psize, $cvpsize_4B_psize ),* ,
                $( $cvdoc_8B_psize, $cvname_8B_psize, $cvtype_8B_psize, $cvpsize_8B_psize ),* ,
                $( $cvdoc_16B_psize, $cvname_16B_psize, $cvtype_16B_psize, $cvpsize_16B_psize ),* ;
                // $( $cvdoc_32B_psize, $cvname_32B_psize, $cvtype_32B_psize, $cvpsize_32B_psize ),* ,
                // $( $cvdoc_64B_psize, $cvname_64B_psize, $cvtype_64B_psize, $cvpsize_64B_psize ),* ,
                // $( $cvdoc_128B_psize, $cvname_128B_psize, $cvtype_128B_psize, $cvpsize_128B_psize ),* ;
            copy_variants_psize_dep: ;
            noncopy_variants:
                // $( $vdoc_4B, $vname_4B, $vtype_4B ),* ,
                // $( $vdoc_8B, $vname_8B, $vtype_8B ),* ,
                // $( $vdoc_16B, $vname_16B, $vtype_16B ),* ,
                // $( $vdoc_32B, $vname_32B, $vtype_32B ),* ;
                // $( $vdoc_64B, $vname_64B, $vtype_64B ),* ;
                // $( $vdoc_128B, $vname_128B, $vtype_128B ),*
                ;
            noncopy_variants_dep:
                // $( $vdoc_1B_dep, $vname_1B_dep, $vtype_1B_dep, $vdep1_1B_dep, $vdep2_1B_dep ),* ,
                // $( $vdoc_2B_dep, $vname_2B_dep, $vtype_2B_dep, $vdep1_2B_dep, $vdep2_2B_dep ),* ,
                $( $vdoc_4B_dep, $vname_4B_dep, $vtype_4B_dep, $vdep1_4B_dep, $vdep2_4B_dep ),* ,
                $( $vdoc_8B_dep, $vname_8B_dep, $vtype_8B_dep, $vdep1_8B_dep, $vdep2_8B_dep ),* ,
                // $( $vdoc_16B_dep, $vname_16B_dep, $vtype_16B_dep, $vdep1_16B_dep, $vdep2_16B_dep ),* ,
                $( $vdoc_32B_dep, $vname_32B_dep, $vtype_32B_dep, $vdep1_32B_dep, $vdep2_32B_dep ),* ,
                $( $vdoc_64B_dep, $vname_64B_dep, $vtype_64B_dep, $vdep1_64B_dep, $vdep2_64B_dep ),* ;
                // $( $vdoc_128B_dep, $vname_128B_dep, $vtype_128B_dep, $vdep1_128B_dep, $vdep2_128B_dep ),* ;
            noncopy_variants_psize: ;
            noncopy_variants_psize_dep:
                $( $vdoc_8B_psize_dep, $vname_8B_psize_dep, $vtype_8B_psize_dep, $vpsize_8B_psize_dep,
                   $vdep1_8B_psize_dep, $vdep2_8B_psize_dep ),* ,
                $( $vdoc_16B_psize_dep, $vname_16B_psize_dep, $vtype_16B_psize_dep, $vpsize_16B_psize_dep,
                   $vdep1_16B_psize_dep, $vdep2_16B_psize_dep ),* ,
                $( $vdoc_32B_psize_dep, $vname_32B_psize_dep, $vtype_32B_psize_dep, $vpsize_32B_psize_dep,
                   $vdep1_32B_psize_dep, $vdep2_32B_psize_dep ),* ,
                $( $vdoc_64B_psize_dep, $vname_64B_psize_dep, $vtype_64B_psize_dep, $vpsize_64B_psize_dep,
                   $vdep1_64B_psize_dep, $vdep2_64B_psize_dep ),* ;
        }
    };
}

/// for defining in one pass: DataType*, DataCell* & DataUnsafe*
/// also DataLine*
macro_rules! define_single_size {
    (
        $tname:ident, $cname:ident, $ucname:ident,
        size: $B:literal, $b:literal,
        copy_variants: $( $cvdoc:literal, $cvname:ident, $cvtype:ty ),* ,
        copy_variants_dep: $( $cvdoc_dep:literal, $cvname_dep:ident, $cvtype_dep:ty,
            $cvdep1_dep:literal, $cvdep2_dep:literal ),* ;
        copy_variants_psize: $( $cvdoc_psize:literal, $cvname_psize:ident, $cvtype_psize:ty,
            $cvpsize_psize:meta ),* ;
        copy_variants_psize_dep:
            $( $cvdoc_psize_dep:literal, $cvname_psize_dep:ident, $cvtype_psize_dep:ty,
            $cvpsize_psize_dep:meta, $cvdep1_psize_dep:literal, $cvdep2_psize_dep:literal ),* ;
        noncopy_variants: $( $vdoc:literal, $vname:ident, $vtype:ty ),* ;
        noncopy_variants_dep: $( $vdoc_dep:literal, $vname_dep:ident, $vtype_dep:ty,
            $vdep1_dep:literal, $vdep2_dep:literal ),* ;
        noncopy_variants_psize: ;
        noncopy_variants_psize_dep:
        $( $vdoc_psize_dep:literal, $vname_psize_dep:ident, $vtype_psize_dep:ty,
           $vpsize_psize_dep:meta, $vdep1_psize_dep:literal, $vdep2_psize_dep:literal ),* ;
    ) => {
        define_type!{
            $tname, size: $B, $b,
            copy_variants: $( $cvdoc, $cvname, $cvtype ),* ,
            copy_variants_dep: $( $cvdoc_dep, $cvname_dep, $cvtype_dep, $cvdep1_dep, $cvdep2_dep ),* ;
            copy_variants_psize: $( $cvdoc_psize, $cvname_psize, $cvtype_psize, $cvpsize_psize ),* ;
            copy_variants_psize_dep: $( $cvdoc_psize_dep, $cvname_psize_dep, $cvtype_psize_dep, $cvpsize_psize_dep,
                $cvdep1_psize_dep, $cvdep2_psize_dep ),* ;
            noncopy_variants: $( $vdoc, $vname, $vtype ),* ;
            noncopy_variants_dep: $( $vdoc_dep, $vname_dep, $vtype_dep, $vdep1_dep, $vdep2_dep ),* ;
            noncopy_variants_psize_dep: $( $vdoc_psize_dep, $vname_psize_dep, $vtype_psize_dep, $vpsize_psize_dep,
                $vdep1_psize_dep, $vdep2_psize_dep ),* ;
        }
        define_cell!{
            c: $cname, t:$tname, u:$ucname, size: $B, $b,
            copy_variants: $( $cvdoc, $cvname, $cvtype ),* ,
            copy_variants_dep: $( $cvdoc_dep, $cvname_dep, $cvtype_dep, $cvdep1_dep, $cvdep2_dep ),* ;
            copy_variants_psize: $( $cvdoc_psize, $cvname_psize, $cvtype_psize, $cvpsize_psize ),* ;
            copy_variants_psize_dep: $( $cvdoc_psize_dep, $cvname_psize_dep, $cvtype_psize_dep, $cvpsize_psize_dep,
                $cvdep1_psize_dep, $cvdep2_psize_dep ),* ;
            noncopy_variants: $( $vdoc, $vname, $vtype ),* ;
            noncopy_variants_dep: $( $vdoc_dep, $vname_dep, $vtype_dep, $vdep1_dep, $vdep2_dep ),* ;
            noncopy_variants_psize_dep: $( $vdoc_psize_dep, $vname_psize_dep, $vtype_psize_dep, $vpsize_psize_dep,
                $vdep1_psize_dep, $vdep2_psize_dep ),* ;
        }
        define_unsafe!{
            $ucname, size: $B, $b,
            copy_variants: $( $cvdoc, $cvname, $cvtype ),* ,
            copy_variants_dep: $( $cvdoc_dep, $cvname_dep, $cvtype_dep, $cvdep1_dep, $cvdep2_dep ),* ;
            copy_variants_psize: $( $cvdoc_psize, $cvname_psize, $cvtype_psize, $cvpsize_psize ),* ;
            copy_variants_psize_dep: $( $cvdoc_psize_dep, $cvname_psize_dep, $cvtype_psize_dep, $cvpsize_psize_dep,
                $cvdep1_psize_dep, $cvdep2_psize_dep ),* ;
        }

        // WIP:lines
        define_line!{
            c: $cname, t:$tname, u:$ucname, size: $B, $b,
        }
    };
}

/// for defining enum DataType*
macro_rules! define_type {
    (
        $tname:ident,
        size: $B:literal, $b:literal,
        copy_variants: $( $cvdoc:literal, $cvname:ident, $cvtype:ty ),* ,
        copy_variants_dep: $( $cvdoc_dep:literal, $cvname_dep:ident, $cvtype_dep:ty,
            $cvdep1_dep:literal, $cvdep2_dep:literal ),* ;
        copy_variants_psize: $( $cvdoc_psize:literal, $cvname_psize:ident, $cvtype_psize:ty,
            $cvpsize_psize:meta ),* ;
        copy_variants_psize_dep: $( $cvdoc_psize_dep:literal, $cvname_psize_dep:ident, $cvtype_psize_dep:ty,
            $cvpsize_psize_dep:meta, $cvdep1_psize_dep:literal, $cvdep2_psize_dep:literal ),* ;
        noncopy_variants: $( $vdoc:literal, $vname:ident, $vtype:ty ),* ;
        noncopy_variants_dep: $( $vdoc_dep:literal, $vname_dep:ident, $vtype_dep:ty,
            $vdep1_dep:literal, $vdep2_dep:literal ),* ;
        noncopy_variants_psize_dep: $( $vdoc_psize_dep:literal, $vname_psize_dep:ident, $vtype_psize_dep:ty,
            $vpsize_psize_dep:meta, $vdep1_psize_dep:literal, $vdep2_psize_dep:literal ),* ;
    ) =>  {
        paste::paste!{
            // ## copy version (DataType)
            // -----------------------------------------------------------------
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

                $( // fundamental types
                    #[doc = $cvdoc ]
                    $cvname,
                )*

                $( // pointer-size dependant
                    #[cfg($cvpsize_psize)]
                    #[doc = $cvdoc_psize]
                    $cvname_psize,
                )*

                $( // feature-gated dependencies
                    #[cfg(all(feature = $cvdep1_dep, feature = $cvdep2_dep))]
                    #[doc = $cvdoc_dep]
                    $cvname_dep,
                )*

                $( // pointer-size & feature-gated dependencies
                    #[cfg(all($cvpsize_psize_dep, feature = $cvdep1_psize_dep, feature = $cvdep2_psize_dep))]
                    #[doc = $cvdoc_psize_dep]
                    $cvname_psize_dep,
                )*

            }
            type_aliases![t: $tname, size: $B, $b, "Copy", "data **Type**", "(Copy)" ];
            impl_data_types![ [< $tname $B Byte Copy With >], DataTypesCopy,
                is_copy: true,
                copy_variants: $( $cvname, $cvtype ),* ;
                copy_variants_dep: $( $cvname_dep, $cvtype_dep, $cvdep1_dep, $cvdep2_dep ),* ;
                copy_variants_psize: $( $cvname_psize, $cvtype_psize, $cvpsize_psize ),* ;
                copy_variants_psize_dep: $( $cvname_psize_dep, $cvtype_psize_dep, $cvpsize_psize_dep,
                    $cvdep1_psize_dep, $cvdep2_psize_dep ),* ;
                noncopy_variants: ;
                noncopy_variants_dep: ;
                noncopy_variants_psize_dep: ;
            ];
            impl<T: DataTypesCopy> DataTypesCopy for [< $tname $B Byte Copy With >]<T> { }

            // ## non-copy version (DataType)
            // -----------------------------------------------------------------
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

                $( // fundamental types
                    #[doc = $cvdoc ]
                    $cvname,
                )*
                $(
                    #[doc = $vdoc ]
                    $vname,
                )*

                $( // pointer-size dependant
                    #[cfg($cvpsize_psize)]
                    #[doc = $cvdoc_psize]
                    $cvname_psize,
                )*

                $( // feature-gated dependencies
                    #[cfg(all(feature = $cvdep1_dep, feature = $cvdep2_dep))]
                    #[doc = $cvdoc_dep]
                    $cvname_dep,
                )*
                $(
                    #[cfg(all(feature = $vdep1_dep, feature = $vdep2_dep))]
                    #[doc = $vdoc_dep]
                    $vname_dep,
                )*
                $( // pointer-size & feature-gated dependencies
                    #[cfg(all($cvpsize_psize_dep, feature = $cvdep1_psize_dep, feature = $cvdep2_psize_dep))]
                    #[doc = $cvdoc_psize_dep]
                    $cvname_psize_dep,
                )*
                $(
                    #[cfg(all($vpsize_psize_dep, feature = $vdep1_psize_dep, feature = $vdep2_psize_dep))]
                    #[doc = $vdoc_psize_dep]
                    $vname_psize_dep,
                )*

            }
            type_aliases![t: $tname, size: $B, $b, "", "data **Type**", ""];
            impl_data_types![ [< $tname $B Byte With >], DataTypes,
                is_copy: false,
                copy_variants: $( $cvname, $cvtype ),* ;
                copy_variants_dep: $( $cvname_dep, $cvtype_dep, $cvdep1_dep, $cvdep2_dep ),* ;
                copy_variants_psize: $( $cvname_psize, $cvtype_psize, $cvpsize_psize ),* ;
                copy_variants_psize_dep: $( $cvname_psize_dep, $cvtype_psize_dep, $cvpsize_psize_dep,
                    $cvdep1_psize_dep, $cvdep2_psize_dep ),* ;
                noncopy_variants: $($vname, $vtype ),* ;
                noncopy_variants_dep: $( $vname_dep, $vtype_dep, $vdep1_dep, $vdep2_dep ),* ;
                noncopy_variants_psize_dep: $( $vname_psize_dep, $vtype_psize_dep, $vpsize_psize_dep,
                    $vdep1_psize_dep, $vdep2_psize_dep ),* ;
            ];
        }
    };
}

/// for defining enum DataCell*
macro_rules! define_cell {
    (
        c: $cname:ident, t: $tname:ident, u: $ucname:ident,
        size: $B:literal, $b:literal,
        copy_variants: $( $cvdoc:literal, $cvname:ident, $cvtype:ty ),* ,
        copy_variants_dep: $( $cvdoc_dep:literal, $cvname_dep:ident, $cvtype_dep:ty,
            $cvdep1_dep:literal, $cvdep2_dep:literal ),* ;
        copy_variants_psize: $( $cvdoc_psize:literal, $cvname_psize:ident, $cvtype_psize:ty,
            $cvpsize_psize:meta ),* ;
        copy_variants_psize_dep: $( $cvdoc_psize_dep:literal, $cvname_psize_dep:ident, $cvtype_psize_dep:ty,
            $cvpsize_psize_dep:meta, $cvdep1_psize_dep:literal, $cvdep2_psize_dep:literal ),* ;
        noncopy_variants: $( $vdoc:literal, $vname:ident, $vtype:ty ),* ;
        noncopy_variants_dep: $( $vdoc_dep:literal, $vname_dep:ident, $vtype_dep:ty,
            $vdep1_dep:literal, $vdep2_dep:literal ),* ;
        noncopy_variants_psize_dep: $( $vdoc_psize_dep:literal, $vname_psize_dep:ident, $vtype_psize_dep:ty,
            $vpsize_psize_dep:meta, $vdep1_psize_dep:literal, $vdep2_psize_dep:literal ),* ;
    ) => {
        paste::paste!{
            // ## copy version (DataCell)
            // -----------------------------------------------------------------
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

                $( // fundamental types
                    #[doc = $cvdoc]
                    $cvname($cvtype),
                )*

                $( // pointer-size dependant
                    #[cfg($cvpsize_psize)]
                    #[doc = $cvdoc_psize]
                    $cvname_psize($cvtype_psize),
                )*

                $( // feature-gated dependencies
                    #[cfg(all(feature = $cvdep1_dep, feature = $cvdep2_dep))]
                    #[doc = $cvdoc_dep]
                    $cvname_dep($cvtype_dep),
                )*

                $( // pointer-size & feature-gated dependencies
                    #[cfg(all($cvpsize_psize_dep, feature = $cvdep1_psize_dep, feature = $cvdep2_psize_dep))]
                    #[doc = $cvdoc_psize_dep]
                    $cvname_psize_dep($cvtype_psize_dep),
                )*
            }
            type_aliases![c: $cname, size: $B, $b, "Copy", "data **Cell**", "(Copy)"];
            impl_data_cells![
                c: [< $cname $B Byte Copy With >], DataCellsCopy,
                t: [< $tname $B Byte Copy With >], DataTypesCopy,
                is_copy: true,
                copy_variants: $( $cvname, $cvtype ),* ;
                copy_variants_dep: $( $cvname_dep, $cvtype_dep, $cvdep1_dep, $cvdep2_dep ),* ;
                copy_variants_psize: $( $cvname_psize, $cvtype_psize, $cvpsize_psize ),* ;
                copy_variants_psize_dep: $( $cvname_psize_dep, $cvtype_psize_dep, $cvpsize_psize_dep,
                    $cvdep1_psize_dep, $cvdep2_psize_dep ),* ;
                noncopy_variants: ;
                noncopy_variants_dep: ;
                noncopy_variants_psize_dep: ;
            ];
            // impl<C: DataCellsCopy, T: DataTypesCopy> DataCellsCopy for [< $cname $B Byte Copy With >]<C, T> { }
            impl<C: DataCellsCopy> DataCellsCopy for [< $cname $B Byte Copy With >]<C> { }

            // ## non-copy version (DataCell)
            // -----------------------------------------------------------------
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

                $( // fundamental types
                    #[doc = $cvdoc]
                    $cvname($cvtype),
                )*
                $(
                    #[doc = $vdoc]
                    $vname($vtype),
                )*

                $( // pointer-size dependant
                    #[cfg($cvpsize_psize)]
                    #[doc = $cvdoc_psize]
                    $cvname_psize($cvtype_psize),
                )*

                $( // feature-gated dependencies
                    #[cfg(all(feature = $cvdep1_dep, feature = $cvdep2_dep))]
                    #[doc = $cvdoc_dep]
                    $cvname_dep($cvtype_dep),
                )*
                $(
                    #[cfg(all(feature = $vdep1_dep, feature = $vdep2_dep))]
                    #[doc = $vdoc_dep]
                    $vname_dep($vtype_dep),
                )*

                $( // pointer-size & feature-gated dependencies
                    #[cfg(all($cvpsize_psize_dep, feature = $cvdep1_psize_dep, feature = $cvdep2_psize_dep))]
                    #[doc = $cvdoc_psize_dep]
                    $cvname_psize_dep($cvtype_psize_dep),
                )*
                $(
                    #[cfg(all($vpsize_psize_dep, feature = $vdep1_psize_dep, feature = $vdep2_psize_dep))]
                    #[doc = $vdoc_psize_dep]
                    $vname_psize_dep($vtype_psize_dep),
                )*
            }

            type_aliases![c: $cname, size: $B, $b, "", "data **Cell**", ""];

            // implement the DataCells trait
            impl_data_cells![
                c: [< $cname $B Byte With >], DataCells,
                t: [< $tname $B Byte With >], DataTypes,
                is_copy: false,
                copy_variants: $( $cvname, $cvtype ),* ;
                copy_variants_dep: $( $cvname_dep, $cvtype_dep, $cvdep1_dep, $cvdep2_dep ),* ;
                copy_variants_psize: $( $cvname_psize, $cvtype_psize, $cvpsize_psize ),* ;
                copy_variants_psize_dep: $( $cvname_psize_dep, $cvtype_psize_dep, $cvpsize_psize_dep,
                    $cvdep1_psize_dep, $cvdep2_psize_dep ),* ;
                noncopy_variants: $($vname, $vtype ),* ;
                noncopy_variants_dep: $( $vname_dep, $vtype_dep, $vdep1_dep, $vdep2_dep ),* ;
                noncopy_variants_psize_dep: $( $vname_psize_dep, $vtype_psize_dep, $vpsize_psize_dep,
                    $vdep1_psize_dep, $vdep2_psize_dep ),* ;
            ];

            // implement `TryFrom`<`DataCell`> for *contained-value*:

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
            $( // Copy feature-bound
                #[cfg(all(feature = $cvdep1_dep, feature = $cvdep2_dep ))]
                impl<C: DataCells> TryFrom<[<$cname $B Byte With>]<C>> for $cvtype_dep {
                    type Error = ();
                    fn try_from(c: [<$cname $B Byte With>]<C>) -> Result<Self, Self::Error> {
                        match c {
                            [<$cname $B Byte With>]::$cvname_dep(c) => Ok(c),
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
            $( // non-Copy feature-bound
                #[cfg(all(feature = $vdep1_dep, feature = $vdep2_dep ))]
                impl<C: DataCells> TryFrom<[<$cname $B Byte With>]<C>> for $vtype_dep {
                    type Error = ();
                    fn try_from(c: [<$cname $B Byte With>]<C>) -> Result<Self, Self::Error> {
                        match c {
                            [<$cname $B Byte With>]::$vname_dep(c) => Ok(c),
                            _ => Err(()),
                        }
                    }

                }
            )*

            // implement `From`<*contained-value*> for `DataCell`:

            $( // Copy
                impl<C: DataCellsCopy> From<$cvtype> for [<$cname $B Byte Copy With>]<C> {
                    fn from(v: $cvtype) -> Self {
                        [<$cname $B Byte Copy With>]::$cvname(v)
                    }

                }
            )*
            $( // Copy feature-bound
                #[cfg(all(feature = $cvdep1_dep, feature = $cvdep2_dep ))]
                impl<C: DataCellsCopy> From<$cvtype_dep> for [<$cname $B Byte Copy With>]<C> {
                    fn from(v: $cvtype_dep) -> Self {
                        [<$cname $B Byte Copy With>]::$cvname_dep(v)
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
            $( // non-Copy feature-bound
                #[cfg(all(feature = $vdep1_dep, feature = $vdep2_dep ))]
                impl<C: DataCells> From<$vtype_dep> for [<$cname $B Byte With>]<C> {
                    fn from(v: $vtype_dep) -> Self {
                        [<$cname $B Byte With>]::$vname_dep(v)
                    }

                }
            )*

            // from DataCell to DataUnsafe
            impl<C: DataCellsCopy> From<[<$cname $B Byte Copy With>]<C>> for [<$ucname $B Byte Copy>] {
                fn from(cell: [<$cname $B Byte Copy With>]<C>) -> Self {
                    match cell {
                        [<$cname $B Byte Copy With>]::None => Self { None: NoData },
                        [<$cname $B Byte Copy With>]::With(_) => Self { None: NoData },

                        $( // fundamental types
                            [<$cname $B Byte Copy With>]::$cvname(v) => Self { $cvname: v },
                        )*

                        $( // pointer-size dependant
                            #[cfg($cvpsize_psize)]
                            [<$cname $B Byte Copy With>]::$cvname_psize(u) => Self { $cvname_psize: u },
                        )*

                        $( // feature-gated dependencies
                            #[cfg(all(feature = $cvdep1_dep, feature = $cvdep2_dep))]
                            [<$cname $B Byte Copy With>]::$cvname_dep(v) => Self { $cvname_dep: v },
                        )*
                    }
                }

            }
        }
    };
}

/// for defining union DataUnsafe*
macro_rules! define_unsafe {
    // # ATM receive only Copy variants (DataUnsafe)
    (
        $ucname:ident,
        size: $B:literal, $b:literal,
        copy_variants: $( $cvdoc:literal, $cvname:ident, $cvtype:ty ),* ,
        copy_variants_dep: $( $cvdoc_dep:literal, $cvname_dep:ident, $cvtype_dep:ty,
            $cvdep1_dep:literal, $cvdep2_dep:literal ),* ;
        copy_variants_psize: $( $cvdoc_psize:literal, $cvname_psize:ident, $cvtype_psize:ty,
            $cvpsize_psize:meta ),* ;
        copy_variants_psize_dep: $( $cvname_psize_dep:ident, $cvtype_psize_dep:ty,
            $cvpsize_psize_dep:meta, $cvdep1_psize_dep:literal, $cvdep2_psize_dep:literal ),* ;
    ) => {
        paste::paste!{
            // ## copy version (DataUnsafe)
            // -----------------------------------------------------------------
            #[repr(C)]
            #[doc = $B "Byte / " $b "bit " "data *unsafe* **Cell**"]
            #[derive(Copy, Clone)]
            pub union [<$ucname $B Byte Copy>] {
                /// Represents the absence of *data*.
                pub None: NoData,

                $(
                    #[doc = $cvdoc]
                    pub $cvname: $cvtype,
                )*

                $( // pointer-size dependant
                    #[cfg($cvpsize_psize)]
                    #[doc = $cvdoc_psize]
                    $cvname_psize: $cvtype_psize,
                )*

                // feature-gated dependencies
                $(
                    #[cfg(all(feature = $cvdep1_dep, feature = $cvdep2_dep))]
                    #[doc = $cvdoc_dep]
                    pub $cvname_dep: $cvtype_dep,
                )*

                $( // pointer-size & feature-gated dependencies
                    #[cfg(all($cvpsize_psize_dep, feature = $cvdep1_psize_dep, feature = $cvdep2_psize_dep))]
                    #[doc = $cvdoc_psize_dep]
                    $cvname_psize_dep($cvtype_psize_dep),
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

            impl_data_unsafes![
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
            // byte version without `With`:
            #[doc = $B "-Byte / " $b "-bit " $dsuf1 " " $dsuf2 ]
            ///
            /// See also:
            #[doc = "- [" [<$name $B Byte $nsuf With>] "][" [<$name $B Byte $nsuf With>] "] +With" ]
            #[doc = "- [" [<$name $b bit $nsuf With>] "][" [<$name $b bit $nsuf With>] "] as bit +With" ]
            #[doc = "- [" [<$name $b bit $nsuf>] "][" [<$name $b bit $nsuf>] "] as bit" ]
            // pub type [< $name $B Byte $nsuf >] = [< $name $B Byte $nsuf With >]<NoData, NoData>;
            pub type [< $name $B Byte $nsuf >] = [< $name $B Byte $nsuf With >]<NoData>;

            // bit version without `With`:
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
        copy_variants_dep: $( $cvname_dep:ident, $cvtype_dep:ty,
            $cvdep1_dep:literal, $cvdep2_dep:literal ),* ;
        copy_variants_psize: $( $cvname_psize:ident, $cvtype_psize:ty, $cvpsize_psize:meta ),* ;
        copy_variants_psize_dep: $( $cvname_psize_dep:ident, $cvtype_psize_dep:ty,
            $cvpsize_psize_dep:meta, $cvdep1_psize_dep:literal, $cvdep2_psize_dep:literal ),* ;
        noncopy_variants: $( $vname:ident, $vtype:ty ),* ;
        noncopy_variants_dep: $( $vname_dep:ident, $vtype_dep:ty,
            $vdep1_dep:literal, $vdep2_dep:literal ),* ;
        noncopy_variants_psize_dep: $( $vname_psize_dep:ident, $vtype_psize_dep:ty,
            $vpsize_psize_dep:meta, $vdep1_psize_dep:literal, $vdep2_psize_dep:literal ),* ;
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

                        $( // pointer-size dependant
                            #[cfg($cvpsize_psize)]
                            $cvname_psize => align_of::<$cvtype_psize>(),
                        )*

                        $( // feature-gated dependencies
                            #[cfg(all(feature = $cvdep1_dep, feature = $cvdep2_dep))]
                            $cvname_dep => align_of::<$cvtype_dep>(),
                        )*
                        $(
                            #[cfg(all(feature = $vdep1_dep, feature = $vdep2_dep))]
                            $vname_dep => align_of::<$vtype_dep>(),
                        )*

                        $( // pointer-size & feature-gated dependencies
                            #[cfg(all($cvpsize_psize_dep, feature = $cvdep1_psize_dep, feature = $cvdep2_psize_dep))]
                            $cvname_psize_dep => align_of::<$cvtype_psize_dep>(),
                        )*
                        $(
                            #[cfg(all($vpsize_psize_dep, feature = $vdep1_psize_dep, feature = $vdep2_psize_dep))]
                            $vname_psize_dep => align_of::<$vtype_psize_dep>(),
                        )*
                    }
                }
                fn data_size(&self) -> usize {
                    use $tname::*;
                    match self {
                        None => size_of::<NoData>(),
                        With(o) => o.data_size(),

                        $( $cvname => size_of::<$cvtype>(), )*
                        $( $vname => size_of::<$vtype>(), )*

                        $( // pointer-size dependant
                            #[cfg($cvpsize_psize)]
                            $cvname_psize => size_of::<$cvtype_psize>(),
                        )*

                        $( // feature-gated dependencies
                            #[cfg(all(feature = $cvdep1_dep, feature = $cvdep2_dep))]
                            $cvname_dep => size_of::<$cvtype_dep>(),
                        )*
                        $(
                            #[cfg(all(feature = $vdep1_dep, feature = $vdep2_dep))]
                            $vname_dep => size_of::<$vtype_dep>(),
                        )*

                        $( // pointer-size & feature-gated dependencies
                            #[cfg(all($cvpsize_psize_dep, feature = $cvdep1_psize_dep, feature = $cvdep2_psize_dep))]
                            $cvname_psize_dep => size_of::<$cvtype_psize_dep>(),
                        )*
                        $(
                            #[cfg(all($vpsize_psize_dep, feature = $vdep1_psize_dep, feature = $vdep2_psize_dep))]
                            $vname_psize_dep => size_of::<$vtype_psize_dep>(),
                        )*
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
        copy_variants_dep: $( $cvname_dep:ident, $cvtype_dep:ty,
            $cvdep1_dep:literal, $cvdep2_dep:literal ),* ;
        copy_variants_psize: $( $cvname_psize:ident, $cvtype_psize:ty, $cvpsize_psize:meta ),* ;
        copy_variants_psize_dep: $( $cvname_psize_dep:ident, $cvtype_psize_dep:ty,
            $cvpsize_psize_dep:meta, $cvdep1_psize_dep:literal, $cvdep2_psize_dep:literal ),* ;
        noncopy_variants: $( $vname:ident, $vtype:ty ),* ;
        noncopy_variants_dep: $( $vname_dep:ident, $vtype_dep:ty,
            $vdep1_dep:literal, $vdep2_dep:literal ),* ;
        noncopy_variants_psize_dep: $( $vname_psize_dep:ident, $vtype_psize_dep:ty,
            $vpsize_psize_dep:meta, $vdep1_psize_dep:literal, $vdep2_psize_dep:literal ),* ;
    ) => {
        paste::paste! {
            // impl<C: $cbound, T: $tbound> DataCells for $cname<C, T> {
            impl<C: $cbound> DataCells for $cname<C> {
                // type TYPE = $tname<$tbound>;
                // WIP
                // fn data_type(&self) -> Self::TYPE {
                //     match self {
                //         Self::None => Self::TYPE::None,
                //         Self::With(c) => c.data_type(),
                //         // WIP
                //         // $(
                //         //     Self::$vname(_) => Self::TYPE::$vname,
                //         // ),*
                //
                //         _ => Self::TYPE::None, // TEMP
                //     }
                // }

                fn is_copy(&self) -> bool { $is_copy }
            }
        }
    };
}
/// implement: DataUnsafes trait
macro_rules! impl_data_unsafes {
    (
      u: $ucname:ident,
    ) => {
        // impl DataCells for $ucname {
        //     fn is_copy(&self) -> bool { true }
        // }
        // impl DataCellsCopy for $ucname {}
        unsafe impl DataUnsafes for $ucname {}
    };
}

// MOCKUPS OF UNUSED DEPENDENCIES
// -----------------------------------------------------------------------------

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
// -----------------------------------------------------------------------------

define_all_sizes! {
    DataType, DataCell, DataUnsafe,

    // -------------------------------------------------------- 1-B / 8-b
    copy_variants_1B:
    "8-bit unsigned integer ", U8, u8,
    "8-bit signed integer", I8, i8,
    "1-Byte array of bytes", ByteArray1, [u8; 1],
    "Boolean value", Bool, bool,
    copy_variants_1B_dep:
    "8-bit [`softposit`](https://crates.io/crates/softposit)'s `Posit` with exp=0",
        P8, softposit::P8, "softposit", "softposit",
    "8-bit Array of bits (implementing [`bv`](https://crates.io/crates/softposit)'s `Bits`)",
        BitArray8, crate::all::BitArray8, "bv", "bv",
    // WIP
    copy_variants_1B_psize:
        "8-bit usize", Usize, usize, target_pointer_width = "8",
        "8-bit isize", Isize, isize, target_pointer_width = "8",
    // noncopy_variants_1B: ,
    // noncopy_variants_1B_dep: ,

    // -------------------------------------------------------- 2-B / 16-b
    copy_variants_2B:
    "16-bit unsigned integer ", U16, u16,
    "16-bit signed integer", I16, i16,
    "2-Byte array of bytes", ByteArray2, [u8; 2],
    copy_variants_2B_dep:
    "16-bit [`half`](https://crates.io/crates/half)'s `binary16` floating-point number",
        F16, half::f16, "half", "half",
    "16-bit [`half`](https://crates.io/crates/half)'s `bfloat16` floating-point number",
        BF16, half::bf16, "half", "half",
    "16-bit [`softposit`](https://crates.io/crates/softposit)'s `Posit` with exp=1",
        P16, softposit::P16, "softposit", "softposit",
    "2-Byte [`arraystring`](https://crates.io/crates/arraystring)'s ArrayString of len()=1",
        ArrayString1, ArrayString<typenum::U1>, "arraystring", "arraystring",
    "16-bit Array of bits (implementing [`bv`](https://crates.io/crates/softposit)'s `Bits`)",
        BitArray16, crate::all::BitArray16, "bv", "bv",
    copy_variants_2B_psize:
        "16-bit usize", Usize, usize, target_pointer_width = "16",
        "16-bit isize", Isize, isize, target_pointer_width = "16",
    // noncopy_variants_2B: ,
    // noncopy_variants_2B_dep: ,

    // -------------------------------------------------------- 4-B / 32-b
    copy_variants_4B:
    "32-bit unsigned integer ", U32, u32,
    "32-bit signed integer", I32, i32,
    "32-bit floating-point number", F32, f32,
    "4-Byte array of bytes", ByteArray4, [u8; 4],
    "4-Byte char ", Char, char,
    copy_variants_4B_dep:
    "32-bit [`softposit`](https://crates.io/crates/softposit)'s `Posit` with exp=2",
        P32, softposit::P32, "softposit", "softposit",
    "4-Byte [`arraystring`](https://crates.io/crates/arraystring)'s ArrayString of len()=3",
        ArrayString3, ArrayString<typenum::U3>, "arraystring", "arraystring",
    "32-bit [`time`](https://crates.io/crates/time)'s `Date`",
        TDate, time::Date, "time", "time",
    "32-bit [`time`](https://crates.io/crates/time)'s `UtcOffset`",
        TUtcOffset, time::UtcOffset, "time", "time",
    "32-bit [`fugit`](https://crates.io/crates/fugit)'s `Duration` in hours",
        FugitDuration32Hours, fugit::Duration<u32, 3_600, 1>, "fugit", "fugit",
    "32-bit [`fugit`](https://crates.io/crates/fugit)'s `Duration` in minutes",
        FugitDuration32Minutes, fugit::Duration<u32, 60, 1>, "fugit", "fugit",
    "32-bit [`fugit`](https://crates.io/crates/fugit)'s `Duration` in seconds",
        FugitDuration32Seconds, fugit::Duration<u32, 1, 1>, "fugit", "fugit",
    "32-bit [`fugit`](https://crates.io/crates/fugit)'s `Duration` in milliseconds",
        FugitDuration32Millis, fugit::Duration<u32, 1, 1_000>, "fugit", "fugit",
    "32-bit [`fugit`](https://crates.io/crates/fugit)'s `Duration` in nanoseconds",
        FugitDuration32Nanos, fugit::Duration<u32, 1, 1_000_000>, "fugit", "fugit",
    "32-bit [`fugit`](https://crates.io/crates/fugit)'s `Instant` in hours",
        FugitInstant32Hours, fugit::Instant<u32, 3_600, 1>, "fugit", "fugit",
    "32-bit [`fugit`](https://crates.io/crates/fugit)'s `Instant` in minutes",
        FugitInstant32Minutes, fugit::Instant<u32, 60, 1>, "fugit", "fugit",
    "32-bit [`fugit`](https://crates.io/crates/fugit)'s `Instant` in seconds",
        FugitInstant32Seconds, fugit::Instant<u32, 1, 1>, "fugit", "fugit",
    "32-bit [`fugit`](https://crates.io/crates/fugit)'s `Instant` in milliseconds",
        FugitInstant32Millis, fugit::Instant<u32, 1, 1_000>, "fugit", "fugit",
    "32-bit [`fugit`](https://crates.io/crates/fugit)'s `Instant` in nanoseconds",
        FugitInstant32Nanos, fugit::Instant<u32, 1, 1_000_000>, "fugit", "fugit",
    "32-bit Array of bits (implementing [`bv`](https://crates.io/crates/softposit)'s `Bits`)",
        BitArray32, crate::all::BitArray32, "bv", "bv",
    copy_variants_4B_psize:
        "32-bit usize", Usize, usize, target_pointer_width = "32",
        "32-bit isize", Isize, isize, target_pointer_width = "32",
    // noncopy_variants_4B: ,
    noncopy_variants_4B_dep:
    "8-bit [`softposit`](https://crates.io/crates/softposit)'s `Quire` with exp=0",
        Q8, softposit::Q8, "softposit", "softposit",

    // ------------------------------------------------------------------------- 8-B / 64-b
    copy_variants_8B:
    "64-bit unsigned integer ", U64, u64,
    "64-bit signed integer", I64, i64,
    "64-bit floating-point number", F64, f64,
    "8-Byte array of bytes", ByteArray8, [u8; 8],
    copy_variants_8B_dep:
    "32-bit [`num_rational`](https://crates.io/crates/num_rational)'s `Ratio` rational number",
        R32, num_rational::Ratio<i32>, "num-rational", "num-rational",
    "8-Byte [`arraystring`](https://crates.io/crates/arraystring)'s ArrayString of len()=7",
        ArrayString7, ArrayString<typenum::U7>, "arraystring", "arraystring",
    "64-bit [`time`](https://crates.io/crates/time)'s `Time`",
        TTime, time::Time, "time", "time",
    "64-bit [`fugit`](https://crates.io/crates/fugit)'s `Duration` in hours",
        FugitDuration64Hours, fugit::Duration<u64, 3_600, 1>, "fugit", "fugit",
    "64-bit [`fugit`](https://crates.io/crates/fugit)'s `Duration` in minutes",
        FugitDuration64Minutes, fugit::Duration<u64, 60, 1>, "fugit", "fugit",
    "64-bit [`fugit`](https://crates.io/crates/fugit)'s `Duration` in seconds",
        FugitDuration64Seconds, fugit::Duration<u64, 1, 1>, "fugit", "fugit",
    "64-bit [`fugit`](https://crates.io/crates/fugit)'s `Duration` in milliseconds",
        FugitDuration64Millis, fugit::Duration<u64, 1, 1_000>, "fugit", "fugit",
    "64-bit [`fugit`](https://crates.io/crates/fugit)'s `Duration` in nanoseconds",
        FugitDuration64Nanos, fugit::Duration<u64, 1, 1_000_000>, "fugit", "fugit",
    "64-bit [`fugit`](https://crates.io/crates/fugit)'s `Instant` in hours",
        FugitInstant64Hours, fugit::Instant<u64, 3_600, 1>, "fugit", "fugit",
    "64-bit [`fugit`](https://crates.io/crates/fugit)'s `Instant` in minutes",
        FugitInstant64Minutes, fugit::Instant<u64, 60, 1>, "fugit", "fugit",
    "64-bit [`fugit`](https://crates.io/crates/fugit)'s `Instant` in seconds",
        FugitInstant64Seconds, fugit::Instant<u64, 1, 1>, "fugit", "fugit",
    "64-bit [`fugit`](https://crates.io/crates/fugit)'s `Instant` in milliseconds",
        FugitInstant64Millis, fugit::Instant<u64, 1, 1_000>, "fugit", "fugit",
    "64-bit [`fugit`](https://crates.io/crates/fugit)'s `Instant` in nanoseconds",
        FugitInstant64Nanos, fugit::Instant<u64, 1, 1_000_000>, "fugit", "fugit",
    "64-bit Array of bits (implementing [`bv`](https://crates.io/crates/softposit)'s `Bits`)",
        BitArray64, crate::all::BitArray64, "bv", "bv",
    copy_variants_8B_psize:
        "64-bit usize", Usize, usize, target_pointer_width = "64",
        "64-bit isize", Isize, isize, target_pointer_width = "64",
    // noncopy_variants_8B: ,
    noncopy_variants_8B_dep:
    "16-bit [`softposit`](https://crates.io/crates/softposit)'s `Quire` with exp=1",
        Q16, softposit::Q16, "softposit", "softposit",
    noncopy_variants_8B_psize_dep:
        "6-Byte fat-pointer String", String, std::string::String, target_pointer_width = "16", "std", "std",

    // ------------------------------------------------------------------------- 16-B /128-b
    copy_variants_16B:
    "128-bit unsigned integer ", U128, u128,
    "128-bit signed integer", I128, i128,
    "16-Byte array of bytes", ByteArray16, [u8; 16],
    "128-bit Duration", Duration, core::time::Duration,
    copy_variants_16B_dep:
    "64-bit [`num_rational`](https://crates.io/crates/num_rational)'s `Ratio` rational number",
        R64, num_rational::Ratio<i64>, "num-rational", "num-rational",
    "16-Byte [rust_decimal] Decimal number",
        Decimal, rust_decimal::Decimal, "rust_decimal", "rust_decimal",
    "16-Byte [`arraystring`](https://crates.io/crates/arraystring)'s ArrayString of len()=15",
        ArrayString15, ArrayString<typenum::U15>, "arraystring", "arraystring",
    "128-bit [`time`](https://crates.io/crates/time)'s `Duration`",
        TDuration, time::Duration, "time", "time",
    "128-bit [`time`](https://crates.io/crates/time)'s `PrimitiveDateTime`",
        TDateTime, time::PrimitiveDateTime, "time", "time",
    "128-bit [`time`](https://crates.io/crates/time)'s `OffsetDateTime`",
        TOffsetDateTime, time::OffsetDateTime, "time", "time",
    "128-bit floating point number",
        F128, twofloat::TwoFloat, "std", "twofloat",
    "128-bit Instant",
        Instant, std::time::Instant, "std", "std",
    "128-bit SystemTime",
        SystemTime, std::time::SystemTime, "std", "std",
    "128-bit [`time`](https://crates.io/crates/time)'s Instant`",
        TInstant, time::Instant, "std", "time",
    "128-bit Array of bits (implementing [`bv`](https://crates.io/crates/softposit)'s `Bits`)",
        BitArray128, crate::all::BitArray128, "bv", "bv",
    copy_variants_16B_psize:
        "128-bit usize", Usize, usize, target_pointer_width = "128",
        "128-bit isize", Isize, isize, target_pointer_width = "128",
    // noncopy_variants_16B: ,
    noncopy_variants_16B_psize_dep:
        "12-Byte fat-pointer String", String, std::string::String, target_pointer_width = "32", "std", "std",

    // ------------------------------------------------------------------------- 32-B / 256-b
    copy_variants_32B:
    "32-Byte array of bytes", ByteArray32, [u8; 32],
    copy_variants_32B_dep:
    "128-bit rational number", R128, num_rational::Ratio<i128>, "num-rational", "num-rational",
    "32-Byte [`arraystring`](https://crates.io/crates/arraystring)'s ArrayString of len()=31",
        ArrayString31, ArrayString<typenum::U31>, "arraystring", "arraystring",
    "256-bit Array of bits (implementing [`bv`](https://crates.io/crates/softposit)'s `Bits`)",
        BitArray256, crate::all::BitArray256, "bv", "bv",
    // noncopy_variants_32B: ,
    noncopy_variants_32B_dep:
    "Big Integer", BigInt, num_bigint::BigInt, "num-bigint", "num-bigint",
    noncopy_variants_32B_psize_dep:
        "24-Byte fat-pointer String", String, std::string::String, target_pointer_width = "64", "std", "std",

    // ------------------------------------------------------------------------- 64 B / 512-b
    copy_variants_64B:
    "64-Byte array of bytes", ByteArray64, [u8; 64],
    copy_variants_64B_dep:
    "64-Byte [`arraystring`](https://crates.io/crates/arraystring)'s ArrayString of len()=63",
        ArrayString63, ArrayString<typenum::U63>, "arraystring", "arraystring",
    "512-bit Array of bits (implementing [`bv`](https://crates.io/crates/softposit)'s `Bits`)",
        BitArray512, crate::all::BitArray512, "bv", "bv",
    // noncopy_variants_64B: ,
    noncopy_variants_64B_dep:
    "32-bit [`softposit`](https://crates.io/crates/softposit)'s `Quire` with exp=2",
        Q32, softposit::Q32, "softposit", "softposit",
    noncopy_variants_64B_psize_dep:
        "48-Byte fat-pointer String", String, std::string::String, target_pointer_width = "128", "std", "std",

    // ------------------------------------------------------------------------- 128-B / 1024-b
    copy_variants_128B:
    "128-Byte array of bytes", ByteArray128, [u8; 128],
    copy_variants_128B_dep:
    "128-Byte [`arraystring`](https://crates.io/crates/arraystring)'s ArrayString of len()=127",
        ArrayString127, ArrayString<typenum::U127>, "arraystring", "arraystring",
    "1024-bit Array of bits (implementing [`bv`](https://crates.io/crates/softposit)'s `Bits`)",
        BitArray1024, crate::all::BitArray1024, "bv", "bv",
    // noncopy_variants_128B: ,
    // noncopy_variants_128B_dep: ,
}
