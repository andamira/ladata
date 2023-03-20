// ladata::grid
//
//! Grid types.
//

use crate::{all::CollectionAdt, error::LadataResult as Result};

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
