// ladata::tests
//
// TODO
// - check alignment
// - check nesting (With)
//   - add sizes of nested...

use crate::all::*;
use core::mem::{size_of, size_of_val};

#[test]
fn test_sizes() {
    assert_eq![1, size_of::<DataType1Byte>()];
    assert_eq![2, size_of::<DataCell1ByteCopy>()];
    assert_eq![8, size_of::<DataCell1Byte>()]; // <<< non-copy size
    assert_eq![1, size_of::<DataUnsafeCell1ByteCopy>()];

    assert_eq![1, size_of::<DataType2Byte>()];
    assert_eq![4, size_of::<DataCell2ByteCopy>()]; // + 100% size
    assert_eq![24, size_of::<DataCell2Byte>()]; // <<< non-copy size
    assert_eq![2, size_of::<DataUnsafeCell2ByteCopy>()];

    assert_eq![1, size_of::<DataType4Byte>()];
    assert_eq![8, size_of::<DataCell4ByteCopy>()]; // +100% size
    assert_eq![72, size_of::<DataCell4Byte>()]; // <<< non-copy size
    assert_eq![4, size_of::<DataUnsafeCell4ByteCopy>()];

    assert_eq![1, size_of::<DataType8Byte>()];
    assert_eq![16, size_of::<DataCell8ByteCopy>()]; // +100% size
    assert_eq![72, size_of::<DataCell8Byte>()]; // <<<
    assert_eq![8, size_of::<DataUnsafeCell8ByteCopy>()];

    assert_eq![1, size_of::<DataType16Byte>()];
    assert_eq![24, size_of::<DataCell16ByteCopy>()]; // +50% size
    assert_eq![72, size_of::<DataCell16Byte>()]; // <<<
    assert_eq![16, size_of::<DataUnsafeCell16ByteCopy>()];

    assert_eq![1, size_of::<DataType32Byte>()];
    assert_eq![40, size_of::<DataCell32ByteCopy>()]; // + 60%
    assert_eq![72, size_of::<DataCell32Byte>()]; // <<<
    assert_eq![32, size_of::<DataUnsafeCell32ByteCopy>()];

    // assert_eq![1, size_of::<DataType64Byte>()];
    // assert_eq![32, size_of::<DataCell64Byte>()];
    // assert_eq![16, size_of::<DataUnsafeCell64ByteCopy>()];
    //
    // assert_eq![1, size_of::<DataType128Byte>()];
    // assert_eq![32, size_of::<DataCell128Byte>()];
    // assert_eq![16, size_of::<DataUnsafeCell128ByteCopy>()];

    //mix WIP
    // assert_eq![1, std::mem::size_of::<MyDataCell8bitWith<MyDataCell8bit>>()];
    // assert_eq![1, std::mem::size_of::<MyDataCell8bitWith<MyDataUnsafeCell8bit>>()];
}
