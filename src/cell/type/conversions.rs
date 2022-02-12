// src/frame/cell/type/conversions
//
//
//!
//!
//

use crate::cell::{
    nested::{
        CategoricalType, CellTypeNested, ContinuousType, DiscreteType, IdType, NumericalType,
    },
    CellData, CellType,
};

/// impl From<[&|mut]CellType> for CellTypeNested
macro_rules! impl_from_cell_type_to_nested {
    () => {
        impl_from_cell_type_to_nested![from: CellType];
        impl_from_cell_type_to_nested![from: &CellType];
        impl_from_cell_type_to_nested![from: &mut CellType];
    };
    (from: $t1:ty) => {
        impl From<$t1> for CellTypeNested {
            fn from(cell_type: $t1) -> CellTypeNested {
                use CellType::*;
                match cell_type {
                    Bool => Self::Categorical(CategoricalType::Bool),
                    // String => Self::Categorical(CategoricalType::String),
                    // Bytes => Self::Categorical(CategoricalType::Bytes),
                    Handle8 => Self::Categorical(CategoricalType::Id(IdType::Handle8)),
                    Handle16 => Self::Categorical(CategoricalType::Id(IdType::Handle16)),
                    Handle32 => Self::Categorical(CategoricalType::Id(IdType::Handle32)),
                    Handle64 => Self::Categorical(CategoricalType::Id(IdType::Handle64)),
                    Handle128 => Self::Categorical(CategoricalType::Id(IdType::Handle128)),
                    // Uuid => Self::Categorical(CategoricalType::Id(IdType::Uuid)),
                    F32 => Self::Numerical(NumericalType::Continuous(ContinuousType::F32)),
                    F64 => Self::Numerical(NumericalType::Continuous(ContinuousType::F64)),
                    I8 => Self::Numerical(NumericalType::Discrete(DiscreteType::I8)),
                    U8 => Self::Numerical(NumericalType::Discrete(DiscreteType::U8)),
                    I16 => Self::Numerical(NumericalType::Discrete(DiscreteType::I16)),
                    U16 => Self::Numerical(NumericalType::Discrete(DiscreteType::U16)),
                    I32 => Self::Numerical(NumericalType::Discrete(DiscreteType::I32)),
                    U32 => Self::Numerical(NumericalType::Discrete(DiscreteType::U32)),
                    I64 => Self::Numerical(NumericalType::Discrete(DiscreteType::I64)),
                    U64 => Self::Numerical(NumericalType::Discrete(DiscreteType::U64)),
                    I128 => Self::Numerical(NumericalType::Discrete(DiscreteType::I128)),
                    U128 => Self::Numerical(NumericalType::Discrete(DiscreteType::U128)),
                }
            }
        }
    };
}
impl_from_cell_type_to_nested![];

/// impl From<[&|mut]CellTypeNested> for CellType
macro_rules! impl_from_cell_type_nested_to_flat {
    () => {
        impl_from_cell_type_nested_to_flat![from: CellTypeNested];
        impl_from_cell_type_nested_to_flat![from: &CellTypeNested];
        impl_from_cell_type_nested_to_flat![from: &mut CellTypeNested];
    };
    (from: $t1:ty) => {
        impl From<$t1> for CellType {
            fn from(cell_type_nested: $t1) -> CellType {
                match cell_type_nested {
                    CellTypeNested::Categorical(c) => match c {
                        CategoricalType::Bool => Self::Bool,
                        // CategoricalType::String => Self::String,
                        // CategoricalType::Bytes => Self::Bytes,
                        CategoricalType::Id(i) => match i {
                            IdType::Handle8 => Self::Handle8,
                            IdType::Handle16 => Self::Handle16,
                            IdType::Handle32 => Self::Handle32,
                            IdType::Handle64 => Self::Handle64,
                            IdType::Handle128 => Self::Handle128,
                            // IdType::Uuid => Self::Uuid,
                        },
                    },
                    CellTypeNested::Numerical(n) => match n {
                        NumericalType::Continuous(f) => match f {
                            ContinuousType::F32 => Self::F32,
                            ContinuousType::F64 => Self::F64,
                        },
                        NumericalType::Discrete(i) => match i {
                            DiscreteType::I8 => Self::I8,
                            DiscreteType::U8 => Self::U8,
                            DiscreteType::I16 => Self::I16,
                            DiscreteType::U16 => Self::U16,
                            DiscreteType::I32 => Self::I32,
                            DiscreteType::U32 => Self::U32,
                            DiscreteType::I64 => Self::I64,
                            DiscreteType::U64 => Self::U64,
                            DiscreteType::I128 => Self::I128,
                            DiscreteType::U128 => Self::U128,
                        },
                    },
                }
            }
        }
    };
}
impl_from_cell_type_nested_to_flat![];

