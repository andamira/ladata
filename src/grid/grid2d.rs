// ladata::grid::grid2d
//
//!
//

use crate::error::{LadataError as Error, LadataResult as Result};
use core::{
    cmp::min,
    ops::{Index, IndexMut},
};

/// A dynamic 2D grid, abstracted over a [`Vec`].
///
/// Internally the elements are stored in *row major order*,
/// meaning the elements of each row are stored sequentially.
#[derive(Clone, PartialEq, Eq)]
pub struct Grid2d<T> {
    rows: usize,
    cols: usize,
    grid: Vec<T>,
}

/// # constructors
impl<T: Clone> Grid2d<T> {
    /// Creates a new `Grid2d` with a size of rows, cols, filled with `element`.
    #[inline]
    pub fn new(element: T, rows: usize, cols: usize) -> Self {
        Self {
            grid: vec![element; rows * cols],
            rows,
            cols,
        }
    }

    /// Creates a new `Grid2d` from a slice of rows.
    ///
    /// All rows must have the same length.
    pub fn from_rows(rows: &[Vec<T>]) -> Result<Self> {
        let row_len = rows.get(0).map(Vec::len).unwrap_or(0);
        if !rows.iter().all(|row| row.len() == row_len) {
            return Err(Error::DimensionMismatch);
        }
        Ok(Self {
            grid: Self::flatten(rows),
            rows: rows.len(),
            cols: row_len,
        })
    }

    /// Creates a new `Grid2d` from a slice of `columns`.
    ///
    /// All columns must have the same length.
    pub fn from_cols(columns: &[Vec<T>]) -> Result<Self> {
        let col_len = columns.get(0).map(Vec::len).unwrap_or(0);
        if !columns.iter().all(|col| col.len() == col_len) {
            return Err(Error::DimensionMismatch);
        }
        let rows = col_len;
        let cols = columns.len();
        let indices_row_order = (0..rows).flat_map(move |row| (0..cols).map(move |col| (row, col)));
        let grid = indices_row_order
            .map(|(row, col)| columns[col][row].clone())
            .collect();
        Ok(Grid2d { grid, rows, cols })
    }

    /// Creates a new `Grid2d` from the given flat slice of `elements`, in *row major order*.
    ///
    /// The number of `elements` must equal `rows`×`cols`.
    pub fn from_row_order(elements: &[T], rows: usize, cols: usize) -> Result<Self> {
        let total_len = rows * cols;
        if total_len != elements.len() {
            return Err(Error::DimensionMismatch);
        }
        Ok(Self {
            grid: elements.to_vec(),
            rows,
            cols,
        })
    }

    /// Creates a new `Grid2d` from the given flat slice of `elements`, in *column major order*.
    ///
    /// The number of `elements` must equal `rows`×`cols`.
    pub fn from_col_order(elements: &[T], rows: usize, cols: usize) -> Result<Self> {
        let total_len = rows * cols;
        if total_len != elements.len() {
            return Err(Error::DimensionMismatch);
        }
        let indices_row_order =
            (0..rows).flat_map(move |row| (0..cols).map(move |column| (row, column)));
        let grid = indices_row_order
            .map(|(row, column)| {
                let index = column * rows + row;
                elements[index].clone()
            })
            .collect();
        Ok(Grid2d { grid, rows, cols })
    }

    /// Creates a new `Grid2d` with the specified number of `rows` and `col`umn`s`,
    /// filling each element with the result of calling the given function.
    ///
    /// The `function` is called once for every location going in *row major order*.
    pub fn from_fn_row_order<F: FnMut() -> T>(mut function: F, rows: usize, cols: usize) -> Self {
        let len = rows * cols;
        let grid = (0..len).map(|_| function()).collect();
        Grid2d { grid, rows, cols }
    }

