// ladata::grid::arr2d
//
//!
//

use core::ops::{Index, IndexMut};

use crate::{
    error::{LadataError as Error, LadataResult as Result},
    list::Array,
    mem::Storage,
};

#[cfg(feature = "std")]
use crate::mem::Boxed;

/// A 2D grid, backed by an [`Array`].
///
/// Internally the elements are stored in *row major order*,
/// meaning the elements of each row are stored sequentially.
pub struct Grid2D<T, S: Storage, const SIZE: usize> {
    cols: usize,
    rows: usize,
    grid: Array<T, S, SIZE>,
}

/// A [`Grid2D`] stored in the stack.
pub type DirectGrid2D<T, const SIZE: usize> = Grid2D<T, (), SIZE>;

/// A [`Grid2D`] stored in the heap.
#[cfg(feature = "std")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "std")))]
pub type BoxedGrid2D<T, const SIZE: usize> = Grid2D<T, Boxed, SIZE>;

// `S:() + T:Clone`
impl<T: Clone, const SIZE: usize> Grid2D<T, (), SIZE> {
    /// Returns a 2d grid, allocated in the stack,
    /// using `element` to fill the remaining free data.
    ///
    /// # Errors
    /// if the const `SIZE` doesn't match `cols` * `rows`.
    ///
    /// # Examples
    /// ```
    /// use ladata::grid::DirectGrid2D;
    ///
    /// let s = DirectGrid2D::<_, 16>::new('.', 4, 4);
    /// ```
    pub fn new(element: T, cols: usize, rows: usize) -> Result<Self> {
        if cols * rows == SIZE {
            Ok(Self {
                cols,
                rows,
                grid: Array::<T, (), SIZE>::with(element),
            })
        } else {
            Err(Error::DimensionMismatch)
        }
    }
}

// `S:Boxed + T:Clone`
#[cfg(feature = "std")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "std")))]
impl<T: Clone, const SIZE: usize> Grid2D<T, Boxed, SIZE> {
    /// Returns a 2d grid, allocated in the heap,
    /// using `element` to fill the remaining free data.
    ///
    /// # Errors
    /// if the const `SIZE` doesn't match `cols` * `rows`.
    ///
    /// # Examples
    /// ```
    /// use ladata::grid::BoxedGrid2D;
    ///
    /// let mut s = BoxedGrid2D::<_, 100>::new(' ', 10, 10);
    /// ```
    pub fn new(element: T, cols: usize, rows: usize) -> Result<Self> {
        if cols * rows == SIZE {
            Ok(Self {
                cols,
                rows,
                grid: Array::<T, Boxed, SIZE>::with(element),
            })
        } else {
            Err(Error::DimensionMismatch)
        }
    }
}

/// # general queries
impl<T, S: Storage, const SIZE: usize> Grid2D<T, S, SIZE> {
    /// Returns the length of the grid (`rows` × `cols`).
    #[inline]
    #[allow(clippy::len_without_is_empty)]
    pub const fn len(&self) -> usize {
        self.cols * self.rows
    }

    /// Returns the number of rows.
    #[inline]
    pub const fn num_rows(&self) -> usize {
        self.rows
    }

    /// Returns the number of columns.
    #[inline]
    pub const fn num_cols(&self) -> usize {
        self.cols
    }

    /// Returns the length of a row (== `num_cols`).
    #[inline]
    pub const fn row_len(&self) -> usize {
        self.cols
    }

    /// Returns the length of a column (== `num_rows`).
    #[inline]
    pub const fn col_len(&self) -> usize {
        self.rows
    }

    /// Translates 2D `col`,`row` coordinates into a 1D index.
    ///
    /// # Errors
    /// If out of bounds.
    #[inline]
    pub const fn get_index(&self, col: usize, row: usize) -> Result<usize> {
        if row < self.rows && col < self.cols {
            Ok(self.get_index_unchecked(col, row))
        } else {
            Err(Error::Indices2dOutOfBounds(col, row))
        }
    }
    /// Translates 2D `col`,`row` coordinates into a 1D index.
    ///
    /// This function doesn't check whether the dimensions are right.
    #[inline]
    pub const fn get_index_unchecked(&self, col: usize, row: usize) -> usize {
        row * self.row_len() + col
    }

    /// Translates 1D index into 2D `col`,`row` coordinates.
    ///
    /// # Errors
    /// If out of bounds.
    #[inline]
    pub const fn get_coords(&self, index: usize) -> Result<(usize, usize)> {
        if index < self.len() {
            Ok((index / self.cols, index % self.cols))
        } else {
            Err(Error::IndexOutOfBounds(index))
        }
    }
    /// Translates 1D index into 2D `col`,`row` coordinates.
    ///
    /// # Panics
    /// If out of bounds.
    #[inline]
    pub const fn get_coords_unchecked(&self, index: usize) -> (usize, usize) {
        (index / self.cols, index % self.cols)
    }

    // chunks

    /// Returns the number of chunks (`capacity()`/`chunk_len`).
    #[inline]
    pub const fn chunked_capacity(&self, chunk_len: usize) -> usize {
        self.len() / chunk_len
    }

    /// Returns the number of chunks per row.
    #[inline]
    pub const fn chunks_per_row(&self, chunk_len: usize) -> usize {
        self.row_len() / chunk_len
    }

