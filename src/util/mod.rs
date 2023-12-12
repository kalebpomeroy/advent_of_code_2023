include!("input.rs");
include!("vector.rs");
include!("coordinate.rs");
use std::fmt::Debug;

pub fn print<T: Debug>(s: T) {
    println!("{:?}", s);
}