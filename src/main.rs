extern crate pest;
#[macro_use]
extern crate pest_derive;

use std::fs;

mod read;

#[derive(Debug)]
pub enum Error {
    ParseError,
}

type Path = Vec<Box<String>>;

#[derive(Debug, Clone, PartialEq)]
pub enum KeywordName {
    Symbol(String),
    String(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    List(Vec<Box<Value>>),
    Vector(Vec<Box<Value>>),
    Map(Vec<Box<Value>>),
    Symbol(Option<Path>, String),
    Keyword(Option<Path>, KeywordName),
    String(String),
}

pub type Result = std::result::Result<Value, Error>;

fn main() {
    let testfile = fs::read_to_string("resources/complex.ras").unwrap();
    let parsed = read::parse(&testfile);
    // let test_thing: Vec<Box<dyn std::fmt::Debug>> = vec![];

    println!("{:?}", parsed);
}
