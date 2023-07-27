// ladata::mem::storage
//
//! The [`Storage`] trait allows the data structure implementations to have
//! specialized methods by storage type (specially useful for constructors).
//!
//! It is already implemented for the [`Boxed`] type and the [`()`][unit] unit
//! type, which wraps their data in a [`Box`] and a [`Direct`], respectively.
//

use core::ops;

mod direct;
pub use direct::*;

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
/// #[cfg(feature = "alloc")]
/// assert_eq![8, size_of::<MyStructure::<u8, ladata::mem::Boxed, 100>>()];
///
/// ```
pub trait Storage {
    /// The stored associated type.
    type Stored<T>: ops::DerefMut<Target = T> + From<T>;

    /// Returns the static name of the storage implementation.
    ///
    /// This can be useful for debugging.
    fn name() -> &'static str;

    // MAYBE WAIT https://github.com/rust-lang/rust/issues/80437
    // fn unstore(self) -> T;
}

/// A storage type that wraps its data in a [`Box`].
#[cfg(feature = "alloc")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "alloc")))]
pub struct Boxed;

#[cfg(feature = "alloc")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "alloc")))]
impl Storage for Boxed {
    type Stored<T> = alloc::boxed::Box<T>;

    fn name() -> &'static str {
        "Boxed"
    }
}

/// A storage type that wraps its data in a [`Direct`].
impl Storage for () {
    type Stored<T> = Direct<T>;

    fn name() -> &'static str {
        "Direct"
    }
}
