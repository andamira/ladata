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
