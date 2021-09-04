//!

mod numerical;
pub use numerical::{FloatVec, IntVec, Numerical};

mod categorical;
pub use categorical::Categorical;

/// A column of data in the [`Table`] can be of two basic data types:
/// [`Numerical`] and [`Categorical`].
#[derive(Debug)]
pub enum Column {
    Numerical(Numerical),
    Categorical(Categorical),
}

impl From<Numerical> for Column {
    fn from(n: Numerical) -> Self {
        Column::Numerical(n)
    }
}

impl From<IntVec> for Column {
    fn from(iv: IntVec) -> Self {
        Column::Numerical(iv.into())
    }
}
impl From<FloatVec> for Column {
    fn from(fv: FloatVec) -> Self {
        Column::Numerical(fv.into())
    }
}

impl From<Vec<i8>> for Column {
    fn from(v: Vec<i8>) -> Self {
        Column::Numerical(v.into())
    }
}
impl From<Vec<u8>> for Column {
    fn from(v: Vec<u8>) -> Self {
        Column::Numerical(v.into())
    }
}
impl From<Vec<i16>> for Column {
    fn from(v: Vec<i16>) -> Self {
        Column::Numerical(v.into())
    }
}
impl From<Vec<u16>> for Column {
    fn from(v: Vec<u16>) -> Self {
        Column::Numerical(v.into())
    }
}
impl From<Vec<i32>> for Column {
    fn from(v: Vec<i32>) -> Self {
        Column::Numerical(v.into())
    }
}
impl From<Vec<u32>> for Column {
    fn from(v: Vec<u32>) -> Self {
        Column::Numerical(v.into())
    }
}
impl From<Vec<i64>> for Column {
    fn from(v: Vec<i64>) -> Self {
        Column::Numerical(v.into())
    }
}
impl From<Vec<u64>> for Column {
    fn from(v: Vec<u64>) -> Self {
        Column::Numerical(v.into())
    }
}
impl From<Vec<i128>> for Column {
    fn from(v: Vec<i128>) -> Self {
        Column::Numerical(v.into())
    }
}
impl From<Vec<u128>> for Column {
    fn from(v: Vec<u128>) -> Self {
        Column::Numerical(v.into())
    }
}

// --

impl From<Categorical> for Column {
    fn from(c: Categorical) -> Self {
        Column::Categorical(c)
    }
}

// impl From<Categorical> for Column {
//     fn from(c: Categorical) -> Self {
//         Column::Categorical(c.into())
//     }
// }
