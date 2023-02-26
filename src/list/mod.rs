// ladata::list
//
//! List types.
//

pub mod array;
pub mod deque;
pub mod link;
pub mod queue;
pub mod stack;

pub use all::*;
pub(crate) mod all {
    #[doc(inline)]
    pub use super::{array::all::*, deque::*, link::*, queue::*, stack::*};
}
