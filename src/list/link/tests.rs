// ladata::list::link::tests
//!
//

use core::mem::size_of;

use super::doubly::*;

#[cfg(feature = "std")]
use crate::mem::Boxed;

// TODO: check padding for more elements
#[test]
fn sizes_raw() {
    /* 8-bit index list */

    assert_eq!(1, size_of::<DoublyLinked8Index>());

    // the size of a node is the sum of:
    // - the size of its 2 indexes (2 * 1)
    // - the size of T
    // - any extra padding (NOTE: may depend on the platform)
    assert_eq!(2 + 0 + 0, size_of::<DoublyLinked8Node::<()>>());
    assert_eq![2 + 1 + 0, size_of::<DoublyLinked8Node::<u8>>()];
    assert_eq![2 + 2 + 0, size_of::<DoublyLinked8Node::<u16>>()];
    assert_eq![2 + 4 + 2, size_of::<DoublyLinked8Node::<u32>>()];
    assert_eq![2 + 8 + 6, size_of::<DoublyLinked8Node::<u64>>()];
    assert_eq![2 + 16 + 6, size_of::<DoublyLinked8Node::<u128>>()];

    // the size of a list of 0 elements:
    assert_eq![3, size_of::<DoublyLinked8::<u8, (), 0>>()];
    assert_eq![4, size_of::<DoublyLinked8::<u16, (), 0>>()];
    assert_eq![4, size_of::<DoublyLinked8::<u32, (), 0>>()];
    assert_eq![8, size_of::<DoublyLinked8::<u64, (), 0>>()];
    assert_eq![8, size_of::<DoublyLinked8::<u128, (), 0>>()];

    // the size of a list of 1 element:
    assert_eq![3 + 1 + 2, size_of::<DoublyLinked8::<u8, (), 1>>()];
    assert_eq![4 + 2 + 2, size_of::<DoublyLinked8::<u16, (), 1>>()];
    assert_eq![4 + 4 + 4, size_of::<DoublyLinked8::<u32, (), 1>>()];
    assert_eq![8 + 8 + 8, size_of::<DoublyLinked8::<u64, (), 1>>()];
    assert_eq![8 + 16 + 8, size_of::<DoublyLinked8::<u128, (), 1>>()];

    /* 16-bit index list */

    assert_eq!(2, size_of::<DoublyLinked16Index>());

    // the size of a node is the sum of:
    // - the size of its 2 indexes (2 * 2)
    // - the size of T
    // - any extra padding (NOTE: may depend on the platform)
    assert_eq!(4 + 0, size_of::<DoublyLinked16Node::<()>>());
    assert_eq![4 + 1 + 1, size_of::<DoublyLinked16Node::<u8>>()];
    assert_eq![4 + 2 + 0, size_of::<DoublyLinked16Node::<u16>>()];
    assert_eq![4 + 4 + 0, size_of::<DoublyLinked16Node::<u32>>()];
    assert_eq![4 + 8 + 4, size_of::<DoublyLinked16Node::<u64>>()];
    assert_eq![4 + 16 + 4, size_of::<DoublyLinked16Node::<u128>>()];

    // the size of a list of 0 elements:
    assert_eq![6, size_of::<DoublyLinked16::<u8, (), 0>>()];
    assert_eq![6, size_of::<DoublyLinked16::<u16, (), 0>>()];
    assert_eq![8, size_of::<DoublyLinked16::<u32, (), 0>>()];
    assert_eq![8, size_of::<DoublyLinked16::<u64, (), 0>>()];
    assert_eq![8, size_of::<DoublyLinked16::<u128, (), 0>>()];

    // the size of a list of 1 element:
    assert_eq![6 + 1 + 5, size_of::<DoublyLinked16::<u8, (), 1>>()];
    assert_eq![6 + 2 + 4, size_of::<DoublyLinked16::<u16, (), 1>>()];
    assert_eq![8 + 4 + 4, size_of::<DoublyLinked16::<u32, (), 1>>()];
    assert_eq![8 + 8 + 8, size_of::<DoublyLinked16::<u64, (), 1>>()];
    assert_eq![8 + 16 + 8, size_of::<DoublyLinked16::<u128, (), 1>>()];

    // the size of a list of 10 elements:
    assert_eq![6 + (1 + 5) * 10, size_of::<DoublyLinked16::<u8, (), 10>>()];
    assert_eq![6 + (2 + 4) * 10, size_of::<DoublyLinked16::<u16, (), 10>>()];
    assert_eq![8 + (4 + 4) * 10, size_of::<DoublyLinked16::<u32, (), 10>>()];
    assert_eq![8 + (8 + 8) * 10, size_of::<DoublyLinked16::<u64, (), 10>>()];
    assert_eq![
        8 + (16 + 8) * 10,
        size_of::<DoublyLinked16::<u128, (), 10>>()
    ];

    /* 32-bit index list */

    assert_eq!(4, size_of::<DoublyLinked32Index>());

    assert_eq!(8 + 0 + 0, size_of::<DoublyLinked32Node::<()>>());
    assert_eq![8 + 1 + 3, size_of::<DoublyLinked32Node::<u8>>()];
    assert_eq![8 + 2 + 2, size_of::<DoublyLinked32Node::<u16>>()];
    assert_eq![8 + 4 + 0, size_of::<DoublyLinked32Node::<u32>>()];
    assert_eq![8 + 8 + 0, size_of::<DoublyLinked32Node::<u64>>()];
    assert_eq![8 + 16 + 0, size_of::<DoublyLinked32Node::<u128>>()];

    // the size of a list of 0 elements:
    assert_eq![12, size_of::<DoublyLinked32::<u8, (), 0>>()];
    assert_eq![12, size_of::<DoublyLinked32::<u16, (), 0>>()];
    assert_eq![12, size_of::<DoublyLinked32::<u32, (), 0>>()];
    assert_eq![16, size_of::<DoublyLinked32::<u64, (), 0>>()];
    assert_eq![16, size_of::<DoublyLinked32::<u128, (), 0>>()];

    // the size of a list of 1 element:
    assert_eq![12 + 1 + 11, size_of::<DoublyLinked32::<u8, (), 1>>()];
    assert_eq![12 + 2 + 10, size_of::<DoublyLinked32::<u16, (), 1>>()];
    assert_eq![12 + 4 + 8, size_of::<DoublyLinked32::<u32, (), 1>>()];
    assert_eq![16 + 8 + 8, size_of::<DoublyLinked32::<u64, (), 1>>()];
    assert_eq![16 + 16 + 8, size_of::<DoublyLinked32::<u128, (), 1>>()];

    /* misc. list sizes */

    // max 8-bit len with a byte per node occupies ± 0.75 KiB
    assert_eq![
        765,
        size_of::<DoublyLinked8::<u8, (), { u8::MAX as usize - 1 }>>()
    ];
    // to store one node more we need 16-bit indexes, occupping 1.5 KiB
    assert_eq![
        1536,
        size_of::<DoublyLinked16::<u8, (), { u8::MAX as usize }>>()
    ];
    // max 16-bit len with a byte per node occupies ± 384 KiB)
    assert_eq![
        393_210,
        size_of::<DoublyLinked16::<u8, (), { u16::MAX as usize - 1 }>>()
    ];
    // to store one node more we need 32-bit indexes, occupping 768 KiB
    assert_eq![
        786_432,
        size_of::<DoublyLinked32::<u8, (), { u16::MAX as usize }>>()
    ];
    // max 32-bit len with a byte per node occupies ± 48 GiB)
    assert_eq![
        51_539_607_540,
        size_of::<DoublyLinked32::<u8, (), { u32::MAX as usize - 1 }>>()
    ];
}

#[test]
#[cfg(feature = "std")]
fn sizes_boxed() {
    // on the heap
    assert_eq![16, size_of::<DoublyLinked8::<u8, Boxed, 10>>()];
    assert_eq![16, size_of::<DoublyLinked8::<u128, Boxed, 10>>()];
    assert_eq![16, size_of::<DoublyLinked16::<u8, Boxed, 10>>()];
    assert_eq![16, size_of::<DoublyLinked16::<u128, Boxed, 10>>()];
    assert_eq![24, size_of::<DoublyLinked32::<u8, Boxed, 10>>()];
    assert_eq![24, size_of::<DoublyLinked32::<u128, Boxed, 10>>()];
}
