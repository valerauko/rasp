use pest::{iterators::Pair, Parser};
use rpds::{List, Vector};

#[derive(Parser)]
#[grammar = "rasp.pest"]
pub struct RaspParser;

use runtime::{Primitive};

fn read(pair: Pair<Rule>) -> Primitive {
    // println!("{:?}", pair.as_rule());
    match pair.as_rule() {
        Rule::expr => read(pair.into_inner().next().unwrap()),
        Rule::symbol => Primitive::Symbol(pair.into_inner().next().unwrap().as_str().into()),
        Rule::decimal => Primitive::Decimal(pair.as_str().parse::<i64>().unwrap()),
        Rule::list => {
            Primitive::List(pair.into_inner().rev().fold(List::new(), |acc, item| acc.push_front(read(item))))
        }
        Rule::vector => {
            Primitive::Vector(pair.into_inner().fold(Vector::new(), |acc, item| acc.push_back(read(item))))
        }
        _ => todo!(),
    }
}

pub fn parse(string: &str) -> Vec<Primitive> {
    RaspParser::parse(Rule::rasp, string).unwrap().next().unwrap()
        .into_inner().map(|item| read(item)).collect()
}
