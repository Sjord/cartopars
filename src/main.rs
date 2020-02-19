use std::fs;
use std::env;
use std::path::Path;
use std::collections::HashMap;

extern crate pest;
#[macro_use]
extern crate pest_derive;
extern crate pest_consume;

extern crate yaml_rust;
use yaml_rust::YamlLoader;

extern crate stopwatch;
use stopwatch::Stopwatch;

use pest_consume::Parser;
use pest_consume::Error;
use pest_consume::match_nodes;
use pest::iterators::Pairs;

type Node<'i> = pest_consume::Node<'i, Rule, ()>;
type Result<T> = std::result::Result<T, Error<Rule>>;

#[derive(Parser)]
#[grammar = "cartocss.pest"]
struct CartoParser;

#[derive(Debug)]
struct Ruleset {
    selectors: String,
    ruleset_body: String
}

#[derive(Debug)]
enum Statement {
    Assignment(Assignment),
    Ruleset(Ruleset)
}

#[derive(Debug)]
struct Assignment {
    key: String,
    value: String,
}

#[derive(Debug)]
struct Stylesheet {
    assignments: Vec<Assignment>,
    rulesets: Vec<Ruleset>
}

#[pest_consume::parser]
impl CartoParser {
    fn EOI(_input: Node) -> Result<()> {
        Ok(())
    }

    fn color_keyword(input: Node) -> Result<&str> {
        Ok(input.as_str())
    }

    fn variable(input: Node) -> Result<String> {
        Ok(input.as_str().to_owned())
    }

    fn values(input: Node) -> Result<String> {
        Ok(input.as_str().to_owned())
    }

    fn selectors(input: Node) -> Result<String> {
        Ok(input.as_str().to_owned())
    }

    fn ruleset_body(input: Node) -> Result<String> {
        Ok(input.as_str().to_owned())
    }

    fn assignment(input: Node) -> Result<Assignment> {
        Ok(match_nodes!(input.into_children();
            [variable(k), values(v)] => Assignment { key: k, value: v }
        ))
    }

    fn ruleset(input: Node) -> Result<Ruleset> {
        Ok(match_nodes!(input.into_children();
            [selectors(selectors), ruleset_body(ruleset_body)] => Ruleset { selectors, ruleset_body }
        ))
    }

    fn statement(input: Node) -> Result<Statement> {
        Ok(match_nodes!(input.into_children();
            [assignment(a)] => Statement::Assignment(a),
            [ruleset(r)] => Statement::Ruleset(r),
        ))
    }

    fn stylesheet(input: Node) -> Result<Stylesheet> {
        let statements : Vec<Statement> = match_nodes!(input.into_children();
            [statement(s).., EOI(_)] => s.collect(),
        );
        let mut assignments : Vec<Assignment> = Vec::new();
        let mut rulesets : Vec<Ruleset> = Vec::new();
        for s in statements {
            match s {
                Statement::Assignment(a) => { assignments.push(a); }
                Statement::Ruleset(r) => { rulesets.push(r) }
            }
        }
        Ok(Stylesheet { assignments, rulesets })
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mml_path = Path::new(&args[1]);
    let contents = fs::read_to_string(mml_path).unwrap();
    let project = YamlLoader::load_from_str(&contents).unwrap();
    let stylesheets = project[0]["Stylesheet"].as_vec().unwrap();
    let mut sw = Stopwatch::new();
    for ss in stylesheets {
        let path = mml_path.with_file_name(ss.as_str().unwrap());
        let contents = fs::read_to_string(&path).unwrap();
        sw.start();
        let node = CartoParser::parse(Rule::stylesheet, &contents)
            .map_err(|e| e.with_path(path.to_str().unwrap()))
            .unwrap().single().unwrap();
        let ast = CartoParser::stylesheet(node);
        sw.stop();
        // println!("{} {}", ss.as_str().unwrap(), sw.elapsed_ms());
        println!("{:#?}", ast);
        sw.reset();
    }
}