    /// Translates 2D `col`,`row` coordinates, with chunk length, into a 1D index.
    ///
    /// - it assumes the `row_len` to be an exact multiple of `chunk_len`.
    /// - only full chunks are allowed.
    ///
    /// # Errors
    /// If out of bounds.
    pub const fn get_chunk_index(&self, chunk_len: usize, col: usize, row: usize) -> Result<usize> {
        if row < self.rows && col < (self.cols / chunk_len) {
            Ok(row * self.row_len() + col * chunk_len)
        } else {
            Err(Error::Indices2dOutOfBounds(col, row))
        }
    }

    /// Translates 2D `col`,`row` coordinates, with chunk length, into a 1D index.
    ///
    /// # Panics
    /// If out of bounds.
    pub const fn get_chunk_index_unchecked(
        &self,
        chunk_len: usize,
        col: usize,
        row: usize,
    ) -> usize {
        row * self.row_len() + col * chunk_len
    }
}

/// # single element get/set
impl<T, S: Storage, const SIZE: usize> Grid2D<T, S, SIZE> {
    // get_ref

    /// Returns a reference to the element at the given `row` and `col`umn.
    ///
    /// # Errors
    /// If out of bounds.
    #[inline]
    pub fn get_ref(&self, col: usize, row: usize) -> Result<&T> {
        self.get_index(col, row).map(|idx| &self.grid[idx])
    }
    /// Returns a reference to the element at the given `row` and `col`umn.
    ///
    /// # Panics
    /// If out of bounds.
    #[inline]
    pub fn get_ref_unchecked(&self, col: usize, row: usize) -> &T {
        &self.grid[self.get_index_unchecked(col, row)]
    }

    /// Returns a reference to the element at the given 1D index, in *row major order*.
    ///
    /// # Errors
    /// If out of bounds.
    #[inline]
    pub fn get_ref_row_order(&self, index: usize) -> Result<&T> {
        if index < self.len() {
            Ok(&self.grid[index])
        } else {
            Err(Error::IndexOutOfBounds(index))
        }
    }
    /// Returns a reference to the element at the given 1D index, in *row major order*.
    ///
    /// # Panics
    /// If out of bounds.
    #[inline]
    pub fn get_ref_row_order_unchecked(&self, index: usize) -> &T {
        &self.grid[index]
    }

    /// Returns a reference to the element at the given 1D index, in *column major order*.
    ///
    /// # Errors
    /// If out of bounds.
    #[inline]
    pub fn get_ref_col_order(&self, index: usize) -> Result<&T> {
        if index < self.len() {
            Ok(self.get_ref_col_order_unchecked(index))
        } else {
            Err(Error::IndexOutOfBounds(index))
        }
    }
    /// Returns a reference to the element at the given 1D index, in *column major order*.
    ///
    /// # Panics
    /// If out of bounds.
    #[inline]
    pub fn get_ref_col_order_unchecked(&self, index: usize) -> &T {
        let col = index / self.rows;
        let row = index % self.rows;
        self.get_ref_unchecked(col, row)
    }

    // get mut

    /// Returns a mutable reference to the element at the given `row` and `col`umn.
    ///
    /// # Errors
    /// If out of bounds.
    #[inline]
    pub fn get_ref_mut(&mut self, col: usize, row: usize) -> Result<&mut T> {
        self.get_index(col, row).map(|idx| &mut self.grid[idx])
    }
    /// Returns a mutable reference to the element at the given `row` and `col`umn.
    ///
    /// # Panics
    /// If out of bounds.
    #[inline]
    pub fn get_ref_mut_unchecked(&mut self, col: usize, row: usize) -> &mut T {
        let idx = self.get_index_unchecked(col, row);
        &mut self.grid[idx]
    }

    /// Returns a mutable reference to the element at the given 1D index, in *row major order*.
    ///
    /// # Errors
    /// If out of bounds.
    #[inline]
    pub fn get_ref_mut_row_order(&mut self, index: usize) -> Result<&mut T> {
        if index < self.len() {
            Ok(&mut self.grid[index])
        } else {
            Err(Error::IndexOutOfBounds(index))
        }
    }

    /// Returns a mutable reference to the element at the given 1D index, in *row major order*.
    ///
    /// # Panics
    /// If out of bounds.
    #[inline]
    pub fn get_ref_mut_row_order_unchecked(&mut self, index: usize) -> &mut T {
        &mut self.grid[index]
    }

    /// Returns a mutable reference to the element at the given 1D index, in *column major order*.
    ///
    /// # Errors
    /// If out of bounds.
    #[inline]
    pub fn get_ref_mut_col_order(&mut self, index: usize) -> Result<&mut T> {
        if index < self.len() {
            Ok(self.get_ref_mut_col_order_unchecked(index))
        } else {
            Err(Error::IndexOutOfBounds(index))
        }
    }

    /// Returns a mutable reference to the element at the given 1D index, in *column major order*.
    ///
    /// # Panics
    /// If out of bounds.
    #[inline]
    pub fn get_ref_mut_col_order_unchecked(&mut self, index: usize) -> &mut T {
        let col = index / self.rows;
        let row = index % self.rows;
        self.get_ref_mut_unchecked(col, row)
    }

    // set

