// src/frame/handle
//!
//! Handles for referencing [`DataFrame`][crate::frame::DataFrame] rows.
//

/// An 8-bit handle.
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Hash, PartialEq)]
pub struct Handle8(u8);

/// A 16-bit handle.
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Hash, PartialEq)]
pub struct Handle16(u16);

/// A 32-bit handle.
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Hash, PartialEq)]
pub struct Handle32(u32);

/// A 64-bit handle.
///
// WIP: Convert from UUID… ?
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Hash, PartialEq)]
pub struct Handle64(u64);

/// A 128-bit handle.
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Hash, PartialEq)]
pub struct Handle128(u128);

macro_rules! impl_handle {
    ($t1:ty, $inner:ty) => {
        impl $t1 {
            pub fn new(value: $inner) -> $t1 {
                Self(value)
            }
        }
    };
}
impl_handle![Handle8, u8];
impl_handle![Handle16, u16];
impl_handle![Handle32, u32];
impl_handle![Handle64, u64];
impl_handle![Handle128, u128];
