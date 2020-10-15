#![feature(prelude_import)]
#[prelude_import]
use std::prelude::v1::*;
#[macro_use]
extern crate std;
use embedded_cli::*;
fn main() {}
enum Command {
    F0(f32, f32),
    Log(Log),
    Log2(Log2),
}
enum Log {
    Adc(),
    Log3(Log3),
    Vtg(),
}
enum Log3 {
    Adc(f32),
    Vtg(),
}
enum Log2 {
    Adc(),
    Vtg(),
}
