// ladata::mem::bit_size
//
//! Traits related to memory size.
//

use super::DataSize;
#[cfg(feature = "alloc")]
use alloc::{
    collections::{BTreeMap, BTreeSet, BinaryHeap, LinkedList, VecDeque},
    string::String,
    vec::Vec,
};
#[cfg(target_has_atomic = "64")]
use core::sync::atomic::{AtomicI64, AtomicU64};
use core::{
    cmp,
    convert::Infallible,
    marker::{PhantomData, PhantomPinned},
    num::{
        NonZeroI128, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8, NonZeroIsize, NonZeroU128,
        NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8, NonZeroUsize,
    },
    sync::atomic::{
        AtomicBool, AtomicI16, AtomicI32, AtomicI8, AtomicIsize, AtomicU16, AtomicU32, AtomicU8,
        AtomicUsize, Ordering,
    },
    time::Duration,
};
#[cfg(feature = "std")]
use std::{
    collections::{HashMap, HashSet},
    rc::Rc,
    sync::{Arc, Mutex},
    time::{Instant, SystemTime},
};

/* trait definitions */

/// Indicates a size of exactly `LEN` bits.
pub trait BitSize<const LEN: usize>: DataSize {
    /// The size of this type in bits.
    const BITS: usize = LEN;

    /// The size of this type in bytes, rounded up if it's not a multiple of 8.
    const BYTES_CEIL: usize = (LEN + 8 - 1) / 8;
}

/// Indicates a size of at least `LEN` bits.
pub trait BitSizeAtLeast<const LEN: usize>: DataSize {}

/// Indicates a size of at most `LEN` bits.
pub trait BitSizeAtMost<const LEN: usize>: DataSize {}

