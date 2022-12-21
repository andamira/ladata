// ladata::storage::lac
//
//!
//
// NOTE: based on https://doc.rust-lang.org/std/boxed/struct.Box.html

use core::{cmp, fmt, hash, ops};

/// A no-op pointer type, like a [`Box`] that doesn't affect how `T` is stored.
///
/// # Examples
/// ```
/// use ladata::storage::Lac;
///
/// let lac = Lac::new(0_u8);
///
/// ```
pub struct Lac<T>(pub T);
impl<T> ops::Deref for Lac<T> {
    type Target = T;
    #[inline]
    fn deref(&self) -> &T {
        &self.0
    }
}
impl<T> Lac<T> {
    #[inline]
    pub const fn new(t: T) -> Self {
        Lac(t)
    }
}
impl<T> ops::DerefMut for Lac<T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut T {
        &mut self.0
    }
}
impl<T> From<T> for Lac<T> {
    #[inline]
    fn from(t: T) -> Self {
        Lac(t)
    }
}
impl<T: Clone> Clone for Lac<T> {
    #[inline]
    fn clone(&self) -> Self {
        Lac(self.0.clone())
    }
}
impl<T: Copy> Copy for Lac<T> {}

impl<T: Default> Default for Lac<T> {
    #[inline]
    fn default() -> Self {
        Lac(T::default())
    }
}

impl<T: PartialEq> PartialEq for Lac<T> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
        // PartialEq::eq(&**self, &**other)
    }
}
impl<T: Eq> Eq for Lac<T> {}

impl<T: PartialOrd> PartialOrd for Lac<T> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        self.0.partial_cmp(&other.0)
        // PartialOrd::partial_cmp(&**self, &**other)
    }
}
impl<T: Ord> Ord for Lac<T> {
    #[inline]
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.0.cmp(&other.0)
        // Ord::cmp(&**self, &**other)
    }
}

impl<T: fmt::Debug> fmt::Debug for Lac<T> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // fmt::Debug::fmt(&**self, f)
        fmt::Debug::fmt(&self, f)
    }
}
impl<T: fmt::Display> fmt::Display for Lac<T> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // fmt::Display::fmt(&**self, f)
        fmt::Display::fmt(&self.0, f)
    }
}

impl<T: hash::Hash> hash::Hash for Lac<T> {
    #[inline]
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        // (**self).hash(state);
        self.0.hash(state);
    }
}
impl<T: hash::Hasher> hash::Hasher for Lac<T> {
    #[inline]
    fn finish(&self) -> u64 {
        // (**self).finish()
        self.0.finish()
    }
    #[inline]
    fn write(&mut self, bytes: &[u8]) {
        // (**self).write(bytes)
        self.0.write(bytes)
    }
}

impl<I: Iterator> Iterator for Lac<I> {
    type Item = I::Item;
    fn next(&mut self) -> Option<I::Item> {
        (**self).next()
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        (**self).size_hint()
    }
    fn nth(&mut self, n: usize) -> Option<I::Item> {
        (**self).nth(n)
    }
    // fn last(self) -> Option<I::Item> {
    //     LacIter::last(self)
    // }
}

// trait LacIter {
//     type Item;
//     fn last(self) -> Option<Self::Item>;
// }
//
// impl<I: Iterator> LacIter for Lac<I> {
//     type Item = I::Item;
//     fn last(self) -> Option<I::Item> {
//         #[inline]
//         fn some<T>(_: Option<T>, x: T) -> Option<T> {
//             Some(x)
//         }
//
//         self.fold(None, some)
//     }
// }
//
