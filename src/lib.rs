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

pub mod line;
// pub mod map;
pub mod mem;
// pub mod tree;
pub mod unit;

/// Everything is directly available in here.
pub mod all {
    #[doc(inline)]
    pub use super::mem::*;
    #[doc(inline)]
    pub use super::line::{bit::*, stack::*, link::*};
    #[doc(inline)]
    pub use crate::unit::{bares::*, cells::*, traits::*, types::*};
}