    /// Creates a new `Grid2d` with the specified number of `rows` and `col`umn`s`,
    /// filling each element with the result of calling the given function.
    ///
    /// The `function` is called once for every location going in *column major order*.
    pub fn from_fn_col_order<F: FnMut() -> T>(mut function: F, rows: usize, cols: usize) -> Self {
        let len = rows * cols;
        let grid_col_order = (0..len).map(|_| function()).collect::<Vec<_>>();
        Grid2d::from_col_order(&grid_col_order, rows, cols)
            .expect("from_fn_col_order should never fail")
    }

    /// Creates a new `Grid2d` with the specified number of `rows` and `col`umn`s`,
    /// filling each element with the elements produced by the provided `iterator`.
    ///
    /// The elements are inserted into the grid in *row major order*.
    pub fn from_iter_row_order<I>(iterator: I, rows: usize, cols: usize) -> Result<Self>
    where
        I: Iterator<Item = T>,
    {
        let len = rows * cols;
        let grid = iterator.take(len).collect::<Vec<_>>();
        if grid.len() < len {
            return Err(Error::NotEnoughElements(len));
        }
        Ok(Grid2d { grid, rows, cols })
    }

    /// Creates a new `Grid2d` with the specified number of `rows` and `col`umn`s`,
    /// filling each element with the elements produced by the provided `iterator`.
    ///
    /// The elements are inserted into the grid in *column major order*.
    pub fn from_iter_col_order<I>(iterator: I, rows: usize, cols: usize) -> Result<Self>
    where
        I: Iterator<Item = T>,
    {
        let total_len = rows * cols;
        let grid_col_order = iterator.take(total_len).collect::<Vec<_>>();
        Grid2d::from_col_order(&grid_col_order, rows, cols)
            .map_err(|_| Error::NotEnoughElements(total_len))
    }

    // Flattens a slice of vecs into a single vec.
    #[inline]
    fn flatten(nested: &[Vec<T>]) -> Vec<T> {
        nested.iter().flat_map(|row| row.clone()).collect()
    }
}

/// # constructors (Copy)
impl<T: Copy> Grid2d<T> {
    /// Creates a new `Grid2d` by concatenating the elements inside `chunk`,
    /// with a total capacity of `rows`×`cols`×`chunk.len()`.
    pub fn from_chunks(chunk: &[T], rows: usize, cols: usize) -> Self {
        let grid = chunk.repeat(rows * cols);
        Grid2d { grid, rows, cols }
    }
}

/// # general query methods
impl<T> Grid2d<T> {
    /// Returns the capacity of the grid (`rows` × `cols`).
    #[inline]
    pub const fn capacity(&self) -> usize {
        self.rows * self.cols
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

    /// Translates 2D `row`,`col` coordinates into a 1D column index.
    #[inline]
    pub const fn get_index(&self, row: usize, col: usize) -> Result<usize> {
        if row < self.rows && col < self.cols {
            Ok(row * self.row_len() + col)
        } else {
            Err(Error::Indices2dOutOfBounds(row, col))
        }
    }
    /// Translates 2D `row`,`col` coordinates into a 1D column index.
    /// Panics if out of bounds.
    #[inline]
    pub const fn get_index_unchecked(&self, row: usize, col: usize) -> usize {
        row * self.row_len() + col
    }

    /// Translates 1D column index into 2D `row`,`col` coordinates.
    #[inline]
    pub const fn get_coords(&self, index: usize) -> Result<(usize, usize)> {
        if index < self.capacity() {
            Ok((index / self.cols, index % self.cols))
        } else {
            Err(Error::IndexOutOfBounds(index))
        }
    }
    /// Translates 1D column index into 2D `row`,`col` coordinates.
    /// Panics if out of bounds.
    #[inline]
    pub const fn get_coords_unchecked(&self, index: usize) -> (usize, usize) {
        (index / self.cols, index % self.cols)
    }

    // chunks

    /// Returns the number of chunks (`capacity()`/`chunk_len`).
    #[inline]
    pub const fn chunked_capacity(&self, chunk_len: usize) -> usize {
        self.capacity() / chunk_len
    }

    /// Returns the number of chunks per row.
    #[inline]
    pub const fn chunks_per_row(&self, chunk_len: usize) -> usize {
        self.row_len() / chunk_len
    }

    /// Translates 2D `row`,`col` coordinates, with chunk length, into a 1D column index.
    ///
    /// - it assumes the `row_len` to be an exact multiple of `chunk_len`.
    /// - only full chunks are allowed.
    pub const fn get_chunk_index(&self, chunk_len: usize, row: usize, col: usize) -> Result<usize> {
        if row < self.rows && col < (self.cols / chunk_len) {
            Ok(row * self.row_len() + col * chunk_len)
        } else {
            Err(Error::Indices2dOutOfBounds(row, col))
        }
    }

    /// Translates 2D `row`,`col` coordinates, with chunk length, into a 1D column index.
    /// Panics if out of bounds.
    pub const fn get_chunk_index_unchecked(
        &self,
        chunk_len: usize,
        row: usize,
        col: usize,
    ) -> usize {
        row * self.row_len() + col * chunk_len
    }
}

/// # single element get/set methods
impl<T> Grid2d<T> {
    // get_ref

