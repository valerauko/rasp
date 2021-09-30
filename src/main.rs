extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;
use std::fs;

#[derive(Parser)]
#[grammar = "rasp.pest"]
pub struct RaspParser;

fn main() {
    let testfile = fs::read_to_string("resources/test.ras").unwrap();
    let result = RaspParser::parse(Rule::main, &testfile);
    println!("{:?}", result);
}
