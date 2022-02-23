/*

/// Common interface for all column types.
pub trait ColumnApi {
    // TODO:
    // - rename new
    // - make more new (copying, taking ownership…)
    fn new<T: InnerCellDataType + Clone>(data: &[T]) -> Result<Self> where Self: Sized;
    fn new_empty(cell_type: CellType) -> Self;


    fn cell_type(&self) -> CellType { self.cell_type }

    #[inline]
    fn num_rows(&self) -> usize { self.vec.len() }
    #[inline]
    fn len(&self) -> usize { self.vec.len() }
}

impl<CELL: CellApi> ColumnApi for Column<CELL> {
    // auto impl?
    fn new_empty(cell_type: CellType) -> Self {
        Self {
            cell_type,
            vec: vec![],
        }
    }

    /// Copies
    fn new<T: InnerCellDataType + Clone>(data: &[T]) -> Result<Self> {
        // CHECK: whether it's possible to get type information on an empty slice
        // https://stackoverflow.com/questions/63092178/how-do-i-annotate-the-type-of-an-empty-slice-in-rust
        if data.is_empty() {
            return Err(DataFrameError::Generic("".into()));
        }

        // let vec =

        Ok(Self {
            cell_type: data[0].cell_type(), // NEEDS
            vec: vec![],
            // vec, // WIP
            // vec: data.to_vec(),
        })
    }

}
*/

/*
// MAYBE?

// impl ColumnApi for CellsColumn {
//     /// Returns a new `CellsColumn`.
//     fn new<T: InnerCellDataType>(data: &[T]) -> Result<Self> {
//         if data.is_empty() {
//             return Err(DataFrameError::Generic("".into()));
//         }
//
//         // WIP: need trait method for wrapping InnerCellDataType into CellData
//         // MAYBE: rename trait to CellDatable or IntoCellData or ValidCellData
//         // Ok(Column {
//         //     cell_type: data[0].cell_type(),
//         //     vec:
//         // })
//         Err(DataFrameError::Generic("".into())) // TEMP
//     }
// }


impl Column {
    /// Returns a new column, filled with at least 1 cell.
    ///
    /// Errors if the `data` is empty.
    pub fn new<T: InnerCellDataType>(data: &[T]) -> Result<Self> {
        if data.is_empty() {
            return Err(DataFrameError::Generic("".into()));
        }

        // WIP
        // let cell_type = data[0].cell_type();
        // let byte_slice: &[u8] = try_cast_slice(data).expect("cast slice");
        //
        // Ok(Column {
        //     cell_type,
        //     vec: byte_slice,
        // })

        Err(DataFrameError::Generic("".into())) // TEMP
    }


    // TODO:
    // - add(&mut self)
}
*/


// MAYBE: abstract over format, using `Format` API
impl<F: Format> Column<F> {
    // WIP RETHINK
    // how to generically return a Vec of the correct format
    // 1. get the format type from the column

    /// Returns a new column from an iterable.
    ///
    pub fn from_iter_wip<I, AD>(i: I) -> Result<Self>
    where
        I: IntoIterator<Item = AD>,
        AD: AcceptableData,
    {
        // let format = Self::format();

        // let vec: Vec<F> = i.into_iter().map(|d| d.to_cell_data()).collect();

        // WIP…
        // let vec: Vec<F> = i.into_iter().map(
        //     |d| d.XXX()).collect();

        // if vec.is_empty() {
        //     return Err(DataFrameError::Generic("empty iterator".into()));
        // }
        // let cell_type = vec[0].cell_type();

        // TEMP
        let cell_type = CellType::Bool;
        let vec = vec![];

        Ok(Self { cell_type, vec })
    }
}

// MAYBE? :/
// impl<F: Format> Column<F> {
//     /// Returns the format type of the column.
//     pub const fn format_dyn(f: F) -> FormatType {
//         match f.format_type() {
//             _ => FormatType::CellData
//         }
//     }
// }