    /// Returns a reference to the element at the given `row` and `col`umn.
    #[inline]
    pub fn get_ref(&self, row: usize, col: usize) -> Result<&T> {
        self.get_index(row, col).map(|idx| &self.grid[idx])
    }
    /// Returns a reference to the element at the given `row` and `col`umn.
    /// Panics if out of bounds.
    #[inline]
    pub fn get_ref_unchecked(&self, row: usize, col: usize) -> &T {
        &self.grid[self.get_index_unchecked(row, col)]
    }

    /// Returns a reference to the element at the given 1D index, in *row major order*.
    #[inline]
    pub fn get_ref_row_order(&self, index: usize) -> Result<&T> {
        if index < self.capacity() {
            Ok(&self.grid[index])
        } else {
            Err(Error::IndexOutOfBounds(index))
        }
    }
    /// Returns a reference to the element at the given 1D index, in *row major order*.
    /// Panics if out of bounds.
    #[inline]
    pub fn get_ref_row_order_unchecked(&self, index: usize) -> &T {
        &self.grid[index]
    }

    /// Returns a reference to the element at the given 1D index, in *column major order*.
    #[inline]
    pub fn get_ref_col_order(&self, index: usize) -> Result<&T> {
        if index < self.capacity() {
            Ok(self.get_ref_col_order_unchecked(index))
        } else {
            Err(Error::IndexOutOfBounds(index))
        }
    }
    /// Returns a reference to the element at the given 1D index, in *column major order*.
    /// Panics if out of bounds.
    #[inline]
    pub fn get_ref_col_order_unchecked(&self, index: usize) -> &T {
        let col = index / self.rows;
        let row = index % self.rows;
        self.get_ref_unchecked(row, col)
    }

    // get mut

    /// Returns a mutable reference to the element at the given `row` and `col`umn.
    #[inline]
    pub fn get_ref_mut(&mut self, row: usize, col: usize) -> Result<&mut T> {
        self.get_index(row, col).map(|idx| &mut self.grid[idx])
    }
    /// Returns a mutable reference to the element at the given `row` and `col`umn.
    /// Panics if any out of bounds.
    #[inline]
    pub fn get_ref_mut_unchecked(&mut self, row: usize, col: usize) -> &mut T {
        let idx = self.get_index_unchecked(row, col);
        &mut self.grid[idx]
    }

    /// Returns a mutable reference to the element at the given 1D index, in *row major order*.
    #[inline]
    pub fn get_ref_mut_row_order(&mut self, index: usize) -> Result<&mut T> {
        if index < self.capacity() {
            Ok(&mut self.grid[index])
        } else {
            Err(Error::IndexOutOfBounds(index))
        }
    }

