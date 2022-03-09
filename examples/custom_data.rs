// ladata::examples::custom_data
//
//! Demonstrates how to use custom data types.

use core::mem::size_of;
use ladata::all::*;

// Define your own data type.
#[derive(Clone, Copy, Debug)]
enum MyCustomData {
    Int(i64),
    Float(f64),
}

impl DataCells for MyCustomData {
    fn is_copy(&self) -> bool {
        true
    }
}

impl DataCellsCopy for MyCustomData {}

fn main() {
    // If you are are only interested in using your custom type you can do so.

    assert_eq![16, size_of::<MyCustomData>()];

    let a1 = [MyCustomData::Int(1_000), MyCustomData::Float(1.5)];

    println!("Iterate an array of `MyCustomData`:");
    for c in a1 {
        match c {
            MyCustomData::Int(i) => println!("- int! {i}"),
            MyCustomData::Float(f) => println!("- float! {f}"),
        }
    }

    // You can also extend a DataCell* with your type for increased flexibility.

    type MyCell = DataCell64bitCopyWith<MyCustomData>;

    assert_eq![16, size_of::<DataCell64bitCopy>()];
    assert_eq![24, size_of::<MyCell>()]; // Size increases when embedding custom data.

    let a2 = [
        MyCell::Bool(true),
        MyCell::With(MyCustomData::Int(38712987)),
        MyCell::With(MyCustomData::Float(49821.871382)),
        MyCell::I8(-23),
        MyCell::Char('®'),
    ];

    println!("\nIterate an array of `MyCell`:");
    for c in a2 {
        match c {
            MyCell::With(w) => {
                print!("- My custom data! ");
                match w {
                    MyCustomData::Int(i) => println!("(int {i})"),
                    MyCustomData::Float(f) => println!("(float {f})"),
                }
            }
            MyCell::Char(c) => println!("- this is a char: '{c}'"),
            other => println!("- other: {other:?}"),
        }
    }
}