//
macro_rules! bit_size {
    /* primitives */

    (= $bits:expr; for $($ty:ty),+) => { $( impl BitSize<$bits> for $ty {} )+ };
    (>= $bits:expr; for $($ty:ty),+) => { $( impl BitSizeAtLeast<$bits> for $ty {} )+ };
    (<= $bits:expr; for $($ty:ty),+) => { $( impl BitSizeAtMost<$bits> for $ty {} )+ };

    /* primitives generic on $T */

    (<$T:ident> = $bits:expr; for $($ty:ty),+) => {
        $( impl<$T> BitSize<$bits> for $ty {} )+
    };
    (<$T:ident> >= $bits:expr; for $($ty:ty),+) => {
        $( impl<$T> BitSizeAtLeast<$bits> for $ty {} )+
    };
    (<$T:ident> <= $bits:expr; for $($ty:ty),+) => {
        $( impl<$T> BitSizeAtMost<$bits> for $ty {} )+
    };

    /* primitives generic on $K, $V */

    (<$K:ident, $V:ident> = $bits:expr; for $($ty:ty),+) => {
        $( impl<$K, $V> BitSize<$bits> for $ty {} )+
    };
    (<$K:ident, $V:ident> >= $bits:expr; for $($ty:ty),+) => {
        $( impl<$K, $V> BitSizeAtLeast<$bits> for $ty {} )+
    };
    (<$K:ident, $V:ident> <= $bits:expr; for $($ty:ty),+) => {
        $( impl<$K, $V> BitSizeAtMost<$bits> for $ty {} )+
    };

    /* pointer primitives */

    // implements BitSize<$bits> for pointer types.
    (pointer = $bits:literal) => {
        bit_size![= $bits; for isize, usize, NonZeroIsize, NonZeroUsize, AtomicIsize, AtomicUsize];

        #[cfg(feature = "std")]
        bit_size![<T> = {$bits * 1}; for Rc<T>, Arc<T>];

        bit_size![= {$bits * 2}; for &str];
        bit_size![<T> = {$bits * 2}; for &[T], &mut [T]];

        #[cfg(feature = "alloc")]
        bit_size![= {$bits * 3}; for String];

        #[cfg(feature = "alloc")]
        bit_size![<T> = {$bits * 3}; for Vec<T>, LinkedList<T>, VecDeque<T>, BTreeSet<T>, BinaryHeap<T>];
        #[cfg(feature = "std")]
        bit_size![<T> = {$bits * 3}; for HashSet<T>, Mutex<T>];

        #[cfg(feature = "alloc")]
        bit_size![<K, V> = {$bits * 3}; for BTreeMap<K, V>];
        #[cfg(feature = "std")]
        bit_size![<K, V> = {$bits * 3}; for HashMap<K, V>];
    };

    // implements BitSizeAtLeast<$bits> for pointer types.
    (pointer >= $($bits:literal),+) => {
        $(
            bit_size![>= $bits; for
                isize, usize, NonZeroIsize, NonZeroUsize, AtomicIsize, AtomicUsize];

            #[cfg(feature = "std")]
            bit_size![<T> >= {$bits * 1}; for Rc<T>, Arc<T>];

            bit_size![>= {$bits * 2}; for &str];
            bit_size![<T> >= {$bits * 2}; for &[T], &mut [T]];

            #[cfg(feature = "alloc")]
            bit_size![>= {$bits * 3}; for String];

            #[cfg(feature = "alloc")]
            bit_size![<T> >= {$bits * 3}; for
                Vec<T>, LinkedList<T>, VecDeque<T>, BTreeSet<T>, BinaryHeap<T>];
            #[cfg(feature = "std")]
            bit_size![<T> >= {$bits * 3}; for HashSet<T>, Mutex<T>];

            #[cfg(feature = "alloc")]
            bit_size![<K, V> >= {$bits * 3}; for BTreeMap<K, V>];
            #[cfg(feature = "std")]
            bit_size![<K, V> >= {$bits * 3}; for HashMap<K, V>];
        )+
    };

    // implements BitSizeAtMost<$bits> for pointer types.
    (pointer <= $($bits:literal),+) => {
        $(
            bit_size![<= $bits; for
                isize, usize, NonZeroIsize, NonZeroUsize, AtomicIsize, AtomicUsize];

            #[cfg(feature = "std")]
            bit_size![<T> <= {$bits * 1}; for Rc<T>, Arc<T>];

            bit_size![<= {$bits * 2}; for &str];
            bit_size![<T> <= {$bits * 2}; for &[T], &mut [T]];

            #[cfg(feature = "alloc")]
            bit_size![<= {$bits * 3}; for String];

            #[cfg(feature = "alloc")]
            bit_size![<T> <= {$bits * 3}; for
                Vec<T>, LinkedList<T>, VecDeque<T>, BTreeSet<T>, BinaryHeap<T>];
            #[cfg(feature = "std")]
            bit_size![<T> <= {$bits * 3}; for HashSet<T>, Mutex<T>];

            #[cfg(feature = "alloc")]
            bit_size![<K, V> <= {$bits * 3}; for BTreeMap<K, V>];
            #[cfg(feature = "std")]
            bit_size![<K, V> <= {$bits * 3}; for HashMap<K, V>];
        )+
    };

    /* arrays */

    (array = $bits:literal * len for T: $tsize:literal * len: $($len:literal),+) => {
        $( impl<T: BitSize<$tsize>> BitSize<{$bits*$len}> for [T; $len] {} )+
    };
    (array >= $bits:literal * len for T: $tsize:literal * len: $($len:literal),+) => {
        $( impl<T: BitSize<$tsize>> BitSize<{$bits*$len}> for [T; $len] {} )+
    };
    (array <= $bits:literal * len for T: $tsize:literal * len: $($len:literal),+) => {
        $( impl<T: BitSize<$tsize>> BitSizeAtMost<{$bits*$len}> for [T; $len] {} )+
    };
}

/* impl BitSize for primitives */

bit_size![<T> = 0; for PhantomData<T>];
bit_size![= 0; for (), Infallible, PhantomPinned];
bit_size![= 8; for i8, u8, bool,
    NonZeroI8, NonZeroU8, AtomicI8, AtomicU8, AtomicBool, Ordering, cmp::Ordering];
bit_size![= 16; for i16, u16, NonZeroI16, NonZeroU16, AtomicI16, AtomicU16];
bit_size![= 32; for i32, u32, f32, char, NonZeroI32, NonZeroU32, AtomicI32, AtomicU32];
bit_size![= 64; for i64, u64, f64, NonZeroI64, NonZeroU64];
#[cfg(target_has_atomic = "64")]
bit_size![= 64; for AtomicI64, AtomicU64];
bit_size![= 128; for i128, u128, NonZeroI128, NonZeroU128, Duration];
#[cfg(feature = "std")]
bit_size![= 128; for Instant, SystemTime];

/* impl BitSizeAtMost for primitives */

