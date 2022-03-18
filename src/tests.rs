// ladata::tests
//
//!

use crate::all::*;
use core::mem::{size_of, size_of_val};

/// Checks the sizes of types without an active `With` variant.
#[test]
fn test_sizes_without() {
    assert_eq![1, size_of::<DataType1Byte>()];
    assert_eq![2, size_of::<DataUnit1Byte>()];
    assert_eq![2, size_of::<DataUnit1ByteCopy>()];
    assert_eq![1, size_of::<DataLone1ByteCopy>()];

    assert_eq![1, size_of::<DataType2Byte>()];
    assert_eq![4, size_of::<DataUnit2Byte>()];
    assert_eq![4, size_of::<DataUnit2ByteCopy>()];
    assert_eq![2, size_of::<DataLone2ByteCopy>()];

    assert_eq![1, size_of::<DataType4Byte>()];
    assert_eq![8, size_of::<DataUnit4Byte>()];
    assert_eq![8, size_of::<DataUnit4ByteCopy>()];
    assert_eq![4, size_of::<DataLone4ByteCopy>()];

    assert_eq![1, size_of::<DataType8Byte>()];

    #[cfg(not(feature = "softposit"))]
    assert_eq![16, size_of::<DataUnit8Byte>()];
    #[cfg(feature = "softposit")]
    assert_eq![24, size_of::<DataUnit8Byte>()]; // Q16

    assert_eq![16, size_of::<DataUnit8ByteCopy>()];
    assert_eq![8, size_of::<DataLone8ByteCopy>()];

    assert_eq![1, size_of::<DataType16Byte>()];
    assert_eq![24, size_of::<DataUnit16Byte>()]; // +0%
    assert_eq![24, size_of::<DataUnit16ByteCopy>()];
    assert_eq![16, size_of::<DataLone16ByteCopy>()];

    assert_eq![1, size_of::<DataType32Byte>()];
    assert_eq![40, size_of::<DataUnit32Byte>()]; // + 60% size
    assert_eq![40, size_of::<DataUnit32ByteCopy>()];
    assert_eq![32, size_of::<DataLone32ByteCopy>()];

    assert_eq![1, size_of::<DataType64Byte>()];
    assert_eq![72, size_of::<DataUnit64Byte>()];
    assert_eq![72, size_of::<DataUnit64ByteCopy>()];
    assert_eq![64, size_of::<DataLone64ByteCopy>()];

    assert_eq![1, size_of::<DataType128Byte>()];
    assert_eq![136, size_of::<DataUnit128Byte>()];
    assert_eq![136, size_of::<DataUnit128ByteCopy>()];
    assert_eq![128, size_of::<DataLone128ByteCopy>()];
}
