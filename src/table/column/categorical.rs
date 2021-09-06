//!
//!
// TODO: Any dynamic type, any user-provided enum.

use std::any::Any;

/// A categorical data type is one that can not be measured (AKA *qualitative*).
///
/// Examples: Gender, Opinion, Rating…
///
/// Note that In practice it's possible to handle some categorical data as if
/// it were numerical, e.g. associating ratings to numbers, and so on.
#[derive(Debug)]
#[non_exhaustive]
pub enum Categorical {
    /// A Vec of boolean values.
    Bool(Vec<bool>),
    /// A Vec of string values.
    String(Vec<String>),
    /// A Vec of boxed dynamic values.
    AnyBoxed(Vec<Box<dyn Any>>),
    /// A Vec of handles for data stored using e.g. an arena.
    Handle(HandleVec),
}

impl From<Vec<String>> for Categorical {
    fn from(v: Vec<String>) -> Self {
        Categorical::String(v)
    }
}
impl From<Vec<bool>> for Categorical {
    fn from(v: Vec<bool>) -> Self {
        Categorical::Bool(v)
    }
}
impl From<Vec<Box<dyn Any>>> for Categorical {
    fn from(v: Vec<Box<dyn Any>>) -> Self {
        Categorical::AnyBoxed(v)
    }
}
impl From<HandleVec> for Categorical {
    fn from(v: HandleVec) -> Self {
        Categorical::Handle(v)
    }
}

/// A vector of handles for data stored in a third party collection,
/// casted as integers of the appropriate size.
//
// TODO: handy methods, derefs, casts…
#[derive(Debug)]
pub enum HandleVec {
    U8(Vec<u8>),
    U16(Vec<u16>),
    U32(Vec<u32>),
    U64(Vec<u64>),
    U128(Vec<u128>),
}
impl From<Vec<u8>> for HandleVec {
    fn from(v: Vec<u8>) -> Self {
        HandleVec::U8(v)
    }
}
impl From<Vec<u16>> for HandleVec {
    fn from(v: Vec<u16>) -> Self {
        HandleVec::U16(v)
    }
}
impl From<Vec<u32>> for HandleVec {
    fn from(v: Vec<u32>) -> Self {
        HandleVec::U32(v)
    }
}
impl From<Vec<u64>> for HandleVec {
    fn from(v: Vec<u64>) -> Self {
        HandleVec::U64(v)
    }
}
impl From<Vec<u128>> for HandleVec {
    fn from(v: Vec<u128>) -> Self {
        HandleVec::U128(v)
    }
}
