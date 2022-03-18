// ladata::other::bit_array
//
//!

use bv::Bits;

macro_rules! build_bit_array {
    (sizes: $( $b:literal $ty:ty ),+ ) => {
        $( build_bit_array![size: $b $ty ]; )+
    };
    (size: $b:literal $ty:ty ) => {
        paste::paste! {
            #[doc = $b "-bit [`bv`](https://crates.io/crates/bv)'s array of `Bits`."]
            #[derive(Clone, Copy, Debug, PartialEq)]
            pub struct [< BitArray $b >](pub $ty);

            impl Bits for [< BitArray $b >] {
                type Block = u8;
                fn bit_len(&self) -> u64 {
                    self.0.bit_len()
                }
            }
        }
    };
}

// build_bit_array![size: 0 8];
build_bit_array![sizes:
    8 [u8; 1],
    16 [u8; 2],
    32 [u8; 4],
    64 [u8; 8],
    128 [u8; 16],
    256 [u8; 32],
    512 [u8; 64],
    1024 [u8; 128]
];