    /// Returns a mutable reference to the element at the given 1D index, in *row major order*.
    /// Panics if out of bounds.
    #[inline]
    pub fn get_ref_mut_row_order_unchecked(&mut self, index: usize) -> &mut T {
        &mut self.grid[index]
    }

    /// Returns a mutable reference to the element at the given 1D index, in *column major order*.
    #[inline]
    pub fn get_ref_mut_col_order(&mut self, index: usize) -> Result<&mut T> {
        if index < self.capacity() {
            Ok(self.get_ref_mut_col_order_unchecked(index))
        } else {
            Err(Error::IndexOutOfBounds(index))
        }
    }

    /// Returns a mutable reference to the element at the given 1D index, in *column major order*.
    /// Panics if out of bounds.
    #[inline]
    pub fn get_ref_mut_col_order_unchecked(&mut self, index: usize) -> &mut T {
        let col = index / self.rows;
        let row = index % self.rows;
        self.get_ref_mut_unchecked(row, col)
    }

    // set

    /// Sets the `element` at the given `row` and `col`umn.
    #[inline]
    pub fn set(&mut self, element: T, row: usize, col: usize) -> Result<()> {
        self.get_ref_mut(row, col).map(|index| {
            *index = element;
        })
    }
    /// Sets the `element` at the given `row` and `col`umn.
    /// Panics if any out of bounds.
    #[inline]
    pub fn set_unchecked(&mut self, element: T, row: usize, col: usize) {
        let index = self.get_ref_mut_unchecked(row, col);
        *index = element;
    }

    /// Sets the `element` at the given 1D index, in *row major order*.
    #[inline]
    pub fn set_row_order(&mut self, element: T, index: usize) -> Result<()> {
        self.get_ref_mut_row_order(index).map(|index| {
            *index = element;
        })
    }
    /// Sets the element at the given 1D index, in *row major order*.
    /// Panics if out of bounds.
    #[inline]
    pub fn set_row_order_unchecked(&mut self, element: T, index: usize) {
        *self.get_ref_mut_row_order_unchecked(index) = element;
    }

    /// Sets the element at the given 1D index, in *column major order*.
    #[inline]
    pub fn set_col_order(&mut self, element: T, index: usize) -> Result<()> {
        self.get_ref_mut_col_order(index).map(|index| {
            *index = element;
        })
    }
    /// Returns a mutable reference to the element at the given 1D index, in *column major order*.
    /// Panics if out of bounds.
    #[inline]
    pub fn set_col_order_unchecked(&mut self, element: T, index: usize) {
        *self.get_ref_mut_col_order_unchecked(index) = element;
    }
}

/// # single element get methods (Copy)
impl<T: Copy> Grid2d<T> {
    // get

    /// Returns the element at the given `row` and `col`umn.
    #[inline]
    pub fn get(&self, row: usize, col: usize) -> Result<T> {
        self.get_index(row, col).map(|idx| self.grid[idx])
    }
    /// Returns the element at the given `row` and `col`umn.
    /// Panics if out of bounds.
    #[inline]
    pub fn get_unchecked(&self, row: usize, col: usize) -> T {
        self.grid[self.get_index_unchecked(row, col)]
    }

    /// Returns the element at the given 1D index, in *row major order*.
    #[inline]
    pub fn get_row_order(&self, index: usize) -> Result<T> {
        if index < self.capacity() {
            Ok(self.grid[index])
        } else {
            Err(Error::IndexOutOfBounds(index))
        }
    }
    /// Returns the element at the given 1D index, in *row major order*.
    /// Panics if out of bounds.
    #[inline]
    pub fn get_row_order_unchecked(&self, index: usize) -> T {
        self.grid[index]
    }