    /// Sets the `element` at the given `row` and `col`umn.
    ///
    /// # Errors
    /// If out of bounds.
    #[inline]
    pub fn set(&mut self, element: T, col: usize, row: usize) -> Result<()> {
        self.get_ref_mut(col, row).map(|index| {
            *index = element;
        })
    }
    /// Sets the `element` at the given `row` and `col`umn.
    ///
    /// # Panics
    /// If out of bounds.
    #[inline]
    pub fn set_unchecked(&mut self, element: T, col: usize, row: usize) {
        let index = self.get_ref_mut_unchecked(col, row);
        *index = element;
    }

    /// Sets the `element` at the given 1D index, in *row major order*.
    ///
    /// # Errors
    /// If out of bounds.
    #[inline]
    pub fn set_row_order(&mut self, element: T, index: usize) -> Result<()> {
        self.get_ref_mut_row_order(index).map(|index| {
            *index = element;
        })
    }
    /// Sets the element at the given 1D index, in *row major order*.
    ///
    /// # Panics
    /// If out of bounds.
    #[inline]
    pub fn set_row_order_unchecked(&mut self, element: T, index: usize) {
        *self.get_ref_mut_row_order_unchecked(index) = element;
    }

    /// Sets the element at the given 1D index, in *column major order*.
    ///
    /// # Errors
    /// If out of bounds.
    #[inline]
    pub fn set_col_order(&mut self, element: T, index: usize) -> Result<()> {
        self.get_ref_mut_col_order(index).map(|index| {
            *index = element;
        })
    }
    /// Returns a mutable reference to the element at the given 1D index, in *column major order*.
    ///
    /// # Panics
    /// If out of bounds.
    #[inline]
    pub fn set_col_order_unchecked(&mut self, element: T, index: usize) {
        *self.get_ref_mut_col_order_unchecked(index) = element;
    }
}

// T: Clone
// FIX
#[cfg(feature = "std")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "std")))]
impl<T: Clone, S: Storage, const SIZE: usize> Grid2D<T, S, SIZE> {
    // /// Creates a new `Grid2D` from a slice of rows.
    // ///
    // /// All rows must have the same length.
    // pub fn from_rows(rows: &[Vec<T>]) -> Result<Self> {
    //     let row_len = rows.get(0).map(Vec::len).unwrap_or(0);
    //     if !rows.iter().all(|row| row.len() == row_len) {
    //         return Err(Error::DimensionMismatch);
    //     }
    //     Ok(Self {
    //         cols: row_len,
    //         rows: rows.len(),
    //         grid: Self::flatten(rows), // FIXME Array::from
    //     })
    // }

    // /// Creates a new `Grid2D` from a slice of `columns`.
    // ///
    // /// All columns must have the same length.
    // pub fn from_cols(columns: &[Vec<T>]) -> Result<Self> {
    //     let col_len = columns.get(0).map(Vec::len).unwrap_or(0);
    //     if !columns.iter().all(|col| col.len() == col_len) {
    //         return Err(Error::DimensionMismatch);
    //     }
    //     let rows = col_len;
    //     let cols = columns.len();
    //     let indices_row_order = (0..rows).flat_map(move |row| (0..cols).map(move |col| (col, row)));
    //     let grid = indices_row_order
    //         .map(|(col, row)| columns[col][row].clone())
    //         .collect();
    //     Ok(Grid2D { cols, rows, grid })
    // }

    // /// Creates a new `Grid2D` from the given flat slice of `elements`, in *row major order*.
    // ///
    // /// The number of `elements` must equal `rows`×`cols`.
    // pub fn from_row_order(elements: &[T], cols: usize, rows: usize) -> Result<Self> {
    //     let total_len = cols * rows;
    //     if total_len != elements.len() {
    //         return Err(Error::DimensionMismatch);
    //     }
    //     Ok(Self {
    //         grid: elements.into(), // FIXME Array:from
    //         rows,
    //         cols,
    //     })
    // }

    // /// Creates a new `Grid2D` from the given flat slice of `elements`, in *column major order*.
    // ///
    // /// The number of `elements` must equal `rows`×`cols`.
    // pub fn from_col_order(elements: &[T], cols: usize, rows: usize) -> Result<Self> {
    //     let total_len = cols * rows;
    //     if total_len != elements.len() {
    //         return Err(Error::DimensionMismatch);
    //     }
    //     let indices_row_order =
    //         (0..rows).flat_map(move |row| (0..cols).map(move |column| (col, rowumn)));
    //     let grid = indices_row_order
    //         .map(|(col, rowumn)| {
    //             let index = column * rows + row;
    //             elements[index].clone()
    //         })
    //         .collect();
    //     Ok(Grid2D { cols, rows, grid })
    // }

    // /// Creates a new `Grid2D` with the specified number of `rows` and `col`umn`s`,
    // /// filling each element with the result of calling the given function.
    // ///
    // /// The `function` is called once for every location going in *row major order*.
    // pub fn from_fn_row_order<F: FnMut() -> T>(mut function: F, cols: usize, rows: usize) -> Self {
    //     let len = cols * rows;
    //     let grid = (0..len).map(|_| function()).collect(); // MAYBE FIX
    //     Grid2D { cols, rows, grid }
    // }

