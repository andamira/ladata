// ladata::unit::macros
//
//! Macros for generating types, and reexporting them
//
// # TOC
//
// - define_all_sizes
// - define_single_size
//
// - define_type
// - define_cell
// - define_bare
//
// - type_aliases
//
// - impl_data_types
// - impl_data_cells
// - impl_data_bares
//
// - reexport

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
/// - the `copy` prefix indicates the types are `Copy`,
///   otherwise the `noncopy` prefix is used.
/// - `nB` indicates the number of bytes of the types in the current group.
/// - the `dep` suffix indicates a dependency on 2 features (dep1 & dep2)
///   (pass the same dependency twice in order to only depend on one).
/// - the `psize` suffix indicates a dependency on a specific pointer size.
///
/// The `define_single_size!` macro is called making sure each size contains
/// all variants with a size less than or equal to the current size.
macro_rules! define_all_sizes {
    (
        $tname:ident, $cname:ident, $bname:ident,

        // 1-Byte / 8-bit
        copy_variants_1B: $( $cvdoc_1B:literal, $cvname_1B:ident, $cvtype_1B:ty ),* ,
        copy_variants_1B_dep: $( $cvdoc_1B_dep:literal, $cvname_1B_dep:ident, $cvtype_1B_dep:ty,
            $cvdep1_1B_dep:literal, $cvdep2_1B_dep:literal  ),* ,
        copy_variants_1B_psize: $( $cvdoc_1B_psize:literal, $cvname_1B_psize:ident,
            $cvtype_1B_psize:ty, $cvpsize_1B_psize:meta ),* ,
        copy_variants_1B_psize_dep: $( $cvdoc_1B_psize_dep:literal, $cvname_1B_psize_dep:ident,
            $cvtype_1B_psize_dep:ty, $cvpsize_dep_1B_psize_dep:meta ),* ,

        noncopy_variants_1B: $( $vdoc_1B:literal, $vname_1B:ident, $vtype_1B:ty ),* ,
        noncopy_variants_1B_dep: $( $vdoc_1B_dep:literal, $vname_1B_dep:ident, $vtype_1B_dep:ty,
            $vdep2_1B_dep:literal ),* ,
        noncopy_variants_1B_psize: $( $vdoc_1B_psize:literal, $vname_1B_psize:ident,
            $vtype_1B_psize:ty, $vpsize_1B_psize:meta ),* ,
        noncopy_variants_1B_psize_dep: $( $vdoc_1B_psize_dep:literal, $vname_1B_psize_dep:ident,
            $vtype_1B_psize_dep:ty, $vpsize_1B_psize_dep:meta, $vdep1_1B_psize_dep:literal,
            $vdep2_1B_psize_dep:literal ),* ,

        // 2-Byte / 16-bit
        copy_variants_2B: $( $cvdoc_2B:literal, $cvname_2B:ident, $cvtype_2B:ty ),* ,
        copy_variants_2B_dep: $( $cvdoc_2B_dep:literal, $cvname_2B_dep:ident, $cvtype_2B_dep:ty,
            $cvdep1_2B_dep:literal, $cvdep2_2B_dep:literal ),* ,
        copy_variants_2B_psize: $( $cvdoc_2B_psize:literal, $cvname_2B_psize:ident,
            $cvtype_2B_psize:ty, $cvpsize_2B_psize:meta ),* ,
        copy_variants_2B_psize_dep: $( $cvdoc_2B_psize_dep:literal, $cvname_2B_psize_dep:ident,
            $cvtype_2B_psize_dep:ty, $cvpsize_dep_2B_psize_dep:meta ),* ,

        noncopy_variants_2B: $( $vdoc_2B:literal, $vname_2B:ident, $vtype_2B:ty ),* ,
        noncopy_variants_2B_dep: $( $vdoc_2B_dep:literal, $vname_2B_dep:ident, $vtype_2B_dep:ty,
            $vdep1_2B_dep:literal ),* ,
        noncopy_variants_2B_psize: $( $vdoc_2B_psize:literal, $vname_2B_psize:ident,
            $vtype_2B_psize:ty, $vpsize_2B_psize:meta ),* ,
        noncopy_variants_2B_psize_dep: $( $vdoc_2B_psize_dep:literal, $vname_2B_psize_dep:ident,
            $vtype_2B_psize_dep:ty, $vpsize_2B_psize_dep:meta, $vdep1_2B_psize_dep:literal,
            $vdep2_2B_psize_dep:literal ),* ,

        // 4-Byte / 32-bit
        copy_variants_4B: $( $cvdoc_4B:literal, $cvname_4B:ident, $cvtype_4B:ty ),* ,
        copy_variants_4B_dep: $( $cvdoc_4B_dep:literal, $cvname_4B_dep:ident, $cvtype_4B_dep:ty,
            $cvdep1_4B_dep:literal, $cvdep2_4B_dep:literal ),* ,
        copy_variants_4B_psize: $( $cvdoc_4B_psize:literal, $cvname_4B_psize:ident,
            $cvtype_4B_psize:ty, $cvpsize_4B_psize:meta ),* ,
        copy_variants_4B_psize_dep: $( $cvdoc_4B_psize_dep:literal, $cvname_4B_psize_dep:ident,
            $cvtype_4B_psize_dep:ty, $cvpsize_dep_4B_psize_dep:meta ),* ,

        noncopy_variants_4B: $( $vdoc_4B:literal, $vname_4B:ident, $vtype_4B:ty ),* ,
        noncopy_variants_4B_dep: $( $vdoc_4B_dep:literal, $vname_4B_dep:ident, $vtype_4B_dep:ty,
            $vdep1_4B_dep:literal, $vdep2_4B_dep:literal ),* ,
        noncopy_variants_4B_psize: $( $vdoc_4B_psize:literal, $vname_4B_psize:ident,
            $vtype_4B_psize:ty, $vpsize_4B_psize:meta ),* ,
        noncopy_variants_4B_psize_dep: $( $vdoc_4B_psize_dep:literal, $vname_4B_psize_dep:ident,
            $vtype_4B_psize_dep:ty, $vpsize_4B_psize_dep:meta, $vdep1_4B_psize_dep:literal,
            $vdep2_4B_psize_dep:literal ),* ,

        // 8-Byte / 64-bit
        copy_variants_8B: $( $cvdoc_8B:literal, $cvname_8B:ident, $cvtype_8B:ty ),* ,
        copy_variants_8B_dep: $( $cvdoc_8B_dep:literal, $cvname_8B_dep:ident, $cvtype_8B_dep:ty,
            $cvdep1_8B_dep:literal, $cvdep2_8B_dep:literal ),* ,
        copy_variants_8B_psize: $( $cvdoc_8B_psize:literal, $cvname_8B_psize:ident,
            $cvtype_8B_psize:ty, $cvpsize_8B_psize:meta ),* ,
        copy_variants_8B_psize_dep: $( $cvdoc_8B_psize_dep:literal, $cvname_8B_psize_dep:ident,
            $cvtype_8B_psize_dep:ty, $cvpsize_dep_8B_psize_dep:meta ),* ,

        noncopy_variants_8B: $( $vdoc_8B:literal, $vname_8B:ident, $vtype_8B:ty ),* ,
        noncopy_variants_8B_dep: $( $vdoc_8B_dep:literal, $vname_8B_dep:ident, $vtype_8B_dep:ty,
            $vdep1_8B_dep:literal, $vdep2_8B_dep:literal ),* ,
        noncopy_variants_8B_psize: $( $vdoc_8B_psize:literal, $vname_8B_psize:ident,
            $vtype_8B_psize:ty, $vpsize_8B_psize:meta ),* , noncopy_variants_8B_psize_dep:
        $( $vdoc_8B_psize_dep:literal, $vname_8B_psize_dep:ident, $vtype_8B_psize_dep:ty,
           $vpsize_8B_psize_dep:meta, $vdep1_8B_psize_dep:literal,
           $vdep2_8B_psize_dep:literal ),* ,

        // 16-Byte / 128-bit
        copy_variants_16B: $( $cvdoc_16B:literal, $cvname_16B:ident, $cvtype_16B:ty ),* ,
        copy_variants_16B_dep: $( $cvdoc_16B_dep:literal, $cvname_16B_dep:ident,
            $cvtype_16B_dep:ty, $cvdep1_16B_dep:literal, $cvdep2_16B_dep:literal ),* ,
        copy_variants_16B_psize: $( $cvdoc_16B_psize:literal, $cvname_16B_psize:ident,
            $cvtype_16B_psize:ty, $cvpsize_16B_psize:meta ),* ,
        copy_variants_16B_psize_dep: $( $cvdoc_16B_psize_dep:literal, $cvname_16B_psize_dep:ident,
            $cvtype_16B_psize_dep:ty, $cvpsize_dep_16B_psize_dep:meta ),* ,

        noncopy_variants_16B: $( $vdoc_16B:literal, $vname_16B:ident, $vtype_16B:ty ),* ,
        noncopy_variants_16B_dep: $( $vdoc_16B_dep:literal, $vname_16B_dep:ident,
            $vtype_16B_dep:ty, $vdep1_16B_dep:literal, $vdep2_16B_dep:literal ),* ,
        noncopy_variants_16B_psize: $( $vdoc_16B_psize:literal, $vname_16B_psize:ident,
            $vtype_16B_psize:ty, $vpsize_16B_psize:meta ),* ,
        noncopy_variants_16B_psize_dep: $( $vdoc_16B_psize_dep:literal, $vname_16B_psize_dep:ident,
            $vtype_16B_psize_dep:ty, $vpsize_16B_psize_dep:meta, $vdep1_16B_psize_dep:literal,
            $vdep2_16B_psize_dep:literal ),* ,

        // 32-Byte / 256-bit
        copy_variants_32B: $( $cvdoc_32B:literal, $cvname_32B:ident, $cvtype_32B:ty ),* ,
        copy_variants_32B_dep: $( $cvdoc_32B_dep:literal, $cvname_32B_dep:ident,
            $cvtype_32B_dep:ty, $cvdep1_32B_dep:literal, $cvdep2_32B_dep:literal ),* ,
        copy_variants_32B_psize: $( $cvdoc_32B_psize:literal, $cvname_32B_psize:ident,
            $cvtype_32B_psize:ty, $cvpsize_32B_psize:meta ),* ,
        copy_variants_32B_psize_dep: $( $cvdoc_32B_psize_dep:literal, $cvname_32B_psize_dep:ident,
            $cvtype_32B_psize_dep:ty, $cvpsize_dep_32B_psize_dep:meta ),* ,

        noncopy_variants_32B: $( $vdoc_32B:literal, $vname_32B:ident, $vtype_32B:ty ),* ,
        noncopy_variants_32B_dep: $( $vdoc_32B_dep:literal, $vname_32B_dep:ident,
            $vtype_32B_dep:ty, $vdep1_32B_dep:literal, $vdep2_32B_dep:literal ),* ,
        noncopy_variants_32B_psize: $( $vdoc_32B_psize:literal, $vname_32B_psize:ident,
            $vtype_32B_psize:ty, $vpsize_32B_psize:meta ),* ,
        noncopy_variants_32B_psize_dep: $( $vdoc_32B_psize_dep:literal, $vname_32B_psize_dep:ident,
            $vtype_32B_psize_dep:ty, $vpsize_32B_psize_dep:meta, $vdep1_32B_psize_dep:literal,
            $vdep2_32B_psize_dep:literal ),* ,

        // 64-Byte / 512-bit
        copy_variants_64B: $( $cvdoc_64B:literal, $cvname_64B:ident, $cvtype_64B:ty ),* ,
        copy_variants_64B_dep: $( $cvdoc_64B_dep:literal, $cvname_64B_dep:ident,
            $cvtype_64B_dep:ty, $cvdep1_64B_dep:literal, $cvdep2_64B_dep:literal ),* ,
        copy_variants_64B_psize: $( $cvdoc_64B_psize:literal, $cvname_64B_psize:ident,
            $cvtype_64B_psize:ty, $cvpsize_64B_psize:meta ),* ,
        copy_variants_64B_psize_dep: $( $cvdoc_64B_psize_dep:literal, $cvname_64B_psize_dep:ident,
            $cvtype_64B_psize_dep:ty, $cvpsize_dep_64B_psize_dep:meta ),* ,

        noncopy_variants_64B: $( $vdoc_64B:literal, $vname_64B:ident, $vtype_64B:ty ),* ,
        noncopy_variants_64B_dep: $( $vdoc_64B_dep:literal, $vname_64B_dep:ident,
            $vtype_64B_dep:ty, $vdep1_64B_dep:literal, $vdep2_64B_dep:literal ),* ,
        noncopy_variants_64B_psize: $( $vdoc_64B_psize:literal, $vname_64B_psize:ident,
            $vtype_64B_psize:ty, $vpsize_64B_psize:meta ),* ,
        noncopy_variants_64B_psize_dep: $( $vdoc_64B_psize_dep:literal, $vname_64B_psize_dep:ident,
            $vtype_64B_psize_dep:ty, $vpsize_64B_psize_dep:meta, $vdep1_64B_psize_dep:literal,
            $vdep2_64B_psize_dep:literal ),* ,

        // 128-Byte / 1024-bit
        copy_variants_128B: $( $cvdoc_128B:literal, $cvname_128B:ident, $cvtype_128B:ty ),* ,
        copy_variants_128B_dep: $( $cvdoc_128B_dep:literal, $cvname_128B_dep:ident,
            $cvtype_128B_dep:ty, $cvdep1_128B_dep:literal, $cvdep2_128B_dep:literal ),* ,
        copy_variants_128B_psize: $( $cvdoc_128B_psize:literal, $cvname_128B_psize:ident,
            $cvtype_128B_psize:ty, $cvpsize_128B_psize:meta ),* ,
        copy_variants_128B_psize_dep: $( $cvdoc_128B_psize_dep:literal,
            $cvname_128B_psize_dep:ident, $cvtype_128B_psize_dep:ty,
            $cvpsize_dep_128B_psize_dep:meta ),* ,

        noncopy_variants_128B: $( $vdoc_128B:literal, $vname_128B:ident, $vtype_128B:ty ),* ,
        noncopy_variants_128B_dep: $( $vdoc_128B_dep:literal, $vname_128B_dep:ident,
            $vtype_128B_dep:ty, $vdep1_128B_dep:literal, $vdep2_128B_dep:literal ),* ,
        noncopy_variants_128B_psize: $( $vdoc_128B_psize:literal, $vname_128B_psize:ident,
            $vtype_128B_psize:ty, $vpsize_128B_psize:meta ),* ,
        noncopy_variants_128B_psize_dep: $( $vdoc_128B_psize_dep:literal,
            $vname_128B_psize_dep:ident, $vtype_128B_psize_dep:ty, $vpsize_128B_psize_dep:meta,
            $vdep1_128B_psize_dep:literal, $vdep2_128B_psize_dep:literal ),* ,

    ) => {
        // 1-Byte / 8-bit
        define_single_size! {
            $tname, $cname, $bname,
            size: 1, 8,
            copy_variants:
                $( $cvdoc_1B, $cvname_1B, $cvtype_1B ),*
                ;
            copy_variants_dep:
                $( $cvdoc_1B_dep, $cvname_1B_dep, $cvtype_1B_dep, $cvdep1_1B_dep,
                    $cvdep2_1B_dep ),*
                ;
            copy_variants_psize:
                $( $cvdoc_1B_psize, $cvname_1B_psize, $cvtype_1B_psize, $cvpsize_1B_psize ),*
                ;
            copy_variants_psize_dep:
                $( $cvdoc_1B_psize_dep, $cvname_1B_psize_dep, $cvtype_1B_psize_dep,
                    $cvpsize_dep_1B_psize_dep ),*
                ;

            noncopy_variants:
                $( $vdoc_1B, $vname_1B, $vtype_1B ),*
                ;
            noncopy_variants_dep:
                $( $vdoc_1B_dep, $vname_1B_dep, $vtype_1B_dep, $vdep1_1B_dep, $vdep2_1B_dep ),*
                ;
            noncopy_variants_psize:
                $( $vdoc_1B_psize, $vname_1B_psize, $vtype_1B_psize,
                    $vpsize_1B_psize, $vdep1_1B_psize, $vdep2_1B_psize ),*
                ;
            noncopy_variants_psize_dep:
                $( $vdoc_1B_psize_dep, $vname_1B_psize_dep, $vtype_1B_psize_dep,
                    $vpsize_1B_psize_dep, $vdep1_1B_psize_dep, $vdep2_1B_psize_dep ),*
                ;
        }

        // 2-Byte / 16-bit
        define_single_size! {
            $tname, $cname, $bname,
            size: 2, 16,
            copy_variants:
                $( $cvdoc_1B, $cvname_1B, $cvtype_1B ),* ,
                $( $cvdoc_2B, $cvname_2B, $cvtype_2B ),*
                ;
            copy_variants_dep:
                $( $cvdoc_1B_dep, $cvname_1B_dep, $cvtype_1B_dep, $cvdep1_1B_dep,
                    $cvdep2_1B_dep ),* ,
                $( $cvdoc_2B_dep, $cvname_2B_dep, $cvtype_2B_dep, $cvdep1_2B_dep,
                    $cvdep2_2B_dep ),*
                ;
            copy_variants_psize:
                $( $cvdoc_1B_psize, $cvname_1B_psize, $cvtype_1B_psize, $cvpsize_1B_psize ),* ,
                $( $cvdoc_2B_psize, $cvname_2B_psize, $cvtype_2B_psize, $cvpsize_2B_psize ),*
                ;
            copy_variants_psize_dep:
                $( $cvdoc_1B_psize_dep, $cvname_1B_psize_dep, $cvtype_1B_psize_dep,
                    $cvpsize_dep_1B_psize_dep ),* ,
                $( $cvdoc_2B_psize_dep, $cvname_2B_psize_dep, $cvtype_2B_psize_dep,
                    $cvpsize_dep_2B_psize_dep ),*
                ;

            noncopy_variants:
                $( $vdoc_1B, $vname_1B, $vtype_1B ),* ,
                $( $vdoc_2B, $vname_2B, $vtype_2B ),*
                ;
            noncopy_variants_dep:
                $( $vdoc_1B_dep, $vname_1B_dep, $vtype_1B_dep, $vdep1_1B_dep, $vdep2_1B_dep ),* ,
                $( $vdoc_2B_dep, $vname_2B_dep, $vtype_2B_dep, $vdep1_2B_dep, $vdep2_2B_dep ),*
                ;
            noncopy_variants_psize:
                $( $vdoc_1B_psize, $vname_1B_psize, $vtype_1B_psize,
                    $vpsize_1B_psize, $vdep1_1B_psize, $vdep2_1B_psize ),* ,
                $( $vdoc_2B_psize, $vname_2B_psize, $vtype_2B_psize,
                    $vpsize_2B_psize, $vdep1_2B_psize, $vdep2_2B_psize ),*
                ;
            noncopy_variants_psize_dep:
                $( $vdoc_1B_psize_dep, $vname_1B_psize_dep, $vtype_1B_psize_dep,
                    $vpsize_1B_psize_dep, $vdep1_1B_psize_dep, $vdep2_1B_psize_dep ),* ,
                $( $vdoc_2B_psize_dep, $vname_2B_psize_dep, $vtype_2B_psize_dep,
                    $vpsize_2B_psize_dep, $vdep1_2B_psize_dep, $vdep2_2B_psize_dep ),*
                ;
        }

        // 4-Byte / 32-bit
        define_single_size! {
            $tname, $cname, $bname,
            size: 4, 32,
            copy_variants:
                $( $cvdoc_1B, $cvname_1B, $cvtype_1B ),* ,
                $( $cvdoc_2B, $cvname_2B, $cvtype_2B ),* ,
                $( $cvdoc_4B, $cvname_4B, $cvtype_4B ),*
                ;
            copy_variants_dep:
                $( $cvdoc_1B_dep, $cvname_1B_dep, $cvtype_1B_dep, $cvdep1_1B_dep,
                    $cvdep2_1B_dep ),* ,
                $( $cvdoc_2B_dep, $cvname_2B_dep, $cvtype_2B_dep, $cvdep1_2B_dep,
                    $cvdep2_2B_dep ),* ,
                $( $cvdoc_4B_dep, $cvname_4B_dep, $cvtype_4B_dep, $cvdep1_4B_dep,
                    $cvdep2_4B_dep ),*
                ;
            copy_variants_psize:
                $( $cvdoc_1B_psize, $cvname_1B_psize, $cvtype_1B_psize, $cvpsize_1B_psize ),* ,
                $( $cvdoc_2B_psize, $cvname_2B_psize, $cvtype_2B_psize, $cvpsize_2B_psize ),* ,
                $( $cvdoc_4B_psize, $cvname_4B_psize, $cvtype_4B_psize, $cvpsize_4B_psize ),*
                ;
            copy_variants_psize_dep:
                $( $cvdoc_1B_psize_dep, $cvname_1B_psize_dep, $cvtype_1B_psize_dep,
                    $cvpsize_dep_1B_psize_dep ),* ,
                $( $cvdoc_2B_psize_dep, $cvname_2B_psize_dep, $cvtype_2B_psize_dep,
                    $cvpsize_dep_2B_psize_dep ),* ,
                $( $cvdoc_4B_psize_dep, $cvname_4B_psize_dep, $cvtype_4B_psize_dep,
                    $cvpsize_dep_4B_psize_dep ),*
                ;
            noncopy_variants:
                $( $vdoc_1B, $vname_1B, $vtype_1B ),* ,
                $( $vdoc_2B, $vname_2B, $vtype_2B ),* ,
                $( $vdoc_4B, $vname_4B, $vtype_4B ),*
                ;
            noncopy_variants_dep:
                $( $vdoc_1B_dep, $vname_1B_dep, $vtype_1B_dep, $vdep1_1B_dep, $vdep2_1B_dep ),* ,
                $( $vdoc_2B_dep, $vname_2B_dep, $vtype_2B_dep, $vdep1_2B_dep, $vdep2_2B_dep ),* ,
                $( $vdoc_4B_dep, $vname_4B_dep, $vtype_4B_dep, $vdep1_4B_dep, $vdep2_4B_dep ),*
                ;
            noncopy_variants_psize:
                $( $vdoc_1B_psize, $vname_1B_psize, $vtype_1B_psize,
                    $vpsize_1B_psize, $vdep1_1B_psize, $vdep2_1B_psize ),* ,
                $( $vdoc_2B_psize, $vname_2B_psize, $vtype_2B_psize,
                    $vpsize_2B_psize, $vdep1_2B_psize, $vdep2_2B_psize ),* ,
                $( $vdoc_4B_psize, $vname_4B_psize, $vtype_4B_psize,
                    $vpsize_4B_psize, $vdep1_4B_psize, $vdep2_4B_psize ),*
                ;
            noncopy_variants_psize_dep:
                $( $vdoc_1B_psize_dep, $vname_1B_psize_dep, $vtype_1B_psize_dep,
                    $vpsize_1B_psize_dep, $vdep1_1B_psize_dep, $vdep2_1B_psize_dep ),* ,
                $( $vdoc_2B_psize_dep, $vname_2B_psize_dep, $vtype_2B_psize_dep,
                    $vpsize_2B_psize_dep, $vdep1_2B_psize_dep, $vdep2_2B_psize_dep ),* ,
                $( $vdoc_4B_psize_dep, $vname_4B_psize_dep, $vtype_4B_psize_dep,
                    $vpsize_4B_psize_dep, $vdep1_4B_psize_dep, $vdep2_4B_psize_dep ),*
                ;
        }

        // 8-Byte / 32-bit
        define_single_size! {
            $tname, $cname, $bname,
            size: 8, 64,
            copy_variants:
                $( $cvdoc_1B, $cvname_1B, $cvtype_1B ),* ,
                $( $cvdoc_2B, $cvname_2B, $cvtype_2B ),* ,
                $( $cvdoc_4B, $cvname_4B, $cvtype_4B ),* ,
                $( $cvdoc_8B, $cvname_8B, $cvtype_8B ),*
                ;
            copy_variants_dep:
                $( $cvdoc_1B_dep, $cvname_1B_dep, $cvtype_1B_dep, $cvdep1_1B_dep,
                    $cvdep2_1B_dep ),* ,
                $( $cvdoc_2B_dep, $cvname_2B_dep, $cvtype_2B_dep, $cvdep1_2B_dep,
                    $cvdep2_2B_dep ),* ,
                $( $cvdoc_4B_dep, $cvname_4B_dep, $cvtype_4B_dep, $cvdep1_4B_dep,
                    $cvdep2_4B_dep ),* ,
                $( $cvdoc_8B_dep, $cvname_8B_dep, $cvtype_8B_dep, $cvdep1_8B_dep,
                    $cvdep2_8B_dep ),*
                ;
            copy_variants_psize:
                $( $cvdoc_1B_psize, $cvname_1B_psize, $cvtype_1B_psize, $cvpsize_1B_psize ),* ,
                $( $cvdoc_2B_psize, $cvname_2B_psize, $cvtype_2B_psize, $cvpsize_2B_psize ),* ,
                $( $cvdoc_4B_psize, $cvname_4B_psize, $cvtype_4B_psize, $cvpsize_4B_psize ),* ,
                $( $cvdoc_8B_psize, $cvname_8B_psize, $cvtype_8B_psize, $cvpsize_8B_psize ),*
                ;
            copy_variants_psize_dep:
                $( $cvdoc_1B_psize_dep, $cvname_1B_psize_dep, $cvtype_1B_psize_dep,
                    $cvpsize_dep_1B_psize_dep ),* ,
                $( $cvdoc_2B_psize_dep, $cvname_2B_psize_dep, $cvtype_2B_psize_dep, ,
                    $cvpsize_dep_2B_psize_dep ),* ,
                $( $cvdoc_4B_psize_dep, $cvname_4B_psize_dep, $cvtype_4B_psize_dep,
                    $cvpsize_dep_4B_psize_dep ),* ,
                $( $cvdoc_8B_psize_dep, $cvname_8B_psize_dep, $cvtype_8B_psize_dep,
                    $cvpsize_dep_8B_psize_dep ),*
                ;
            noncopy_variants:
                $( $vdoc_1B, $vname_1B, $vtype_1B ),* ,
                $( $vdoc_2B, $vname_2B, $vtype_2B ),* ,
                $( $vdoc_4B, $vname_4B, $vtype_4B ),* ,
                $( $vdoc_8B, $vname_8B, $vtype_8B ),*
                ;
            noncopy_variants_dep:
                $( $vdoc_1B_dep, $vname_1B_dep, $vtype_1B_dep, $vdep1_1B_dep, $vdep2_1B_dep ),* ,
                $( $vdoc_2B_dep, $vname_2B_dep, $vtype_2B_dep, $vdep1_2B_dep, $vdep2_2B_dep ),* ,
                $( $vdoc_4B_dep, $vname_4B_dep, $vtype_4B_dep, $vdep1_4B_dep, $vdep2_4B_dep ),* ,
                $( $vdoc_8B_dep, $vname_8B_dep, $vtype_8B_dep, $vdep1_8B_dep, $vdep2_8B_dep ),*
                ;
            noncopy_variants_psize:
                $( $vdoc_1B_psize, $vname_1B_psize, $vtype_1B_psize,
                    $vpsize_1B_psize, $vdep1_1B_psize, $vdep2_1B_psize ),* ,
                $( $vdoc_2B_psize, $vname_2B_psize, $vtype_2B_psize,
                    $vpsize_2B_psize, $vdep1_2B_psize, $vdep2_2B_psize ),* ,
                $( $vdoc_4B_psize, $vname_4B_psize, $vtype_4B_psize,
                    $vpsize_4B_psize, $vdep1_4B_psize, $vdep2_4B_psize ),* ,
                $( $vdoc_8B_psize, $vname_8B_psize, $vtype_8B_psize,
                    $vpsize_8B_psize, $vdep1_8B_psize, $vdep2_8B_psize ),*
                ;
            noncopy_variants_psize_dep:
                // NOTE:missing-commas
                $( $vdoc_1B_psize_dep, $vname_1B_psize_dep, $vtype_1B_psize_dep,
                    $vpsize_1B_psize_dep, $vdep1_1B_psize_dep, $vdep2_1B_psize_dep ),*
                $( $vdoc_2B_psize_dep, $vname_2B_psize_dep, $vtype_2B_psize_dep,
                    $vpsize_2B_psize_dep, $vdep1_2B_psize_dep, $vdep2_2B_psize_dep ),*
                $( $vdoc_4B_psize_dep, $vname_4B_psize_dep, $vtype_4B_psize_dep,
                    $vpsize_4B_psize_dep, $vdep1_4B_psize_dep, $vdep2_4B_psize_dep ),*
                $( $vdoc_8B_psize_dep, $vname_8B_psize_dep, $vtype_8B_psize_dep,
                    $vpsize_8B_psize_dep, $vdep1_8B_psize_dep, $vdep2_8B_psize_dep ),*
                ;
        }

        // 16-Byte / 64-bit
        define_single_size! {
            $tname, $cname, $bname,
            size: 16, 128,
            copy_variants:
                $( $cvdoc_1B, $cvname_1B, $cvtype_1B ),* ,
                $( $cvdoc_2B, $cvname_2B, $cvtype_2B ),* ,
                $( $cvdoc_4B, $cvname_4B, $cvtype_4B ),* ,
                $( $cvdoc_8B, $cvname_8B, $cvtype_8B ),* ,
                $( $cvdoc_16B, $cvname_16B, $cvtype_16B ),*
                ;
            copy_variants_dep:
                $( $cvdoc_1B_dep, $cvname_1B_dep, $cvtype_1B_dep, $cvdep1_1B_dep,
                    $cvdep2_1B_dep ),* ,
                $( $cvdoc_2B_dep, $cvname_2B_dep, $cvtype_2B_dep, $cvdep1_2B_dep,
                    $cvdep2_2B_dep ),* ,
                $( $cvdoc_4B_dep, $cvname_4B_dep, $cvtype_4B_dep, $cvdep1_4B_dep,
                    $cvdep2_4B_dep ),* ,
                $( $cvdoc_8B_dep, $cvname_8B_dep, $cvtype_8B_dep, $cvdep1_8B_dep,
                    $cvdep2_8B_dep ),* ,
                $( $cvdoc_16B_dep, $cvname_16B_dep, $cvtype_16B_dep, $cvdep1_16B_dep,
                    $cvdep2_16B_dep ),*
                ;
            copy_variants_psize:
                $( $cvdoc_1B_psize, $cvname_1B_psize, $cvtype_1B_psize, $cvpsize_1B_psize ),* ,
                $( $cvdoc_2B_psize, $cvname_2B_psize, $cvtype_2B_psize, $cvpsize_2B_psize ),* ,
                $( $cvdoc_4B_psize, $cvname_4B_psize, $cvtype_4B_psize, $cvpsize_4B_psize ),* ,
                $( $cvdoc_8B_psize, $cvname_8B_psize, $cvtype_8B_psize, $cvpsize_8B_psize ),* ,
                $( $cvdoc_16B_psize, $cvname_16B_psize, $cvtype_16B_psize, $cvpsize_16B_psize ),*
                ;
            copy_variants_psize_dep:
                $( $cvdoc_1B_psize_dep, $cvname_1B_psize_dep, $cvtype_1B_psize_dep,
                    $cvpsize_dep_1B_psize_dep ),* ,
                $( $cvdoc_2B_psize_dep, $cvname_2B_psize_dep, $cvtype_2B_psize_dep,
                    $cvpsize_dep_2B_psize_dep ),* ,
                $( $cvdoc_4B_psize_dep, $cvname_4B_psize_dep, $cvtype_4B_psize_dep,
                    $cvpsize_dep_4B_psize_dep ),* ,
                $( $cvdoc_8B_psize_dep, $cvname_8B_psize_dep, $cvtype_8B_psize_dep,
                    $cvpsize_dep_8B_psize_dep ),* ,
                $( $cvdoc_16B_psize_dep, $cvname_16B_psize_dep, $cvtype_16B_psize_dep,
                    $cvpsize_dep_16B_psize_dep ),*
                ;
            noncopy_variants:
                $( $vdoc_1B, $vname_1B, $vtype_1B ),* ,
                $( $vdoc_2B, $vname_2B, $vtype_2B ),* ,
                $( $vdoc_4B, $vname_4B, $vtype_4B ),* ,
                $( $vdoc_8B, $vname_8B, $vtype_8B ),* ,
                $( $vdoc_16B, $vname_16B, $vtype_16B ),*
                ;
            noncopy_variants_dep:
                $( $vdoc_1B_dep, $vname_1B_dep, $vtype_1B_dep, $vdep1_1B_dep, $vdep2_1B_dep ),* ,
                $( $vdoc_2B_dep, $vname_2B_dep, $vtype_2B_dep, $vdep1_2B_dep, $vdep2_2B_dep ),* ,
                $( $vdoc_4B_dep, $vname_4B_dep, $vtype_4B_dep, $vdep1_4B_dep, $vdep2_4B_dep ),* ,
                $( $vdoc_8B_dep, $vname_8B_dep, $vtype_8B_dep, $vdep1_8B_dep, $vdep2_8B_dep ),* ,
                $( $vdoc_16B_dep, $vname_16B_dep, $vtype_16B_dep, $vdep1_16B_dep, $vdep2_16B_dep ),*
                ;
            noncopy_variants_psize:
                $( $vdoc_1B_psize, $vname_1B_psize, $vtype_1B_psize,
                    $vpsize_1B_psize, $vdep1_1B_psize, $vdep2_1B_psize ),* ,
                $( $vdoc_2B_psize, $vname_2B_psize, $vtype_2B_psize,
                    $vpsize_2B_psize, $vdep1_2B_psize, $vdep2_2B_psize ),* ,
                $( $vdoc_4B_psize, $vname_4B_psize, $vtype_4B_psize,
                    $vpsize_4B_psize, $vdep1_4B_psize, $vdep2_4B_psize ),* ,
                $( $vdoc_8B_psize, $vname_8B_psize, $vtype_8B_psize,
                    $vpsize_8B_psize, $vdep1_8B_psize, $vdep2_8B_psize ),* ,
                $( $vdoc_16B_psize, $vname_16B_psize, $vtype_16B_psize,
                    $vpsize_16B_psize, $vdep1_16B_psize, $vdep2_16B_psize ),*
                ;
            noncopy_variants_psize_dep:
                // NOTE: missing-commas
                $( $vdoc_1B_psize_dep, $vname_1B_psize_dep, $vtype_1B_psize_dep,
                    $vpsize_1B_psize_dep, $vdep1_1B_psize_dep, $vdep2_1B_psize_dep ),*
                $( $vdoc_2B_psize_dep, $vname_2B_psize_dep, $vtype_2B_psize_dep,
                    $vpsize_2B_psize_dep, $vdep1_2B_psize_dep, $vdep2_2B_psize_dep ),*
                $( $vdoc_4B_psize_dep, $vname_4B_psize_dep, $vtype_4B_psize_dep,
                    $vpsize_4B_psize_dep, $vdep1_4B_psize_dep, $vdep2_4B_psize_dep ),*
                $( $vdoc_8B_psize_dep, $vname_8B_psize_dep, $vtype_8B_psize_dep,
                    $vpsize_8B_psize_dep, $vdep1_8B_psize_dep, $vdep2_8B_psize_dep ),* ,
                $( $vdoc_16B_psize_dep, $vname_16B_psize_dep, $vtype_16B_psize_dep,
                    $vpsize_16B_psize_dep, $vdep1_16B_psize_dep, $vdep2_16B_psize_dep ),*
                ;
        }

        // 32-Byte / 128-bit
        define_single_size! {
            $tname, $cname, $bname,
            size: 32, 256,
            copy_variants:
                $( $cvdoc_1B, $cvname_1B, $cvtype_1B ),* ,
                $( $cvdoc_2B, $cvname_2B, $cvtype_2B ),* ,
                $( $cvdoc_4B, $cvname_4B, $cvtype_4B ),* ,
                $( $cvdoc_8B, $cvname_8B, $cvtype_8B ),* ,
                $( $cvdoc_16B, $cvname_16B, $cvtype_16B ),* ,
                $( $cvdoc_32B, $cvname_32B, $cvtype_32B ),*
                ;
            copy_variants_dep:
                $( $cvdoc_1B_dep, $cvname_1B_dep, $cvtype_1B_dep, $cvdep1_1B_dep,
                    $cvdep2_1B_dep ),* ,
                $( $cvdoc_2B_dep, $cvname_2B_dep, $cvtype_2B_dep, $cvdep1_2B_dep,
                    $cvdep2_2B_dep ),* ,
                $( $cvdoc_4B_dep, $cvname_4B_dep, $cvtype_4B_dep, $cvdep1_4B_dep,
                    $cvdep2_4B_dep ),* ,
                $( $cvdoc_8B_dep, $cvname_8B_dep, $cvtype_8B_dep, $cvdep1_8B_dep,
                    $cvdep2_8B_dep ),* ,
                $( $cvdoc_16B_dep, $cvname_16B_dep, $cvtype_16B_dep, $cvdep1_16B_dep,
                    $cvdep2_16B_dep ),* ,
                $( $cvdoc_32B_dep, $cvname_32B_dep, $cvtype_32B_dep, $cvdep1_32B_dep,
                    $cvdep2_32B_dep ),*
                ;
            copy_variants_psize:
                $( $cvdoc_1B_psize, $cvname_1B_psize, $cvtype_1B_psize, $cvpsize_1B_psize ),* ,
                $( $cvdoc_2B_psize, $cvname_2B_psize, $cvtype_2B_psize, $cvpsize_2B_psize ),* ,
                $( $cvdoc_4B_psize, $cvname_4B_psize, $cvtype_4B_psize, $cvpsize_4B_psize ),* ,
                $( $cvdoc_8B_psize, $cvname_8B_psize, $cvtype_8B_psize, $cvpsize_8B_psize ),* ,
                $( $cvdoc_16B_psize, $cvname_16B_psize, $cvtype_16B_psize, $cvpsize_16B_psize ),* ,
                $( $cvdoc_32B_psize, $cvname_32B_psize, $cvtype_32B_psize, $cvpsize_32B_psize ),*
                ;
            copy_variants_psize_dep:
                $( $cvdoc_1B_psize_dep, $cvname_1B_psize_dep, $cvtype_1B_psize_dep,
                    $cvpsize_dep_1B_psize_dep ),* ,
                $( $cvdoc_2B_psize_dep, $cvname_2B_psize_dep, $cvtype_2B_psize_dep,
                    $cvpsize_dep_2B_psize_dep ),* ,
                $( $cvdoc_4B_psize_dep, $cvname_4B_psize_dep, $cvtype_4B_psize_dep,
                    $cvpsize_dep_4B_psize_dep ),* ,
                $( $cvdoc_8B_psize_dep, $cvname_8B_psize_dep, $cvtype_8B_psize_dep,
                    $cvpsize_dep_8B_psize_dep ),* ,
                $( $cvdoc_16B_psize_dep, $cvname_16B_psize_dep, $cvtype_16B_psize_dep,
                    $cvpsize_dep_16B_psize_dep ),* ,
                $( $cvdoc_32B_psize_dep, $cvname_32B_psize_dep, $cvtype_32B_psize_dep,
                    $cvpsize_dep_32B_psize_dep ),*
                ;
            noncopy_variants:
                $( $vdoc_1B, $vname_1B, $vtype_1B ),* ,
                $( $vdoc_2B, $vname_2B, $vtype_2B ),* ,
                $( $vdoc_4B, $vname_4B, $vtype_4B ),* ,
                $( $vdoc_8B, $vname_8B, $vtype_8B ),* ,
                $( $vdoc_16B, $vname_16B, $vtype_16B ),* ,
                $( $vdoc_32B, $vname_32B, $vtype_32B ),*
                ;
            noncopy_variants_dep:
                // NOTE:missing-commas
                $( $vdoc_1B_dep, $vname_1B_dep, $vtype_1B_dep, $vdep1_1B_dep, $vdep2_1B_dep ),*
                $( $vdoc_2B_dep, $vname_2B_dep, $vtype_2B_dep, $vdep1_2B_dep, $vdep2_2B_dep ),*
                $( $vdoc_4B_dep, $vname_4B_dep, $vtype_4B_dep, $vdep1_4B_dep, $vdep2_4B_dep ),*
                $( $vdoc_8B_dep, $vname_8B_dep, $vtype_8B_dep, $vdep1_8B_dep, $vdep2_8B_dep ),*
                $( $vdoc_16B_dep, $vname_16B_dep, $vtype_16B_dep, $vdep1_16B_dep, $vdep2_16B_dep ),*
                $( $vdoc_32B_dep, $vname_32B_dep, $vtype_32B_dep, $vdep1_32B_dep, $vdep2_32B_dep ),*
                ;
            noncopy_variants_psize:
                $( $vdoc_1B_psize, $vname_1B_psize, $vtype_1B_psize,
                    $vpsize_1B_psize, $vdep1_1B_psize, $vdep2_1B_psize ),* ,
                $( $vdoc_2B_psize, $vname_2B_psize, $vtype_2B_psize,
                    $vpsize_2B_psize, $vdep1_2B_psize, $vdep2_2B_psize ),* ,
                $( $vdoc_4B_psize, $vname_4B_psize, $vtype_4B_psize,
                    $vpsize_4B_psize, $vdep1_4B_psize, $vdep2_4B_psize ),* ,
                $( $vdoc_8B_psize, $vname_8B_psize, $vtype_8B_psize,
                    $vpsize_8B_psize, $vdep1_8B_psize, $vdep2_8B_psize ),* ,
                $( $vdoc_16B_psize, $vname_16B_psize, $vtype_16B_psize,
                    $vpsize_16B_psize, $vdep1_16B_psize, $vdep2_16B_psize ),* ,
                $( $vdoc_32B_psize, $vname_32B_psize, $vtype_32B_psize,
                    $vpsize_32B_psize, $vdep1_32B_psize, $vdep2_32B_psize ),*
                ;
            noncopy_variants_psize_dep:
                // NOTE:missing-commas
                $( $vdoc_1B_psize_dep, $vname_1B_psize_dep, $vtype_1B_psize_dep,
                    $vpsize_1B_psize_dep, $vdep1_1B_psize_dep, $vdep2_1B_psize_dep ),*
                $( $vdoc_2B_psize_dep, $vname_2B_psize_dep, $vtype_2B_psize_dep,
                    $vpsize_2B_psize_dep, $vdep1_2B_psize_dep, $vdep2_2B_psize_dep ),*
                $( $vdoc_4B_psize_dep, $vname_4B_psize_dep, $vtype_4B_psize_dep,
                    $vpsize_4B_psize_dep, $vdep1_4B_psize_dep, $vdep2_4B_psize_dep ),*
                $( $vdoc_8B_psize_dep, $vname_8B_psize_dep, $vtype_8B_psize_dep,
                    $vpsize_8B_psize_dep, $vdep1_8B_psize_dep, $vdep2_8B_psize_dep ),* ,
                $( $vdoc_16B_psize_dep, $vname_16B_psize_dep, $vtype_16B_psize_dep,
                    $vpsize_16B_psize_dep, $vdep1_16B_psize_dep, $vdep2_16B_psize_dep ),* ,
                $( $vdoc_32B_psize_dep, $vname_32B_psize_dep, $vtype_32B_psize_dep,
                    $vpsize_32B_psize_dep, $vdep1_32B_psize_dep, $vdep2_32B_psize_dep ),*
                ;
        }

        define_single_size! {
            $tname, $cname, $bname,
            size: 64, 512, // 64-Byte / 512-bit
            copy_variants:
                $( $cvdoc_1B, $cvname_1B, $cvtype_1B ),* ,
                $( $cvdoc_2B, $cvname_2B, $cvtype_2B ),* ,
                $( $cvdoc_4B, $cvname_4B, $cvtype_4B ),* ,
                $( $cvdoc_8B, $cvname_8B, $cvtype_8B ),* ,
                $( $cvdoc_16B, $cvname_16B, $cvtype_16B ),* ,
                $( $cvdoc_32B, $cvname_32B, $cvtype_32B ),* ,
                $( $cvdoc_64B, $cvname_64B, $cvtype_64B ),*
                ;
            copy_variants_dep:
                $( $cvdoc_1B_dep, $cvname_1B_dep, $cvtype_1B_dep, $cvdep1_1B_dep,
                    $cvdep2_1B_dep ),* ,
                $( $cvdoc_2B_dep, $cvname_2B_dep, $cvtype_2B_dep, $cvdep1_2B_dep,
                    $cvdep2_2B_dep ),* ,
                $( $cvdoc_4B_dep, $cvname_4B_dep, $cvtype_4B_dep, $cvdep1_4B_dep,
                    $cvdep2_4B_dep ),* ,
                $( $cvdoc_8B_dep, $cvname_8B_dep, $cvtype_8B_dep, $cvdep1_8B_dep,
                    $cvdep2_8B_dep ),* ,
                $( $cvdoc_16B_dep, $cvname_16B_dep, $cvtype_16B_dep, $cvdep1_16B_dep,
                    $cvdep2_16B_dep ),* ,
                $( $cvdoc_32B_dep, $cvname_32B_dep, $cvtype_32B_dep, $cvdep1_32B_dep,
                    $cvdep2_32B_dep ),* ,
                $( $cvdoc_64B_dep, $cvname_64B_dep, $cvtype_64B_dep, $cvdep1_64B_dep,
                    $cvdep2_64B_dep ),*
                ;
            copy_variants_psize:
                $( $cvdoc_1B_psize, $cvname_1B_psize, $cvtype_1B_psize, $cvpsize_1B_psize ),* ,
                $( $cvdoc_2B_psize, $cvname_2B_psize, $cvtype_2B_psize, $cvpsize_2B_psize ),* ,
                $( $cvdoc_4B_psize, $cvname_4B_psize, $cvtype_4B_psize, $cvpsize_4B_psize ),* ,
                $( $cvdoc_8B_psize, $cvname_8B_psize, $cvtype_8B_psize, $cvpsize_8B_psize ),* ,
                $( $cvdoc_16B_psize, $cvname_16B_psize, $cvtype_16B_psize, $cvpsize_16B_psize ),* ,
                $( $cvdoc_32B_psize, $cvname_32B_psize, $cvtype_32B_psize, $cvpsize_32B_psize ),* ,
                $( $cvdoc_64B_psize, $cvname_64B_psize, $cvtype_64B_psize, $cvpsize_64B_psize ),*
                ;
            copy_variants_psize_dep:
                $( $cvdoc_1B_psize_dep, $cvname_1B_psize_dep, $cvtype_1B_psize_dep,
                    $cvpsize_dep_1B_psize_dep ),* ,
                $( $cvdoc_2B_psize_dep, $cvname_2B_psize_dep, $cvtype_2B_psize_dep,
                    $cvpsize_dep_2B_psize_dep ),* ,
                $( $cvdoc_4B_psize_dep, $cvname_4B_psize_dep, $cvtype_4B_psize_dep,
                    $cvpsize_dep_4B_psize_dep ),* ,
                $( $cvdoc_8B_psize_dep, $cvname_8B_psize_dep, $cvtype_8B_psize_dep,
                    $cvpsize_dep_8B_psize_dep ),* ,
                $( $cvdoc_16B_psize_dep, $cvname_16B_psize_dep, $cvtype_16B_psize_dep,
                    $cvpsize_dep_16B_psize_dep ),* ,
                $( $cvdoc_32B_psize_dep, $cvname_32B_psize_dep, $cvtype_32B_psize_dep,
                    $cvpsize_dep_32B_psize_dep ),* ,
                $( $cvdoc_64B_psize_dep, $cvname_64B_psize_dep, $cvtype_64B_psize_dep,
                    $cvpsize_dep_64B_psize_dep ),*
                ;
            noncopy_variants:
                $( $vdoc_1B, $vname_1B, $vtype_1B ),* ,
                $( $vdoc_2B, $vname_2B, $vtype_2B ),* ,
                $( $vdoc_4B, $vname_4B, $vtype_4B ),* ,
                $( $vdoc_8B, $vname_8B, $vtype_8B ),* ,
                $( $vdoc_16B, $vname_16B, $vtype_16B ),* ,
                $( $vdoc_32B, $vname_32B, $vtype_32B ),* ,
                $( $vdoc_64B, $vname_64B, $vtype_64B ),*
                ;
            noncopy_variants_dep:
                // NOTE:missing-commas
                $( $vdoc_1B_dep, $vname_1B_dep, $vtype_1B_dep, $vdep1_1B_dep, $vdep2_1B_dep ),*
                $( $vdoc_2B_dep, $vname_2B_dep, $vtype_2B_dep, $vdep1_2B_dep, $vdep2_2B_dep ),*
                $( $vdoc_4B_dep, $vname_4B_dep, $vtype_4B_dep, $vdep1_4B_dep, $vdep2_4B_dep ),*
                $( $vdoc_8B_dep, $vname_8B_dep, $vtype_8B_dep, $vdep1_8B_dep, $vdep2_8B_dep ),*
                $( $vdoc_16B_dep, $vname_16B_dep, $vtype_16B_dep, $vdep1_16B_dep,
                    $vdep2_16B_dep ),*
                $( $vdoc_32B_dep, $vname_32B_dep, $vtype_32B_dep, $vdep1_32B_dep,
                    $vdep2_32B_dep ),* ,
                $( $vdoc_64B_dep, $vname_64B_dep, $vtype_64B_dep, $vdep1_64B_dep,
                    $vdep2_64B_dep ),*
                ;
            noncopy_variants_psize:
                $( $vdoc_1B_psize, $vname_1B_psize, $vtype_1B_psize,
                    $vpsize_1B_psize, $vdep1_1B_psize, $vdep2_1B_psize ),* ,
                $( $vdoc_2B_psize, $vname_2B_psize, $vtype_2B_psize,
                    $vpsize_2B_psize, $vdep1_2B_psize, $vdep2_2B_psize ),* ,
                $( $vdoc_4B_psize, $vname_4B_psize, $vtype_4B_psize,
                    $vpsize_4B_psize, $vdep1_4B_psize, $vdep2_4B_psize ),* ,
                $( $vdoc_8B_psize, $vname_8B_psize, $vtype_8B_psize,
                    $vpsize_8B_psize, $vdep1_8B_psize, $vdep2_8B_psize ),* ,
                $( $vdoc_16B_psize, $vname_16B_psize, $vtype_16B_psize,
                    $vpsize_16B_psize, $vdep1_16B_psize, $vdep2_16B_psize ),* ,
                $( $vdoc_32B_psize, $vname_32B_psize, $vtype_32B_psize,
                    $vpsize_32B_psize, $vdep1_32B_psize, $vdep2_32B_psize ),* ,
                $( $vdoc_64B_psize, $vname_64B_psize, $vtype_64B_psize,
                    $vpsize_64B_psize, $vdep1_64B_psize, $vdep2_64B_psize ),* ,
                ;
            noncopy_variants_psize_dep:
                // NOTE:missing-commas
                $( $vdoc_1B_psize_dep, $vname_1B_psize_dep, $vtype_1B_psize_dep,
                    $vpsize_1B_psize_dep, $vdep1_1B_psize_dep, $vdep2_1B_psize_dep ),*
                $( $vdoc_2B_psize_dep, $vname_2B_psize_dep, $vtype_2B_psize_dep,
                    $vpsize_2B_psize_dep, $vdep1_2B_psize_dep, $vdep2_2B_psize_dep ),*
                $( $vdoc_4B_psize_dep, $vname_4B_psize_dep, $vtype_4B_psize_dep,
                    $vpsize_4B_psize_dep, $vdep1_4B_psize_dep, $vdep2_4B_psize_dep ),*
                $( $vdoc_8B_psize_dep, $vname_8B_psize_dep, $vtype_8B_psize_dep,
                    $vpsize_8B_psize_dep, $vdep1_8B_psize_dep, $vdep2_8B_psize_dep ),* ,
                $( $vdoc_16B_psize_dep, $vname_16B_psize_dep, $vtype_16B_psize_dep,
                    $vpsize_16B_psize_dep, $vdep1_16B_psize_dep, $vdep2_16B_psize_dep ),* ,
                $( $vdoc_32B_psize_dep, $vname_32B_psize_dep, $vtype_32B_psize_dep,
                    $vpsize_32B_psize_dep, $vdep1_32B_psize_dep, $vdep2_32B_psize_dep ),* ,
                $( $vdoc_64B_psize_dep, $vname_64B_psize_dep, $vtype_64B_psize_dep,
                    $vpsize_64B_psize_dep, $vdep1_64B_psize_dep, $vdep2_64B_psize_dep ),*
                ;
        }

        define_single_size! {
            $tname, $cname, $bname,
            size: 128, 1024, // 128-Byte / 1024-bit
            copy_variants:
                $( $cvdoc_1B, $cvname_1B, $cvtype_1B ),* ,
                $( $cvdoc_2B, $cvname_2B, $cvtype_2B ),* ,
                $( $cvdoc_4B, $cvname_4B, $cvtype_4B ),* ,
                $( $cvdoc_8B, $cvname_8B, $cvtype_8B ),* ,
                $( $cvdoc_16B, $cvname_16B, $cvtype_16B ),* ,
                $( $cvdoc_32B, $cvname_32B, $cvtype_32B ),* ,
                $( $cvdoc_64B, $cvname_64B, $cvtype_64B ),* ,
                $( $cvdoc_128B, $cvname_128B, $cvtype_128B ),*
                ;
            copy_variants_dep:
                $( $cvdoc_1B_dep, $cvname_1B_dep, $cvtype_1B_dep, $cvdep1_1B_dep,
                    $cvdep2_1B_dep ),* ,
                $( $cvdoc_2B_dep, $cvname_2B_dep, $cvtype_2B_dep, $cvdep1_2B_dep,
                    $cvdep2_2B_dep ),* ,
                $( $cvdoc_4B_dep, $cvname_4B_dep, $cvtype_4B_dep, $cvdep1_4B_dep,
                    $cvdep2_4B_dep ),* ,
                $( $cvdoc_8B_dep, $cvname_8B_dep, $cvtype_8B_dep, $cvdep1_8B_dep,
                    $cvdep2_8B_dep ),* ,
                $( $cvdoc_16B_dep, $cvname_16B_dep, $cvtype_16B_dep, $cvdep1_16B_dep,
                    $cvdep2_16B_dep ),* ,
                $( $cvdoc_32B_dep, $cvname_32B_dep, $cvtype_32B_dep, $cvdep1_32B_dep,
                    $cvdep2_32B_dep ),* ,
                $( $cvdoc_64B_dep, $cvname_64B_dep, $cvtype_64B_dep, $cvdep1_64B_dep,
                    $cvdep2_64B_dep ),* ,
                $( $cvdoc_128B_dep, $cvname_128B_dep, $cvtype_128B_dep, $cvdep1_128B_dep,
                    $cvdep2_128B_dep ),*
                ;
            copy_variants_psize:
                $( $cvdoc_1B_psize, $cvname_1B_psize, $cvtype_1B_psize, $cvpsize_1B_psize ),* ,
                $( $cvdoc_2B_psize, $cvname_2B_psize, $cvtype_2B_psize, $cvpsize_2B_psize ),* ,
                $( $cvdoc_4B_psize, $cvname_4B_psize, $cvtype_4B_psize, $cvpsize_4B_psize ),* ,
                $( $cvdoc_8B_psize, $cvname_8B_psize, $cvtype_8B_psize, $cvpsize_8B_psize ),* ,
                $( $cvdoc_16B_psize, $cvname_16B_psize, $cvtype_16B_psize, $cvpsize_16B_psize ),* ,
                $( $cvdoc_32B_psize, $cvname_32B_psize, $cvtype_32B_psize, $cvpsize_32B_psize ),* ,
                $( $cvdoc_64B_psize, $cvname_64B_psize, $cvtype_64B_psize, $cvpsize_64B_psize ),* ,
                $( $cvdoc_128B_psize, $cvname_128B_psize, $cvtype_128B_psize,
                    $cvpsize_128B_psize ),*
                ;
            copy_variants_psize_dep:
                $( $cvdoc_1B_psize_dep, $cvname_1B_psize_dep, $cvtype_1B_psize_dep,
                    $cvpsize_dep_1B_psize_dep ),* ,
                $( $cvdoc_2B_psize_dep, $cvname_2B_psize_dep, $cvtype_2B_psize_dep,
                    $cvpsize_dep_2B_psize_dep ),* ,
                $( $cvdoc_4B_psize_dep, $cvname_4B_psize_dep, $cvtype_4B_psize_dep,
                    $cvpsize_dep_4B_psize_dep ),* ,
                $( $cvdoc_8B_psize_dep, $cvname_8B_psize_dep, $cvtype_8B_psize_dep,
                    $cvpsize_dep_8B_psize_dep ),* ,
                $( $cvdoc_16B_psize_dep, $cvname_16B_psize_dep, $cvtype_16B_psize_dep,
                    $cvpsize_dep_16B_psize_dep ),* ,
                $( $cvdoc_32B_psize_dep, $cvname_32B_psize_dep, $cvtype_32B_psize_dep,
                    $cvpsize_dep_32B_psize_dep ),* ,
                $( $cvdoc_64B_psize_dep, $cvname_64B_psize_dep, $cvtype_64B_psize_dep,
                    $cvpsize_dep_64B_psize_dep ),* ,
                $( $cvdoc_128B_psize_dep, $cvname_128B_psize_dep, $cvtype_128B_psize_dep,
                    $cvpsize_dep_128B_psize_dep ),*
                ;
            noncopy_variants:
                $( $vdoc_1B, $vname_1B, $vtype_1B ),* ,
                $( $vdoc_2B, $vname_2B, $vtype_2B ),* ,
                $( $vdoc_4B, $vname_4B, $vtype_4B ),* ,
                $( $vdoc_8B, $vname_8B, $vtype_8B ),* ,
                $( $vdoc_16B, $vname_16B, $vtype_16B ),* ,
                $( $vdoc_32B, $vname_32B, $vtype_32B ),* ,
                $( $vdoc_64B, $vname_64B, $vtype_64B ),* ,
                $( $vdoc_128B, $vname_128B, $vtype_128B ),*
                ;
            noncopy_variants_dep:
                // NOTE:missing-commas
                $( $vdoc_1B_dep, $vname_1B_dep, $vtype_1B_dep, $vdep1_1B_dep, $vdep2_1B_dep ),*
                $( $vdoc_2B_dep, $vname_2B_dep, $vtype_2B_dep, $vdep1_2B_dep, $vdep2_2B_dep ),*
                $( $vdoc_4B_dep, $vname_4B_dep, $vtype_4B_dep, $vdep1_4B_dep, $vdep2_4B_dep ),*
                $( $vdoc_8B_dep, $vname_8B_dep, $vtype_8B_dep, $vdep1_8B_dep, $vdep2_8B_dep ),*
                $( $vdoc_16B_dep, $vname_16B_dep, $vtype_16B_dep, $vdep1_16B_dep,
                    $vdep2_16B_dep ),*
                $( $vdoc_32B_dep, $vname_32B_dep, $vtype_32B_dep, $vdep1_32B_dep,
                    $vdep2_32B_dep ),* ,
                $( $vdoc_64B_dep, $vname_64B_dep, $vtype_64B_dep, $vdep1_64B_dep,
                    $vdep2_64B_dep ),* ,
                $( $vdoc_128B_dep, $vname_128B_dep, $vtype_128B_dep, $vdep1_128B_dep,
                    $vdep2_128B_dep ),*
                ;
            noncopy_variants_psize:
                $( $vdoc_1B_psize, $vname_1B_psize, $vtype_1B_psize,
                    $vpsize_1B_psize, $vdep1_1B_psize, $vdep2_1B_psize ),* ,
                $( $vdoc_2B_psize, $vname_2B_psize, $vtype_2B_psize,
                    $vpsize_2B_psize, $vdep1_2B_psize, $vdep2_2B_psize ),* ,
                $( $vdoc_4B_psize, $vname_4B_psize, $vtype_4B_psize,
                    $vpsize_4B_psize, $vdep1_4B_psize, $vdep2_4B_psize ),* ,
                $( $vdoc_8B_psize, $vname_8B_psize, $vtype_8B_psize,
                    $vpsize_8B_psize, $vdep1_8B_psize, $vdep2_8B_psize ),* ,
                $( $vdoc_16B_psize, $vname_16B_psize, $vtype_16B_psize,
                    $vpsize_16B_psize, $vdep1_16B_psize, $vdep2_16B_psize ),* ,
                $( $vdoc_32B_psize, $vname_32B_psize, $vtype_32B_psize,
                    $vpsize_32B_psize, $vdep1_32B_psize, $vdep2_32B_psize ),* ,
                $( $vdoc_64B_psize, $vname_64B_psize, $vtype_64B_psize,
                    $vpsize_64B_psize, $vdep1_64B_psize, $vdep2_64B_psize ),* ,
                $( $vdoc_128B_psize, $vname_128B_psize, $vtype_128B_psize,
                    $vpsize_128B_psize, $vdep1_128B_psize, $vdep2_128B_psize ),*
                ;
            noncopy_variants_psize_dep:
                // NOTE:missing-commas
                $( $vdoc_1B_psize_dep, $vname_1B_psize_dep, $vtype_1B_psize_dep,
                    $vpsize_1B_psize_dep, $vdep1_1B_psize_dep, $vdep2_1B_psize_dep ),*
                $( $vdoc_2B_psize_dep, $vname_2B_psize_dep, $vtype_2B_psize_dep,
                    $vpsize_2B_psize_dep, $vdep1_2B_psize_dep, $vdep2_2B_psize_dep ),*
                $( $vdoc_4B_psize_dep, $vname_4B_psize_dep, $vtype_4B_psize_dep,
                    $vpsize_4B_psize_dep, $vdep1_4B_psize_dep, $vdep2_4B_psize_dep ),*
                $( $vdoc_8B_psize_dep, $vname_8B_psize_dep, $vtype_8B_psize_dep,
                    $vpsize_8B_psize_dep, $vdep1_8B_psize_dep, $vdep2_8B_psize_dep ),* ,
                $( $vdoc_16B_psize_dep, $vname_16B_psize_dep, $vtype_16B_psize_dep,
                    $vpsize_16B_psize_dep, $vdep1_16B_psize_dep, $vdep2_16B_psize_dep ),* ,
                $( $vdoc_32B_psize_dep, $vname_32B_psize_dep, $vtype_32B_psize_dep,
                    $vpsize_32B_psize_dep, $vdep1_32B_psize_dep, $vdep2_32B_psize_dep ),* ,
                $( $vdoc_64B_psize_dep, $vname_64B_psize_dep, $vtype_64B_psize_dep,
                    $vpsize_64B_psize_dep, $vdep1_64B_psize_dep, $vdep2_64B_psize_dep ),*
                $( $vdoc_128B_psize_dep, $vname_128B_psize_dep, $vtype_128B_psize_dep,
                    $vpsize_128B_psize_dep, $vdep1_128B_psize_dep, $vdep2_128B_psize_dep ),*
                ;
        }
    };
}
pub(crate) use define_all_sizes;

