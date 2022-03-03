// src/types/classes
//
//!
//

/// The basic classes of data types.
///
// TODO: impl traits, Default, etc.
#[derive(Clone, Copy, Debug)]
#[non_exhaustive]
pub enum DataTypeClass {
    /// A unitary type of data.
    Bool,

    /// A sequence of data.
    BitField,

    /// Data distributed in two dimensions.
    Integer,

    /// Data distributed in two dimensions.
    UnsignedInteger,

    /// Data distributed in multiple number of dimensions.
    Float,

    /// A list of linked individual elements.
    Rational,

    /// A tree-like organization of individual elements.
    DateTime,

    /// A dictionary
    Duration,

    ///
    String,

    /// A unicode code-point, a character.
    Char,

    ///
    Other,

    // IDEA (maybe in a separate structure):
    // /// A combination of two classes
    // Combination((DataStructureClass, DataStructureClass))

    /// Represents the absence of a data structure.
    None,
}