    // /// Creates a new `Grid2D` with the specified number of `rows` and `col`umn`s`,
    // /// filling each element with the result of calling the given function.
    // ///
    // /// The `function` is called once for every location going in *column major order*.
    // pub fn from_fn_col_order<F: FnMut() -> T>(mut function: F, cols: usize, rows: usize) -> Self {
    //     let len = cols * rows;
    //     let grid_col_order = (0..len).map(|_| function()).collect::<Vec<_>>();
    //     Grid2D::from_col_order(&grid_col_order, rows, cols)
    //         .expect("from_fn_col_order should never fail")
    // }

    // MAYBE FIX
    // /// Creates a new `Grid2D` with the specified number of `rows` and `col`umn`s`,
    // /// filling each element with the elements produced by the provided `iterator`.
    // ///
    // /// The elements are inserted into the grid in *row major order*.
    // pub fn from_iter_row_order<I>(iterator: I, cols: usize, rows: usize) -> Result<Self>
    // where
    //     I: Iterator<Item = T>,
    // {
    //     let len = cols * rows;
    //     let grid = iterator.take(len).collect::<Vec<_>>();
    //     if grid.len() < len {
    //         return Err(Error::NotEnoughElements(len));
    //     }
    //     Ok(Grid2D { cols, rows, grid })
    // }

    // /// Creates a new `Grid2D` with the specified number of `rows` and `col`umn`s`,
    // /// filling each element with the elements produced by the provided `iterator`.
    // ///
    // /// The elements are inserted into the grid in *column major order*.
    // pub fn from_iter_col_order<I>(iterator: I, cols: usize, rows: usize) -> Result<Self>
    // where
    //     I: Iterator<Item = T>,
    // {
    //     let total_len = cols * rows;
    //     let grid_col_order = iterator.take(total_len).collect::<Vec<_>>();
    //     Grid2D::from_col_order(&grid_col_order, rows, cols)
    //         .map_err(|_| Error::NotEnoughElements(total_len))
    // }

    // Flattens a slice of vecs into a single vec.
    #[inline]
    #[allow(dead_code)]
    fn flatten(nested: &[Vec<T>]) -> Vec<T> {
        nested.iter().flat_map(|row| row.clone()).collect()
    }
}

/// # single element get methods (Copy)
impl<T: Copy, S: Storage, const SIZE: usize> Grid2D<T, S, SIZE> {
    // get

    /// Returns the element at the given `row` and `col`umn.
    #[inline]
    pub fn get(&self, col: usize, row: usize) -> Result<T> {
        self.get_index(col, row).map(|idx| self.grid[idx])
    }
    /// Returns the element at the given `row` and `col`umn.
    ///
    /// # Panics
    /// If out of bounds.
    #[inline]
    pub fn get_unchecked(&self, col: usize, row: usize) -> T {
        self.grid[self.get_index_unchecked(col, row)]
    }

    /// Returns the element at the given 1D index, in *row major order*.
    #[inline]
    pub fn get_row_order(&self, index: usize) -> Result<T> {
        if index < self.len() {
            Ok(self.grid[index])
        } else {
            Err(Error::IndexOutOfBounds(index))
        }
    }
    /// Returns the element at the given 1D index, in *row major order*.
    ///
    /// # Panics
    /// If out of bounds.
    #[inline]
    pub fn get_row_order_unchecked(&self, index: usize) -> T {
        self.grid[index]
    }

    /// Returns the element at the given 1D index, in *column major order*.
    #[inline]
    pub fn get_col_order(&self, index: usize) -> Result<T> {
        if index < self.len() {
            Ok(self.get_col_order_unchecked(index))
        } else {
            Err(Error::IndexOutOfBounds(index))
        }
    }
    /// Returns the element at the given 1D index, in *column major order*.
    ///
    /// # Panics
    /// If out of bounds.
    #[inline]
    pub fn get_col_order_unchecked(&self, index: usize) -> T {
        let col = index / self.rows;
        let row = index % self.rows;
        self.get_unchecked(col, row)
    }
}

/// # iterators
impl<T, S: Storage, const SIZE: usize> Grid2D<T, S, SIZE> {
    // all elements iter

    /// Returns an iterator over references to all elements in *row major order*.
    #[inline]
    pub fn iter_ref(&self) -> impl DoubleEndedIterator<Item = &T> {
        self.grid.iter()
    }

    /// Returns an iterator over mutable references to all elements in *row major order*.
    #[inline]
    pub fn iter_ref_mut(&mut self) -> impl DoubleEndedIterator<Item = &mut T> {
        self.grid.iter_mut()
    }

    // MAYBE FIX
    // /// Returns an iterator over references to all elements in *col major order*.
    // #[inline]
    // pub fn iter_ref_col_order(&self) -> impl DoubleEndedIterator<Item = &T> {
    //     (0..self.cols).flat_map(move |col| (0..self.rows).map(move |row| &self[(col, row)]))
    // }
    // /// Returns an iterator over mutable references to all elements in *col major order*.
    // pub fn iter_mut_col_order(&mut self) -> impl Iterator<Item = &mut T> {
    // }

    // row iter

