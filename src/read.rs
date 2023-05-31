use pest::{iterators::Pair, Parser};

#[derive(Parser)]
#[grammar = "rasp.pest"]
pub struct RaspParser;

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

fn rasp_children(parsed: Pair<Rule>) -> std::result::Result<Vec<Box<Value>>, Error> {
    let mut children = vec![];
    for child in parsed.into_inner() {
        children.push(Box::new(rasp_read(child)?));
    }
    Ok(children)
}

fn rasp_symbol(parsed: Pair<Rule>) -> Result {
    let mut pieces = parsed.into_inner().rev();
    let name = pieces.next().unwrap().as_str().into();
    let ns = match pieces.next() {
        Some(pair) => match pair.as_rule() {
            Rule::slash => Some(
                pieces
                    .rev()
                    .map(|item| Box::new(item.as_str().into()))
                    .collect(),
            ),
            _ => None,
        },
        None => None,
    };
    Ok(Value::Symbol(ns, name))
}

fn rasp_keyword(parsed: Pair<Rule>) -> Result {
    let mut pieces = parsed.into_inner().next().unwrap().into_inner().rev();
    let namebit = pieces.next().unwrap();
    let name = match namebit.as_rule() {
        Rule::string => KeywordName::String(namebit.into_inner().next().unwrap().as_str().into()),
        Rule::segment | Rule::slash => KeywordName::Symbol(namebit.as_str().into()),
        _ => unreachable!(),
    };
    let ns = match pieces.next() {
        Some(pair) => match pair.as_rule() {
            Rule::slash => Some(
                pieces
                    .rev()
                    .map(|item| Box::new(item.as_str().into()))
                    .collect(),
            ),
            _ => None,
        },
        None => None,
    };
    Ok(Value::Keyword(ns, name))
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
        Rule::rasp => Ok(Value::List(rasp_children(parsed)?)),
        _ => unreachable!(),
    }
}

pub fn parse(string: &str) -> std::result::Result<Pair<'_, Rule>, Error> {
    // let parsed =
    Ok(RaspParser::parse(Rule::rasp, string)
        .unwrap()
        .next()
        .unwrap())

    // rasp_read(parsed)
}
