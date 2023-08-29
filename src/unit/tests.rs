// ladata::unit::tests
//
//!

use crate::all::*;
use core::mem::size_of;

/// Checks the sizes of types without an active `With` variant.
#[test]
fn test_sizes_without() {
    assert_eq![1, size_of::<DataType8bit>()];
    assert_eq![2, size_of::<DataUnit8bit>()];
    assert_eq![2, size_of::<DataUnit8bitCopy>()];

    assert_eq![1, size_of::<DataType16bit>()];
    assert_eq![4, size_of::<DataUnit16bit>()];
    assert_eq![4, size_of::<DataUnit16bitCopy>()];

    assert_eq![1, size_of::<DataType32bit>()];
    assert_eq![8, size_of::<DataUnit32bit>()];
    assert_eq![8, size_of::<DataUnit32bitCopy>()];

    assert_eq![1, size_of::<DataType64bit>()];

    assert_eq![16, size_of::<DataUnit64bitCopy>()];

    assert_eq![1, size_of::<DataType128bit>()];
    assert_eq![24, size_of::<DataUnit128bit>()]; // +0%
    assert_eq![24, size_of::<DataUnit128bitCopy>()];

    assert_eq![1, size_of::<DataType256bit>()];
    assert_eq![40, size_of::<DataUnit256bit>()]; // + 60% size
    assert_eq![40, size_of::<DataUnit256bitCopy>()];

    assert_eq![1, size_of::<DataType512bit>()];
    assert_eq![72, size_of::<DataUnit512bit>()];
    assert_eq![72, size_of::<DataUnit512bitCopy>()];

    assert_eq![1, size_of::<DataType1024bit>()];
    assert_eq![136, size_of::<DataUnit1024bit>()];
    assert_eq![136, size_of::<DataUnit1024bitCopy>()];
}

#[test]
#[cfg(feature = "unsafe_unit")]
fn test_unsafe_sizes_without() {
    assert_eq![1, size_of::<DataRaw8bitCopy>()];
    assert_eq![2, size_of::<DataRaw16bitCopy>()];
    assert_eq![4, size_of::<DataRaw32bitCopy>()];
    assert_eq![8, size_of::<DataRaw64bitCopy>()];
    assert_eq![16, size_of::<DataRaw128bitCopy>()];
    assert_eq![32, size_of::<DataRaw256bitCopy>()];
    assert_eq![64, size_of::<DataRaw512bitCopy>()];
    assert_eq![128, size_of::<DataRaw1024bitCopy>()];
}