/// for defining in one pass: DataType*, DataCell*, DataBare*
macro_rules! define_single_size {
    (
        $tname:ident, $cname:ident, $bname:ident,
        size: $B:literal, $b:literal,
        copy_variants:
            $( $cvdoc:literal, $cvname:ident, $cvtype:ty ),*
            $(,)* ;
        copy_variants_dep:
            $( $cvdoc_dep:literal, $cvname_dep:ident, $cvtype_dep:ty,
            $cvdep1_dep:literal, $cvdep2_dep:literal ),*
            $(,)* ;
        copy_variants_psize:
            $( $cvdoc_psize:literal, $cvname_psize:ident, $cvtype_psize:ty,
            $cvpsize_psize:meta ),*
            $(,)* ;
        copy_variants_psize_dep:
            $( $cvdoc_psize_dep:literal, $cvname_psize_dep:ident, $cvtype_psize_dep:ty,
            $cvpsize_psize_dep:meta, $cvdep1_psize_dep:literal, $cvdep2_psize_dep:literal ),*
            $(,)* ;
        noncopy_variants:
            $( $vdoc:literal, $vname:ident, $vtype:ty ),*
            $(,)* ;
        noncopy_variants_dep:
            $( $vdoc_dep:literal, $vname_dep:ident, $vtype_dep:ty,
            $vdep1_dep:literal, $vdep2_dep:literal ),*
            $(,)* ;
        noncopy_variants_psize:
            $( $vdoc_psize:literal, $vname_psize:ident, $vtype_psize:ty,
            $vpsize_psize:meta, $vdep1_psize:literal, $vdep2_psize:literal ),*
            $(,)* ;
        noncopy_variants_psize_dep:
            $( $vdoc_psize_dep:literal, $vname_psize_dep:ident, $vtype_psize_dep:ty,
            $vpsize_psize_dep:meta, $vdep1_psize_dep:literal, $vdep2_psize_dep:literal ),*
            $(,)* ;
    ) => {
        define_type!{
            $tname, size: $B, $b,
            copy_variants:
                $( $cvdoc, $cvname, $cvtype ),* ;
            copy_variants_dep:
                $( $cvdoc_dep, $cvname_dep, $cvtype_dep, $cvdep1_dep, $cvdep2_dep ),* ;
            copy_variants_psize:
                $( $cvdoc_psize, $cvname_psize, $cvtype_psize, $cvpsize_psize ),* ;
            copy_variants_psize_dep:
                $( $cvdoc_psize_dep, $cvname_psize_dep, $cvtype_psize_dep, $cvpsize_psize_dep,
                $cvdep1_psize_dep, $cvdep2_psize_dep ),* ;
            noncopy_variants:
                $( $vdoc, $vname, $vtype ),* ;
            noncopy_variants_dep:
                $( $vdoc_dep, $vname_dep, $vtype_dep, $vdep1_dep, $vdep2_dep ),* ;
            noncopy_variants_psize:
                $( $vdoc_psize, $vname_psize, $vtype_psize, $vpsize_psize,
                $vdep1_psize, $vdep2_psize ),* ;
            noncopy_variants_psize_dep:
                $( $vdoc_psize_dep, $vname_psize_dep, $vtype_psize_dep, $vpsize_psize_dep,
                $vdep1_psize_dep, $vdep2_psize_dep ),* ;
        }
        define_cell!{
            c: $cname, t:$tname, b:$bname, size: $B, $b,
            copy_variants:
                $( $cvdoc, $cvname, $cvtype ),* ;
            copy_variants_dep:
                $( $cvdoc_dep, $cvname_dep, $cvtype_dep, $cvdep1_dep, $cvdep2_dep ),* ;
            copy_variants_psize:
                $( $cvdoc_psize, $cvname_psize, $cvtype_psize, $cvpsize_psize ),* ;
            copy_variants_psize_dep:
                $( $cvdoc_psize_dep, $cvname_psize_dep, $cvtype_psize_dep, $cvpsize_psize_dep,
                $cvdep1_psize_dep, $cvdep2_psize_dep ),* ;
            noncopy_variants:
                $( $vdoc, $vname, $vtype ),* ;
            noncopy_variants_dep:
                $( $vdoc_dep, $vname_dep, $vtype_dep, $vdep1_dep, $vdep2_dep ),* ;
            noncopy_variants_psize:
                $( $vdoc_psize, $vname_psize, $vtype_psize, $vpsize_psize,
                $vdep1_psize, $vdep2_psize ),* ;
            noncopy_variants_psize_dep:
                $( $vdoc_psize_dep, $vname_psize_dep, $vtype_psize_dep, $vpsize_psize_dep,
                $vdep1_psize_dep, $vdep2_psize_dep ),* ;
        }
        define_bare!{
            $bname, size: $B, $b,
            copy_variants:
                $( $cvdoc, $cvname, $cvtype ),* ;
            copy_variants_dep:
                $( $cvdoc_dep, $cvname_dep, $cvtype_dep, $cvdep1_dep, $cvdep2_dep ),* ;
            copy_variants_psize:
                $( $cvdoc_psize, $cvname_psize, $cvtype_psize, $cvpsize_psize ),* ;
            copy_variants_psize_dep:
                $( $cvdoc_psize_dep, $cvname_psize_dep, $cvtype_psize_dep, $cvpsize_psize_dep,
                $cvdep1_psize_dep, $cvdep2_psize_dep ),* ;
            noncopy_variants:
                $( $vdoc, $vname, $vtype ),* ;
            noncopy_variants_dep:
                $( $vdoc_dep, $vname_dep, $vtype_dep, $vdep1_dep, $vdep2_dep ),* ;
            noncopy_variants_psize:
                $( $vdoc_psize, $vname_psize, $vtype_psize, $vpsize_psize,
                $vdep1_psize, $vdep2_psize ),* ;
            noncopy_variants_psize_dep:
                $( $vdoc_psize_dep, $vname_psize_dep, $vtype_psize_dep, $vpsize_psize_dep,
                $vdep1_psize_dep, $vdep2_psize_dep ),* ;
        }
    };
}
pub(crate) use define_single_size;

