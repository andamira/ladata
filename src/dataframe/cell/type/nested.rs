// src/dataframe/cell/type/nested
//!

use crate::dataframe::cell::CellType;

/// A nested representation of cell types (3 bytes).
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CellTypeNested {
    Categorical(CategoricalType),
    Numerical(NumericalType),
}

/// Categorical types (1 byte).
///
/// A categorical data type is one that can not be measured (AKA *qualitative*).
///
/// Examples: Gender, Opinion, Rating…
///
/// Note that In practice it's possible to handle some categorical data as if
/// it were numerical, e.g. associating ratings to numbers, and so on.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CategoricalType {
    /// Boolean value.
    Bool,

    /// String value.
    String,

    /// A handle ID for relational operations.
    Id(IdType),

    /// A collection of bytes.
    Bytes,
}

///

/// Identifier types (1 byte).
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IdType {
    /// A UUID.
    Uuid,

    /// An 8-bit handle.
    Handle8,

    /// A 16-bit handle.
    Handle16,

    /// A 32-bit handle.
    Handle32,

    /// A 64-bit handle.
    Handle64,

    /// A 128-bit handle.
    Handle128,
}

/// Numerical types (2 bytes).
///
/// A numerical data type can be measured (AKA *quantitative*).
///
/// Examples: Age, Height, purchases per month…
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NumericalType {
    /// A floating-point number.
    Continuous(ContinuousType),

    /// An integer number.
    Discrete(DiscreteType),
}

/// Discrete types (1 byte).
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DiscreteType {
    I8,
    U8,
    I16,
    U16,
    I32,
    U32,
    I64,
    U64,
    I128,
    U128,
}

/// Continuous types (1 byte).
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ContinuousType {
    // F16, // Half
    F32,
    F64,
    // F128, // TwoFloat
}

impl CellTypeNested {
    /// Returns the `CellType` equivalent to the current `CellTypeNested`.
    pub fn flat(&self) -> CellType {
        self.into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem::size_of;

    #[test]
    fn size() {
        assert_eq![3, size_of::<CellTypeNested>()];
        assert_eq![1, size_of::<CategoricalType>()];
        assert_eq![1, size_of::<IdType>()];
        assert_eq![2, size_of::<NumericalType>()];
        assert_eq![1, size_of::<DiscreteType>()];
        assert_eq![1, size_of::<ContinuousType>()];
    }
}
