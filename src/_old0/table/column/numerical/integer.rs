//!

// re-export enum variants defined in the current module, for conciseness.
use IntVec::{
    I128 as I128V, I16 as I16V, I32 as I32V, I64 as I64V, I8 as I8V, U128 as U128V, U16 as U16V,
    U32 as U32V, U64 as U64V, U8 as U8V,
};
use IntVecItem::{I128, I16, I32, I64, I8, U128, U16, U32, U64, U8};

// IntVec ----------------------------------------------------------------------

/// An abstraction over all the possible vector of integers.
#[derive(Debug)]
pub enum IntVec {
    I8(Vec<i8>),
    U8(Vec<u8>),
    I16(Vec<i16>),
    U16(Vec<u16>),
    I32(Vec<i32>),
    U32(Vec<u32>),
    I64(Vec<i64>),
    U64(Vec<u64>),
    I128(Vec<i128>),
    U128(Vec<u128>),
}
impl From<Vec<i8>> for IntVec {
    fn from(v: Vec<i8>) -> Self {
        IntVec::I8(v)
    }
}
impl From<Vec<u8>> for IntVec {
    fn from(v: Vec<u8>) -> Self {
        IntVec::U8(v)
    }
}
impl From<Vec<i16>> for IntVec {
    fn from(v: Vec<i16>) -> Self {
        IntVec::I16(v)
    }
}
impl From<Vec<u16>> for IntVec {
    fn from(v: Vec<u16>) -> Self {
        IntVec::U16(v)
    }
}
impl From<Vec<i32>> for IntVec {
    fn from(v: Vec<i32>) -> Self {
        IntVec::I32(v)
    }
}
impl From<Vec<u32>> for IntVec {
    fn from(v: Vec<u32>) -> Self {
        IntVec::U32(v)
    }
}
impl From<Vec<i64>> for IntVec {
    fn from(v: Vec<i64>) -> Self {
        IntVec::I64(v)
    }
}
impl From<Vec<u64>> for IntVec {
    fn from(v: Vec<u64>) -> Self {
        IntVec::U64(v)
    }
}
impl From<Vec<i128>> for IntVec {
    fn from(v: Vec<i128>) -> Self {
        IntVec::I128(v)
    }
}
impl From<Vec<u128>> for IntVec {
    fn from(v: Vec<u128>) -> Self {
        IntVec::U128(v)
    }
}

impl IntVec {
    /// Returns the length of the vector.
    pub fn len(&self) -> usize {
        crate::__match_variants_method![
            self,
            len,
            (I8V, U8V, I16V, U16V, I32V, U32V, I64V, U64V, I128V, U128V)
        ]
        // crate::__match_variants_method_args![self, len(), (I8V, U8V, I16V, U16V, I32V, U32V, I64V, U64V, I128V, U128V)] // WIP
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

// IntVecItem ------------------------------------------------------------------

/// An enum of the possible values in a [`IntVec`].
#[derive(Debug)]
pub enum IntVecItem {
    I8(i8),
    U8(u8),
    I16(i16),
    U16(u16),
    I32(i32),
    U32(u32),
    I64(i64),
    U64(u64),
    I128(i128),
    U128(u128),
}

crate::__impl_tryinto![IntVecItem, self, i8, I8];
crate::__impl_tryinto![IntVecItem, self, u8, U8];
crate::__impl_tryinto![IntVecItem, self, i16, I16];
crate::__impl_tryinto![IntVecItem, self, u16, U16];
crate::__impl_tryinto![IntVecItem, self, i32, I32];
crate::__impl_tryinto![IntVecItem, self, u32, U32];
crate::__impl_tryinto![IntVecItem, self, i64, I64];
crate::__impl_tryinto![IntVecItem, self, u64, U64];
crate::__impl_tryinto![IntVecItem, self, i128, I128];
crate::__impl_tryinto![IntVecItem, self, u128, U128];

// impl .as_* methods
impl IntVecItem {
    crate::__variants_values_as![self,
        (i8, u8, i16, u16, i32, u32, i64, u64, i128, u128);
        (I8, U8, I16, U16, I32, U32, I64, U64, I128, U128)
    ];
}

// IntVecIterator --------------------------------------------------------------

/// An iterator over values in a [`IntVec`].
#[derive(Debug)]
pub struct IntVecIterator {
    data: IntVec,
    pos: usize,
}
impl IntVecIterator {
    /// Returns a new iterator over a [`IntVec`], which itself returns
    /// [`IntVecItem`].
    pub fn new(data: IntVec) -> Self {
        Self { data, pos: 0 }
    }
}

impl Iterator for IntVecIterator {
    type Item = IntVecItem;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos < self.data.len() {
            // intvec_variants![by_ref, self.data, len]

            match self.data {
                I8V(ref vec) => {
                    self.pos += 1;
                    Some(I8(vec[self.pos - 1]))
                }
                U8V(ref vec) => {
                    self.pos += 1;
                    Some(U8(vec[self.pos - 1]))
                }
                I16V(ref vec) => {
                    self.pos += 1;
                    Some(I16(vec[self.pos - 1]))
                }
                U16V(ref vec) => {
                    self.pos += 1;
                    Some(U16(vec[self.pos - 1]))
                }
                I32V(ref vec) => {
                    self.pos += 1;
                    Some(I32(vec[self.pos - 1]))
                }
                U32V(ref vec) => {
                    self.pos += 1;
                    Some(U32(vec[self.pos - 1]))
                }
                I64V(ref vec) => {
                    self.pos += 1;
                    Some(I64(vec[self.pos - 1]))
                }
                U64V(ref vec) => {
                    self.pos += 1;
                    Some(U64(vec[self.pos - 1]))
                }
                I128V(ref vec) => {
                    self.pos += 1;
                    Some(I128(vec[self.pos - 1]))
                }
                U128V(ref vec) => {
                    self.pos += 1;
                    Some(U128(vec[self.pos - 1]))
                }
            }
        } else {
            None
        }
    }
}
