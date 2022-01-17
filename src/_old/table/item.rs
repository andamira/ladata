//!
//!

use std::any::Any;

// TODO: a flat structure, with inquiry methods, e.g.: is_categorical(), is_…?
//
//
// RETHINK: whether to use this for what the column & Row gives out.

/// A *flat?* enum of all the possible data values contained in a
/// [`Table`][crate::Table].
pub enum DataItem {
    // Numerical(NumericalItem) // ?

    // Float(FloatVecItem), // ?
    F32(f32),
    F64(f64),

    // Integer(IntVecItem), // ?
    I8(i8),
    U8(u8),
    I16(i16),
    U16(u16),
    I32(i32),
    U32(u32),
    I64(i64),
    U64(u64),
    I128(i128),
    U128(u128),

    // Categorical(CategoricalItem) // ?
    Bool(bool),
    String(String),
    Anyboxed(Box<dyn Any>),
    // Handle(HandleVecItem), //← this may indicate the need for nesting…
}
