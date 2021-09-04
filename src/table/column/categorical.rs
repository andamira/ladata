//!
//!
// TODO: Any dynamic type, any user-provided enum.

/// A categorical data type is one that can not be measured (AKA *qualitative*).
///
/// Examples: Gender, Opinion, Rating…
///
/// Note that In practice it's possible to handle some categorical data as if
/// it were numerical, e.g. associating ratings to numbers, and so on.
#[derive(Debug)]
#[non_exhaustive]
pub enum Categorical {
    Bool(Vec<bool>),
    String(Vec<String>),
    // Other(Vec<_>),
}

impl From<Vec<String>> for Categorical {
    fn from(vs: Vec<String>) -> Self {
        Categorical::String(vs)
    }
}
impl From<Vec<bool>> for Categorical {
    fn from(vb: Vec<bool>) -> Self {
        Categorical::Bool(vb)
    }
}
