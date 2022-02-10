// src/frame/cell/type
//
//!
//!
//

use core::{
    any::TypeId,
    mem::{align_of, size_of},
};

use crate::frame::handle;

mod conversions;

mod nested;
pub use nested::{
    CategoricalType, CellTypeNested, ContinuousType, DiscreteType, IdType, NumericalType,
};

/// A flat representation of cell types (1 byte).
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CellType {
    // Categorical
    // -----------
    /// Boolean value.
    Bool,
    // /// String value.
    // String,
    // /// A collection of bytes.
    // Bytes,

    // Id
    // /// UUID
    // Uuid,
    // Handles
    Handle8,
    Handle16,
    Handle32,
    Handle64,
    Handle128,

    // Numerical
    // ---------

    // Continuous
    F32,
    F64,

    // Discrete
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

impl CellType {
    /// Returns the `CellTypeNested` equivalent to the current `CellType`.
    pub fn nested(&self) -> CellTypeNested {
        self.into()
    }

    /// Returns the [`TypeId`] of the underlying data type.
    pub fn type_id(&self) -> TypeId {
        use CellType::*;
        match self {
            Bool => TypeId::of::<bool>(),
            // String => TypeId::of::<std::string::String>(),
            // Bytes => TypeId::of::<Vec<u8>>(),
            Handle8 => TypeId::of::<handle::Handle8>(),
            Handle16 => TypeId::of::<handle::Handle16>(),
            Handle32 => TypeId::of::<handle::Handle32>(),
            Handle64 => TypeId::of::<handle::Handle64>(),
            Handle128 => TypeId::of::<handle::Handle128>(),
            // Uuid => TypeId::of::<handle::Uuid>(),
            F32 => TypeId::of::<f32>(),
            F64 => TypeId::of::<f64>(),
            I8 => TypeId::of::<i8>(),
            U8 => TypeId::of::<u8>(),
            I16 => TypeId::of::<i16>(),
            U16 => TypeId::of::<u16>(),
            I32 => TypeId::of::<i32>(),
            U32 => TypeId::of::<u32>(),
            I64 => TypeId::of::<i64>(),
            U64 => TypeId::of::<u64>(),
            I128 => TypeId::of::<i128>(),
            U128 => TypeId::of::<u128>(),
        }
    }

    /// Returns the size of the underlying data type in bytes.
    pub const fn size(&self) -> usize {
        use CellType::*;
        match self {
            Bool => size_of::<bool>(),
            // String => size_of::<std::string::String>(),
            // Bytes => size_of::<Vec<u8>>(),
            Handle8 => size_of::<handle::Handle8>(),
            Handle16 => size_of::<handle::Handle16>(),
            Handle32 => size_of::<handle::Handle32>(),
            Handle64 => size_of::<handle::Handle64>(),
            Handle128 => size_of::<handle::Handle128>(),
            // Uuid => size_of::<handle::Uuid>(),
            F32 => size_of::<f32>(),
            F64 => size_of::<f64>(),
            I8 => size_of::<i8>(),
            U8 => size_of::<u8>(),
            I16 => size_of::<i16>(),
            U16 => size_of::<u16>(),
            I32 => size_of::<i32>(),
            U32 => size_of::<u32>(),
            I64 => size_of::<i64>(),
            U64 => size_of::<u64>(),
            I128 => size_of::<i128>(),
            U128 => size_of::<u128>(),
        }
    }

    /// Returns the alignment of the underlying data type in bytes.
    pub const fn alignment(&self) -> usize {
        use CellType::*;
        match self {
            Bool => align_of::<bool>(),
            // String => align_of::<std::string::String>(),
            // Bytes => align_of::<Vec<u8>>(),
            Handle8 => align_of::<handle::Handle8>(),
            Handle16 => align_of::<handle::Handle16>(),
            Handle32 => align_of::<handle::Handle32>(),
            Handle64 => align_of::<handle::Handle64>(),
            Handle128 => align_of::<handle::Handle128>(),
            // Uuid => align_of::<handle::Uuid>(),
            F32 => align_of::<f32>(),
            F64 => align_of::<f64>(),
            I8 => align_of::<i8>(),
            U8 => align_of::<u8>(),
            I16 => align_of::<i16>(),
            U16 => align_of::<u16>(),
            I32 => align_of::<i32>(),
            U32 => align_of::<u32>(),
            I64 => align_of::<i64>(),
            U64 => align_of::<u64>(),
            I128 => align_of::<i128>(),
            U128 => align_of::<u128>(),
        }
    }
}

mod std_impls {
    use super::CellType;
    use std::fmt;

    impl fmt::Display for CellType {
        fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
            use CellType::*;
            let s = match self {
                Bool => "Bool",
                // String => "String",
                // Bytes => "Bytes",
                Handle8 => "Handle8",
                Handle16 => "Handle16",
                Handle32 => "Handle32",
                Handle64 => "Handle64",
                Handle128 => "Handle128",
                // Uuid => "Uuid",
                F32 => "F32",
                F64 => "F64",
                I8 => "I8",
                U8 => "U8",
                I16 => "I16",
                U16 => "U16",
                I32 => "I32",
                U32 => "U32",
                I64 => "I64",
                U64 => "U64",
                I128 => "I128",
                U128 => "U128",
            };
            write!(f, "{}", &s)
        }
    }
}
