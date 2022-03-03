// ladata::structures::cell
//!
//
// TODO:
// - Design a good API of DataCell*, symmetric to DataType.
// - traits
//   - find the right name for the trait DataCells, IntoDataCell
//     (compare with DataTypes mechanics)
//
//

// mod nested;
// pub use nested::{DataCellNested};

// use crate::structures::Data0D;


use crate::{
    types::DataTypes,
    structures::DataStructures,
};

///
// RETHINK
pub enum DataCell<T: DataTypes> {
    Boo
}


// WIP make a macro
mod subcells {
    // pub enum 
}
