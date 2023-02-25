// ladata::mem
//
//! Memory-management types.
//!
//! The trait [`Storage`] allows data structure implementations to have
//! methods (specially constructors) be specialized by storage type.
//!
//! It is implemented for the [`Boxed`] type and the [`()`][unit] unit type,
//! which wraps their data in a [`Box`] and a [`Direct`], respectively.
//

use core::ops;

// mod arena;
// #[doc(inline)]
// pub use arena::*;

// mod cache;
// #[doc(inline)]
// pub use cache::*;

// mod pool;
// #[doc(inline)]
// pub use pool::*;

mod direct;
pub use direct::Direct;

/// Allows to be generic in respect of the data storage.
///
/// There are two reference implementations:
/// - [`Boxed`], which wraps the data in a [`Box`].
/// - [`()`][unit], which wraps the data in a [`Direct`].
///
/// # Examples
/// ```
/// use core::{array, mem::size_of};
/// use ladata::mem::Storage;
///
/// /// Generically store a generic array of generic size.
/// pub struct MyStructure<T, S: Storage, const L: usize> {
///     data: S::Stored<[T; L]>,
/// }
///
/// impl<T, S: Storage, const L: usize> MyStructure<T, S, L> {
///     pub fn new() -> Self
///     where
///         T: Default,
///     {
///         Self {
///             data: S::Stored::from(array::from_fn(|_| T::default())),
///         }
///     }
/// }
///
/// // The array is stored in the stack
/// assert_eq![100, size_of::<MyStructure::<u8, (), 100>>()];
///
/// // The array is stored in the heap.
/// #[cfg(feature = "std")]
/// assert_eq![8, size_of::<MyStructure::<u8, ladata::mem::Boxed, 100>>()];
///
/// ```
pub trait Storage {
    type Stored<T>: ops::DerefMut<Target = T> + From<T>;
}

/// A storage type that wraps its data in a [`Box`].
#[cfg(feature = "std")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "std")))]
pub struct Boxed;

#[cfg(feature = "std")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "std")))]
impl Storage for Boxed {
    type Stored<T> = Box<T>;
}

/// A storage type that wraps its data in a [`Direct`].
impl Storage for () {
    type Stored<T> = Direct<T>;
}
