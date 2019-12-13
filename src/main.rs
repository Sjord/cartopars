use std::fs;
use std::env;
use std::path::Path;

extern crate pest;
#[macro_use]
extern crate pest_derive;

extern crate yaml_rust;
use yaml_rust::YamlLoader;

extern crate stopwatch;
use stopwatch::Stopwatch;


use pest::Parser;
use pest::iterators::Pairs;

#[derive(Parser)]
#[grammar = "cartocss.pest"]
struct CartoParser;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mml_path = Path::new(&args[1]);
    let contents = fs::read_to_string(mml_path).unwrap();
    let project = YamlLoader::load_from_str(&contents).unwrap();
    let stylesheets = project[0]["Stylesheet"].as_vec().unwrap();
    let mut sw = Stopwatch::new();
    for ss in stylesheets {
        let path = mml_path.with_file_name(ss.as_str().unwrap());
        let contents = fs::read_to_string(path).unwrap();
        sw.start();
        let ast = CartoParser::parse(Rule::stylesheet, &contents).unwrap();
        sw.stop();
        println!("{} {}", ss.as_str().unwrap(), sw.elapsed_ms());
        sw.reset();
    }
}