/// for defining enum DataType*
macro_rules! define_type {
    (
        $tname:ident,
        size: $B:literal, $b:literal,
        copy_variants:
            $( $cvdoc:literal, $cvname:ident, $cvtype:ty ),*
            $(,)* ;
        copy_variants_dep:
            $( $cvdoc_dep:literal, $cvname_dep:ident, $cvtype_dep:ty,
            $cvdep1_dep:literal, $cvdep2_dep:literal ),*
            $(,)* ;
        copy_variants_psize:
            $( $cvdoc_psize:literal, $cvname_psize:ident, $cvtype_psize:ty,
            $cvpsize_psize:meta ),*
            $(,)* ;
        copy_variants_psize_dep:
            $( $cvdoc_psize_dep:literal, $cvname_psize_dep:ident, $cvtype_psize_dep:ty,
            $cvpsize_psize_dep:meta, $cvdep1_psize_dep:literal, $cvdep2_psize_dep:literal ),*
            $(,)* ;
        noncopy_variants:
            $( $vdoc:literal, $vname:ident, $vtype:ty ),*
            $(,)* ;
        noncopy_variants_dep:
            $( $vdoc_dep:literal, $vname_dep:ident, $vtype_dep:ty,
            $vdep1_dep:literal, $vdep2_dep:literal ),*
            $(,)* ;
        noncopy_variants_psize:
            $( $vdoc_psize:literal, $vname_psize:ident, $vtype_psize:ty,
            $vpsize_psize:meta, $vdep1_psize:literal, $vdep2_psize:literal ),*
            $(,)* ;
        noncopy_variants_psize_dep:
            $( $vdoc_psize_dep:literal, $vname_psize_dep:ident, $vtype_psize_dep:ty,
            $vpsize_psize_dep:meta, $vdep1_psize_dep:literal, $vdep2_psize_dep:literal ),*
            $(,)* ;
    ) =>  {
        paste::paste!{
            // ## copy version (DataType)
            // -----------------------------------------------------------------
            #[doc = $b "-bit / " $B "-Byte " "data **Type** (extendable) (Copy)" ]
            ///
            /// See also:
            #[doc = "- [" [<$tname $b bit With>]  "][" [<$tname $b bit With>] "] -Copy" ]
            #[doc = "- [" [<$tname $b bit Copy>]  "][" [<$tname $b bit Copy>] "] -With" ]
            #[doc = "- [" [<$tname $b bit>]  "][" [<$tname $b bit>] "] -Copy -With" ]
            #[derive(Clone, Copy, Debug)]
            pub enum [< $tname $b bit Copy With >]<T: DataTypesCopy> {
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

            // DataType Copy (-With) alias
            #[doc = $b "-bit/" $B "-Byte " "data **Type** (Copy)"]
            ///
            /// See also:
            #[doc = "- [" [<$tname $b bit>] "][" [<$tname $b bit>] "] -Copy" ]
            #[doc = "- [" [<$tname $b bit Copy With>] "][" [<$tname $b bit Copy With>] "] +With" ]
            #[doc = "- [" [<$tname $b bit With>] "][" [<$tname $b bit With>] "] -Copy +With" ]
            pub type [<$tname $b bit Copy>] = [< $tname $b bit Copy With>]<()>;

            impl_data_types![ [< $tname $b bit Copy With >], DataTypesCopy,
                is_copy: true,
                copy_variants:
                    $( $cvname, $cvtype ),* ;
                copy_variants_dep:
                    $( $cvname_dep, $cvtype_dep, $cvdep1_dep, $cvdep2_dep ),* ;
                copy_variants_psize:
                    $( $cvname_psize, $cvtype_psize, $cvpsize_psize ),* ;
                copy_variants_psize_dep:
                    $( $cvname_psize_dep, $cvtype_psize_dep, $cvpsize_psize_dep,
                    $cvdep1_psize_dep, $cvdep2_psize_dep ),* ;
                noncopy_variants: ;
                noncopy_variants_dep: ;
                noncopy_variants_psize_dep: ;
            ];
            impl<T: DataTypesCopy> DataTypesCopy for [< $tname $b bit Copy With >]<T> { }

            // ## non-copy version (DataType)
            // -----------------------------------------------------------------
            #[doc = $b "-bit / " $B "-Byte " "data **Type** (extendable)"]
            ///
            /// See also:
            #[doc = "- [" [<$tname $b bit Copy With>] "][" [<$tname $b bit Copy With>] "] +Copy" ]
            #[doc = "- [" [<$tname $b bit>] "][" [<$tname $b bit>] "] -With" ]
            #[doc = "- [" [<$tname $b bit Copy>] "][" [<$tname $b bit Copy>] "] +Copy -With" ]
            #[derive(Clone, Copy, Debug)]
            pub enum [< $tname $b bit With >]<T: DataTypes> {
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
                    #[cfg(all($cvpsize_psize_dep,
                            feature = $cvdep1_psize_dep,
                            feature = $cvdep2_psize_dep))]
                    #[doc = $cvdoc_psize_dep]
                    $cvname_psize_dep,
                )*
                $(
                    #[cfg(all($vpsize_psize_dep,
                            feature = $vdep1_psize_dep,
                            feature = $vdep2_psize_dep))]
                    #[doc = $vdoc_psize_dep]
                    $vname_psize_dep,
                )*

            }

            // DataType (-With) alias
            #[doc = $b "-bit/" $B "-Byte " "data **Type**"]
            ///
            /// See also:
            #[doc = "- [" [<$tname $b bit Copy>] "][" [<$tname $b bit Copy>] "] +Copy" ]
            #[doc = "- [" [<$tname $b bit With>] "][" [<$tname $b bit With>] "] +With" ]
            #[doc = "- [" [<$tname $b bit Copy With>] "][" [<$tname $b bit Copy With>] "] +Copy +With" ]
            pub type [<$tname $b bit>] = [< $tname $b bit With>]<()>;

            impl_data_types![ [< $tname $b bit With >], DataTypes,
                is_copy: false,
                copy_variants:
                    $( $cvname, $cvtype ),* ;
                copy_variants_dep:
                    $( $cvname_dep, $cvtype_dep, $cvdep1_dep, $cvdep2_dep ),* ;
                copy_variants_psize:
                    $( $cvname_psize, $cvtype_psize, $cvpsize_psize ),* ;
                copy_variants_psize_dep:
                    $( $cvname_psize_dep, $cvtype_psize_dep, $cvpsize_psize_dep,
                    $cvdep1_psize_dep, $cvdep2_psize_dep ),* ;
                noncopy_variants:
                    $($vname, $vtype ),* ;
                noncopy_variants_dep:
                    $( $vname_dep, $vtype_dep, $vdep1_dep, $vdep2_dep ),* ;
                noncopy_variants_psize_dep:
                    $( $vname_psize_dep, $vtype_psize_dep, $vpsize_psize_dep,
                    $vdep1_psize_dep, $vdep2_psize_dep ),* ;
            ];
        }
    };
}
pub(crate) use define_type;

