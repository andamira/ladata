// ladata::grid
//
//! Grid types.
//

use crate::{all::CollectionAdt, error::LadataResult as Result};

#[cfg(feature = "std")]
#[cfg(test)]
mod tests;

mod arr2d;

#[cfg(feature = "std")]
mod dyn2d;

pub use arr2d::{BoxedGrid2D, DirectGrid2D, Grid2D};

#[cfg(feature = "std")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "std")))]
pub use dyn2d::DynGrid2D;

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
