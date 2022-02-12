// ladata::frame::column::tests
//
//
//!
//!
//
#![cfg(test)]

use crate::cell::{CellData, CellType};
use crate::error::DataResult;
use super::Column;

#[test]
fn new_empty() -> DataResult<()> {
    let empty_u8 = Column::<CellData>::new_empty(CellType::U8);
    assert_eq![empty_u8.cell_type(), CellType::U8];
    assert_eq![empty_u8.len(), 0];
    assert![empty_u8.is_empty()];
    Ok(())
}

#[test]
fn from_iter() -> DataResult<()> {
    let bools = Column::<CellData>::from_iter(&[true, false])?;
    assert_eq![2, bools.len()];
    assert_eq![CellType::Bool, bools.cell_type()];
    assert![!bools.is_empty()];

    let floats = Column::<CellData>::from_iter([0.4_f32])?;
    assert_eq![1, floats.len()];
    assert_eq![CellType::F32, floats.cell_type()];

    let ints = Column::<CellData>::from_iter(vec![4 ^ 10, 0, -8192])?;
    assert_eq![3, ints.len()];
    assert_eq![CellType::I32, ints.cell_type()];

    // TODO: Impl CellAble
    // assert_eq![CellType::from(0_i32), ints.cell_type()];
    // assert_eq![0_i32.cell_type(), ints.cell_type()];

    Ok(())
}
