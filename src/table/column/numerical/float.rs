//!
//!

// re-export enum variants defined in the current module, for conciseness.
use FloatVec::{F32 as F32V, F64 as F64V};
use FloatVecItem::{F32, F64};

// FloatVec --------------------------------------------------------------------

/// An abstraction over all the possible vectors of floats.
#[derive(Debug)]
pub enum FloatVec {
    F32(Vec<f32>),
    F64(Vec<f64>),
}

crate::__impl_from![Vec<f32>, FloatVec, F32V];
crate::__impl_from![Vec<f64>, FloatVec, F64V];

impl IntoIterator for FloatVec {
    type Item = FloatVecItem;
    type IntoIter = FloatVecIterator;

    fn into_iter(self) -> Self::IntoIter {
        FloatVecIterator::new(self)
    }
}

impl FloatVec {
    /// Returns the length of the vector.
    pub fn len(&self) -> usize {
        crate::__match_variants_method![self, len, (F32V, F64V)]
        // crate::__match_variants_method_args![self, len(); (F32V, F64V)]; // WIP
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

// FloatVecItem ----------------------------------------------------------------

/// An enum of the possible values in a [`FloatVec`].
#[derive(Debug)]
pub enum FloatVecItem {
    F32(f32),
    F64(f64),
}

// impl From
// TODO: generalize
crate::__impl_tryinto![FloatVecItem, self, f32, F32];
crate::__impl_tryinto![FloatVecItem, self, f64, F64];

// impl .as_* methods
impl FloatVecItem {
    crate::__variants_values_as![self, (f32, f64); (F32, F64)];
}

// FloatVecIterator ------------------------------------------------------------

/// An iterator over values in a [`FloatVec`].
#[derive(Debug)]
pub struct FloatVecIterator {
    data: FloatVec,
    pos: usize,
}
impl FloatVecIterator {
    /// Returns a new iterator over a [`FloatVec`], which itself returns
    /// [`FloatVecItem`].
    pub fn new(data: FloatVec) -> Self {
        Self { data, pos: 0 }
    }
}
impl Iterator for FloatVecIterator {
    type Item = FloatVecItem;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos < self.data.len() {
            match self.data {
                F32V(ref vec) => {
                    self.pos += 1;
                    Some(F32(vec[self.pos - 1]))
                }
                F64V(ref vec) => {
                    self.pos += 1;
                    Some(F64(vec[self.pos - 1]))
                }
            }
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {
    use super::FloatVec;
    use float_eq::*;

    #[test]
    fn methods_as() {
        let fv32 = FloatVec::F32(vec![0.32]);
        let fv64 = FloatVec::F64(vec![0.32]);

        for f in fv32 {
            assert_eq!(0.32, f.as_f32());
            assert_float_eq!(0.32, f.as_f32(), r2nd <= f32::EPSILON);

            // FIXME?: casting from f32 to f64 has a lot of imprecision (10e9 × ε)
            assert_float_ne!(0.32, f.as_f64(), r2nd <= f64::EPSILON);
            assert_float_ne!(0.32, f.as_f64(), r2nd <= f64::EPSILON * 100_663_298.3);
            assert_float_eq!(0.32, f.as_f64(), r2nd <= f64::EPSILON * 100_663_298.4);
            assert_float_eq!(0.32, f.as_f64(), ulps <= 128849019); // min ulps
        }

        for f in fv64 {
            assert_eq!(0.32, f.as_f64());
            assert_float_eq!(0.32, f.as_f64(), r2nd <= f64::EPSILON);
            assert_float_eq!(0.32, f.as_f32(), r2nd <= f32::EPSILON);
        }
    }
}
