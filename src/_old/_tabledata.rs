//! experiment...
//!
//!
//! Trying to have a separate type
//! with:
//! - columns, each can be a separate type.
//! - header
//!
//! I NEED:
//!

#![allow(unused_variables,dead_code)]

pub struct Table {
    header: Vec<String>,
    cols: Vec<Vec<ColType>>
}


// try 1:
//
// pub enum ColType {
//     Int(i64)
// }


// try 2:
pub enum ColType {
    Numeric(ColNumeric),
    Categorical(ColCategorical),
}
pub enum ColNumeric {
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
    F32,
    F64,
}
pub enum ColCategorical {
    String,
    Bool,
    Other,
}


// the IDEA is to implement this trait for i32, i64, etc…
// MAYBE there are better ideas
pub trait TableData {}


#[cfg(test)]
mod test {

    #[test]
    fn table1() {

        // table
    }
}
