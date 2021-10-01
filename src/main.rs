extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::{Parser, iterators::Pair};
use std::fs;

#[derive(Parser)]
#[grammar = "rasp.pest"]
pub struct RaspParser;

#[derive(Debug)]
pub enum Error {
    ParseError,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    String(String),
}

pub type Result = std::result::Result<Value, Error>;

fn rasp_read(parsed: Pair<Rule>) -> Result {
    match parsed.as_rule() {
        Rule::expr => rasp_read(parsed.into_inner().next().unwrap()),
        Rule::string => {
            // string by definition has a charseq inner
            let charseq = parsed.into_inner().next().unwrap().as_str().into();
            Ok(Value::String(charseq))
        }
        _ => unreachable!(),
    }
}

fn main() {
    let testfile = fs::read_to_string("resources/test.ras").unwrap();
    let parsed = RaspParser::parse(Rule::expr, &testfile).unwrap().next().unwrap();

    println!("{:?}", rasp_read(parsed));
}
