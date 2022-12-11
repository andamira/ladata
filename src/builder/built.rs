// ladata::builder::built
//
//! All the types are built here
//!
//! Firstly some mockups of unimported types are defined in order for the
//! `define_all_sizes` macro to work correctly under any feature combination.
//!
//! Secondly the crate types are built for the following sizes:
//! - bytes: 1, 2, 4, 8, 16, 32, 64, 128
//! - bits: 8, 16, 32, 64, 128, 256, 512, 1024
//

use super::macros::*;
use crate::traits::{DataBare, DataCell, DataCellCopy, DataType, DataTypeCopy};

// 1. Mockups for substituting unused dependencies
// -----------------------------------------------------------------------------

// "deps_continuous"

#[cfg(not(feature = "half"))]
mod half {
    #![allow(dead_code)]
    pub struct f16;
    pub struct bf16;
}
#[cfg(not(feature = "twofloat"))]
mod twofloat {
    #![allow(dead_code)]
    pub struct TwoFloat;
}

// "deps_discrete"

#[cfg(not(feature = "num-rational"))]
mod num_rational {
    #![allow(dead_code)]
    pub struct Ratio;
}

#[cfg(not(feature = "num-bigint"))]
mod num_bigint {
    #![allow(dead_code)]
    pub struct BigInt;
}
#[cfg(not(feature = "rust_decimal"))]
mod rust_decimal {
    #![allow(dead_code)]
    pub struct Decimal;
}

// "deps_string"

#[cfg(not(feature = "arraystring"))]
mod arraystring {
    #![allow(dead_code)]
    pub struct ArrayString<T> {
        _t: T,
    }
}
#[cfg(feature = "arraystring")]
use arraystring::{typenum, ArrayString};

// "deps_time"

#[cfg(not(feature = "fugit"))]
mod fugit {
    #![allow(dead_code)]
    pub struct Instant<T, const A: usize, B: Into<usize>> {
        _t: T,
        _b: B,
    }
    pub struct Duration<T, const A: usize, B: Into<usize>> {
        _t: T,
        _b: B,
    }
}
#[cfg(not(feature = "time"))]
mod time {
    #![allow(dead_code)]
    pub struct Date;
    pub struct Time;
    pub struct Instant;
    pub struct UtcOffset;
    pub struct OffsetDateTime;
    pub struct PrimitiveDateTime;
    pub struct Duration;
}

// 2. TYPES DEFINITIONS
// -----------------------------------------------------------------------------

