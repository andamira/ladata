// ladata::examples::custom_data
//
//! Demonstrates how to use custom data types.

use core::mem::size_of;
use ladata::all::*;

///  custom data.
#[derive(Clone, Copy, Debug)]
enum CustomCell {
    A(i64),
    B(f64),
}

impl DataCells for CustomCell {
    fn is_copy(&self) -> bool {
        true
    }
}
impl DataCellsCopy for CustomCell {}

fn main() {
    // Standalone custom data:

    assert_eq![16, size_of::<CustomCell>()];

    let a1 = [CustomCell::A(1_000), CustomCell::B(1.5)];

    println!("Iterate an array of `CustomCell`:");
    for c in a1 {
        match c {
            CustomCell::A(i) => println!("- int! {i}"),
            CustomCell::B(f) => println!("- float! {f}"),
        }
    }

    // Embedded custom data:

    type EmbeddedCell = DataCell64bitCopyWith<CustomCell>;

    assert_eq![16, size_of::<DataCell64bitCopy>()];
    assert_eq![24, size_of::<EmbeddedCell>()]; // note the size increase

    let a2 = [
        EmbeddedCell::Bool(true),
        EmbeddedCell::With(CustomCell::A(38712987)),
        EmbeddedCell::With(CustomCell::B(49821.871382)),
        EmbeddedCell::I8(-23),
        EmbeddedCell::Char('®'),
    ];

    println!("\nIterate an array of `EmbeddedCell`:");
    for c in a2 {
        match c {
            EmbeddedCell::With(w) => {
                print!("-  custom data! ");
                match w {
                    CustomCell::A(i) => println!("(int {i})"),
                    CustomCell::B(f) => println!("(float {f})"),
                }
            }
            EmbeddedCell::Char(c) => println!("- this is a char: '{c}'"),
            other => println!("- other: {other:?}"),
        }
    }
}