    /// Returns an iterator over references to all elements in the given row.
    #[inline]
    pub fn row_iter_ref(&self, row: usize) -> Result<impl DoubleEndedIterator<Item = &T>> {
        let start = self.get_index(0, row)?;
        let end = start + self.row_len();
        Ok(self.grid[start..end].iter())
    }
    /// Returns an iterator over references to all elements in the given row.
    ///
    /// # Panics
    /// If out of bounds.
    #[inline]
    pub fn row_iter_ref_unchecked(&self, row: usize) -> impl DoubleEndedIterator<Item = &T> {
        let start = self.get_index_unchecked(0, row);
        let end = start + self.row_len();
        self.grid[start..end].iter()
    }

    /// Returns an iterator over mutable references to all elements in the given row.
    #[inline]
    pub fn row_iter_ref_mut(
        &mut self,
        row: usize,
    ) -> Result<impl DoubleEndedIterator<Item = &mut T>> {
        let start = self.get_index(0, row)?;
        let end = start + self.row_len();
        Ok(self.grid[start..end].iter_mut())
    }
    /// Returns an iterator over mutable references to all elements in the given row.
    ///
    /// # Panics
    /// If out of bounds.
    #[inline]
    pub fn row_iter_ref_mut_unchecked(
        &mut self,
        row: usize,
    ) -> impl DoubleEndedIterator<Item = &mut T> {
        let start = self.get_index_unchecked(0, row);
        let end = start + self.row_len();
        self.grid[start..end].iter_mut()
    }

    // column iter

    /// Returns an iterator over references to all elements in the given `col`umn.
    #[inline]
    // IMPROVE: DoubleEndedIterator?
    // pub fn col_iter_ref(&self, col: usize) -> Result<impl DoubleEndedIterator<Item = &T>> {
    pub fn col_iter_ref(&self, col: usize) -> Result<impl Iterator<Item = &T>> {
        if col >= self.cols {
            return Err(Error::Indices2dOutOfBounds(0, col));
        }
        let col_len = self.col_len();
        Ok(self.iter_ref().skip(col).step_by(col_len))
        // Ok((0..self.col_len()).map(move |row| &self[(col, row)]))
    }
    /// Returns an iterator over references to all elements in the given `col`umn.
    ///
    /// # Panics
    /// If out of bounds.
    #[inline]
    // IMPROVE: DoubleEndedIterator?
    // pub fn col_iter_ref_unchecked(&self, col: usize) -> impl DoubleEndedIterator<Item = &T> {
    pub fn col_iter_ref_unchecked(&self, col: usize) -> impl Iterator<Item = &T> {
        // (0..self.col_len()).map(move |row| &self[(col, row)])
        let col_len = self.col_len();
        self.iter_ref().skip(col).step_by(col_len)
    }

    /// Returns an iterator over references to all elements in the given `col`umn.
    // IMPROVE: DoubleEndedIterator?
    #[inline]
    pub fn col_iter_ref_mut(&mut self, col: usize) -> Result<impl Iterator<Item = &mut T>> {
        if col >= self.cols {
            return Err(Error::Indices2dOutOfBounds(0, col));
        }
        let col_len = self.col_len();
        Ok(self.iter_ref_mut().skip(col).step_by(col_len))
    }
    /// Returns an iterator over references to all elements in the given `col`umn.
    ///
    /// # Panics
    /// If out of bounds.
    // IMPROVE: DoubleEndedIterator?
    #[inline]
    pub fn col_iter_ref_mut_unchecked(&mut self, col: usize) -> impl Iterator<Item = &mut T> {
        let col_len = self.col_len();
        self.iter_ref_mut().skip(col).step_by(col_len)
    }

    // all rows iter

    /// Returns an iterator over all rows.
    ///
    /// Each `Item` is itself another `Iterator` over references to the elements in that column.
    #[inline]
    pub fn rows_iter_ref(
        &self,
    ) -> impl DoubleEndedIterator<Item = impl DoubleEndedIterator<Item = &T>> {
        (0..self.rows).map(move |row| self.row_iter_ref(row).expect("rows_iter should never fail"))
    }

    // MAYBE FIX cannot infer an appropriate lifetime for autoref due to conflicting requirements
    //
    // /// Returns an iterator over all rows.
    // ///
    // /// Each `Item` is itself another `Iterator` over mutable references to the elements in that column.
    // #[inline]
    // pub fn rows_iter_ref_mut(
    //     &mut self,
    // ) -> impl DoubleEndedIterator<Item = impl DoubleEndedIterator<Item = &mut T>> {
    //     (0..self.rows).map(move |row| self.row_iter_ref_mut(row).expect("rows_iter should never fail"))
    // }

    // all columns iter

    // MAYBE FIX
    // /// Returns an iterator over all columns.
    // ///
    // /// Each `Item` is itself another `Iterator` over references to the elements in that column.
    // #[inline]
    // pub fn cols_iter_ref(
    //     &self,
    // ) -> impl DoubleEndedIterator<Item = impl DoubleEndedIterator<Item = &T>> {
    //     (0..self.cols).map(move |col| self.col_iter_ref(col).expect("cols_iter should never fail"))
    // }
    // pub fn cols_iter_ref_mut(&mut self) {}

    // chunks iter

    /// Returns an iterator over `chunk_len` references to elements in *row major order*.
    ///
    /// # Panics
    /// If `chunk_size` is 0.
    #[inline]
    pub fn chunks_iter_ref(&self, chunk_size: usize) -> impl DoubleEndedIterator<Item = &[T]> {
        self.grid.chunks(chunk_size)
    }

