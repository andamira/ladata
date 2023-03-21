// ladata::list::queue::data
//
//!
//

use crate::{error::LadataResult as Result, misc::DataCollection};

/// An abstract Queue.
pub trait DataQueue: DataCollection {
    fn queue_dequeue(&mut self) -> Result<<Self as DataCollection>::Element>;
    fn queue_enqueue(&mut self, element: <Self as DataCollection>::Element) -> Result<()>;
}
