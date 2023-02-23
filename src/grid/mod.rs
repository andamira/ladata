// ladata::grid
//
//! Grids.
//

#[cfg(feature = "std")]
mod grid2d;

#[cfg(feature = "std")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "std")))]
pub use grid2d::Grid2d;

#[cfg(feature = "std")]
#[cfg(test)]
mod tests;
