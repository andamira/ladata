// dataframe _SACAR

/// # …
impl<F: Format> DataFrame<F> {
    /*
    // RETHINK:
    // - choose the format of the Rows… or convert them
    //   - … receieve as DataFrame? owned? or ref?
    //
    // LIKE: return (mut) slice.

    /// Returns the first `num_rows` rows as a slice.
    // head_rows
    // head_rows_ref
    fn head(&self, num_rows: usize) -> &[Row] { }

    /// Returns the first `num_rows` rows as a mutable slice.
    fn head_mut(&mut self usize) -> &mut [Row] { }

    /// Returns the last `num_rows` rows as a slice.
    fn tail(&self usize) -> &[Row] { }

    /// Returns the last `num_rows` rows as a mutable slice.
    fn head_mut(&mut self usize) -> &mut [Row] { }
    */
}

/*
pub struct DataFrame<C: ColumnApi + Sized> {
    num_cols: usize,
    name_cols: Vec<String>,
    type_cols: Vec<CellType>,
    // columns: Vec<Column>,
    columns: Vec<C>,
    num_rows: usize,
}

/// # Constructors
impl<C: ColumnApi + Sized> DataFrame<C> {
    /// Returns a new `DataFrame`.
    pub fn new() -> Self {
        Self {
            num_cols: 0,
            name_cols: vec![],
            type_cols: vec![],
            columns: vec![],
            num_rows: 0,
        }
    }
}
*/


/*
/// # Manipulation
impl<C: ColumnApi + Sized> DataFrame<C> {

    /// Returns a reference to the `nth` `Column`.
    pub fn col(&self, index: usize) -> &C {
        &self.columns[index]
    }

    /// Returns a mutable reference to the `nth` `Column`.
    pub fn col_mut(&mut self, index: usize) -> &mut C {
        &mut self.columns[index]
    }

    /// Adds a column
    ///
    // WIP
    // RETHINK:
    // - length must coinc ide, or… fill
    // - return result?
    pub fn add_col(&mut self, c: C) {
        self.columns.push(c)
    }
}
*/

