// ladata::builder
//
//! Types are generated in these submodules and re-exported from elsewhere.

pub(crate) mod types_units;
pub use types_units::*;

pub(crate) mod lines;
pub use lines::*;

/// re-exports types from public modules.
#[macro_export]
#[doc(hidden)]
macro_rules! reexport {
    // modules
    // -------------------------------------------------------------------------

    // Generates `::sizes` sub-modules for all sizes.
    (mod_sizes $path:path; all_sizes) => {
        use crate::reexport;
        reexport![mod_sizes $path; 1, 8];
        reexport![mod_sizes $path; 2, 16];
        reexport![mod_sizes $path; 4, 32];
        reexport![mod_sizes $path; 8, 64];
        reexport![mod_sizes $path; 16, 128];
        reexport![mod_sizes $path; 32, 256];
        reexport![mod_sizes $path; 64, 512];
        reexport![mod_sizes $path; 128, 1024];
    };
    // Generates `::sizes::` sub-modules for the pair of (Byte, bit) sizes.
    // - Each module is referenced as a submodule of each other.
    (mod_sizes $path:path; $B:literal, $b:literal ) => {
        paste::paste!{
            #[doc = $B " Byte data (== " $b " bit)" ]
            pub mod [< B $B >] {
                #[doc(inline)]
                pub use super::[< b $b >];
                crate::reexport![@unit_type super::$path; size: $B; Byte ByteWith ByteCopy ByteCopyWith ];
                crate::reexport![@unsafe super::$path; size: $B; ByteCopy ];
            }
            #[doc = $b " bit data (== " $B " Byte)" ]
            pub mod [< b $b >] {
                #[doc(inline)]
                pub use super::[< B $B >];
                crate::reexport![@unit_type super::$path; size: $b; bit bitWith bitCopy bitCopyWith ];
                crate::reexport![@unsafe super::$path; size: $b; bitCopy ];
            }
            // TODO:add lines
        }
    };

    // Generates `::lines::` sub-modules for all sizes.
    (mod_lines $path:path; all_sizes) => {
        use crate::reexport;
        reexport![mod_lines $path; 1, 8];
        reexport![mod_lines $path; 2, 16];
        reexport![mod_lines $path; 4, 32];
        reexport![mod_lines $path; 8, 64];
        reexport![mod_lines $path; 16, 128];
        reexport![mod_lines $path; 32, 256];
        reexport![mod_lines $path; 64, 512];
        reexport![mod_lines $path; 128, 1024];
    };

    // Generates `::lines::` sub-modules for the pair of (Byte, bit) sizes.
    (mod_lines $path:path; $B:literal, $b:literal ) => {
        paste::paste!{
            crate::reexport![@line $path; size: $B; Byte ByteWith ByteCopy ByteCopyWith ];
            // crate::reexport![@line $path; size: $b; bit bitWith bitCopy bitCopyWith ];
        }
    };

    // -------------------------------------------------------------------------

    // generic re-export
    (@type $path:path; $type:ty; size: $size:literal; $( $suf:ident )+ ) => {
        $( paste::paste!{
            pub use $path::[< $type $size $suf >];
        } )+
    };

    // re-exports DataUnit & DataType
    (@unit_type $path:path; size: $size:literal; $( $suf:ident )+ ) => {
        crate::reexport![@type $path; DataUnit; size: $size ; $( $suf )+ ];
        crate::reexport![@type $path; DataType; size: $size ; $( $suf )+ ];
        // crate::reexport![@type $path; DataLone; size: $size ; $( $suf )+ ];
    };

    // re-exports DataLone
    // NOTE DataLone can't accept non-copy (for now) so must be treated separately
    (@unsafe $path:path; size: $size:literal; $( $suf:ident )+ ) => {
        crate::reexport![@type $path; DataLone; size: $size ; $( $suf )+ ];
    };

    // re-exports DataLine
    (@line $path:path; size: $size:literal; $( $suf:ident )+ ) => {
        crate::reexport![@type $path; DataLine; size: $size ; $( $suf )+ ];
        crate::reexport![@type $path; DataLineGrow; size: $size ; $( $suf )+ ];
        // TODO:WIP
    };
}