/// impl From<[&|mut]CellData> for CellType
macro_rules! impl_from_cell_data_to_type {
    () => {
        impl_from_cell_data_to_type![from: CellData];
        impl_from_cell_data_to_type![from: &CellData];
        impl_from_cell_data_to_type![from: &mut CellData];
    };
    (from: $t1:ty) => {
        impl From<$t1> for CellType {
            fn from(cell_data: $t1) -> CellType {
                use CellData::*;
                match cell_data {
                    Bool(_) => Self::Bool,
                    // String(_) => Self::String,
                    // Bytes(_) => Self::Bytes,
                    Handle8(_) => Self::Handle8,
                    Handle16(_) => Self::Handle16,
                    Handle32(_) => Self::Handle32,
                    Handle64(_) => Self::Handle64,
                    Handle128(_) => Self::Handle128,
                    // Uuid(_) => Self::Uuid,
                    F32(_) => Self::F32,
                    F64(_) => Self::F64,
                    I8(_) => Self::I8,
                    U8(_) => Self::U8,
                    I16(_) => Self::I16,
                    U16(_) => Self::U16,
                    I32(_) => Self::I32,
                    U32(_) => Self::U32,
                    I64(_) => Self::I64,
                    U64(_) => Self::U64,
                    I128(_) => Self::I128,
                    U128(_) => Self::U128,
                }
            }
        }
    };
}
impl_from_cell_data_to_type![];

#[cfg(test)]
mod tests {
    use super::{
        CategoricalType::Bool, CellData, CellType, CellTypeNested, ContinuousType::*,
        DiscreteType::*, NumericalType::*,
    };

    #[test]
    fn conversions() {
        // CellData to CellType
        assert_eq![CellType::Bool, CellData::Bool(true).into()];
        // assert_eq![CellType::String, CellData::String("hi".into()).into()];
        assert_eq![CellType::I8, CellData::I8(127).into()];
        assert_eq![CellType::F64, CellData::F64(0.04).into()];

        // CellType to CellTypeNested
        assert_eq![CellType::Bool, CellTypeNested::Categorical(Bool).into()];
        // assert_eq![CellType::String, CellTypeNested::Categorical(String).into()];
        assert_eq![
            CellType::I16,
            CellTypeNested::Numerical(Discrete(I16)).into()
        ];
        assert_eq![
            CellType::F32,
            CellTypeNested::Numerical(Continuous(F32)).into()
        ];
        // references
        assert_eq![CellType::Bool, (&CellTypeNested::Categorical(Bool)).into()];
        assert_eq![
            CellType::Bool,
            (&mut CellTypeNested::Categorical(Bool)).into()
        ];

        // CellTypeNested to CellType
        assert_eq![CellTypeNested::Categorical(Bool), CellType::Bool.into()];
        // assert_eq![CellTypeNested::Categorical(String), CellType::String.into()];
        assert_eq![
            CellTypeNested::Numerical(Discrete(I16)),
            CellType::I16.into()
        ];
        assert_eq![
            CellTypeNested::Numerical(Continuous(F32)),
            CellType::F32.into()
        ];
        // references
        assert_eq![CellTypeNested::Categorical(Bool), (&CellType::Bool).into()];
        assert_eq![
            CellTypeNested::Categorical(Bool),
            (&mut CellType::Bool).into()
        ];
    }
}
