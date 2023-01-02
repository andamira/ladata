// ladata::line::stack::tests
//
//!
//

use core::fmt;

use super::*;
use crate::error::LadataResult as Result;

// `T: Debug` TEMP
impl<T: fmt::Debug, S: Storage, const CAP: usize> Stack<T, S, CAP>
where
    S::Container<[T; CAP]>: fmt::Debug,
{
    #[inline]
    pub fn dbg(&mut self) {
        println!["{:?}", self];
    }
}

#[test]
fn rotate() {
    let mut arr = ['a', 'b', 'c'];

    // == rot
    // (a b c -- b c a)
    arr.rotate_left(1);
    assert_eq![arr, ['b', 'c', 'a']];
    // println!("{arr:?}");

    // undo
    arr.rotate_right(1);

    // == rotCC
    // (a b c -- c a b)
    arr.rotate_right(1);
    assert_eq![arr, ['c', 'a', 'b']];
    // println!("{arr:?}");
}

#[test]
fn stack() -> Result<()> {
    let mut s = Stack::<char, (), 4>::default();

    s.push('a')?;
    s.push('b')?;
    s.push('c')?;
    s.dbg();
    s.rotate()?;
    s.dbg();

    Ok(())
}
