// ladata::storage
//
//! Choose how the data is stored.
//!
//! The trait [`Storage`] allows data structure implementations to have
//! methods (specially constructors) be specialized by storage type.
//!
//! It is already implemented for the [`Boxed`] type, that wraps the data
//! in a [`Box`], and for the [`()`] type which wraps the data in a [`Lac`].
//!
//! A new [`Lac`] type is defined, analogous to [`Box`], but without moving
//! the data to the heap. In order to be able to be generic over the storage.
//

use core::ops;

mod lac;
pub use lac::Lac;

/// Allows to be generic in respect of the data storage.
///
/// There are two reference implementations:
/// - [`Boxed`], which wraps the data in a [`Box`].
/// - [`()`][unit], which wraps the data in a [`Lac`].
///
/// # Examples
/// ```
/// use core::{array, mem::size_of};
/// use ladata::storage::{Boxed, Storage};
///
/// /// Generically store a generic array of generic size.
/// pub struct MyStructure<T, S: Storage, const L: usize> {
///     data: S::Container<[T; L]>,
/// }
///
/// impl<T, S: Storage, const L: usize> MyStructure<T, S, L> {
///     pub fn new() -> Self
///     where
///         T: Default,
///     {
///         Self {
///             data: S::Container::from(array::from_fn(|_| T::default())),
///         }
///     }
/// }
///
/// // The array is stored in the stack
/// assert_eq![100, size_of::<MyStructure::<u8, (), 100>>()];
///
/// // The array is stored in the heap.
/// assert_eq![8, size_of::<MyStructure::<u8, Boxed, 100>>()];
///
/// ```
pub trait Storage {
    type Container<T>: ops::DerefMut<Target = T> + From<T>;
}

/// A storage that wraps its data in a [`Box`].
pub struct Boxed;
impl Storage for Boxed {
    type Container<T> = Box<T>;
}

/// A storage that wraps its data in a [`Lac`].
impl Storage for () {
    type Container<T> = Lac<T>;
}
