// ladata::unit::tests
//
//!

use crate::all::*;
use core::mem::size_of;

/// Checks the sizes of types without an active `With` variant.
#[test]
fn test_sizes_without() {
    assert_eq![1, size_of::<DataType8bit>()];
    assert_eq![2, size_of::<DataCell8bit>()];
    assert_eq![2, size_of::<DataCell8bitCopy>()];
    assert_eq![1, size_of::<DataBare8bitCopy>()];

    assert_eq![1, size_of::<DataType16bit>()];
    assert_eq![4, size_of::<DataCell16bit>()];
    assert_eq![4, size_of::<DataCell16bitCopy>()];
    assert_eq![2, size_of::<DataBare16bitCopy>()];

    assert_eq![1, size_of::<DataType32bit>()];
    assert_eq![8, size_of::<DataCell32bit>()];
    assert_eq![8, size_of::<DataCell32bitCopy>()];
    assert_eq![4, size_of::<DataBare32bitCopy>()];

    assert_eq![1, size_of::<DataType64bit>()];

    assert_eq![16, size_of::<DataCell64bitCopy>()];
    assert_eq![8, size_of::<DataBare64bitCopy>()];

    assert_eq![1, size_of::<DataType128bit>()];
    assert_eq![24, size_of::<DataCell128bit>()]; // +0%
    assert_eq![24, size_of::<DataCell128bitCopy>()];
    assert_eq![16, size_of::<DataBare128bitCopy>()];

    assert_eq![1, size_of::<DataType256bit>()];
    assert_eq![40, size_of::<DataCell256bit>()]; // + 60% size
    assert_eq![40, size_of::<DataCell256bitCopy>()];
    assert_eq![32, size_of::<DataBare256bitCopy>()];

    assert_eq![1, size_of::<DataType512bit>()];
    assert_eq![72, size_of::<DataCell512bit>()];
    assert_eq![72, size_of::<DataCell512bitCopy>()];
    assert_eq![64, size_of::<DataBare512bitCopy>()];

    assert_eq![1, size_of::<DataType1024bit>()];
    assert_eq![136, size_of::<DataCell1024bit>()];
    assert_eq![136, size_of::<DataCell1024bitCopy>()];
    assert_eq![128, size_of::<DataBare1024bitCopy>()];
}
