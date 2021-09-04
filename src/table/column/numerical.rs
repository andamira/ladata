//!

/// A numerical data type can be measured (AKA *quantitative*).
///
/// Examples: Age, Height, purchases per month…
#[derive(Debug)]
pub enum Numerical {
    Int(IntVec),
    Float(FloatVec),
}

impl From<FloatVec> for Numerical {
    fn from(fv: FloatVec) -> Self {
        Numerical::Float(fv)
    }
}
impl From<IntVec> for Numerical {
    fn from(iv: IntVec) -> Self {
        Numerical::Int(iv)
    }
}

impl From<Vec<i8>> for Numerical {
    fn from(v: Vec<i8>) -> Self {
        Numerical::Int(v.into())
    }
}
impl From<Vec<u8>> for Numerical {
    fn from(v: Vec<u8>) -> Self {
        Numerical::Int(v.into())
    }
}
impl From<Vec<i16>> for Numerical {
    fn from(v: Vec<i16>) -> Self {
        Numerical::Int(v.into())
    }
}
impl From<Vec<u16>> for Numerical {
    fn from(v: Vec<u16>) -> Self {
        Numerical::Int(v.into())
    }
}
impl From<Vec<i32>> for Numerical {
    fn from(v: Vec<i32>) -> Self {
        Numerical::Int(v.into())
    }
}
impl From<Vec<u32>> for Numerical {
    fn from(v: Vec<u32>) -> Self {
        Numerical::Int(v.into())
    }
}
impl From<Vec<i64>> for Numerical {
    fn from(v: Vec<i64>) -> Self {
        Numerical::Int(v.into())
    }
}
impl From<Vec<u64>> for Numerical {
    fn from(v: Vec<u64>) -> Self {
        Numerical::Int(v.into())
    }
}
impl From<Vec<i128>> for Numerical {
    fn from(v: Vec<i128>) -> Self {
        Numerical::Int(v.into())
    }
}
impl From<Vec<u128>> for Numerical {
    fn from(v: Vec<u128>) -> Self {
        Numerical::Int(v.into())
    }
}

///
#[derive(Debug)]
pub enum FloatVec {
    F32(Vec<f32>),
    F64(Vec<f64>),
}
impl From<Vec<f32>> for FloatVec {
    fn from(v: Vec<f32>) -> Self {
        FloatVec::F32(v)
    }
}
impl From<Vec<f64>> for FloatVec {
    fn from(v: Vec<f64>) -> Self {
        FloatVec::F64(v)
    }
}

///
#[derive(Debug)]
pub enum IntVec {
    I8(Vec<i8>),
    U8(Vec<u8>),
    I16(Vec<i16>),
    U16(Vec<u16>),
    I32(Vec<i32>),
    U32(Vec<u32>),
    I64(Vec<i64>),
    U64(Vec<u64>),
    I128(Vec<i128>),
    U128(Vec<u128>),
}
impl From<Vec<i8>> for IntVec {
    fn from(v: Vec<i8>) -> Self {
        IntVec::I8(v)
    }
}
impl From<Vec<u8>> for IntVec {
    fn from(v: Vec<u8>) -> Self {
        IntVec::U8(v)
    }
}
impl From<Vec<i16>> for IntVec {
    fn from(v: Vec<i16>) -> Self {
        IntVec::I16(v)
    }
}
impl From<Vec<u16>> for IntVec {
    fn from(v: Vec<u16>) -> Self {
        IntVec::U16(v)
    }
}
impl From<Vec<i32>> for IntVec {
    fn from(v: Vec<i32>) -> Self {
        IntVec::I32(v)
    }
}
impl From<Vec<u32>> for IntVec {
    fn from(v: Vec<u32>) -> Self {
        IntVec::U32(v)
    }
}
impl From<Vec<i64>> for IntVec {
    fn from(v: Vec<i64>) -> Self {
        IntVec::I64(v)
    }
}
impl From<Vec<u64>> for IntVec {
    fn from(v: Vec<u64>) -> Self {
        IntVec::U64(v)
    }
}
impl From<Vec<i128>> for IntVec {
    fn from(v: Vec<i128>) -> Self {
        IntVec::I128(v)
    }
}
impl From<Vec<u128>> for IntVec {
    fn from(v: Vec<u128>) -> Self {
        IntVec::U128(v)
    }
}