bit_size![<T> <= 0; for PhantomData<T>];
bit_size![<= 0; for (), Infallible, PhantomPinned];
bit_size![<T> <= 8; for PhantomData<T>];
bit_size![<= 8; for (), i8, u8, bool,
    Infallible, PhantomPinned,
    NonZeroI8, NonZeroU8, AtomicI8, AtomicU8, AtomicBool, Ordering, cmp::Ordering
];
bit_size![<T> <= 16; for PhantomData<T>];
bit_size![<= 16; for (), i8, u8, bool, i16, u16,
    Infallible, PhantomPinned,
    NonZeroI8, NonZeroU8, AtomicI8, AtomicU8, AtomicBool, Ordering, cmp::Ordering,
    NonZeroI16, NonZeroU16, AtomicI16, AtomicU16
];
bit_size![<T> <= 32; for PhantomData<T>];
bit_size![<= 32; for (), i8, u8, bool, i16, u16, i32, u32, f32, char,
    Infallible, PhantomPinned,
    NonZeroI8, NonZeroU8, AtomicI8, AtomicU8, AtomicBool, Ordering, cmp::Ordering,
    NonZeroI16, NonZeroU16, AtomicI16, AtomicU16,
    NonZeroI32, NonZeroU32, AtomicI32, AtomicU32
];
bit_size![<T> <= 64; for PhantomData<T>];
bit_size![<= 64; for (), i8, u8, bool, i16, u16, i32, u32, f32, char, i64, u64, f64,
    Infallible, PhantomPinned,
    NonZeroI8, NonZeroU8, AtomicI8, AtomicU8, AtomicBool, Ordering, cmp::Ordering,
    NonZeroI16, NonZeroU16, AtomicI16, AtomicU16,
    NonZeroI32, NonZeroU32, AtomicI32, AtomicU32,
    NonZeroI64, NonZeroU64
];
#[cfg(target_has_atomic = "64")]
bit_size![<= 64; for AtomicI64, AtomicU64];
bit_size![<T> <= 128; for PhantomData<T>];
bit_size![<= 128; for (), i8, u8, bool, i16, u16, i32, u32, f32, char, i64, u64, f64, i128, u128,
    Infallible, PhantomPinned,
    NonZeroI8, NonZeroU8, AtomicI8, AtomicU8, AtomicBool, Ordering, cmp::Ordering,
    NonZeroI16, NonZeroU16, AtomicI16, AtomicU16,
    NonZeroI32, NonZeroU32, AtomicI32, AtomicU32,
    NonZeroI64, NonZeroU64,
    NonZeroI128, NonZeroU128, Duration
];
#[cfg(target_has_atomic = "64")]
bit_size![<= 128; for AtomicI64, AtomicU64];
#[cfg(feature = "std")]
bit_size![<= 128; for Instant, SystemTime];

/* impl BitSizeAtLeast for primitives */

impl<T> BitSizeAtLeast<0> for T {}
bit_size![>= 8; for i8, u8, bool, i16, u16, i32, u32, f32, char, i64, u64, f64, i128, u128,
    NonZeroI8, NonZeroU8, AtomicI8, AtomicU8, AtomicBool, Ordering, cmp::Ordering,
    NonZeroI16, NonZeroU16, AtomicI16, AtomicU16,
    NonZeroI32, NonZeroU32, AtomicI32, AtomicU32,
    NonZeroI64, NonZeroU64,
    NonZeroI128, NonZeroU128, Duration
];
#[cfg(target_has_atomic = "64")]
bit_size![>= 8; for AtomicI64, AtomicU64];
bit_size![>= 16; for i16, u16, i32, u32, f32, char, i64, u64, f64, i128, u128,
    NonZeroI16, NonZeroU16, AtomicI16, AtomicU16,
    NonZeroI32, NonZeroU32, AtomicI32, AtomicU32,
    NonZeroI64, NonZeroU64,
    NonZeroI128, NonZeroU128, Duration
];
#[cfg(target_has_atomic = "64")]
bit_size![>= 16; for AtomicI64, AtomicU64];
bit_size![>= 32; for i32, u32, f32, char, i64, u64, f64, i128, u128,
    NonZeroI32, NonZeroU32, AtomicI32, AtomicU32,
    NonZeroI64, NonZeroU64,
    NonZeroI128, NonZeroU128, Duration
];
#[cfg(target_has_atomic = "64")]
bit_size![>= 32; for AtomicI64, AtomicU64];
bit_size![>= 64; for i64, u64, f64, i128, u128,
    NonZeroI64, NonZeroU64,
    NonZeroI128, NonZeroU128, Duration
];
#[cfg(target_has_atomic = "64")]
bit_size![>= 64; for AtomicI64, AtomicU64];
bit_size![>= 128; for i128, u128,
    NonZeroI128, NonZeroU128, Duration
];
#[cfg(feature = "std")]
bit_size![>= 128; for Instant, SystemTime];

