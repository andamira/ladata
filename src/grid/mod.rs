// ladata::grid
//
//! Grid types.
//

use crate::{all::DataCollection, error::LadataResult as Result};

#[cfg(feature = "alloc")]
#[cfg(test)]
mod tests;

mod arr2d;
#[cfg(feature = "alloc")]
mod dyn2d;

pub use arr2d::{DirectGrid2D, Grid2D};

#[cfg(feature = "alloc")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "alloc")))]
pub use arr2d::BoxedGrid2D;

#[cfg(feature = "alloc")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "alloc")))]
pub use dyn2d::DynGrid2D;

/// An abstract Grid.
pub trait DataGrid<const R: usize = 2>: DataCollection {
    ///
    fn grid_get(&mut self, index: [usize; R]) -> Result<<Self as DataCollection>::Element>;
    ///
    fn grid_set(
        &mut self,
        index: [usize; R],
        element: <Self as DataCollection>::Element,
    ) -> Result<()>;
}