/// for defining enum DataCell*
macro_rules! define_cell {
    (
        c: $cname:ident, t: $tname:ident, b: $bname:ident,
        size: $B:literal, $b:literal,

        copy_variants:
            $( $cvdoc:literal, $cvname:ident, $cvtype:ty ),*
            $(,)* ;
        copy_variants_dep:
            $( $cvdoc_dep:literal, $cvname_dep:ident, $cvtype_dep:ty,
            $cvdep1_dep:literal, $cvdep2_dep:literal ),*
            $(,)* ;
        copy_variants_psize:
            $( $cvdoc_psize:literal, $cvname_psize:ident, $cvtype_psize:ty,
            $cvpsize_psize:meta ),*
            $(,)* ;
        copy_variants_psize_dep:
            $( $cvdoc_psize_dep:literal, $cvname_psize_dep:ident, $cvtype_psize_dep:ty,
            $cvpsize_psize_dep:meta, $cvdep1_psize_dep:literal, $cvdep2_psize_dep:literal ),*
            $(,)* ;
        noncopy_variants:
            $( $vdoc:literal, $vname:ident, $vtype:ty ),*
            $(,)* ;
        noncopy_variants_dep:
            $( $vdoc_dep:literal, $vname_dep:ident, $vtype_dep:ty,
            $vdep1_dep:literal, $vdep2_dep:literal ),*
            $(,)* ;
        noncopy_variants_psize:
            $( $vdoc_psize:literal, $vname_psize:ident, $vtype_psize:ty,
            $vpsize_psize:meta, $vdep1_psize:literal, $vdep2_psize:literal ),*
            $(,)* ;
        noncopy_variants_psize_dep:
            $( $vdoc_psize_dep:literal, $vname_psize_dep:ident, $vtype_psize_dep:ty,
            $vpsize_psize_dep:meta, $vdep1_psize_dep:literal, $vdep2_psize_dep:literal ),*
            $(,)* ;
    ) => {
        paste::paste!{
            // ## copy version (DataCell)
            // -----------------------------------------------------------------
            #[doc = $b "-bit/" $B "-Byte " "data **Cell** (extendable) (Copy)"]
            ///
            /// See also:
            #[doc = "- [" [<$cname $b bit With>] "][" [<$cname $b bit With>] "] -Copy" ]
            #[doc = "- [" [<$cname $b bit Copy>] "][" [<$cname $b bit Copy>] "] -With" ]
            #[doc = "- [" [<$cname $b bit>] "][" [<$cname $b bit>] "] -Copy -With" ]
            #[derive(Clone, Copy, Debug)]
            // pub enum [<$cname $b bit Copy With>]<C: DataCellsCopy, T: DataTypesCopy> { // MAYBE
            pub enum [<$cname $b bit Copy With>]<C: DataCellsCopy> {
                /// Represents the absence of *data*.
                None,
                /// A custom *data cell* extension.
                With(C),
                // _data_type(core::marker::PhantomData<*const T>), // MAYBE

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
                    #[cfg(all($cvpsize_psize_dep,
                            feature = $cvdep1_psize_dep,
                            feature = $cvdep2_psize_dep))]
                    #[doc = $cvdoc_psize_dep]
                    $cvname_psize_dep($cvtype_psize_dep),
                )*
            }

            // alias DataCell Copy
            #[doc = $b "-bit/" $B "-Byte " "data **Cell** (Copy)"]
            ///
            /// See also:
            #[doc = "- [" [<$cname $b bit>] "][" [<$cname $b bit>] "] -Copy" ]
            #[doc = "- [" [<$cname $b bit Copy With>] "][" [<$cname $b bit Copy With>] "] +With" ]
            #[doc = "- [" [<$cname $b bit With>] "][" [<$cname $b bit With>] "] -Copy +With" ]
            pub type [<$cname $b bit Copy>] = [< $cname $b bit With>]<()>;

            impl_data_cells![
                c: [< $cname $b bit Copy With >], DataCellsCopy,
                t: [< $tname $b bit Copy With >], DataTypesCopy,
                is_copy: true,
                copy_variants:
                    $( $cvname, $cvtype ),* ;
                copy_variants_dep:
                    $( $cvname_dep, $cvtype_dep, $cvdep1_dep, $cvdep2_dep ),* ;
                copy_variants_psize:
                    $( $cvname_psize, $cvtype_psize, $cvpsize_psize ),* ;
                copy_variants_psize_dep:
                    $( $cvname_psize_dep, $cvtype_psize_dep, $cvpsize_psize_dep,
                    $cvdep1_psize_dep, $cvdep2_psize_dep ),* ;
                noncopy_variants: ;
                noncopy_variants_dep: ;
                noncopy_variants_psize_dep: ;
            ];
            // MAYBE
            // impl<C: DataCellsCopy, T: DataTypesCopy> DataCellsCopy for [< $cname $b bit Copy With >]<C, T> { }
            impl<C: DataCellsCopy> DataCellsCopy for [< $cname $b bit Copy With >]<C> { }

            // ## non-copy version (DataCell)
            // -----------------------------------------------------------------
            #[doc = $b "-bit/" $B "-Byte " "data **Cell** (extendable)"]
            ///
            /// See also:
            #[doc = "- [" [<$cname $b bit Copy With>] "][" [<$cname $b bit Copy With>] "] +Copy" ]
            #[doc = "- [" [<$cname $b bit>] "][" [<$cname $b bit>] "] -Width" ]
            #[doc = "- [" [<$cname $b bit Copy>] "][" [<$cname $b bit Copy>] "] +Copy -Width" ]
            #[derive(Debug)]
            // pub enum [<$cname $b bit With>]<C: DataCells, T: DataTypes> { // MAYBE
            pub enum [<$cname $b bit With>]<C: DataCells> {
                /// Represents the absence of *data*.
                None,
                /// A custom *data cell* extension.
                With(C),
                // _data_type(core::marker::PhantomData<*const T>), // MAYBE

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
                    #[cfg(all($cvpsize_psize_dep,
                            feature = $cvdep1_psize_dep,
                            feature = $cvdep2_psize_dep))]
                    #[doc = $cvdoc_psize_dep]
                    $cvname_psize_dep($cvtype_psize_dep),
                )*
                $(
                    #[cfg(all($vpsize_psize_dep,
                            feature = $vdep1_psize_dep,
                            feature = $vdep2_psize_dep))]
                    #[doc = $vdoc_psize_dep]
                    $vname_psize_dep($vtype_psize_dep),
                )*
            }

            // alias DataCell
            #[doc = $b "-bit/" $B "-Byte " "data **Cell**"]
            ///
            /// See also:
            #[doc = "- [" [<$cname $b bit Copy>] "][" [<$cname $b bit Copy>] "] +Copy" ]
            #[doc = "- [" [<$cname $b bit With>] "][" [<$cname $b bit With>] "] +With" ]
            #[doc = "- [" [<$cname $b bit Copy With>] "][" [<$cname $b bit Copy With>] "] +Copy +With" ]
            pub type [<$cname $b bit>] = [< $cname $b bit With>]<()>;

            // implement the DataCells trait
            impl_data_cells![
                c: [< $cname $b bit With >], DataCells,
                t: [< $tname $b bit With >], DataTypes,
                is_copy: false,
                copy_variants:
                    $( $cvname, $cvtype ),* ;
                copy_variants_dep:
                    $( $cvname_dep, $cvtype_dep, $cvdep1_dep, $cvdep2_dep ),* ;
                copy_variants_psize:
                    $( $cvname_psize, $cvtype_psize, $cvpsize_psize ),* ;
                copy_variants_psize_dep:
                    $( $cvname_psize_dep, $cvtype_psize_dep, $cvpsize_psize_dep,
                    $cvdep1_psize_dep, $cvdep2_psize_dep ),* ;
                noncopy_variants:
                    $($vname, $vtype ),* ;
                noncopy_variants_dep:
                    $( $vname_dep, $vtype_dep, $vdep1_dep, $vdep2_dep ),* ;
                noncopy_variants_psize_dep:
                    $( $vname_psize_dep, $vtype_psize_dep, $vpsize_psize_dep,
                    $vdep1_psize_dep, $vdep2_psize_dep ),* ;
            ];

            // implement `TryFrom`<`DataCell`> for *contained-value*:

            $( // Copy
                impl<C: DataCellsCopy> TryFrom<[<$cname $b bit Copy With>]<C>> for $cvtype {
                    type Error = ();
                    fn try_from(c: [<$cname $b bit Copy With>]<C>) -> Result<Self, Self::Error> {
                        match c {
                            [<$cname $b bit Copy With>]::$cvname(c) => Ok(c),
                            _ => Err(()),
                        }
                    }

                }
            )*
            $( // Copy feature-bound
                #[cfg(all(feature = $cvdep1_dep, feature = $cvdep2_dep ))]
                impl<C: DataCells> TryFrom<[<$cname $b bit With>]<C>> for $cvtype_dep {
                    type Error = ();
                    fn try_from(c: [<$cname $b bit With>]<C>) -> Result<Self, Self::Error> {
                        match c {
                            [<$cname $b bit With>]::$cvname_dep(c) => Ok(c),
                            _ => Err(()),
                        }
                    }

                }
            )*
            $( // non-Copy
                impl<C: DataCells> TryFrom<[<$cname $b bit With>]<C>> for $vtype {
                    type Error = ();
                    fn try_from(c: [<$cname $b bit With>]<C>) -> Result<Self, Self::Error> {
                        match c {
                            [<$cname $b bit With>]::$vname(c) => Ok(c),
                            _ => Err(()),
                        }
                    }

                }
            )*
            $( // non-Copy feature-bound
                #[cfg(all(feature = $vdep1_dep, feature = $vdep2_dep ))]
                impl<C: DataCells> TryFrom<[<$cname $b bit With>]<C>> for $vtype_dep {
                    type Error = ();
                    fn try_from(c: [<$cname $b bit With>]<C>) -> Result<Self, Self::Error> {
                        match c {
                            [<$cname $b bit With>]::$vname_dep(c) => Ok(c),
                            _ => Err(()),
                        }
                    }

                }
            )*

            // implement `From`<*contained-value*> for `DataCell`:

            $( // Copy
                impl<C: DataCellsCopy> From<$cvtype> for [<$cname $b bit Copy With>]<C> {
                    fn from(v: $cvtype) -> Self {
                        [<$cname $b bit Copy With>]::$cvname(v)
                    }

                }
            )*
            $( // Copy feature-bound
                #[cfg(all(feature = $cvdep1_dep, feature = $cvdep2_dep ))]
                impl<C: DataCellsCopy> From<$cvtype_dep> for [<$cname $b bit Copy With>]<C> {
                    fn from(v: $cvtype_dep) -> Self {
                        [<$cname $b bit Copy With>]::$cvname_dep(v)
                    }

                }
            )*
            $( // non-Copy
                impl<C: DataCells> From<$vtype> for [<$cname $b bit With>]<C> {
                    fn from(v: $vtype) -> Self {
                        [<$cname $b bit With>]::$vname(v)
                    }

                }
            )*
            $( // non-Copy feature-bound
                #[cfg(all(feature = $vdep1_dep, feature = $vdep2_dep ))]
                impl<C: DataCells> From<$vtype_dep> for [<$cname $b bit With>]<C> {
                    fn from(v: $vtype_dep) -> Self {
                        [<$cname $b bit With>]::$vname_dep(v)
                    }

                }
            )*

            // from DataCell to DataBare
            impl<C: DataCellsCopy> From<[<$cname $b bit Copy With>]<C>> for [<$bname $b bit Copy>] {
                fn from(cell: [<$cname $b bit Copy With>]<C>) -> Self {
                    match cell {
                        [<$cname $b bit Copy With>]::None => Self { None: () },
                        [<$cname $b bit Copy With>]::With(_) => Self { None: () },

                        $( // fundamental types
                            [<$cname $b bit Copy With>]::$cvname(v) => Self { $cvname: v },
                        )*

                        $( // pointer-size dependant
                            #[cfg($cvpsize_psize)]
                            [<$cname $b bit Copy With>]::$cvname_psize(c) => Self { $cvname_psize: c },
                        )*

                        $( // feature-gated dependencies
                            #[cfg(all(feature = $cvdep1_dep, feature = $cvdep2_dep))]
                            [<$cname $b bit Copy With>]::$cvname_dep(v) => Self { $cvname_dep: v },
                        )*
                    }
                }

            }
        }
    };
}
pub(crate) use define_cell;

