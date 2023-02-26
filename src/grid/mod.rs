// ladata::grid
//
//! Grid types.
//

use crate::{all::CollectionAdt, error::LadataResult as Result};

#[cfg(feature = "std")]
mod grid2d;

#[cfg(feature = "std")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "std")))]
pub use grid2d::Grid2d;

#[cfg(feature = "std")]
#[cfg(test)]
mod tests;

/// An abstract Grid.
pub trait GridAdt<const R: usize = 2>: CollectionAdt {
    ///
    fn grid_get(&mut self, index: [usize; R]) -> Result<<Self as CollectionAdt>::Element>;
    ///
    fn grid_set(
        &mut self,
        index: [usize; R],
        element: <Self as CollectionAdt>::Element,
    ) -> Result<()>;
}