/* impl BitSize* for pointer primitives */

#[cfg(target_pointer_width = "8")]
bit_size![pointer = 8];
#[cfg(target_pointer_width = "8")]
bit_size![pointer >= 8];
#[cfg(target_pointer_width = "8")]
bit_size![pointer <= 8, 16, 32, 64, 128];

#[cfg(target_pointer_width = "16")]
bit_size![pointer = 16];
#[cfg(target_pointer_width = "16")]
bit_size![pointer >= 8, 16];
#[cfg(target_pointer_width = "16")]
bit_size![pointer <= 16, 32, 64, 128];

#[cfg(target_pointer_width = "32")]
bit_size![pointer = 32];
#[cfg(target_pointer_width = "32")]
bit_size![pointer >= 8, 16, 32];
#[cfg(target_pointer_width = "32")]
bit_size![pointer <= 32, 64, 128];

#[cfg(target_pointer_width = "64")]
bit_size![pointer = 64];
#[cfg(target_pointer_width = "64")]
bit_size![pointer >= 8, 16, 32, 64];
#[cfg(target_pointer_width = "64")]
bit_size![pointer <= 64, 128];

#[cfg(target_pointer_width = "128")]
bit_size![pointer = 128];
#[cfg(target_pointer_width = "128")]
bit_size![pointer >= 8, 16, 32, 64, 128];
#[cfg(target_pointer_width = "128")]
bit_size![pointer <= 128];

/* impl BitSize for arrays */

bit_size![array = 8 * len for T: 8 * len: 1, 2, 4, 8, 16];
bit_size![array = 16 * len for T: 16 * len: 1, 2, 4, 8, 16];
bit_size![array = 24 * len for T: 24 * len: 1, 2, 4, 8, 16]; // *
bit_size![array = 32 * len for T: 32 * len: 1, 2, 4, 8, 16];
bit_size![array = 40 * len for T: 40 * len: 1, 2, 4, 8, 16]; // *
bit_size![array = 48 * len for T: 48 * len: 1, 2, 4, 8, 16]; // *
bit_size![array = 56 * len for T: 56 * len: 1, 2, 4, 8, 16]; // *
bit_size![array = 64 * len for T: 64 * len: 1, 2, 4, 8, 16];
bit_size![array = 72 * len for T: 72 * len: 1, 2, 4, 8, 16]; // *
bit_size![array = 80 * len for T: 80 * len: 1, 2, 4, 8, 16]; // *
bit_size![array = 88 * len for T: 88 * len: 1, 2, 4, 8, 16]; // *
bit_size![array = 96 * len for T: 96 * len: 1, 2, 4, 8, 16]; // *
bit_size![array = 104 * len for T: 104 * len: 1, 2, 4, 8, 16]; // *
bit_size![array = 112 * len for T: 112 * len: 1, 2, 4, 8, 16]; // *
bit_size![array = 120 * len for T: 120 * len: 1, 2, 4, 8, 16]; // *
bit_size![array = 128 * len for T: 128 * len: 1, 2, 4, 8, 16];

/* impl BitSizeAtLeast for arrays */

impl<T: BitSize<8>, const LEN: usize> BitSizeAtLeast<8> for [T; LEN] {}
impl<T: BitSize<16>, const LEN: usize> BitSizeAtLeast<16> for [T; LEN] {}
impl<T: BitSize<32>, const LEN: usize> BitSizeAtLeast<32> for [T; LEN] {}
impl<T: BitSize<64>, const LEN: usize> BitSizeAtLeast<64> for [T; LEN] {}
impl<T: BitSize<128>, const LEN: usize> BitSizeAtLeast<128> for [T; LEN] {}

/* impl BitSizeAtMost for arrays */

bit_size![array<= 8 * len for T: 8 * len: 1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16];
bit_size![array<= 16 * len for T: 16 * len: 1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16];
bit_size![array<= 32 * len for T: 32 * len: 1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16];
bit_size![array<= 64 * len for T: 64 * len: 1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16];
bit_size![array<= 128 * len for T: 128 * len: 1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16];