/// for defining union DataBare*
macro_rules! define_bare {
    (
        $bname:ident,
        size: $B:literal, $b:literal,

        copy_variants:
            $( $cvdoc:literal, $cvname:ident, $cvtype:ty ),*
            $(,)* ;
        copy_variants_dep:
            $( $cvdoc_dep:literal, $cvname_dep:ident, $cvtype_dep:ty,
            $cvdep1_dep:literal, $cvdep2_dep:literal ),*
            $(,)* ;
        copy_variants_psize:
            $( $cvdoc_psize:literal, $cvname_psize:ident, $cvtype_psize:ty,
            $cvpsize_psize:meta ),*
            $(,)* ;
        copy_variants_psize_dep:
            $( $cvname_psize_dep:ident, $cvtype_psize_dep:ty,
            $cvpsize_psize_dep:meta, $cvdep1_psize_dep:literal, $cvdep2_psize_dep:literal ),*
            $(,)* ;
        noncopy_variants:
            $( $vdoc:literal, $vname:ident, $vtype:ty ),*
            $(,)* ;
        noncopy_variants_dep:
            $( $vdoc_dep:literal, $vname_dep:ident, $vtype_dep:ty,
            $vdep1_dep:literal, $vdep2_dep:literal ),*
            $(,)* ;
        noncopy_variants_psize:
            $( $vdoc_psize:literal, $vname_psize:ident, $vtype_psize:ty,
            $vpsize_psize:meta, $vdep1_psize:literal, $vdep2_psize:literal ),*
            $(,)* ;
        noncopy_variants_psize_dep:
            $( $vdoc_psize_dep:literal, $vname_psize_dep:ident, $vtype_psize_dep:ty,
            $vpsize_psize_dep:meta, $vdep1_psize_dep:literal, $vdep2_psize_dep:literal ),*
            $(,)* ;

    ) => {
        paste::paste!{
            // ## copy version (DataBare)
            // -----------------------------------------------------------------
            #[repr(C)]
            #[doc = $B "Byte / " $b "bit untyped *bare* data (Copy)"]
            #[derive(Copy, Clone)]
            pub union [<$bname $b bit Copy>] {
                /// Represents the absence of *data*.
                pub None: (),

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
            // TODO
            // #[doc = $B "Byte / " $b "bit *untyped (bare)* data"]
            // pub type [< $bname $b bit Copy >] = [< $bname $b bit Copy >];

            // Debug
            impl core::fmt::Debug for [<$bname $b bit Copy>] {
                fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                    write!(f, "{} {{...}}", stringify!{[< $bname $b bit Copy >]})
                }
            }

            impl_data_bares![
                b: [< $bname $b bit Copy >],
            ];
        }
    };
}
pub(crate) use define_bare;