    /// Returns the element at the given 1D index, in *column major order*.
    #[inline]
    pub fn get_col_order(&self, index: usize) -> Result<T> {
        if index < self.capacity() {
            Ok(self.get_col_order_unchecked(index))
        } else {
            Err(Error::IndexOutOfBounds(index))
        }
    }
    /// Returns the element at the given 1D index, in *column major order*.
    /// Panics if out of bounds.
    #[inline]
    pub fn get_col_order_unchecked(&self, index: usize) -> T {
        let col = index / self.rows;
        let row = index % self.rows;
        self.get_unchecked(row, col)
    }
}

/// # iterators
impl<T> Grid2d<T> {
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

    /// Returns an iterator over references to all elements in *col major order*.
    #[inline]
    pub fn iter_ref_col_order(&self) -> impl DoubleEndedIterator<Item = &T> {
        (0..self.cols).flat_map(move |col| (0..self.rows).map(move |row| &self[(row, col)]))
    }

    // // TODO FIXME
    // /// Returns an iterator over mutable references to all elements in *col major order*.
    // pub fn iter_mut_col_order(&mut self) -> impl Iterator<Item = &mut T> {
    // }

    // row iter

    /// Returns an iterator over references to all elements in the given row.
    #[inline]
    pub fn row_iter_ref(&self, row: usize) -> Result<impl DoubleEndedIterator<Item = &T>> {
        let start = self.get_index(row, 0)?;
        let end = start + self.row_len();
        Ok(self.grid[start..end].iter())
    }
    /// Returns an iterator over references to all elements in the given row.
    /// Panics if out of bounds.
    #[inline]
    pub fn row_iter_ref_unchecked(&self, row: usize) -> impl DoubleEndedIterator<Item = &T> {
        let start = self.get_index_unchecked(row, 0);
        let end = start + self.row_len();
        self.grid[start..end].iter()
    }

    /// Returns an iterator over mutable references to all elements in the given row.
    #[inline]
    pub fn row_iter_ref_mut(
        &mut self,
        row: usize,
    ) -> Result<impl DoubleEndedIterator<Item = &mut T>> {
        let start = self.get_index(row, 0)?;
        let end = start + self.row_len();
        Ok(self.grid[start..end].iter_mut())
    }
    /// Returns an iterator over mutable references to all elements in the given row.
    /// Panics if out of bounds.
    #[inline]
    pub fn row_iter_ref_mut_unchecked(
        &mut self,
        row: usize,
    ) -> impl DoubleEndedIterator<Item = &mut T> {
        let start = self.get_index_unchecked(row, 0);
        let end = start + self.row_len();
        self.grid[start..end].iter_mut()
    }

    // column iter

    /// Returns an iterator over references to all elements in the given `col`umn.
    #[inline]
    pub fn col_iter_ref(&self, col: usize) -> Result<impl DoubleEndedIterator<Item = &T>> {
        if col >= self.cols {
            return Err(Error::Indices2dOutOfBounds(0, col));
        }
        Ok((0..self.col_len()).map(move |row| &self[(row, col)]))
    }
    /// Returns an iterator over references to all elements in the given `col`umn.
    /// Panics if out of bounds.
    #[inline]
    pub fn col_iter_ref_unchecked(&self, col: usize) -> impl DoubleEndedIterator<Item = &T> {
        (0..self.col_len()).map(move |row| &self[(row, col)])
    }

    /// Returns an iterator over references to all elements in the given row.
    // IMPROVE: DoubleEndedIterator?
    #[inline]
    pub fn col_iter_ref_mut(&mut self, col: usize) -> Result<impl Iterator<Item = &mut T>> {
        if col >= self.cols {
            return Err(Error::Indices2dOutOfBounds(0, col));
        }
        let col_len = self.col_len();
        Ok(self.iter_ref_mut().skip(col).step_by(col_len))
    }
    /// Returns an iterator over references to all elements in the given row.
    /// Panics if out of bounds.
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

    // TODO MAYBE FIX cannot infer an appropriate lifetime for autoref due to conflicting requirements
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

