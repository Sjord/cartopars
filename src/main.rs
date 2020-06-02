use std::fs;
use std::env;
use std::path::Path;
use std::ops::Deref;
use std::iter::FromIterator;

extern crate pest;
extern crate pest_derive;
extern crate pest_consume;

extern crate yaml_rust;
use yaml_rust::YamlLoader;

extern crate stopwatch;
use stopwatch::Stopwatch;

use pest_consume::Parser;
use pest_consume::Error;
use pest_consume::match_nodes;

type Node<'i> = pest_consume::Node<'i, Rule, ()>;
type Result<T> = std::result::Result<T, Error<Rule>>;
type Values = Vec<Value>;
type Url = String;
type Boolean = String;
type StringExpr = String;
type Percentage = String;
type Expression = String;
type Keyword = String;
type Field = String;

#[derive(Debug, Copy, Clone)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
    opacity: u8,
}

impl Color {
    fn rgba(red: u8, green: u8, blue: u8, opacity: u8) -> Color {
        Color { red, green, blue, opacity }
    }

    fn from_hex(hex : &String) -> Color {
        match hex.len() {
            4 => {
                // #abc
                let red = 17 * u8::from_str_radix(&hex[1..2], 16).unwrap();
                let green = 17 * u8::from_str_radix(&hex[2..3], 16).unwrap();
                let blue = 17 * u8::from_str_radix(&hex[3..4],16).unwrap();
                Color::rgba(red, green, blue, 255)
            }
            7 => {
                // #aabbcc
                let red = u8::from_str_radix(&hex[1..3], 16).unwrap();
                let green = u8::from_str_radix(&hex[3..5], 16).unwrap();
                let blue = u8::from_str_radix(&hex[5..7],16).unwrap();
                Color::rgba(red, green, blue, 255)
            }
            _ => panic!("color not recognized")
        }
    }

    fn from_keyword(keyword : &String) -> Color {
        // FIXME map keyword to color
        Color::rgba(123,123,123,123)
    }

    fn darken(&self, perc : &Percentage) -> Color {
        // FIXME implement
        self.clone()
    }
}

#[derive(Debug)]
struct Body(Vec<BodyPart>);

#[derive(Parser)]
#[grammar = "cartocss.pest"]
struct CartoParser;

#[derive(Debug)]
enum BodyPart {
    Declarations(Vec<Declaration>),
    Ruleset(Ruleset)
}


#[derive(Debug)]
struct Ruleset {
    selectors: Vec<String>,
    body: Body,
}

impl Deref for Body {
    type Target = Vec<BodyPart>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromIterator<BodyPart> for Body {
    fn from_iter<I: IntoIterator<Item=BodyPart>>(iter: I) -> Self {
        Body(iter.into_iter().collect())
    }
}

impl Body {
    /// Get declarations from the body, but not of nested Rulesets
    fn get_declarations(&self) -> Vec<&Declaration> {
        let mut result : Vec<&Declaration> = Vec::new();
        for bp in &self.0 {
            match bp {
                BodyPart::Declarations(ds) => {
                    for d in ds {
                        result.push(d);
                    }
                }
                BodyPart::Ruleset(_) => (),
            };
        }
        result
    }
}

#[derive(Debug)]
enum Statement {
    Assignment(Assignment),
    Ruleset(Ruleset)
}

#[derive(Debug)]
struct Function {
    identifier: String,
    values: Values,
}

impl Function {
    fn evaluate(&self) -> std::result::Result<Value, &str> {
        match self.identifier.as_str() {
            "darken" => match &self.values[0] {
                Value::Color(c) => match &self.values[1] {
                    Value::Percentage(perc) => Ok(Value::Color(c.darken(perc))),
                    _ => Err("darken expects a percentage as second parameter")
                }
                _ => Err("darken expects a color as first parameter")
            }
            _ => Err("unimplemented function")
        }
    }
}

#[derive(Debug)]
enum Value {
    Url(Url),
    Boolean(Boolean),
    StringExpr(StringExpr),
    Percentage(Percentage),
    Expression(Expression),
    Color(Color),
    Function(Function),
    Keyword(Keyword),
    Field(Field),
}

impl Value {
    fn get_color(&self) -> std::result::Result<Color, &str> {
        match &self {
            Value::Color(c) => Ok(c.clone()),
            Value::Function(f) => match f.evaluate() {
                Ok(Value::Color(c)) => Ok(c),
                _ => Err("not a Color")
            }
            _ => Err("not a Color")
        }
    }
}

#[derive(Debug)]
struct Assignment {
    key: String,
    values: Vec<Value>,
}

#[derive(Debug)]
struct Declaration {
    property: String,
    values: Values
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