// -----------------------------------------------------------------------------

/// implement: DataTypes trait
macro_rules! impl_data_types {
    (
        $tname:ident, $tbound:ident,
        is_copy: $is_copy:stmt,
        copy_variants:
            $( $cvname:ident, $cvtype:ty ),* ;
        copy_variants_dep:
            $( $cvname_dep:ident, $cvtype_dep:ty,
            $cvdep1_dep:literal, $cvdep2_dep:literal ),* ;
        copy_variants_psize:
            $( $cvname_psize:ident, $cvtype_psize:ty, $cvpsize_psize:meta ),* ;
        copy_variants_psize_dep:
            $( $cvname_psize_dep:ident, $cvtype_psize_dep:ty,
            $cvpsize_psize_dep:meta, $cvdep1_psize_dep:literal, $cvdep2_psize_dep:literal ),* ;
        noncopy_variants:
            $( $vname:ident, $vtype:ty ),* ;
        noncopy_variants_dep:
            $( $vname_dep:ident, $vtype_dep:ty,
            $vdep1_dep:literal, $vdep2_dep:literal ),* ;
        noncopy_variants_psize_dep:
            $( $vname_psize_dep:ident, $vtype_psize_dep:ty,
            $vpsize_psize_dep:meta, $vdep1_psize_dep:literal, $vdep2_psize_dep:literal ),* ;
    ) => {
        paste::paste!{
            impl<T: $tbound> DataTypes for $tname<T> {
                fn data_align(&self) -> usize {
                    use $tname::*;
                    match self {
                        None => core::mem::align_of::<()>(),
                        With(o) => o.data_align(),

                        $( $cvname => core::mem::align_of::<$cvtype>(), )*
                        $( $vname => core::mem::align_of::<$vtype>(), )*

                        $( // pointer-size dependant
                            #[cfg($cvpsize_psize)]
                            $cvname_psize => core::mem::align_of::<$cvtype_psize>(),
                        )*

                        $( // feature-gated dependencies
                            #[cfg(all(feature = $cvdep1_dep, feature = $cvdep2_dep))]
                            $cvname_dep => core::mem::align_of::<$cvtype_dep>(),
                        )*
                        $(
                            #[cfg(all(feature = $vdep1_dep, feature = $vdep2_dep))]
                            $vname_dep => core::mem::align_of::<$vtype_dep>(),
                        )*

                        $( // pointer-size & feature-gated dependencies
                            #[cfg(all($cvpsize_psize_dep, feature = $cvdep1_psize_dep, feature = $cvdep2_psize_dep))]
                            $cvname_psize_dep => core::mem::align_of::<$cvtype_psize_dep>(),
                        )*
                        $(
                            #[cfg(all($vpsize_psize_dep, feature = $vdep1_psize_dep, feature = $vdep2_psize_dep))]
                            $vname_psize_dep => core::mem::align_of::<$vtype_psize_dep>(),
                        )*
                    }
                }
                fn data_size(&self) -> usize {
                    use $tname::*;
                    match self {
                        None => core::mem::size_of::<()>(),
                        With(o) => o.data_size(),

                        $( $cvname => core::mem::size_of::<$cvtype>(), )*
                        $( $vname => core::mem::size_of::<$vtype>(), )*

                        $( // pointer-size dependant
                            #[cfg($cvpsize_psize)]
                            $cvname_psize => core::mem::size_of::<$cvtype_psize>(),
                        )*

                        $( // feature-gated dependencies
                            #[cfg(all(feature = $cvdep1_dep, feature = $cvdep2_dep))]
                            $cvname_dep => core::mem::size_of::<$cvtype_dep>(),
                        )*
                        $(
                            #[cfg(all(feature = $vdep1_dep, feature = $vdep2_dep))]
                            $vname_dep => core::mem::size_of::<$vtype_dep>(),
                        )*

                        $( // pointer-size & feature-gated dependencies
                            #[cfg(all($cvpsize_psize_dep, feature = $cvdep1_psize_dep, feature = $cvdep2_psize_dep))]
                            $cvname_psize_dep => core::mem::size_of::<$cvtype_psize_dep>(),
                        )*
                        $(
                            #[cfg(all($vpsize_psize_dep, feature = $vdep1_psize_dep, feature = $vdep2_psize_dep))]
                            $vname_psize_dep => core::mem::size_of::<$vtype_psize_dep>(),
                        )*
                    }
                }
                fn is_copy(&self) -> bool { $is_copy }
            }
        }
    };
}
pub(crate) use impl_data_types;