    /// Returns an iterator over all columns.
    ///
    /// Each `Item` is itself another `Iterator` over references to the elements in that column.
    #[inline]
    pub fn cols_iter_ref(
        &self,
    ) -> impl DoubleEndedIterator<Item = impl DoubleEndedIterator<Item = &T>> {
        (0..self.cols).map(move |col| self.col_iter_ref(col).expect("cols_iter should never fail"))
    }

    // TODO MAYBE
    // pub fn cols_iter_ref_mut(&mut self) {}

    // chunks iter

    /// Returns an iterator over `chunk_len` references to elements in *row major order*.
    /// Panics if `chunk_size` is 0.
    #[inline]
    pub fn chunks_iter_ref(&self, chunk_size: usize) -> impl DoubleEndedIterator<Item = &[T]> {
        self.grid.chunks(chunk_size)
    }

    /// Returns an iterator over `chunk_len` mutable references to elements in *row major order*.
    /// Panics if `chunk_size` is 0.
    #[inline]
    pub fn chunks_iter_ref_mut(
        &mut self,
        chunk_size: usize,
    ) -> impl DoubleEndedIterator<Item = &mut [T]> {
        self.grid.chunks_mut(chunk_size)
    }
}

/// # iterators (Copy)
impl<T: Copy> Grid2d<T> {
    // all elements iter

    /// Returns an iterator over copies of all elements in *row major order*.
    #[inline]
    pub fn iter(&self) -> impl DoubleEndedIterator<Item = T> + '_ {
        self.grid.iter().copied()
    }

    /// Returns an iterator over references to all elements in *col major order*.
    #[inline]
    pub fn iter_col_order(&self) -> impl DoubleEndedIterator<Item = T> + '_ {
        (0..self.cols).flat_map(move |col| (0..self.rows).map(move |row| self[(row, col)]))
    }

    // row iter

    /// Returns an iterator over references to all elements in the given row.
    #[inline]
    pub fn row_iter(&self, row: usize) -> Result<impl DoubleEndedIterator<Item = T> + '_> {
        let start = self.get_index(row, 0)?;
        let end = start + self.row_len();
        Ok(self.grid[start..end].iter().copied())
    }

    /// Returns an iterator over references to all elements in the given row.
    /// Panics if out of bounds.
    #[inline]
    pub fn row_iter_unchecked(&self, row: usize) -> impl DoubleEndedIterator<Item = T> + '_ {
        let start = self.get_index_unchecked(row, 0);
        let end = start + self.row_len();
        self.grid[start..end].iter().copied()
    }

    /// Returns an iterator over references to all elements in the given row,
    /// starting on the given `col`umn, for `len` elements.
    //
    // TODO FIXME: do not go over the row len limit
    #[inline]
    pub fn row_iter_bounded(
        &self,
        row: usize,
        col: usize,
        len: usize,
    ) -> Result<impl DoubleEndedIterator<Item = T> + '_> {
        println!("\nrow_iter_bounded → row:{row}, col:{col}, len:{len}");
        println!(
            "grid row_len:{}, col_len:{} cap:{}",
            self.row_len(),
            self.col_len(),
            self.capacity()
        );

        let start = self.get_index(row, col)?;
        println!("start: ({row},{col}) = index {start}");
        let end = min(start + len, start + self.row_len());
        println!("end = min({0}+{1}, {0}+{2})", start, len, self.row_len());

        let len2 = min(len, self.row_len().saturating_sub(col));
        println!("len1:{len} vs len2:{len2}");

        println!();

        // THINK:
        // EXAMPLE for row_len:4
        // len

        // let end = min(start + len, start + self.row_len().saturating_sub(len));
        // println!("end = min({0}+{1}, {0}+{2})", start, len, self.row_len().saturating_sub(len));

        // let len = min(self.row_len().saturating_sub(len)); // TODO: row_len - len
        // let num_cells = min(cells.len(), self.row_len().saturating_sub(col as usize)); // TEMP
        Ok(self.grid[start..end].iter().copied())
    }
    // TODO: row_iter_bounded_unchecked

    // column iter

    /// Returns an iterator over copies of all elements in the given `col`umn.
    #[inline]
    pub fn col_iter(&self, col: usize) -> Result<impl DoubleEndedIterator<Item = T> + '_> {
        if col >= self.cols {
            return Err(Error::Indices2dOutOfBounds(0, col));
        }
        Ok((0..self.col_len()).map(move |row| self[(row, col)]))
    }
    /// Returns an iterator over copies of all elements in the given `col`umn.
    /// Panics if out of bounds.
    #[inline]
    pub fn col_iter_unchecked(&self, col: usize) -> impl DoubleEndedIterator<Item = T> + '_ {
        (0..self.col_len()).map(move |row| self[(row, col)])
    }

    /// Returns an iterator over all rows.
    ///
    /// Each `Item` is itself another `Iterator` over copies of the elements in that column.
    #[inline]
    pub fn rows_iter(
        &self,
    ) -> impl DoubleEndedIterator<Item = impl DoubleEndedIterator<Item = T> + '_> {
        (0..self.rows).map(move |row| self.row_iter(row).expect("rows_iter should never fail"))
    }

    /// Returns an iterator over all columns.
    ///
    /// Each `Item` is itself another `Iterator` over copies of the elements in that column.
    #[inline]
    pub fn cols_iter(
        &self,
    ) -> impl DoubleEndedIterator<Item = impl DoubleEndedIterator<Item = T> + '_> {
        (0..self.cols).map(move |col| self.col_iter(col).expect("cols_iter should never fail"))
    }
}

