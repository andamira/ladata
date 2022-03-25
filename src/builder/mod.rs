// ladata::builder
//
//! Types are generated in these submodules and re-exported from elsewhere.
//

pub(crate) mod units;
pub use units::*;

pub(crate) mod lines;
pub use lines::*;

/// re-exports types from public modules.
#[macro_export]
#[doc(hidden)]
macro_rules! reexport {
    // external branches, multi-type re-export
    // -------------------------------------------------------------------------

    // reexports all the sizes
    ($mod:ident, $path:path; all_sizes) => {
        use crate::reexport;
        reexport![$mod $path; 1, 8];
        reexport![$mod $path; 2, 16];
        reexport![$mod $path; 4, 32];
        reexport![$mod $path; 8, 64];
        reexport![$mod $path; 16, 128];
        reexport![$mod $path; 32, 256];
        reexport![$mod $path; 64, 512];
        reexport![$mod $path; 128, 1024];
    };

    // `::sizes::` sub-modules reexports, single size.
    (mod_sizes $path:path; $B:literal, $b:literal ) => {
        paste::paste!{
            #[doc = $B " Byte data (== " $b " bit)" ]
            pub mod [< B $B >] {
                pub use super::[< b $b >];
                crate::reexport![@CellType $path; size: $B; Byte ByteWith ByteCopy ByteCopyWith ];
                crate::reexport![@Bare $path; size: $B; ByteCopy ];
                crate::reexport![@Line $path; size: $B; Byte ByteWith ByteCopy ByteCopyWith ];
            }
            #[doc = $b " bit data (== " $B " Byte)" ]
            pub mod [< b $b >] {
                pub use super::[< B $B >];
                crate::reexport![@CellType $path; size: $b; bit bitWith bitCopy bitCopyWith ];
                crate::reexport![@Bare $path; size: $b; bitCopy ];
                // crate::reexport![@line $path; size: $b; bit bitWith bitCopy bitCopyWith ]; // TODO
            }
        }
    };

    // `::lines::` reexports, single size
    (mod_lines $path:path; $B:literal, $b:literal ) => {
        paste::paste!{
            crate::reexport![@Line $path; size: $B; Byte ByteWith ByteCopy ByteCopyWith ];
            // crate::reexport![@Line $path; size: $b; bit bitWith bitCopy bitCopyWith ]; // TODO
        }
    };

    // `::cells::` reexports, single size
    (mod_cells $path:path; $B:literal, $b:literal ) => {
        paste::paste!{
            crate::reexport![@Cell $path; size: $B; Byte ByteWith ByteCopy ByteCopyWith ];
            crate::reexport![@Cell $path; size: $b; bit bitWith bitCopy bitCopyWith ];
        }
    };
    // `::types::` reexports, single size
    (mod_types $path:path; $B:literal, $b:literal ) => {
        paste::paste!{
            crate::reexport![@Type $path; size: $B; Byte ByteWith ByteCopy ByteCopyWith ];
            crate::reexport![@Type $path; size: $b; bit bitWith bitCopy bitCopyWith ];
        }
    };
    // `::bares::` reexports, single size
    (mod_bares $path:path; $B:literal, $b:literal ) => {
        paste::paste!{
            // WIP
            crate::reexport![@Bare $path; size: $B; ByteCopy ];
            crate::reexport![@Bare $path; size: $b; bitCopy ];
            // crate::reexport![@Bare $path; size: $B; Byte ByteWith ByteCopy ByteCopyWith ];
            // crate::reexport![@Bare $path; size: $b; bit bitWith bitCopy bitCopyWith ];
        }
    };

    // internal branches, single-type re-export
    // -------------------------------------------------------------------------

    // generic re-export
    (@ $path:path; $type:ty; size: $size:literal; $( $suf:ident )+ ) => {
        $( paste::paste!{
            pub use $path::[< $type $size $suf >];
        } )+
    };

    // re-exports DataCell
    (@Cell $path:path; size: $size:literal; $( $suf:ident )+ ) => {
        crate::reexport![@ $path; DataCell; size: $size ; $( $suf )+ ];
    };

    // re-exports DataType
    (@Type $path:path; size: $size:literal; $( $suf:ident )+ ) => {
        crate::reexport![@ $path; DataType; size: $size ; $( $suf )+ ];
    };

    // re-exports DataBare
    (@Bare $path:path; size: $size:literal; $( $suf:ident )+ ) => {
        crate::reexport![@ $path; DataBare; size: $size ; $( $suf )+ ];
    };

    // re-exports both DataCell & DataType
    (@CellType $path:path; size: $size:literal; $( $suf:ident )+ ) => {
        crate::reexport![@ $path; DataCell; size: $size ; $( $suf )+ ];
        crate::reexport![@ $path; DataType; size: $size ; $( $suf )+ ];
        // NOTE DataBare can't accept non-copy (for now) so must be treated separately
        // crate::reexport![@ $path; DataBare; size: $size ; $( $suf )+ ];
    };

    // re-exports DataLine
    (@Line $path:path; size: $size:literal; $( $suf:ident )+ ) => {
        crate::reexport![@ $path; DataLine; size: $size ; $( $suf )+ ];
        crate::reexport![@ $path; DataLineGrow; size: $size ; $( $suf )+ ];
    };
}