    /// Returns an iterator over `chunk_len` mutable references to elements in *row major order*.
    ///
    /// # Panics
    /// If `chunk_size` is 0.
    #[inline]
    pub fn chunks_iter_ref_mut(
        &mut self,
        chunk_size: usize,
    ) -> impl DoubleEndedIterator<Item = &mut [T]> {
        self.grid.chunks_mut(chunk_size)
    }
}

/// # iterators (Copy)
impl<T: Copy, S: Storage, const SIZE: usize> Grid2D<T, S, SIZE> {
    // all elements iter

    /// Returns an iterator over copies of all elements in *row major order*.
    #[inline]
    pub fn iter(&self) -> impl DoubleEndedIterator<Item = T> + '_ {
        self.grid.iter().copied()
    }

    // MAYBE FIX
    // /// Returns an iterator over references to all elements in *col major order*.
    // #[inline]
    // pub fn iter_col_order(&self) -> impl DoubleEndedIterator<Item = T> + '_ {
    //     (0..self.cols).flat_map(move |col| (0..self.rows).map(move |row| self[(col, row)]))
    // }

    // row iter

    /// Returns an iterator over references to all elements in the given row.
    #[inline]
    pub fn row_iter(&self, row: usize) -> Result<impl DoubleEndedIterator<Item = T> + '_> {
        let start = self.get_index(0, row)?;
        let end = start + self.row_len();
        Ok(self.grid[start..end].iter().copied())
    }

    /// Returns an iterator over references to all elements in the given row.
    ///
    /// # Panics
    /// If out of bounds.
    #[inline]
    pub fn row_iter_unchecked(&self, row: usize) -> impl DoubleEndedIterator<Item = T> + '_ {
        let start = self.get_index_unchecked(0, row);
        let end = start + self.row_len();
        self.grid[start..end].iter().copied()
    }

    // /// Returns an iterator over references to all elements in the given row,
    // /// starting on the given `col`umn, for `len` elements.
    // //
    // // FIX: do not go over the row len limit
    // #[inline]
    // pub fn row_iter_bounded(
    //     &self,
    //     row: usize,
    //     col: usize,
    //     len: usize,
    // ) -> Result<impl DoubleEndedIterator<Item = T> + '_> {
    //     println!("\nrow_iter_bounded → row:{row}, col:{col}, len:{len}");
    //     println!(
    //         "grid row_len:{}, col_len:{} cap:{}",
    //         self.row_len(),
    //         self.col_len(),
    //         self.len()
    //     );
    //
    //     let start = self.get_index(col, row)?;
    //     println!("start: ({row},{col}) = index {start}");
    //     let end = min(start + len, start + self.row_len());
    //     println!("end = min({0}+{1}, {0}+{2})", start, len, self.row_len());
    //
    //     let len2 = min(len, self.row_len().saturating_sub(col));
    //     println!("len1:{len} vs len2:{len2}");
    //
    //     println!();
    //
    //     // THINK:
    //     // EXAMPLE for row_len:4
    //     // len
    //
    //     // let end = min(start + len, start + self.row_len().saturating_sub(len));
    //     // println!("end = min({0}+{1}, {0}+{2})", start, len, self.row_len().saturating_sub(len));
    //
    //     // let len = min(self.row_len().saturating_sub(len)); // TODO: row_len - len
    //     // let num_cells = min(cells.len(), self.row_len().saturating_sub(col as usize)); // TEMP
    //     Ok(self.grid[start..end].iter().copied())
    // }
    // TODO: row_iter_bounded_unchecked

    // column iter

    // MAYBE FIX
    // /// Returns an iterator over copies of all elements in the given `col`umn.
    // #[inline]
    // pub fn col_iter(&self, col: usize) -> Result<impl DoubleEndedIterator<Item = T> + '_> {
    //     if col >= self.cols {
    //         return Err(Error::Indices2dOutOfBounds(0, col));
    //     }
    //     Ok((0..self.col_len()).map(move |row| self[(col, row)]))
    // }
    // /// Returns an iterator over copies of all elements in the given `col`umn.
    // ///
    // /// # Panics
    // /// If out of bounds.
    // #[inline]
    // pub fn col_iter_unchecked(&self, col: usize) -> impl DoubleEndedIterator<Item = T> + '_ {
    //     (0..self.col_len()).map(move |row| self[(col, row)])
    // }

    /// Returns an iterator over all rows.
    ///
    /// Each `Item` is itself another `Iterator` over copies of the elements in that column.
    #[inline]
    pub fn rows_iter(
        &self,
    ) -> impl DoubleEndedIterator<Item = impl DoubleEndedIterator<Item = T> + '_> {
        (0..self.rows).map(move |row| self.row_iter(row).expect("rows_iter should never fail"))
    }

    // MAYBE FIX
    // /// Returns an iterator over all columns.
    // ///
    // /// Each `Item` is itself another `Iterator` over copies of the elements in that column.
    // #[inline]
    // pub fn cols_iter(
    //     &self,
    // ) -> impl DoubleEndedIterator<Item = impl DoubleEndedIterator<Item = T> + '_> {
    //     (0..self.cols).map(move |col| self.col_iter(col).expect("cols_iter should never fail"))
    // }
}

