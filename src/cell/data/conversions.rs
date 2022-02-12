// ladata::cell::data::conversions
//
//
//!
//!
//

use crate::cell::{
    nested::{
        CategoricalData, CellDataNested, ContinuousData, DiscreteData, IdData, NumericalData,
    },
    CellData,
};

/// impl From<[&|mut]CellData> for CellDataNested
macro_rules! impl_from_cell_data_to_nested {
    () => {
        impl_from_cell_data_to_nested![from: CellData];
        impl_from_cell_data_to_nested![from_ref: &CellData];
        impl_from_cell_data_to_nested![from_ref: &mut CellData];
    };

    (from: $t1:ty) => {
        impl From<$t1> for CellDataNested {
            fn from(cell_data: $t1) -> CellDataNested {
                use {
                    CategoricalData::*, CellDataNested::*, ContinuousData::*, DiscreteData::*,
                    IdData::*, NumericalData::*,
                };
                match cell_data {
                    CellData::Bool(d) => Categorical(Bool(d)),
                    // CellData::String(d) => Categorical(String(d)),
                    // CellData::Bytes(d) => Categorical(Bytes(d)),
                    CellData::Handle8(d) => Categorical(Id(Handle8(d))),
                    CellData::Handle16(d) => Categorical(Id(Handle16(d))),
                    CellData::Handle32(d) => Categorical(Id(Handle32(d))),
                    CellData::Handle64(d) => Categorical(Id(Handle64(d))),
                    CellData::Handle128(d) => Categorical(Id(Handle128(d))),
                    // CellData::Uuid(d) => Categorical(Id(Uuid(d))),
                    CellData::F32(d) => Numerical(Continuous(F32(d))),
                    CellData::F64(d) => Numerical(Continuous(F64(d))),
                    CellData::I8(d) => Numerical(Discrete(I8(d))),
                    CellData::U8(d) => Numerical(Discrete(U8(d))),
                    CellData::I16(d) => Numerical(Discrete(I16(d))),
                    CellData::U16(d) => Numerical(Discrete(U16(d))),
                    CellData::I32(d) => Numerical(Discrete(I32(d))),
                    CellData::U32(d) => Numerical(Discrete(U32(d))),
                    CellData::I64(d) => Numerical(Discrete(I64(d))),
                    CellData::U64(d) => Numerical(Discrete(U64(d))),
                    CellData::I128(d) => Numerical(Discrete(I128(d))),
                    CellData::U128(d) => Numerical(Discrete(U128(d))),
                }
            }
        }
    };

    (from_ref: $t1:ty) => {
        impl From<$t1> for CellDataNested {
            fn from(cell_data: $t1) -> CellDataNested {
                use {
                    CategoricalData::*, CellDataNested::*, ContinuousData::*, DiscreteData::*,
                    IdData::*, NumericalData::*,
                };

                match cell_data {
                    CellData::Bool(d) => Categorical(Bool(*d)),
                    // CellData::String(d) => Categorical(String(*d)),
                    // CellData::Bytes(d) => Categorical(Bytes(*d)),
                    CellData::Handle8(d) => Categorical(Id(Handle8(*d))),
                    CellData::Handle16(d) => Categorical(Id(Handle16(*d))),
                    CellData::Handle32(d) => Categorical(Id(Handle32(*d))),
                    CellData::Handle64(d) => Categorical(Id(Handle64(*d))),
                    CellData::Handle128(d) => Categorical(Id(Handle128(*d))),
                    // CellData::Uuid(d) => Categorical(Id(Uuid(*d))),
                    CellData::F32(d) => Numerical(Continuous(F32(*d))),
                    CellData::F64(d) => Numerical(Continuous(F64(*d))),
                    CellData::I8(d) => Numerical(Discrete(I8(*d))),
                    CellData::U8(d) => Numerical(Discrete(U8(*d))),
                    CellData::I16(d) => Numerical(Discrete(I16(*d))),
                    CellData::U16(d) => Numerical(Discrete(U16(*d))),
                    CellData::I32(d) => Numerical(Discrete(I32(*d))),
                    CellData::U32(d) => Numerical(Discrete(U32(*d))),
                    CellData::I64(d) => Numerical(Discrete(I64(*d))),
                    CellData::U64(d) => Numerical(Discrete(U64(*d))),
                    CellData::I128(d) => Numerical(Discrete(I128(*d))),
                    CellData::U128(d) => Numerical(Discrete(U128(*d))),
                }
            }
        }
    };
}
impl_from_cell_data_to_nested![];