/// # collecting to Vec
impl<T: Clone> Grid2d<T> {
    /// Collects the `Grid2d` into a `Vec` of rows.
    #[inline]
    pub fn as_rows(&self) -> Vec<Vec<T>> {
        self.rows_iter_ref()
            .map(|row_iter| row_iter.cloned().collect())
            .collect()
    }

    /// Collects the `Grid2d` into a `Vec` of columns.
    #[inline]
    pub fn as_cols(&self) -> Vec<Vec<T>> {
        self.cols_iter_ref()
            .map(|col_iter| col_iter.cloned().collect())
            .collect()
    }

    /// Collects the `Grid2d` into a `Vec` of elements in *row major order*.
    #[inline]
    pub fn as_row_order(&self) -> Vec<T> {
        self.iter_ref().cloned().collect()
    }

    /// Collects the `Grid2d` into a `Vec` of elements in *column major order*.
    #[inline]
    pub fn as_col_order(&self) -> Vec<T> {
        self.iter_ref_col_order().cloned().collect()
    }
}

/// # exposing the inner Vec
impl<T> Grid2d<T> {
    /// Returns the underlying Vec.
    #[inline]
    pub fn vec(self) -> Vec<T> {
        self.grid
    }

    /// Returns a reference to the underlying Vec.
    #[inline]
    pub fn vec_ref(&self) -> &Vec<T> {
        &self.grid
    }

    /// Returns a mutable reference to the underlying Vec.
    #[inline]
    pub fn vec_ref_mut(&mut self) -> &mut Vec<T> {
        &mut self.grid
    }
}

/// # slices
impl<T> Grid2d<T> {
    /// Returns a slice of the grid.
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
        let start = self.get_index(row, 0)?;
        let end = start + self.row_len();
        Ok(&self.grid[start..end])
    }

    /// Returns a slice of requested `row`.
    /// Panics if out of bounds.
    #[inline]
    pub fn row_slice_unchecked(&self, row: usize) -> &[T] {
        let start = self.get_index_unchecked(row, 0);
        let end = start + self.row_len();
        &self.grid[start..end]
    }

    /// Returns a mutable slice of requested `row`.
    #[inline]
    pub fn row_mut_slice(&mut self, row: usize) -> Result<&mut [T]> {
        let start = self.get_index(row, 0)?;
        let end = start + self.row_len();
        Ok(&mut self.grid[start..end])
    }

    /// Returns a mutable slice of requested `row`.
    /// Panics if out of bounds.
    #[inline]
    pub fn row_mut_slice_unchecked(&mut self, row: usize) -> &mut [T] {
        let start = self.get_index_unchecked(row, 0);
        let end = start + self.row_len();
        &mut self.grid[start..end]
    }
}