/// # collecting to Vec
#[cfg(feature = "std")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "std")))]
impl<T: Clone, S: Storage, const SIZE: usize> Grid2D<T, S, SIZE> {
    /// Collects the `Grid2D` into a `Vec` of rows.
    #[inline]
    pub fn as_rows(&self) -> Vec<Vec<T>> {
        self.rows_iter_ref()
            .map(|row_iter| row_iter.cloned().collect())
            .collect()
    }

    // /// Collects the `Grid2D` into a `Vec` of columns.
    // #[inline]
    // pub fn as_cols(&self) -> Vec<Vec<T>> {
    //     self.cols_iter_ref()
    //         .map(|col_iter| col_iter.cloned().collect())
    //         .collect()
    // }

    /// Collects the `Grid2D` into a `Vec` of elements in *row major order*.
    #[inline]
    pub fn as_row_order(&self) -> Vec<T> {
        self.iter_ref().cloned().collect()
    }

    // /// Collects the `Grid2D` into a `Vec` of elements in *column major order*.
    // #[inline]
    // pub fn as_col_order(&self) -> Vec<T> {
    //     self.iter_ref_col_order().cloned().collect()
    // }
}

/// # exposing the inner Vec
impl<T, S: Storage, const SIZE: usize> Grid2D<T, S, SIZE> {
    /// Returns the underlying `Array`.
    #[inline]
    pub fn into_array(self) -> Array<T, S, SIZE> {
        self.grid
    }

    /// Returns a shared reference to the underlying `Array`.
    #[inline]
    pub fn ref_array(&self) -> &Array<T, S, SIZE> {
        &self.grid
    }

    /// Returns an exclusive reference to the underlying `Array`.
    #[inline]
    pub fn mut_array(&mut self) -> &mut Array<T, S, SIZE> {
        &mut self.grid
    }
}

/// # slices
impl<T, S: Storage, const SIZE: usize> Grid2D<T, S, SIZE> {
    /// Returns a slice of the grid.
    ///
    /// # Examples
    /// ```ignore
    /// use ladata::grid::DirectGrid2D;
    ///
    // let mut s = DirectGrid2D::<_, 3>::from([1, 2, 3]);
    // assert_eq![s.as_mut_slice(), &mut [1, 2, 3]];
    /// ```
    // TODO FIX: example
    #[inline]
    pub fn as_slice(&self) -> &[T] {
        self.grid.as_slice()
    }

    /// Returns a mutable slice of the grid.
    #[inline]
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        self.grid.as_mut_slice()
    }

    /// Returns a slice of requested `row`.
    #[inline]
    pub fn row_slice(&self, row: usize) -> Result<&[T]> {
        let start = self.get_index(0, row)?;
        let end = start + self.row_len();
        Ok(&self.grid[start..end])
    }

    /// Returns a slice of requested `row`.
    ///
    /// # Panics
    /// If out of bounds.
    #[inline]
    pub fn row_slice_unchecked(&self, row: usize) -> &[T] {
        let start = self.get_index_unchecked(0, row);
        let end = start + self.row_len();
        &self.grid[start..end]
    }

    /// Returns a mutable slice of requested `row`.
    #[inline]
    pub fn row_mut_slice(&mut self, row: usize) -> Result<&mut [T]> {
        let start = self.get_index(0, row)?;
        let end = start + self.row_len();
        Ok(&mut self.grid[start..end])
    }

    /// Returns a mutable slice of requested `row`.
    ///
    /// # Panics
    /// If out of bounds.
    #[inline]
    pub fn row_mut_slice_unchecked(&mut self, row: usize) -> &mut [T] {
        let start = self.get_index_unchecked(0, row);
        let end = start + self.row_len();
        &mut self.grid[start..end]
    }
}

/// # get chunks
impl<T, S: Storage, const SIZE: usize> Grid2D<T, S, SIZE> {
    /// Returns a slice of the chunk of elements at the given `row` and `col`umn.
    #[inline]
    pub fn get_chunk(&self, chunk_len: usize, col: usize, row: usize) -> Result<&[T]> {
        self.get_chunk_index(chunk_len, col, row)
            .map(|index| &self.grid[index..index + chunk_len])
    }
    /// Returns a slice of the chunk of elements at the given `row` and `col`umn.
    ///
    /// # Panics
    /// If out of bounds.
    #[inline]
    pub fn get_chunk_unchecked(&self, chunk_len: usize, col: usize, row: usize) -> &[T] {
        let index = self.get_chunk_index_unchecked(chunk_len, col, row);
        &self.grid[index..index + chunk_len]
    }

    /// Returns a mutable slice of the chunk of elements at the given `row` and `col`umn.
    #[inline]
    pub fn get_chunk_mut(&mut self, chunk_len: usize, col: usize, row: usize) -> Result<&mut [T]> {
        self.get_chunk_index(chunk_len, col, row)
            .map(move |index| &mut self.grid[index..index + chunk_len])
    }

    /// Returns a mutable slice of the chunk of elements at the given `row` and `col`umn.
    ///
    /// # Panics
    /// If out of bounds.
    #[inline]
    pub fn get_chunk_mut_unchecked(
        &mut self,
        chunk_len: usize,
        row: usize,
        col: usize,
    ) -> &mut [T] {
        let index = self.get_chunk_index_unchecked(chunk_len, col, row);
        &mut self.grid[index..index + chunk_len]
    }
}

