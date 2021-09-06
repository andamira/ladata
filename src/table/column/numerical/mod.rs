//!

mod float;
mod integer;

pub use float::{FloatVec, FloatVecItem, FloatVecIterator};
pub use integer::{IntVec, IntVecItem, IntVecIterator};

/// A numerical data type can be measured (AKA *quantitative*).
///
/// Examples: Age, Height, purchases per month…
#[derive(Debug)]
pub enum Numerical {
    Int(IntVec),
    Float(FloatVec),
}

// ---

impl From<FloatVec> for Numerical {
    fn from(fv: FloatVec) -> Self {
        Numerical::Float(fv)
    }
}
impl From<Vec<f32>> for Numerical {
    fn from(v: Vec<f32>) -> Self {
        Numerical::Float(v.into())
    }
}
impl From<Vec<f64>> for Numerical {
    fn from(v: Vec<f64>) -> Self {
        Numerical::Float(v.into())
    }
}

// ---

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
