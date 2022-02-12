// ladata::cell::type::nested
//
//
//!
//!
//

use crate::cell::CellType;

/// A nested representation of cell types (3 bytes).
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CellTypeNested {
    None,
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

    // /// String value.
    // String,
    //
    // /// A collection of bytes.
    // Bytes,
    /// A handle ID for relational operations.
    Id(IdType),
}

///

/// Identifier types (1 byte).
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IdType {
    // /// A UUID.
    // Uuid,
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

mod std_impls {
    use super::CellTypeNested::{self, *};
    use crate::cell::nested::{
        CategoricalType::*, ContinuousType::*, DiscreteType::*, IdType::*, NumericalType::*,
    };
    use std::fmt;

    impl fmt::Display for CellTypeNested {
        fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
            let s = match self {
                None => "None",
                Categorical(c) => match c {
                    Bool => "Categorical::Bool",
                    // String => "Categorical::String",
                    // Bytes => "Categorical::Bytes",
                    Id(i) => match i {
                        Handle8 => "Categorical::Id::Handle8",
                        Handle16 => "Categorical::Id::Handle16",
                        Handle32 => "Categorical::Id::Handle32",
                        Handle64 => "Categorical::Id::Handle64",
                        Handle128 => "Categorical::Id::Handle128",
                        // IdType::Uuid => "Categorical::Id::Uuid",
                    },
                },
                Numerical(n) => match n {
                    Continuous(f) => match f {
                        F32 => "Numerical::Continuous::F32",
                        F64 => "Numerical::Continuous::F64",
                    },
                    Discrete(i) => match i {
                        I8 => "Numerical::Discrete::I8",
                        U8 => "Numerical::Discrete::U8",
                        I16 => "Numerical::Discrete::I16",
                        U16 => "Numerical::Discrete::U16",
                        I32 => "Numerical::Discrete::I32",
                        U32 => "Numerical::Discrete::U32",
                        I64 => "Numerical::Discrete::I64",
                        U64 => "Numerical::Discrete::U64",
                        I128 => "Numerical::Discrete::I128",
                        U128 => "Numerical::Discrete::U128",
                    },
                },
            };
            write!(f, "{}", &s)
        }
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
