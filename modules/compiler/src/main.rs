extern crate pest;
#[macro_use]
extern crate pest_derive;
extern crate rasp_runtime as runtime;
use crate::runtime::Primitive;

use std::fs;

mod read;

fn main() {
    let testfile = fs::read_to_string("resources/test.ras").unwrap();
    let parsed = read::parse(&testfile);

    println!("{:?}", parsed)
}
