// ladata::examples::basic
//
//! Basic example

use ladata::all::*;

fn main() {
    let arr = [
        DataUnit32bit::F32(3.14),
        DataUnit32bit::Char('π'),
        DataUnit32bit::I16(-314),
        DataUnit32bit::ByteArray4([3, 141, 59, 26]),
    ];

    for c in arr {
        match c {
            DataUnit32bit::F32(f) => println!("{f}"),
            DataUnit32bit::Char(c) => println!("{c:?}"),
            DataUnit32bit::I16(i) => println!("{i:?}"),
            DataUnit32bit::ByteArray4(ba) => println!("{ba:?}"),
            _ => (),
        }
    }
}