    fn url(input: Node) -> Result<Url> {
        Ok(input.as_str().to_owned())
    }

    fn boolean(input: Node) -> Result<Boolean> {
        Ok(input.as_str().to_owned())
    }

    fn field(input: Node) -> Result<Field> {
        Ok(input.as_str().to_owned())
    }

    fn keyword(input: Node) -> Result<Keyword> {
        Ok(input.as_str().to_owned())
    }

    fn expression(input: Node) -> Result<Expression> {
        Ok(input.as_str().to_owned())
    }

    fn color_hex_long(input: Node) -> Result<String> {
        Ok(input.as_str().to_owned())
    }

    fn color_hex_short(input: Node) -> Result<String> {
        Ok(input.as_str().to_owned())
    }

    fn color_keyword(input: Node) -> Result<String> {
        Ok(input.as_str().to_owned())
    }

    fn color(input: Node) -> Result<Color> {
        Ok(match_nodes!(input.into_children();
            [color_hex_long(c)] => Color::from_hex(&c),
            [color_hex_short(c)] => Color::from_hex(&c),
            [color_keyword(c)] => Color::from_keyword(&c)
        ))
    }

    fn percentage(input: Node) -> Result<Percentage> {
        Ok(input.as_str().to_owned())
    }

    fn variable(input: Node) -> Result<String> {
        Ok(input.as_str().to_owned())
    }

    fn string_expr(input: Node) -> Result<String> {
        Ok(input.as_str().to_owned())
    }

    fn selectors(input: Node) -> Result<Vec<String>> {
        Ok(match_nodes!(input.into_children();
            [selector(s)..] => s.collect()
        ))
    }

    fn ruleset_body(input: Node) -> Result<BodyPart> {
        Ok(match_nodes!(input.into_children();
            [declarations(d)] => BodyPart::Declarations(d),
            [ruleset(rs)] => BodyPart::Ruleset(rs),
        ))
    }

    fn identifier(input: Node) -> Result<String> {
        Ok(input.as_str().to_owned())
    }

    fn function(input: Node) -> Result<Function> {
        Ok(match_nodes!(input.into_children();
            [identifier(i), values(v)] => Function { identifier: i, values: v }
        ))
    }

    fn value(input: Node) -> Result<Value> {
        Ok(match_nodes!(input.into_children();
            [url(u)] => Value::Url(u),
            [boolean(b)] => Value::Boolean(b),
            [string_expr(se)] => Value::StringExpr(se),
            [percentage(p)] => Value::Percentage(p),
            [expression(e)] => Value::Expression(e),
            [color(c)] => Value::Color(c),
            [function(f)] => Value::Function(f),
            [keyword(k)] => Value::Keyword(k),
            [field(f)] => Value::Field(f),
        ))
    }

    fn values(input: Node) -> Result<Values> {
        Ok(match_nodes!(input.into_children();
            [value(v)..] => v.collect()
        ))
    }

    fn selector(input: Node) -> Result<String> {
        Ok(input.as_str().to_owned())
    }

    fn instance(input: Node) -> Result<String> {
        Ok(input.as_str().to_owned())
    }

    fn property(input: Node) -> Result<String> {
        Ok(input.as_str().to_owned())
    }

    fn declaration(input: Node) -> Result<Declaration> {
        Ok(match_nodes!(input.into_children();
            // [instance(i), property(p), values(v)] => 
            [property(p), values(v)] => Declaration { property: p, values: v}
        ))
    }

    fn declarations(input: Node) -> Result<Vec<Declaration>> {
        Ok(match_nodes!(input.into_children();
            [declaration(d)..] => d.collect()
        ))
    }

    fn assignment(input: Node) -> Result<Assignment> {
        Ok(match_nodes!(input.into_children();
            [variable(k), values(v)] => Assignment { key: k, values: v }
        ))
    }

    fn ruleset(input: Node) -> Result<Ruleset> {
        Ok(match_nodes!(input.into_children();
            [selectors(selectors), ruleset_body(body)..] => Ruleset { selectors, body: body.collect() }
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
        // println!("{:#?}", ast);
        println!("{:#?}", ast.unwrap().rulesets[0].body.get_declarations()[0].values[0].get_color());
        sw.reset();
    }
}