/// impl From<[&|mut]CellTypeNested> for CellData
macro_rules! impl_from_cell_data_nested_to_flat {
    () => {
        impl_from_cell_data_nested_to_flat![from: CellDataNested];
        impl_from_cell_data_nested_to_flat![from_ref: &CellDataNested];
        impl_from_cell_data_nested_to_flat![from_ref: &mut CellDataNested];
    };
    (from: $t1:ty) => {
        impl From<$t1> for CellData {
            fn from(cell_data_nested: $t1) -> CellData {
                use {
                    CategoricalData::*, CellDataNested::*, ContinuousData::*, DiscreteData::*,
                    IdData::*, NumericalData::*,
                };

                match cell_data_nested {
                    Categorical(c) => match c {
                        Bool(d) => Self::Bool(d),
                        // String(d) => Self::String(d),
                        // Bytes(d) => Self::Bytes(d),
                        Id(i) => match i {
                            Handle8(d) => Self::Handle8(d),
                            Handle16(d) => Self::Handle16(d),
                            Handle32(d) => Self::Handle32(d),
                            Handle64(d) => Self::Handle64(d),
                            Handle128(d) => Self::Handle128(d),
                            // Uuid(d) => Self::Uuid(d),
                        },
                    },
                    Numerical(n) => match n {
                        Continuous(f) => match f {
                            F32(d) => Self::F32(d),
                            F64(d) => Self::F64(d),
                        },
                        Discrete(i) => match i {
                            I8(d) => Self::I8(d),
                            U8(d) => Self::U8(d),
                            I16(d) => Self::I16(d),
                            U16(d) => Self::U16(d),
                            I32(d) => Self::I32(d),
                            U32(d) => Self::U32(d),
                            I64(d) => Self::I64(d),
                            U64(d) => Self::U64(d),
                            I128(d) => Self::I128(d),
                            U128(d) => Self::U128(d),
                        },
                    },
                }
            }
        }
    };

    (from_ref: $t1:ty) => {
        impl From<$t1> for CellData {
            fn from(cell_data_nested: $t1) -> CellData {
                use {
                    CategoricalData::*, CellDataNested::*, ContinuousData::*, DiscreteData::*,
                    IdData::*, NumericalData::*,
                };

                match cell_data_nested {
                    Categorical(c) => match c {
                        Bool(d) => Self::Bool(*d),
                        // String(d) => Self::String(*d),
                        // Bytes(d) => Self::Bytes(*d),
                        Id(i) => match i {
                            Handle8(d) => Self::Handle8(*d),
                            Handle16(d) => Self::Handle16(*d),
                            Handle32(d) => Self::Handle32(*d),
                            Handle64(d) => Self::Handle64(*d),
                            Handle128(d) => Self::Handle128(*d),
                            // Uuid(d) => Self::Uuid(*d),
                        },
                    },
                    Numerical(n) => match n {
                        Continuous(f) => match f {
                            F32(d) => Self::F32(*d),
                            F64(d) => Self::F64(*d),
                        },
                        Discrete(i) => match i {
                            I8(d) => Self::I8(*d),
                            U8(d) => Self::U8(*d),
                            I16(d) => Self::I16(*d),
                            U16(d) => Self::U16(*d),
                            I32(d) => Self::I32(*d),
                            U32(d) => Self::U32(*d),
                            I64(d) => Self::I64(*d),
                            U64(d) => Self::U64(*d),
                            I128(d) => Self::I128(*d),
                            U128(d) => Self::U128(*d),
                        },
                    },
                }
            }
        }
    };
}
impl_from_cell_data_nested_to_flat![];

#[cfg(test)]
mod tests {
    use super::{
        CategoricalData::*, CellData, CellDataNested::*, ContinuousData::*, DiscreteData::*,
        IdData::*, NumericalData::*,
    };
    use crate::ids;

    #[test]
    fn conversions() {
        // CellData to CellDataNested:

        assert_eq![Categorical(Bool(true)), CellData::Bool(true).into()];
        // assert_eq![CellData::String, CellData::String("hi".into()).into()];
        assert_eq![
            Categorical(Id(Handle8(ids::Handle8::new(7)))),
            CellData::Handle8(ids::Handle8::new(7)).into()
        ];
        assert_eq![Numerical(Discrete(I8(127))), CellData::I8(127).into()];
        assert_eq![Numerical(Continuous(F64(0.04))), CellData::F64(0.04).into()];

        // CellDataNested to CellData:

        assert_eq![CellData::Bool(true), Categorical(Bool(true)).into()];
        // assert_eq![CellData::String("hi".into()), CellData::String.into()];
        assert_eq![
            CellData::Handle8(ids::Handle8::new(7)),
            Categorical(Id(Handle8(ids::Handle8::new(7)))).into()
        ];
        assert_eq![CellData::I8(127), Numerical(Discrete(I8(127))).into()];
        assert_eq![CellData::F64(0.04), Numerical(Continuous(F64(0.04))).into()];
    }
}
