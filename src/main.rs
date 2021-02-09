extern crate pest;
#[macro_use]
extern crate pest_derive;
// use std::ffi::CString;

use pest::Parser;
use pest::error::Error;
use std::collections::HashMap;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct LangParser;

#[derive(Debug)]
pub enum AstNode {
    FnCall {
        name: String,
        args: Vec<AstNode>,
    },
    FnDef {
        name: String,
        // args:
        body: Vec<AstNode>,
    },
    Integer(i32),
    DoublePrecisionFloat(f64),
    Ident(String),
    Str(String),
}

#[derive(Debug)]
pub enum Value {
    Integer(i32),
    DoublePrecisionFloat(f64),
    Str(String),
    None,
    Undefined,
}


pub fn parse_fncallargs(pair: pest::iterators::Pair<Rule>) -> Vec<AstNode> {
    let mut ast = vec![];
    let pairs = pair.into_inner();
    for pair in pairs {
        ast.push(parse_expr(pair));
    }
    ast
}

pub fn parse_fnbody(pair: pest::iterators::Pair<Rule>) -> Vec<AstNode> {
    let mut ast = vec![];
    let pairs = pair.into_inner();
    for pair in pairs {
        ast.push(parse_expr(pair));
    }
    ast
}

pub fn parse_expr(pair: pest::iterators::Pair<Rule>) -> AstNode {
    match pair.as_rule() {
        Rule::expr => parse_expr(pair.into_inner().next().unwrap()),
        Rule::integer => {
            let istr = pair.as_str();
            let (sign, istr) = match &istr[..1] {
                "_" => (-1, &istr[1..]),
                _ => (1, &istr[..]),
            };
            let integer: i32 = istr.parse().unwrap();
            AstNode::Integer(sign * integer)
        }
        Rule::decimal => {
            let dstr = pair.as_str();
            let (sign, dstr) = match &dstr[..1] {
                "_" => (-1.0, &dstr[1..]),
                _ => (1.0, &dstr[..]),
            };
            let mut flt: f64 = dstr.parse().unwrap();
            if flt != 0.0 {
                // Avoid negative zeroes; only multiply sign by nonzeroes.
                flt *= sign;
            }
            AstNode::DoublePrecisionFloat(flt)
        }
        Rule::string => {
            let str = &pair.as_str();
            // Strip leading and ending quotes.
            let str = &str[1..str.len() - 1];
            // Escaped string quotes become single quotes here.
            let str = str.replace("''", "'");
            AstNode::Str(String::from(str))
        }
        Rule::fnCall => {
            let mut pair = pair.into_inner();
            let name = String::from(pair.next().unwrap().as_str());
            let args = match pair.next() {
                Some(inner) => parse_fncallargs(inner),
                None => vec![],
            };
            // let args = parse_fncall_args(argspair);
            AstNode::FnCall {
                name,
                args,
            }
        },
        Rule::funcDec => {
            let mut pair = pair.into_inner();
            let name = String::from(pair.next().unwrap().as_str());
            let body = parse_fnbody(pair.next().unwrap());
            AstNode::FnDef {
                name,
                body,
            }
        },
        _ => panic!("Not yet implemented: {} rule type {:?}", pair.as_str(), pair.as_rule()),
    }
}

pub fn parse(source: &str) -> Result<Vec<AstNode>, Error<Rule>> {
    let mut ast = vec![];

    let pairs = LangParser::parse(Rule::program, source)?;
    for pair in pairs {
        match pair.as_rule() {
            Rule::expr => {
                ast.push(parse_expr(pair));
            }
            _ => {}
        }
    }

    Ok(ast)
}

pub fn print_builtin(args: Vec<Value>) -> Value {
    let mut print_strs = vec![];
    for arg in args {
        let s = match arg {
            Value::DoublePrecisionFloat(x) => format!("{}", x),
            Value::Integer(x) => format!("{}", x),
            Value::Str(x) => format!("{}", x),
            Value::None => String::from("None"),
            Value::Undefined => String::from("undefined"),
        };
        print_strs.push(s);
    }
    let joined = print_strs.join(" ");
    println!("{}", joined);
    Value::None
}

// pub fn exec_fn(name: &str, body: &Vec<AstNode>, scope: &mut Scope) -> Value {
pub fn exec_fn(name: &str, scope: &mut Scope) -> Value {
    let body = scope.fns.get_mut(name).unwrap();
    for ast in body {
        eval(ast, scope);
    }
    Value::None
}

pub fn fn_call(name: &str, args: &Vec<AstNode>, scope: &mut Scope) -> Value {
    let mut evalled_args = vec![];
    for arg in args {
        evalled_args.push(eval(arg, scope));
    }
    if name == "print" {
        print_builtin(evalled_args)
    } else {
        // let body = match scope.fns.get(name) {
        //     Some(body) => body,
        //     None => panic!("unknown function {}", name),
        // };
        // exec_fn(name, &body, scope)
        exec_fn(name, scope)
    }
}



pub struct Scope {
    pub fns: HashMap<String, Vec<AstNode>>,
    pub vars: HashMap<String, Value>,
}
// impl Scope {
//    pub fn add_fn(&mut self, func: AstNode) {
       
//    }
// }

pub fn eval(ast: &AstNode, scope: &mut Scope) -> Value {
    match ast {
        AstNode::FnCall{ name, args } => fn_call(name, args, scope),
        AstNode::FnDef{ name, body } => {
            scope.fns.insert(String::from(name), *body);
            Value::None
        }
        AstNode::Str(x) => Value::Str(*x),
        AstNode::Integer(x) => Value::Integer(*x),
        AstNode::DoublePrecisionFloat(x) => Value::DoublePrecisionFloat(*x),
        _ => panic!("Unexpected ast {:?}", ast),
    }
}

pub fn run(ast_list: Vec<AstNode>) {
    let mut scope = Scope { fns: HashMap::new(), vars: HashMap::new() };
    for ast_node in ast_list {
        eval(&ast_node, &mut scope);
    }
}


fn main() {
    let unparsed_file = std::fs::read_to_string("main.iris")
    .expect("cannot read iris file");
    // let ast = match parse(&unparsed_file) {
    //     Ok(res) => res,
    //     Err(e) => panic!(e),
    // };
    let ast = parse(&unparsed_file).expect("unsuccessful parse");
    run(ast);
}
