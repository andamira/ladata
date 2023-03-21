// ladata::list::deque::data
//
//!
//

use crate::all::{DataCollection, DataQueue, LadataResult as Result};

/// An abstract Deque.
pub trait DataDeque: DataCollection + DataQueue {
    ///
    fn deque_dequeue_back(&mut self) -> Result<<Self as DataCollection>::Element>;
    ///
    fn deque_enqueue_front(&mut self, element: <Self as DataCollection>::Element) -> Result<()>;
    //
    ///
    fn deque_dequeue_front(&mut self) -> Result<<Self as DataCollection>::Element> {
        self.queue_dequeue()
    }
    ///
    fn deque_enqueue_back(&mut self, element: <Self as DataCollection>::Element) -> Result<()> {
        self.queue_enqueue(element)
    }
}
