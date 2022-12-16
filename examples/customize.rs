// ladata::examples::customize
//
//! Demonstrates how to use custom data types.

use core::mem::size_of;
use ladata::all::*;

///  custom data.
#[derive(Clone, Copy, Debug)]
enum MyCell {
    A(i64),
    B(f64),
}

impl DataCells for MyCell {
    fn is_copy(&self) -> bool {
        true
    }
}
impl DataCellsCopy for MyCell {}

fn main() {
    // Standalone custom data:

    assert_eq![16, size_of::<MyCell>()];

    let a1 = [MyCell::A(1_000), MyCell::B(1.5)];

    println!("Iterate an array of `MyCell`:");
    for c in a1 {
        match c {
            MyCell::A(i) => println!("- int! {i}"),
            MyCell::B(f) => println!("- float! {f}"),
        }
    }

    // Embedded custom data:

    type MyCellInside = DataCell64bitCopyWith<MyCell>;

    assert_eq![16, size_of::<DataCell64bitCopy>()];
    assert_eq![16, size_of::<MyCellInside>()];

    let a2 = [
        MyCellInside::Bool(true),
        MyCellInside::With(MyCell::A(38712987)),
        MyCellInside::With(MyCell::B(49821.871382)),
        MyCellInside::I8(-23),
        MyCellInside::Char('®'),
    ];

    println!("\nIterate an array of `MyCellInside`:");
    for c in a2 {
        match c {
            MyCellInside::With(w) => {
                print!("-  custom data! ");
                match w {
                    MyCell::A(i) => println!("(int {i})"),
                    MyCell::B(f) => println!("(float {f})"),
                }
            }
            MyCellInside::Char(c) => println!("- this is a char: '{c}'"),
            other => println!("- other: {other:?}"),
        }
    }
}
