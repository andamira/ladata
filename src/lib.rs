// ladata::lib
//
//! A simple & modular data model.
//

#![warn(clippy::all)]
#![allow(non_snake_case, non_camel_case_types, clippy::module_inception)]
//
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(feature = "safe", forbid(unsafe_code))]
#![cfg_attr(feature = "nightly", feature(doc_cfg))]

#[cfg(all(feature = "std", feature = "no-std"))]
compile_error!("You can't enable the `std` and `no-std` features at the same time.");
#[cfg(all(feature = "safe", feature = "unsafe"))]
compile_error!("You can't enable the `safe` and `unsafe` features at the same time.");

#[cfg(feature = "alloc")]
extern crate alloc;

pub mod error;
pub mod grid;
pub mod list;
// pub mod hybrid;
// pub mod key;
pub mod mem;
pub mod misc;
// pub mod tree;
pub mod unit;

/// All types.
///
/// Everything is re-exported from here.
pub mod all {
    #[doc(inline)]
    pub use super::{error::*, grid::*, list::all::*, mem::all::*, misc::*, unit::all::*};
}
