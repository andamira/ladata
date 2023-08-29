// ladata::unit
#![doc = include_str!("./Mod.md")]

pub use all::*;
pub(crate) mod all {
    #[doc(inline)]
    pub use super::{traits::*, types::*, unit::*};

    #[doc(inline)]
    #[cfg(feature = "unsafe_unit")]
    #[cfg_attr(feature = "nightly", doc(cfg(feature = "unsafe_unit")))]
    pub use super::raw::*;
}

/// *Raw* Data (only the unsafe *raw* data).
#[cfg(feature = "unsafe_unit")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "unsafe_unit")))]
pub mod raw {
    super::macros::reexport![mod_raw, crate::unit::build; all_sizes];
}

/// Data *Type* (only the *type* of the data).
pub mod types {
    super::macros::reexport![mod_types, crate::unit::build; all_sizes];
}

/// Data *Unit* (*data* and *type* unified).
pub mod unit {
    super::macros::reexport![mod_unit, crate::unit::build; all_sizes];
}

/// Every unitary type, organized by size.
pub mod size {
    super::macros::reexport![mod_size, crate::unit::build; all_sizes];
}

pub mod traits;

mod build;
mod macros;
mod nodata;

#[cfg(test)]
mod tests;
