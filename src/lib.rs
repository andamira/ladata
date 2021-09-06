//! Table

mod table;
pub use table::*;

// WIP helper macros

/// Implements `TryInto` over a single enum variant.
#[macro_export]
#[doc(hidden)]
macro_rules! __impl_tryinto {
    ($from:ty, $self:ident, $into:ty, $variant:ident) => {
        impl std::convert::TryInto<$into> for $from {
            type Error = String;
            fn try_into($self) -> Result<$into, Self::Error> {
                match $self {
                    $variant(v) => Ok(v),
                    _ => Err("".into())
                }
            }
        }

    };
}

/// Implements `From<$from> for $for`.
#[macro_export]
#[doc(hidden)]
macro_rules! __impl_from {
    ($from:ty, $for:ty, $variant:ident) => {
        impl From<$from> for $for {
            fn from(from: $from) -> Self {
                $variant(from)
            }
        }
    };
}

/// Creates a method to cast
///
// TODO: make this more composable, modular…
//
#[macro_export]
#[doc(hidden)]
macro_rules! __variants_values_as {
    ( $self:ident, ($($as:ty),+); $variants:tt ) => {
        $(
        $crate::__variants_values_as![@create_fn $self, $as, $variants];
        )+
    };
    ( @create_fn $self:ident, $as:ty, ($($var:expr),+) ) => {
        paste::paste! {
        #[doc = "Returns the contained value casted as `" $as "`." ]
            pub fn [<as_$as>](&$self) -> $as {
                match *$self {
                    $(
                        $var(v) => v as $as
                    ),+
                }
            }
        }
    };
}

// TODO: make an IMPROVED version that supports optional args
//
/// call a custom method (no arguments) on all provided variants.
#[macro_export]
#[doc(hidden)]
macro_rules! __match_variants_method {
    ( $self:ident, $method:ident, ($( $variant:tt ),+) ) => {
        match *$self {
            $(
                $variant(ref v) => { v.$method() },
            )+
        }
    };
}
