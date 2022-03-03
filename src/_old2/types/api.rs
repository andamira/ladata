// ladata::types::api
//
//!
//

pub use super::{DataType, DataTypePlus};

/// Supported data types must implement this trait (and `Copy`).
///
/// ## Example
// ```
// ```
pub trait DataTypes: Copy {
    // type DT;

    /// Returns `true` if the represented data type is [`Copy`].
    fn is_copy(&self) -> bool;

    // /// Returns a `DataType` that includes the current data type.
    // fn to_data_type(&self) -> Self::DT;

    // RETHINK: assume DataCell<T: DataCellAble /*:/+ Default*/> ?
    // fn to_data_cell_default() -> DataCell;

    // TODO: MORE

    // type_id() -> TypeId;
    // size() -> usize;
    // alignment() -> usize;

    // into_nested() -> NestedDataType;
    // into_flat() -> DataType;

    // MAYBE:
    // is_growable(&self) -> bool; // only non-copy types
    // is_mutable(&self) // … not sure if useful
}

mod impls {
    //!

    use super::{DataTypePlus, DataTypes};

    impl<T: DataTypes> DataTypes for DataTypePlus<T> {

        #[inline]
        fn is_copy(&self) -> bool {
            use DataTypePlus::*;

            match self {
                Bool => true,
                // BitField => true,
                // DiscreteSigned => true,

                // non copy

                String => false,

                Other(o) => o.is_copy(),

                //
                _ => false,
            }
        }
    }

}
