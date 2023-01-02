//! A simple & modular data model.
//!
//! # Overview
//!
//! TODO
//!
//! ## External dependencies
//!
//! The standard library is enabled by default. You can remove it independently
//! of other features, by not including the "std" feature.
//!
//! The default features provide a curated selection of external types than can
//! complement the standard Rust types.
//!
//! ### Customizing features
//!
//! * Only `no_std`, without any dependencies:
//!   ```shell
//!   $ cargo build --no-default-features
//!   ```
//!
//! * only the standard library:
//!   ```shell
//!   $ cargo build --no-default-features --features=std
//!   ```
//!
//! * `no_unsafe` forbids unsafe code.
//!   ```shell
//!   $ cargo build --features=no_unsafe
//!   ```
//!
//! * all the dependencies (See the full list in
//! [`Cargo.toml`](https://github.com/andamira/ladata/blob/main/Cargo.toml)):
//!   ```shell
//!   $ cargo build --features=deps_all
//!   ```
//!
//!
//! [w]: https://en.wikipedia.org/wiki/List_of_data_structures#Linear_data_structures

#![allow(non_snake_case, non_camel_case_types)]
//
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(feature = "no_unsafe", forbid(unsafe_code))]

pub mod error;
pub mod line;
// pub mod map;
pub mod mem;
// pub mod tree;
pub mod unit;

/// Everything re-exported.
pub mod all {
    #[doc(inline)]
    pub use super::error::*;

    #[doc(inline)]
    pub use super::line::{link::*, stack::*};

    #[doc(inline)]
    #[cfg(feature = "std")]
    pub use super::line::bit::*;

    #[doc(inline)]
    pub use super::mem::*;

    #[doc(inline)]
    pub use crate::unit::{bares::*, cells::*, traits::*, types::*};
}
