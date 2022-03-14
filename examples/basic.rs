// ladata::examples::basic
//
//! Basic example

use ladata::all::*;

fn main() {

    let arr = [
        DataCell32bit::F32(3.14),
        DataCell32bit::Char('π'),
        DataCell32bit::I16(-314),
        DataCell32bit::ByteArray4([3, 141, 59, 26]),
    ];

    for c in arr {
        match c {
            DataCell32bit::F32(f) => println!("{f}"),
            DataCell32bit::Char(c) => println!("{c:?}"),
            DataCell32bit::I16(i) => println!("{i:?}"),
            DataCell32bit::ByteArray4(ba) => println!("{ba:?}"),
            _ => (),

        }
    }
}