// NOTE that right now several groups are empty. In some cases the macro can
// error when adding new elements, because of unexpected commas in the matching
// rules. The macro must be updated in the necessary places, by adding or
// removing commas as needed. See for example the `NOTE:missing-commas` tags.
define_all_sizes! {
    DataType, DataCell, DataBare,

    // -------------------------------------------------------- 1-B / 8-b
    copy_variants_1B:
    "8-bit unsigned integer ", U8, u8,
    "8-bit signed integer", I8, i8,
    "1-Byte array of bytes", ByteArray1, [u8; 1],
    "Boolean value", Bool, bool,
    copy_variants_1B_dep:
    "8-bit Array of bits (implementing [`bv`](https://crates.io/crates/bv/)'s `Bits`)",
        BitArray8, crate::all::BitArray8, "bv", "bv",
    copy_variants_1B_psize:
        "8-bit usize", Usize, usize, target_pointer_width = "8",
        "8-bit isize", Isize, isize, target_pointer_width = "8",
    copy_variants_1B_psize_dep: ,

    noncopy_variants_1B: ,
    noncopy_variants_1B_dep: ,
    noncopy_variants_1B_psize: ,
    noncopy_variants_1B_psize_dep: ,

    // -------------------------------------------------------- 2-B / 16-b
    copy_variants_2B:
    "16-bit unsigned integer ", U16, u16,
    "16-bit signed integer", I16, i16,
    "2-Byte array of bytes", ByteArray2, [u8; 2],
    copy_variants_2B_dep:
    "16-bit [`half`](https://crates.io/crates/half)'s `binary16` floating-point number",
        F16, half::f16, "half", "half",
    "16-bit [`half`](https://crates.io/crates/half)'s `bfloat16` floating-point number",
        BF16, half::bf16, "half", "half",
    "2-Byte [`arraystring`](https://crates.io/crates/arraystring)'s ArrayString of len()=1",
        ArrayString1, ArrayString<typenum::U1>, "arraystring", "arraystring",
    "16-bit Array of bits (implementing [`bv`](https://crates.io/crates/bv/)'s `Bits`)",
        BitArray16, crate::all::BitArray16, "bv", "bv",
    copy_variants_2B_psize:
        "16-bit usize", Usize, usize, target_pointer_width = "16",
        "16-bit isize", Isize, isize, target_pointer_width = "16",
    copy_variants_2B_psize_dep: ,

    noncopy_variants_2B: ,
    noncopy_variants_2B_dep: ,
    noncopy_variants_2B_psize: ,
    noncopy_variants_2B_psize_dep: ,

    // -------------------------------------------------------- 4-B / 32-b
    copy_variants_4B:
    "32-bit unsigned integer ", U32, u32,
    "32-bit signed integer", I32, i32,
    "32-bit floating-point number", F32, f32,
    "4-Byte array of bytes", ByteArray4, [u8; 4],
    "4-Byte char ", Char, char,
    copy_variants_4B_dep:
    "4-Byte [`arraystring`](https://crates.io/crates/arraystring)'s ArrayString of len()=3",
        ArrayString3, ArrayString<typenum::U3>, "arraystring", "arraystring",
    "32-bit [`time`](https://crates.io/crates/time)'s `Date`",
        TDate, time::Date, "time", "time",
    "32-bit [`time`](https://crates.io/crates/time)'s `UtcOffset`",
        TUtcOffset, time::UtcOffset, "time", "time",
    "32-bit [`fugit`](https://crates.io/crates/fugit)'s `Duration` in hours",
        FugitDuration32Hours, fugit::Duration<u32, 3_600, 1>, "fugit", "fugit",
    "32-bit [`fugit`](https://crates.io/crates/fugit)'s `Duration` in minutes",
        FugitDuration32Minutes, fugit::Duration<u32, 60, 1>, "fugit", "fugit",
    "32-bit [`fugit`](https://crates.io/crates/fugit)'s `Duration` in seconds",
        FugitDuration32Seconds, fugit::Duration<u32, 1, 1>, "fugit", "fugit",
    "32-bit [`fugit`](https://crates.io/crates/fugit)'s `Duration` in milliseconds",
        FugitDuration32Millis, fugit::Duration<u32, 1, 1_000>, "fugit", "fugit",
    "32-bit [`fugit`](https://crates.io/crates/fugit)'s `Duration` in nanoseconds",
        FugitDuration32Nanos, fugit::Duration<u32, 1, 1_000_000>, "fugit", "fugit",
    "32-bit [`fugit`](https://crates.io/crates/fugit)'s `Instant` in hours",
        FugitInstant32Hours, fugit::Instant<u32, 3_600, 1>, "fugit", "fugit",
    "32-bit [`fugit`](https://crates.io/crates/fugit)'s `Instant` in minutes",
        FugitInstant32Minutes, fugit::Instant<u32, 60, 1>, "fugit", "fugit",
    "32-bit [`fugit`](https://crates.io/crates/fugit)'s `Instant` in seconds",
        FugitInstant32Seconds, fugit::Instant<u32, 1, 1>, "fugit", "fugit",
    "32-bit [`fugit`](https://crates.io/crates/fugit)'s `Instant` in milliseconds",
        FugitInstant32Millis, fugit::Instant<u32, 1, 1_000>, "fugit", "fugit",
    "32-bit [`fugit`](https://crates.io/crates/fugit)'s `Instant` in nanoseconds",
        FugitInstant32Nanos, fugit::Instant<u32, 1, 1_000_000>, "fugit", "fugit",
    "32-bit Array of bits (implementing [`bv`](https://crates.io/crates/bv/)'s `Bits`)",
        BitArray32, crate::all::BitArray32, "bv", "bv",
    copy_variants_4B_psize:
        "32-bit usize", Usize, usize, target_pointer_width = "32",
        "32-bit isize", Isize, isize, target_pointer_width = "32",
    copy_variants_4B_psize_dep: ,

    noncopy_variants_4B: ,
    noncopy_variants_4B_dep: ,
    noncopy_variants_4B_psize: ,
    noncopy_variants_4B_psize_dep: ,

    // ------------------------------------------------------------------------- 8-B / 64-b
    copy_variants_8B:
    "64-bit unsigned integer ", U64, u64,
    "64-bit signed integer", I64, i64,
    "64-bit floating-point number", F64, f64,
    "8-Byte array of bytes", ByteArray8, [u8; 8],
    copy_variants_8B_dep:
    "32-bit [`num_rational`](https://crates.io/crates/num_rational)'s `Ratio` rational number",
        R32, num_rational::Ratio<i32>, "num-rational", "num-rational",
    "8-Byte [`arraystring`](https://crates.io/crates/arraystring)'s ArrayString of len()=7",
        ArrayString7, ArrayString<typenum::U7>, "arraystring", "arraystring",
    "64-bit [`time`](https://crates.io/crates/time)'s `Time`",
        TTime, time::Time, "time", "time",
    "64-bit [`fugit`](https://crates.io/crates/fugit)'s `Duration` in hours",
        FugitDuration64Hours, fugit::Duration<u64, 3_600, 1>, "fugit", "fugit",
    "64-bit [`fugit`](https://crates.io/crates/fugit)'s `Duration` in minutes",
        FugitDuration64Minutes, fugit::Duration<u64, 60, 1>, "fugit", "fugit",
    "64-bit [`fugit`](https://crates.io/crates/fugit)'s `Duration` in seconds",
        FugitDuration64Seconds, fugit::Duration<u64, 1, 1>, "fugit", "fugit",
    "64-bit [`fugit`](https://crates.io/crates/fugit)'s `Duration` in milliseconds",
        FugitDuration64Millis, fugit::Duration<u64, 1, 1_000>, "fugit", "fugit",
    "64-bit [`fugit`](https://crates.io/crates/fugit)'s `Duration` in nanoseconds",
        FugitDuration64Nanos, fugit::Duration<u64, 1, 1_000_000>, "fugit", "fugit",
    "64-bit [`fugit`](https://crates.io/crates/fugit)'s `Instant` in hours",
        FugitInstant64Hours, fugit::Instant<u64, 3_600, 1>, "fugit", "fugit",
    "64-bit [`fugit`](https://crates.io/crates/fugit)'s `Instant` in minutes",
        FugitInstant64Minutes, fugit::Instant<u64, 60, 1>, "fugit", "fugit",
    "64-bit [`fugit`](https://crates.io/crates/fugit)'s `Instant` in seconds",
        FugitInstant64Seconds, fugit::Instant<u64, 1, 1>, "fugit", "fugit",
    "64-bit [`fugit`](https://crates.io/crates/fugit)'s `Instant` in milliseconds",
        FugitInstant64Millis, fugit::Instant<u64, 1, 1_000>, "fugit", "fugit",
    "64-bit [`fugit`](https://crates.io/crates/fugit)'s `Instant` in nanoseconds",
        FugitInstant64Nanos, fugit::Instant<u64, 1, 1_000_000>, "fugit", "fugit",
    "64-bit Array of bits (implementing [`bv`](https://crates.io/crates/bv/)'s `Bits`)",
        BitArray64, crate::all::BitArray64, "bv", "bv",
    copy_variants_8B_psize:
        "64-bit usize", Usize, usize, target_pointer_width = "64",
        "64-bit isize", Isize, isize, target_pointer_width = "64",
    copy_variants_8B_psize_dep: ,

    noncopy_variants_8B: ,
    noncopy_variants_8B_dep: ,
    noncopy_variants_8B_psize: ,
    noncopy_variants_8B_psize_dep:
        "6-Byte fat-pointer String", String, std::string::String,
            target_pointer_width = "16", "std", "std",

    // ------------------------------------------------------------------------- 16-B /128-b
    copy_variants_16B:
    "128-bit unsigned integer ", U128, u128,
    "128-bit signed integer", I128, i128,
    "16-Byte array of bytes", ByteArray16, [u8; 16],
    "128-bit Duration", Duration, core::time::Duration,
    copy_variants_16B_dep:
    "64-bit [`num_rational`](https://crates.io/crates/num_rational)'s `Ratio` rational number",
        R64, num_rational::Ratio<i64>, "num-rational", "num-rational",
    "16-Byte [rust_decimal] Decimal number",
        Decimal, rust_decimal::Decimal, "rust_decimal", "rust_decimal",
    "16-Byte [`arraystring`](https://crates.io/crates/arraystring)'s ArrayString of len()=15",
        ArrayString15, ArrayString<typenum::U15>, "arraystring", "arraystring",
    "128-bit [`time`](https://crates.io/crates/time)'s `Duration`",
        TDuration, time::Duration, "time", "time",
    "128-bit [`time`](https://crates.io/crates/time)'s `PrimitiveDateTime`",
        TDateTime, time::PrimitiveDateTime, "time", "time",
    "128-bit [`time`](https://crates.io/crates/time)'s `OffsetDateTime`",
        TOffsetDateTime, time::OffsetDateTime, "time", "time",
    "128-bit floating point number",
        F128, twofloat::TwoFloat, "std", "twofloat",
    "128-bit Instant",
        Instant, std::time::Instant, "std", "std",
    "128-bit SystemTime",
        SystemTime, std::time::SystemTime, "std", "std",
    "128-bit [`time`](https://crates.io/crates/time)'s Instant`",
        TInstant, time::Instant, "std", "time",
    "128-bit Array of bits (implementing [`bv`](https://crates.io/crates/bv/)'s `Bits`)",
        BitArray128, crate::all::BitArray128, "bv", "bv",
    copy_variants_16B_psize:
        "128-bit usize", Usize, usize, target_pointer_width = "128",
        "128-bit isize", Isize, isize, target_pointer_width = "128",
    copy_variants_16B_psize_dep: ,

    noncopy_variants_16B: ,
    noncopy_variants_16B_dep: ,
    noncopy_variants_16B_psize: ,
    noncopy_variants_16B_psize_dep:
        "12-Byte fat-pointer String", String, std::string::String,
            target_pointer_width = "32", "std", "std",

    // ------------------------------------------------------------------------- 32-B / 256-b
    copy_variants_32B:
    "32-Byte array of bytes", ByteArray32, [u8; 32],
    copy_variants_32B_dep:
    "128-bit rational number", R128, num_rational::Ratio<i128>, "num-rational", "num-rational",
    "32-Byte [`arraystring`](https://crates.io/crates/arraystring)'s ArrayString of len()=31",
        ArrayString31, ArrayString<typenum::U31>, "arraystring", "arraystring",
    "256-bit Array of bits (implementing [`bv`](https://crates.io/crates/bv/)'s `Bits`)",
        BitArray256, crate::all::BitArray256, "bv", "bv",
    copy_variants_32B_psize: ,
    copy_variants_32B_psize_dep: ,

    noncopy_variants_32B: ,
    noncopy_variants_32B_dep:
    "Big Integer", BigInt, num_bigint::BigInt, "num-bigint", "num-bigint",
    noncopy_variants_32B_psize: ,
    noncopy_variants_32B_psize_dep:
        "24-Byte fat-pointer String", String, std::string::String,
            target_pointer_width = "64", "std", "std",

    // ------------------------------------------------------------------------- 64 B / 512-b
    copy_variants_64B:
    "64-Byte array of bytes", ByteArray64, [u8; 64],
    copy_variants_64B_dep:
    "64-Byte [`arraystring`](https://crates.io/crates/arraystring)'s ArrayString of len()=63",
        ArrayString63, ArrayString<typenum::U63>, "arraystring", "arraystring",
    "512-bit Array of bits (implementing [`bv`](https://crates.io/crates/bv/)'s `Bits`)",
        BitArray512, crate::all::BitArray512, "bv", "bv",
    copy_variants_64B_psize: ,
    copy_variants_64B_psize_dep: ,

    noncopy_variants_64B: ,
    noncopy_variants_64B_dep: ,
    noncopy_variants_64B_psize: ,
    noncopy_variants_64B_psize_dep:
        "48-Byte fat-pointer String", String, std::string::String,
            target_pointer_width = "128", "std", "std",

    // ------------------------------------------------------------------------- 128-B / 1024-b
    copy_variants_128B:
    "128-Byte array of bytes", ByteArray128, [u8; 128],
    copy_variants_128B_dep:
    "128-Byte [`arraystring`](https://crates.io/crates/arraystring)'s ArrayString of len()=127",
        ArrayString127, ArrayString<typenum::U127>, "arraystring", "arraystring",
    "1024-bit Array of bits (implementing [`bv`](https://crates.io/crates/bv/)'s `Bits`)",
        BitArray1024, crate::all::BitArray1024, "bv", "bv",
    copy_variants_128B_psize: ,
    copy_variants_128B_psize_dep: ,

    noncopy_variants_128B: ,
    noncopy_variants_128B_dep: ,
    noncopy_variants_128B_psize: ,
    noncopy_variants_128B_psize_dep: ,
}
