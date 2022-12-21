// ladata::structs::list
//
//!
//

mod double;
pub use double::{LinkedList8, LinkedList16, LinkedList32};

#[cfg(feature="std")]
pub use std::collections::LinkedList as DoubleLinkedListUsize;
