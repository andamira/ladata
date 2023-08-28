// ladata::lib
//
#![doc = include_str!("./Lib.md")]
//

// warnings
#![warn(clippy::all)]
#![allow(non_snake_case, non_camel_case_types, clippy::module_inception)]
#![allow(unused_attributes, unused_doc_comments)] // needed in libera
// environment
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(feature = "safe", forbid(unsafe_code))]
#![cfg_attr(feature = "nightly", feature(doc_cfg))]
#[cfg(feature = "alloc")]
extern crate alloc;

// safeguards
#[cfg(all(feature = "std", feature = "no-std"))]
compile_error!("You can't enable the `std` and `no-std` features at the same time.");
#[cfg(all(
    feature = "safe",
    any(feature = "unsafe", feature = "unsafe_constructors",)
))]
compile_error!("You can't enable the `safe` and `unsafe*` features at the same time.");
// deprecated
devela::deprecate_feature![old: "no-std", new: "no_std", since: "0.8.0"];

pub mod error;
pub mod grid;
pub mod list;
// pub mod hybrid;
// pub mod key;
pub mod mem;
pub mod misc;
// pub mod tree;
pub mod unit;

/// All items are reexported here.
pub mod all {
    #[doc(inline)]
    pub use super::{error::*, grid::*, list::all::*, mem::all::*, misc::*, unit::all::*};
}
