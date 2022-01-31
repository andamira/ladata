// src/dataframe/handle
//!
//! Handles for relationships between dataframes.
//
// WIP

/// An 8-bit handle.
#[derive(Copy, Clone, Debug, Hash, PartialEq)]
pub struct Handle8(u8);

/// A 16-bit handle.
#[derive(Copy, Clone, Debug, Hash, PartialEq)]
pub struct Handle16(u16);

/// A 32-bit handle.
#[derive(Copy, Clone, Debug, Hash, PartialEq)]
pub struct Handle32(u32);

/// A 64-bit handle.
#[derive(Copy, Clone, Debug, Hash, PartialEq)]
pub struct Handle64(u64);

/// A 128-bit handle.
#[derive(Copy, Clone, Debug, Hash, PartialEq)]
pub struct Handle128(u128);
