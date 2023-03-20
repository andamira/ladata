// ladata::list::link::tests
//!
//

use core::mem::size_of;

use super::{doubly::*, *};

// use crate::error::LadataError as Error;

#[cfg(feature = "alloc")]
use crate::mem::Boxed;

// TODO: check padding for more elements
#[test]
#[rustfmt::skip]
fn doubly_linked_sizes_raw() {
    /* 8-bit index list */

    // the size of a node is the sum of:
    // - the size of its 2 indexes (2 * 1)
    // - the size of T
    // - any extra padding (NOTE: may depend on the platform)
    assert_eq!(2 + 0 + 0, size_of::<DoublyLinkedList8Node::<()>>());
    assert_eq![2 + 1 + 0, size_of::<DoublyLinkedList8Node::<u8>>()];
    assert_eq![2 + 2 + 0, size_of::<DoublyLinkedList8Node::<u16>>()];
    assert_eq![2 + 4 + 2, size_of::<DoublyLinkedList8Node::<u32>>()];
    assert_eq![2 + 8 + 6, size_of::<DoublyLinkedList8Node::<u64>>()];
    assert_eq![2 + 16 + 6, size_of::<DoublyLinkedList8Node::<u128>>()];

    // the size of a list of 0 elements:
    assert_eq![3, size_of::<DoublyLinkedList8::<u8, (), 0>>()];
    assert_eq![4, size_of::<DoublyLinkedList8::<u16, (), 0>>()];
    assert_eq![4, size_of::<DoublyLinkedList8::<u32, (), 0>>()];
    assert_eq![8, size_of::<DoublyLinkedList8::<u64, (), 0>>()];
    assert_eq![8, size_of::<DoublyLinkedList8::<u128, (), 0>>()];

    // the size of a list of 1 element:
    assert_eq![3 + 1 + 2, size_of::<DoublyLinkedList8::<u8, (), 1>>()];
    assert_eq![4 + 2 + 2, size_of::<DoublyLinkedList8::<u16, (), 1>>()];
    assert_eq![4 + 4 + 4, size_of::<DoublyLinkedList8::<u32, (), 1>>()];
    assert_eq![8 + 8 + 8, size_of::<DoublyLinkedList8::<u64, (), 1>>()];
    assert_eq![8 + 16 + 8, size_of::<DoublyLinkedList8::<u128, (), 1>>()];

    /* 16-bit index list */

    // the size of a node is the sum of:
    // - the size of its 2 indexes (2 * 2)
    // - the size of T
    // - any extra padding (NOTE: may depend on the platform)
    assert_eq!(4 + 0, size_of::<DoublyLinkedList16Node::<()>>());
    assert_eq![4 + 1 + 1, size_of::<DoublyLinkedList16Node::<u8>>()];
    assert_eq![4 + 2 + 0, size_of::<DoublyLinkedList16Node::<u16>>()];
    assert_eq![4 + 4 + 0, size_of::<DoublyLinkedList16Node::<u32>>()];
    assert_eq![4 + 8 + 4, size_of::<DoublyLinkedList16Node::<u64>>()];
    assert_eq![4 + 16 + 4, size_of::<DoublyLinkedList16Node::<u128>>()];

    // the size of a list of 0 elements:
    assert_eq![6, size_of::<DoublyLinkedList16::<u8, (), 0>>()];
    assert_eq![6, size_of::<DoublyLinkedList16::<u16, (), 0>>()];
    assert_eq![8, size_of::<DoublyLinkedList16::<u32, (), 0>>()];
    assert_eq![8, size_of::<DoublyLinkedList16::<u64, (), 0>>()];
    assert_eq![8, size_of::<DoublyLinkedList16::<u128, (), 0>>()];

    // the size of a list of 1 element:
    assert_eq![6 + 1 + 5, size_of::<DoublyLinkedList16::<u8, (), 1>>()];
    assert_eq![6 + 2 + 4, size_of::<DoublyLinkedList16::<u16, (), 1>>()];
    assert_eq![8 + 4 + 4, size_of::<DoublyLinkedList16::<u32, (), 1>>()];
    assert_eq![8 + 8 + 8, size_of::<DoublyLinkedList16::<u64, (), 1>>()];
    assert_eq![8 + 16 + 8, size_of::<DoublyLinkedList16::<u128, (), 1>>()];

    // the size of a list of 10 elements:
    assert_eq![6 + (1 + 5) * 10, size_of::<DoublyLinkedList16::<u8, (), 10>>()];
    assert_eq![6 + (2 + 4) * 10, size_of::<DoublyLinkedList16::<u16, (), 10>>()];
    assert_eq![8 + (4 + 4) * 10, size_of::<DoublyLinkedList16::<u32, (), 10>>()];
    assert_eq![8 + (8 + 8) * 10, size_of::<DoublyLinkedList16::<u64, (), 10>>()];
    assert_eq![8 + (16 + 8) * 10, size_of::<DoublyLinkedList16::<u128, (), 10>>()];

    /* 32-bit index list */

    assert_eq!(8 + 0 + 0, size_of::<DoublyLinkedList32Node::<()>>());
    assert_eq![8 + 1 + 3, size_of::<DoublyLinkedList32Node::<u8>>()];
    assert_eq![8 + 2 + 2, size_of::<DoublyLinkedList32Node::<u16>>()];
    assert_eq![8 + 4 + 0, size_of::<DoublyLinkedList32Node::<u32>>()];
    assert_eq![8 + 8 + 0, size_of::<DoublyLinkedList32Node::<u64>>()];
    assert_eq![8 + 16 + 0, size_of::<DoublyLinkedList32Node::<u128>>()];

    // the size of a list of 0 elements:
    assert_eq![12, size_of::<DoublyLinkedList32::<u8, (), 0>>()];
    assert_eq![12, size_of::<DoublyLinkedList32::<u16, (), 0>>()];
    assert_eq![12, size_of::<DoublyLinkedList32::<u32, (), 0>>()];
    assert_eq![16, size_of::<DoublyLinkedList32::<u64, (), 0>>()];
    assert_eq![16, size_of::<DoublyLinkedList32::<u128, (), 0>>()];

    // the size of a list of 1 element:
    assert_eq![12 + 1 + 11, size_of::<DoublyLinkedList32::<u8, (), 1>>()];
    assert_eq![12 + 2 + 10, size_of::<DoublyLinkedList32::<u16, (), 1>>()];
    assert_eq![12 + 4 + 8, size_of::<DoublyLinkedList32::<u32, (), 1>>()];
    assert_eq![16 + 8 + 8, size_of::<DoublyLinkedList32::<u64, (), 1>>()];
    assert_eq![16 + 16 + 8, size_of::<DoublyLinkedList32::<u128, (), 1>>()];

    /* misc. list sizes */

    // max 8-bit len with a byte per node occupies ± 0.75 KiB
    assert_eq![
        765,
        size_of::<DoublyLinkedList8::<u8, (), { u8::MAX as usize - 1 }>>()
    ];
    // to store one node more we need 16-bit indexes, occupping 1.5 KiB
    assert_eq![
        1536,
        size_of::<DoublyLinkedList16::<u8, (), { u8::MAX as usize }>>()
    ];
    // max 16-bit len with a byte per node occupies ± 384 KiB)
    assert_eq![
        393_210,
        size_of::<DoublyLinkedList16::<u8, (), { u16::MAX as usize - 1 }>>()
    ];
    // to store one node more we need 32-bit indexes, occupping 768 KiB
    assert_eq![
        786_432,
        size_of::<DoublyLinkedList32::<u8, (), { u16::MAX as usize }>>()
    ];
    // max 32-bit len with a byte per node occupies ± 48 GiB)
    assert_eq![
        51_539_607_540,
        size_of::<DoublyLinkedList32::<u8, (), { u32::MAX as usize - 1 }>>()
    ];
}

