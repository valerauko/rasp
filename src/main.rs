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
    List(Vec<Box<Value>>),
    Vector(Vec<Box<Value>>),
    Map(Vec<Box<Value>>),
    Symbol(Option<String>, String),
    Keyword(Option<String>, String),
    String(String),
}

pub type Result = std::result::Result<Value, Error>;

fn rasp_children(parsed: Pair<Rule>) -> std::result::Result<Vec<Box<Value>>, Error> {
    let mut children = vec![];
    for child in parsed.into_inner() {
        children.push(Box::new(rasp_read(child)?));
    }
    Ok(children)
}

fn rasp_symbol(parsed: Pair<Rule>) -> Result {
    let mut inner = parsed.into_inner();
    let first = inner.next().unwrap();
    match inner.next() {
        Some(name) => Ok(Value::Symbol(Some(first.as_str().into()), name.as_str().into())),
        None => Ok(Value::Symbol(None, first.as_str().into())),
    }
}

fn rasp_keyword(parsed: Pair<Rule>) -> Result {
    let mut inner = parsed.into_inner();
    let first = inner.next().unwrap();
    match inner.next() {
        Some(name) => Ok(Value::Keyword(Some(first.as_str().into()), name.as_str().into())),
        None => Ok(Value::Keyword(None, first.as_str().into())),
    }
}

fn rasp_read(parsed: Pair<Rule>) -> Result {
    match parsed.as_rule() {
        Rule::expr => rasp_read(parsed.into_inner().next().unwrap()),
        Rule::list => Ok(Value::List(rasp_children(parsed)?)),
        Rule::vector => Ok(Value::Vector(rasp_children(parsed)?)),
        Rule::map => Ok(Value::Map(rasp_children(parsed)?)),
        Rule::string => {
            // string by definition has a charseq inner
            let charseq = parsed.into_inner().next().unwrap().as_str().into();
            Ok(Value::String(charseq))
        }
        Rule::symbol => Ok(rasp_symbol(parsed)?),
        Rule::keyword => Ok(rasp_keyword(parsed)?),
        Rule::rasp => Ok(Value::Vector(rasp_children(parsed)?)),
        x => { println!("{:?}", x); unreachable!()},
    }
}

fn main() {
    let testfile = fs::read_to_string("resources/dream.ras").unwrap();
    let parsed = RaspParser::parse(Rule::rasp, &testfile).unwrap().next().unwrap();

    println!("{:?}", rasp_read(parsed));
}
