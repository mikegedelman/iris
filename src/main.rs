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

#[derive(Clone, Debug)]
pub enum Term {
    Integer(i32),
    DoublePrecisionFloat(f64),
    Ident(String),
    Str(String),
}

#[derive(Clone, Debug)]
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
    Term(Term),
    Assignment(Term, Box<AstNode>),
    Return(Box<AstNode>),
}

#[derive(Clone, Debug)]
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

pub fn parse_stmt(pair: pest::iterators::Pair<Rule>) -> AstNode {
    match pair.as_rule() {
        Rule::expr => parse_expr(pair),
        Rule::funcDec => {
            let mut pair = pair.into_inner();
            let name = String::from(pair.next().unwrap().as_str());
            let body = parse_fnbody(pair.next().unwrap());
            AstNode::FnDef {
                name,
                body,
            }
        },
        Rule::assignStmt => {
            let mut pair = pair.into_inner();
            let lhs = String::from(pair.next().unwrap().as_str());
            let rhs = parse_expr(pair.next().unwrap());
            AstNode::Assignment(Term::Ident(lhs), Box::new(rhs))
        },
        Rule::returnStmt => {
            let mut pair = pair.into_inner();
            let expr = parse_expr(pair.next().unwrap());
            AstNode::Return(Box::new(expr))
        },
        _ => panic!("not implemented {}", pair),
    }
}

pub fn parse_expr(pair: pest::iterators::Pair<Rule>) -> AstNode {
    match pair.as_rule() {
        Rule::expr => parse_expr(pair.into_inner().next().unwrap()),
        Rule::stmt => parse_stmt(pair.into_inner().next().unwrap()),
        Rule::integer => {
            let istr = pair.as_str();
            let (sign, istr) = match &istr[..1] {
                "_" => (-1, &istr[1..]),
                _ => (1, &istr[..]),
            };
            let integer: i32 = istr.parse().unwrap();
            AstNode::Term(Term::Integer(sign * integer))
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
            AstNode::Term(Term::DoublePrecisionFloat(flt))
        }
        Rule::string => {
            let str = &pair.as_str();
            // Strip leading and ending quotes.
            let str = &str[1..str.len() - 1];
            // Escaped string quotes become single quotes here.
            let str = str.replace("''", "'");
            AstNode::Term(Term::Str(String::from(str)))
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
        Rule::ident => AstNode::Term(Term::Ident(pair.as_str().to_string())),
        _ => panic!("Not yet implemented: {} rule type {:?}", pair.as_str(), pair.as_rule()),
    }
}

pub fn parse(source: &str) -> Result<Vec<AstNode>, Error<Rule>> {
    let mut ast = vec![];

    let pairs = LangParser::parse(Rule::program, source)?;
    for pair in pairs {
        match pair.as_rule() {
            Rule::stmt => {
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



pub fn fn_call(name: &str, args: &Vec<AstNode>, outer_scope: &mut Scope) -> Value {
    let mut evalled_args = vec![];
    for arg in args {
        evalled_args.push(eval(arg, outer_scope));
    }
    if name == "print" {
        print_builtin(evalled_args)
    } else {
        // let body = match scope.fns.get(name) {
        //     Some(body) => body,
        //     None => panic!("unknown function {}", name),
        // };
        // exec_fn(name, &body, scope)
        outer_scope.exec_fn(name)
    }
}



pub struct Scope {
    fns: HashMap<String, Vec<AstNode>>,
    vars: HashMap<String, Value>,
}
impl Scope {
    pub fn new() -> Scope  {  Scope{ fns: HashMap::new(), vars: HashMap::new() } }
    pub fn add_fn(&mut self, name: String, body: Vec<AstNode>) {
        self.fns.insert(name, body);
    }
    pub fn get_fn_body(&self, name: &str) -> Vec<AstNode> {
        self.fns.get(name).unwrap().to_vec()
    }

    pub fn exec_fn(&mut self, name: &str) -> Value {
        let mut inner_scope = Scope::new();
        let mut ret: Option<Value> = None;
        for ast in self.get_fn_body(name) {
            match ast {
                AstNode::Return(ast) => {
                    ret = Some(eval(&ast, &mut inner_scope));
                    break;
                },
                _ => stmt(&ast, self, &mut inner_scope)
            };
        }

        match ret {
            Some(val) => val,
            None => Value::None,
        }
    }
    pub fn set_var(&mut self, name: &str, val: Value) {
        self.vars.insert(name.to_string(), val);
    }
    pub fn get_var(&mut self, name: &str) -> Value {
        let val = self.vars.get(name).unwrap();
        val.clone()
    }
}

pub fn stmt(ast: &AstNode, outer_scope: &mut Scope, inner_scope: &mut Scope) {
    match ast {
        // AstNode::FnDef{ name, body } => {
        //     outer_scope.add_fn(String::from(name), body.to_vec());
        // },
        AstNode::Assignment(Term::Ident(var), astbox) => {
            let val = eval(astbox, inner_scope);
            inner_scope.set_var(var, val);
        }
        _ => { eval(ast, inner_scope); },
    };
}

pub fn eval(ast: &AstNode, scope: &mut Scope) -> Value {
    match ast {
        AstNode::FnCall{ name, args } => fn_call(name, args, scope),
        AstNode::FnDef{ name, body } => {
            scope.add_fn(String::from(name), body.to_vec());
            Value::None
        }
        AstNode::Term(Term::Str(x)) => Value::Str(x.to_string()),
        AstNode::Term(Term::Integer(x)) => Value::Integer(*x),
        AstNode::Term(Term::DoublePrecisionFloat(x)) => Value::DoublePrecisionFloat(*x),
        AstNode::Term(Term::Ident(var)) => {
            scope.get_var(var)
        }
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
    // println!("{:?}", ast);
    run(ast);
}
