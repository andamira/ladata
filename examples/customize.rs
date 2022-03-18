// ladata::examples::customize
//
//! Demonstrates how to use custom data types.

use core::mem::size_of;
use ladata::all::*;

///  custom data.
#[derive(Clone, Copy, Debug)]
enum MyCustomData {
    A(i64),
    B(f64),
}

impl DataUnits for MyCustomData {
    fn is_copy(&self) -> bool {
        true
    }
}
impl DataUnitsCopy for MyCustomData {}

fn main() {
    // Standalone custom data:

    assert_eq![16, size_of::<MyCustomData>()];

    let a1 = [MyCustomData::A(1_000), MyCustomData::B(1.5)];

    println!("Iterate an array of `MyCustomData`:");
    for c in a1 {
        match c {
            MyCustomData::A(i) => println!("- int! {i}"),
            MyCustomData::B(f) => println!("- float! {f}"),
        }
    }

    // Embedded custom data:

    type MyEmbeddedCustomData = DataUnit64bitCopyWith<MyCustomData>;

    assert_eq![16, size_of::<DataUnit64bitCopy>()];
    assert_eq![24, size_of::<MyEmbeddedCustomData>()]; // note the size increase

    let a2 = [
        MyEmbeddedCustomData::Bool(true),
        MyEmbeddedCustomData::With(MyCustomData::A(38712987)),
        MyEmbeddedCustomData::With(MyCustomData::B(49821.871382)),
        MyEmbeddedCustomData::I8(-23),
        MyEmbeddedCustomData::Char('®'),
    ];

    println!("\nIterate an array of `MyEmbeddedCustomData`:");
    for c in a2 {
        match c {
            MyEmbeddedCustomData::With(w) => {
                print!("-  custom data! ");
                match w {
                    MyCustomData::A(i) => println!("(int {i})"),
                    MyCustomData::B(f) => println!("(float {f})"),
                }
            }
            MyEmbeddedCustomData::Char(c) => println!("- this is a char: '{c}'"),
            other => println!("- other: {other:?}"),
        }
    }
}