/// # get chunks
impl<T> Grid2d<T> {
    /// Returns a slice of the chunk of elements at the given `row` and `col`umn.
    #[inline]
    pub fn get_chunk(&self, chunk_len: usize, row: usize, col: usize) -> Result<&[T]> {
        self.get_chunk_index(chunk_len, row, col)
            .map(|index| &self.grid[index..index + chunk_len])
    }
    /// Returns a slice of the chunk of elements at the given `row` and `col`umn.
    /// Panics if out of bounds.
    #[inline]
    pub fn get_chunk_unchecked(&self, chunk_len: usize, row: usize, col: usize) -> &[T] {
        let index = self.get_chunk_index_unchecked(chunk_len, row, col);
        &self.grid[index..index + chunk_len]
    }

    /// Returns a mutable slice of the chunk of elements at the given `row` and `col`umn.
    #[inline]
    pub fn get_chunk_mut(&mut self, chunk_len: usize, row: usize, col: usize) -> Result<&mut [T]> {
        self.get_chunk_index(chunk_len, row, col)
            .map(move |index| &mut self.grid[index..index + chunk_len])
    }

    /// Returns a mutable slice of the chunk of elements at the given `row` and `col`umn.
    /// Panics if out of bounds.
    #[inline]
    pub fn get_chunk_mut_unchecked(
        &mut self,
        chunk_len: usize,
        row: usize,
        col: usize,
    ) -> &mut [T] {
        let index = self.get_chunk_index_unchecked(chunk_len, row, col);
        &mut self.grid[index..index + chunk_len]
    }
}

/// # set chunks
impl<T: Clone> Grid2d<T> {
    /// Sets the elements on a chunk.
    #[inline]
    pub fn set_chunk(
        &mut self,
        chunk_len: usize,
        row: usize,
        col: usize,
        elements: &[T],
    ) -> Result<()> {
        let chunk = self.get_chunk_mut(chunk_len, row, col)?;
        for (n, element) in chunk.iter_mut().enumerate() {
            *element = elements[n].clone();
        }
        Ok(())
    }

    /// Sets the elements on a chunk.
    /// Panics if out of bounds.
    #[inline]
    pub fn set_chunk_unchecked(
        &mut self,
        chunk_len: usize,
        row: usize,
        col: usize,
        elements: &[T],
    ) {
        let chunk = self.get_chunk_mut_unchecked(chunk_len, row, col);
        for (n, element) in chunk.iter_mut().enumerate() {
            *element = elements[n].clone();
        }
    }
}

mod std_impls {
    use super::{Grid2d, Index, IndexMut};
    use core::any::type_name;
    use std::fmt;

    impl<T> fmt::Debug for Grid2d<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(
                f,
                "Grid2d {{ {}×{}, {} }}",
                self.rows,
                self.cols,
                type_name::<T>()
            )
        }
    }

    impl<T> Index<(usize, usize)> for Grid2d<T> {
        type Output = T;
        fn index(&self, (row, col): (usize, usize)) -> &Self::Output {
            self.get_ref(row, col)
                .unwrap_or_else(|_| panic!("Index indices {}, {} out of bounds", row, col))
        }
    }

    impl<T> IndexMut<(usize, usize)> for Grid2d<T> {
        fn index_mut(&mut self, (row, col): (usize, usize)) -> &mut Self::Output {
            self.get_ref_mut(row, col)
                .unwrap_or_else(|_| panic!("Index mut indices {}, {} out of bounds", row, col))
        }
    }
}
