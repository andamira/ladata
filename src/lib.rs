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
//! * `safe` forbids unsafe code.
//!   ```shell
//!   $ cargo build --features=safe
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
#![cfg_attr(feature = "safe", forbid(unsafe_code))]
#![cfg_attr(feature = "nightly", feature(doc_cfg))]

pub mod error;
pub mod list;
// pub mod hybrid;
// pub mod key;
pub mod mem;
// pub mod tree;
pub mod unit;

/// Everything re-exported.
pub mod all {
    #[doc(inline)]
    pub use super::error::*;

    #[doc(inline)]
    #[cfg(feature = "bv")]
    pub use super::list::bit::*;
    #[doc(inline)]
    pub use super::list::{deque::*, link::*, queue::*, stack::*};

    #[doc(inline)]
    #[cfg(feature = "std")]
    pub use super::mem::Boxed;
    #[doc(inline)]
    pub use super::mem::{array::*, Raw, Storage};

    // #[doc(inline)]
    // pub use super::tree::*;

    #[doc(inline)]
    pub use crate::unit::{bares::*, cells::*, traits::*, types::*};
}