#[test]
#[cfg(feature = "alloc")]
fn doubly_linked_sizes_boxed() {
    // on the heap
    assert_eq![16, size_of::<DoublyLinkedList8::<u8, Boxed, 10>>()];
    assert_eq![16, size_of::<DoublyLinkedList8::<u128, Boxed, 10>>()];
    assert_eq![16, size_of::<DoublyLinkedList16::<u8, Boxed, 10>>()];
    assert_eq![16, size_of::<DoublyLinkedList16::<u128, Boxed, 10>>()];
    assert_eq![24, size_of::<DoublyLinkedList32::<u8, Boxed, 10>>()];
    assert_eq![24, size_of::<DoublyLinkedList32::<u128, Boxed, 10>>()];
}

// #[test]
// fn doubly_linked_push_pop_front() {
//     let mut list = DirectDoublyLinkedList8::<i32, 3>::default();
//     assert_eq!(list.push_front(1), Ok(0));
//     assert_eq!(list.push_front(2), Ok(1));
//     assert_eq!(list.push_front(3), Ok(2));
//     assert_eq!(list.push_front(4), Err(Error::NotEnoughSpace(Some(1))));
//     assert_eq!(list.pop_front(), Ok(3));
//     assert_eq!(list.pop_front(), Ok(2));
//     assert_eq!(list.pop_front(), Ok(1));
//     assert_eq!(list.pop_front(), Err(Error::NotEnoughElements(1)));
// }

// #[test]
// fn doubly_linked_push_pop_back() {
//     let mut list = DirectDoublyLinkedList8::<i32, 3>::default();
//     assert_eq!(list.push_back(1), Ok(0));
//     assert_eq!(list.push_back(2), Ok(1));
//     assert_eq!(list.push_back(3), Ok(2));
//     assert_eq!(list.push_back(4), Err(Error::NotEnoughSpace(Some(1))));
//     assert_eq!(list.pop_back(), Ok(3));
//     assert_eq!(list.pop_back(), Ok(2));
//     assert_eq!(list.pop_back(), Ok(1));
//     assert_eq!(list.pop_back(), Err(Error::NotEnoughElements(1)));
// }

// #[test]
// fn doubly_linked_push_mixed() {
//     let mut list = DirectDoublyLinkedList8::<i32, 3>::default();
//     assert_eq!(list.push_front(1), Ok(0));
//     assert_eq!(list.push_back(2), Ok(1));
//     assert_eq!(list.push_front(3), Ok(2));
//     assert_eq!(list.push_back(4), Err(Error::NotEnoughSpace(Some(1))));
//     assert_eq!(list.pop_front(), Ok(3));
//     // FIXME
//     // assert_eq!(list.pop_back(), Ok(2));
//     // assert_eq!(list.pop_back(), Ok(1));
//     // assert_eq!(list.pop_back(), Err(Error::NotEnoughElements(1)));
//     //
//     // TEMP
//     // assert_eq!(list.pop_front(), Ok(1));
//     // assert_eq!(list.pop_front(), Ok(2));
//     // assert_eq!(list.pop_front(), Err(Error::NotEnoughElements(1)));
// }

// TODO
#[test]
fn singly_linked() {
    let mut list = SinglyLinkedList8::<i32, (), 3>::default();



}