// T: Clone
/// # set chunks
impl<T: Clone, S: Storage, const SIZE: usize> Grid2D<T, S, SIZE> {
    /// Sets the elements on a chunk.
    #[inline]
    pub fn set_chunk(
        &mut self,
        chunk_len: usize,
        row: usize,
        col: usize,
        elements: &[T],
    ) -> Result<()> {
        let chunk = self.get_chunk_mut(chunk_len, col, row)?;
        for (n, element) in chunk.iter_mut().enumerate() {
            *element = elements[n].clone();
        }
        Ok(())
    }

    /// Sets the elements on a chunk.
    ///
    /// # Panics
    /// If out of bounds.
    #[inline]
    pub fn set_chunk_unchecked(
        &mut self,
        chunk_len: usize,
        row: usize,
        col: usize,
        elements: &[T],
    ) {
        let chunk = self.get_chunk_mut_unchecked(chunk_len, col, row);
        for (n, element) in chunk.iter_mut().enumerate() {
            *element = elements[n].clone();
        }
    }
}

mod core_impls {
    use super::{Array, Grid2D, Index, IndexMut, Storage};
    use core::{any::type_name, fmt};

    // T:Clone
    impl<T: Clone, S: Storage, const SIZE: usize> Clone for Grid2D<T, S, SIZE>
    where
        S::Stored<[T; SIZE]>: Clone,
    {
        fn clone(&self) -> Self {
            Self {
                cols: self.cols,
                rows: self.rows,
                grid: self.grid.clone(),
            }
        }
    }

    // T:Copy
    impl<T: Copy, S: Storage, const SIZE: usize> Copy for Grid2D<T, S, SIZE> where
        S::Stored<[T; SIZE]>: Copy
    {
    }

    // T:PartialEq
    impl<T: PartialEq, S: Storage, const SIZE: usize> PartialEq for Grid2D<T, S, SIZE>
    where
        S::Stored<[T; SIZE]>: PartialEq,
    {
        fn eq(&self, other: &Self) -> bool {
            self.grid == other.grid && self.cols == other.cols && self.rows == other.rows
        }
    }
    // T:Eq
    impl<T: Eq, S: Storage, const SIZE: usize> Eq for Grid2D<T, S, SIZE> where S::Stored<[T; SIZE]>: Eq {}

    // Debug
    impl<T, S: Storage, const SIZE: usize> fmt::Debug for Grid2D<T, S, SIZE> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(
                f,
                "Grid2D {{ {}×{}, {} }}",
                self.rows,
                self.cols,
                type_name::<T>()
            )
        }
    }

    // S:() + T:Default
    impl<T: Default, const SIZE: usize> Default for Grid2D<T, (), SIZE> {
        /// Returns a square grid of side `SIZE / 2`, allocated in the stack,
        /// using the default value to fill the data.
        ///
        /// # Panics
        /// If `SIZE` is not even.
        ///
        /// # Examples
        /// ```
        /// use ladata::grid::DirectGrid2D;
        ///
        /// let g = DirectGrid2D::<u8, 128>::default();
        /// assert_eq![g.num_cols(), 64];
        /// assert_eq![g.num_rows(), 64];
        /// ```
        fn default() -> Self {
            assert_eq![0, SIZE % 2];
            Self {
                cols: SIZE / 2,
                rows: SIZE / 2,
                grid: Array::default(),
            }
        }
    }

    impl<T, S: Storage, const SIZE: usize> Index<(usize, usize)> for Grid2D<T, S, SIZE> {
        type Output = T;
        fn index(&self, (col, row): (usize, usize)) -> &Self::Output {
            self.get_ref(col, row)
                .unwrap_or_else(|_| panic!("Index indices {}, {} out of bounds", col, row))
        }
    }

    impl<T, S: Storage, const SIZE: usize> IndexMut<(usize, usize)> for Grid2D<T, S, SIZE> {
        fn index_mut(&mut self, (col, row): (usize, usize)) -> &mut Self::Output {
            self.get_ref_mut(col, row)
                .unwrap_or_else(|_| panic!("Index mut indices {}, {} out of bounds", col, row))
        }
    }
}

#[cfg(feature = "std")]
mod std_impls {
    use super::{Array, Boxed, Grid2D};

    // S:Boxed + T:Default
    #[cfg(feature = "std")]
    #[cfg_attr(feature = "nightly", doc(cfg(feature = "std")))]
    impl<T: Default, const SIZE: usize> Default for Grid2D<T, Boxed, SIZE> {
        /// Returns a square grid of side `SIZE / 2`, allocated in the stack,
        /// using the default value to fill the data.
        ///
        /// # Panics
        /// If `SIZE` is not even.
        ///
        /// # Examples
        /// ```
        /// use ladata::grid::BoxedGrid2D;
        ///
        /// let g = BoxedGrid2D::<u8, 1_024>::default();
        /// assert_eq![g.num_cols(), 512];
        /// assert_eq![g.num_rows(), 512];
        /// ```
        fn default() -> Self {
            assert_eq![0, SIZE % 2];
            Self {
                cols: SIZE / 2,
                rows: SIZE / 2,
                grid: Array::default(),
            }
        }
    }
}