/// implement: DataCells trait
macro_rules! impl_data_cells {
    (
        c: $cname:ident, $cbound:ident,
        t: $tname:ident, $tbound:ident,
        is_copy: $is_copy:stmt,
        copy_variants:
            $( $cvname:ident, $cvtype:ty ),* ;
        copy_variants_dep:
            $( $cvname_dep:ident, $cvtype_dep:ty,
            $cvdep1_dep:literal, $cvdep2_dep:literal ),* ;
        copy_variants_psize:
            $( $cvname_psize:ident, $cvtype_psize:ty, $cvpsize_psize:meta ),* ;
        copy_variants_psize_dep:
            $( $cvname_psize_dep:ident, $cvtype_psize_dep:ty,
            $cvpsize_psize_dep:meta, $cvdep1_psize_dep:literal, $cvdep2_psize_dep:literal ),* ;
        noncopy_variants:
            $( $vname:ident, $vtype:ty ),* ;
        noncopy_variants_dep:
            $( $vname_dep:ident, $vtype_dep:ty,
            $vdep1_dep:literal, $vdep2_dep:literal ),* ;
        noncopy_variants_psize_dep:
            $( $vname_psize_dep:ident, $vtype_psize_dep:ty,
            $vpsize_psize_dep:meta, $vdep1_psize_dep:literal, $vdep2_psize_dep:literal ),* ;
    ) => {
        paste::paste! {
            impl<C: $cbound> DataCells for $cname<C> {
                fn is_copy(&self) -> bool { $is_copy }
            }
        }
    };
}
pub(crate) use impl_data_cells;

