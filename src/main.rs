use std::fs;
use std::env;

extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;

#[derive(Parser)]
#[grammar = "cartocss.pest"]
struct CartoParser;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename).unwrap();

    let p = CartoParser::parse(Rule::stylesheet, 
    // "@steps-width-z14 + 2 * (@paths-background-width + @paths-bridge-casing-width)"
    &contents 
    ).unwrap_or_else(|e| panic!("{}", e));
    println!("{:#?}", p);
}
