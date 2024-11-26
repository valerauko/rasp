// extern crate pest;
// #[macro_use]
// extern crate pest_derive;
// extern crate rasp_runtime as runtime;

// use std::fs;

// mod read;

fn do_thing(x: impl Into<Option<String>>) -> Option<String> {
    match x.into() {
        Some(s) => Some(s),
        None => Some("foo".to_string())
    }
}

fn main() {
    // let testfile = fs::read_to_string("resources/macro.ras").unwrap();
    // let parsed = read::parse(&testfile);

    // println!("{:?}", parsed)

    let value = do_thing("foo".to_string());
    match Into::<Option<String>>::into(value) {
        Some(x) => println!("{}", x),
        None => println!("uh oh"),
    }
}