/// implement: DataBares trait
macro_rules! impl_data_bares {
    (
      b: $bname:ident,
    ) => {
        // impl DataCells for $bname {
        //     fn is_copy(&self) -> bool { true }
        // }
        // impl DataCellsCopy for $bname {}

        #[cfg(not(feature = "safe"))]
        unsafe impl DataBares for $bname {}
    };
}
pub(crate) use impl_data_bares;

/// re-exports types from public modules.
macro_rules! reexport {
    // external branches, multi-type re-export
    // -------------------------------------------------------------------------

    // reexports all the sizes
    ($mod:ident, $path:path; all_sizes) => {
        use $crate::unit::macros::reexport;
        reexport![$mod $path; 1, 8];
        reexport![$mod $path; 2, 16];
        reexport![$mod $path; 4, 32];
        reexport![$mod $path; 8, 64];
        reexport![$mod $path; 16, 128];
        reexport![$mod $path; 32, 256];
        reexport![$mod $path; 64, 512];
        reexport![$mod $path; 128, 1024];
    };

    // `::size::` sub-modules reexports, single size.
    (mod_size $path:path; $B:literal, $b:literal ) => {
        paste::paste!{
            #[doc = $b " bit data (== " $B " Byte)" ]
            pub mod [< b $b >] {
                $crate::unit::macros::reexport![@CellType $path; size: $b; bit bitWith bitCopy bitCopyWith ];
                $crate::unit::macros::reexport![@Bare $path; size: $b; bitCopy ];
            }
        }
    };

    // `::cells::` reexports, single size
    (mod_cells $path:path; $B:literal, $b:literal ) => {
        paste::paste!{
            $crate::unit::macros::reexport![@Cell $path; size: $b; bit bitWith bitCopy bitCopyWith ];
        }
    };
    // `::types::` reexports, single size
    (mod_types $path:path; $B:literal, $b:literal ) => {
        paste::paste!{
            $crate::unit::macros::reexport![@Type $path; size: $b; bit bitWith bitCopy bitCopyWith ];
        }
    };
    // `::bares::` reexports, single size
    (mod_bares $path:path; $B:literal, $b:literal ) => {
        paste::paste!{
            // WIP
            $crate::unit::macros::reexport![@Bare $path; size: $b; bitCopy ];
            // $crate::unit::macros::reexport![@Bare $path; size: $b; bit bitWith bitCopy bitCopyWith ];
        }
    };

    // internal branches, single-type re-export
    // -------------------------------------------------------------------------

    // re-exports DataCell
    (@Cell $path:path; size: $size:literal; $( $suf:ident )+ ) => {
        $crate::unit::macros::reexport![@ $path; DataCell; size: $size ; $( $suf )+ ];
    };

    // re-exports DataType
    (@Type $path:path; size: $size:literal; $( $suf:ident )+ ) => {
        $crate::unit::macros::reexport![@ $path; DataType; size: $size ; $( $suf )+ ];
    };

    // re-exports DataBare
    (@Bare $path:path; size: $size:literal; $( $suf:ident )+ ) => {
        $crate::unit::macros::reexport![@ $path; DataBare; size: $size ; $( $suf )+ ];
    };

    // re-exports both DataCell & DataType
    (@CellType $path:path; size: $size:literal; $( $suf:ident )+ ) => {
        $crate::unit::macros::reexport![@ $path; DataCell; size: $size ; $( $suf )+ ];
        $crate::unit::macros::reexport![@ $path; DataType; size: $size ; $( $suf )+ ];
        // NOTE DataBare can't accept non-copy (for now) so must be treated separately
        // $crate::unit::macros::reexport![@ $path; DataBare; size: $size ; $( $suf )+ ];
    };

    // generic re-export
    (@ $path:path; $type:ty; size: $size:literal; $( $suf:ident )+ ) => {
        $( paste::paste!{
            pub use $path::[< $type $size $suf >];
        } )+
    };
}
pub(crate) use reexport;
