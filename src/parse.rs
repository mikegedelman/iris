use pest::Parser;
use pest::error::Error;

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
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Shl,
    Shr,
    And,
    Or,
    Xor,
    Exp,
}

#[derive(Clone, Debug)]
pub enum AstNode {
    FnCall {
        name: String,
        args: Vec<AstNode>,
    },
    FnDef {
        name: String,
        args: Vec<String>,
        body: Vec<AstNode>,
    },
    Term(Term),
    Arithmetic(Box<AstNode>, Op, Box<AstNode>),
    Assignment(Term, Box<AstNode>),
    Return(Box<AstNode>),
}

pub fn parse_fncallargs(pair: pest::iterators::Pair<Rule>) -> Vec<AstNode> {
    let mut ast = vec![];
    let pairs = pair.into_inner();
    for pair in pairs {
        ast.push(parse_expr(pair));
    }
    ast
}

pub fn parse_fndefargs(pair: pest::iterators::Pair<Rule>) -> Vec<String> {
    let mut terms = vec![];
    let pairs = pair.into_inner();
    for pair in pairs {
        terms.push(pair.as_str().to_string());
    }
    terms
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

            let mut next = pair.next().unwrap();
            let args = match next.as_rule() {
                Rule::funcDecArgsList => {
                    let res = parse_fndefargs(next);
                    next = pair.next().unwrap();
                    res
                }
                _ => vec![],
            };

            let body = parse_fnbody(next);
            AstNode::FnDef {
                name,
                args,
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
        Rule::arithmetic => {
            // https://docs.rs/pest/1.0.0-beta.2/pest/prec_climber/struct.PrecClimber.html here?
            let mut pair = pair.into_inner();
            let lhs = parse_expr(pair.next().unwrap());
            let op = match pair.next().unwrap().as_str() {
                "+"  => Op::Add,
                "-"  => Op::Sub,
                "*"  => Op::Mul,
                "/"  => Op::Div,
                "%"  => Op::Mod,
                "<<" => Op::Shl,
                ">>" => Op::Shr,
                "&"  => Op::And,
                "|"  => Op::Or,
                "^"  => Op::Xor,
                "**" => Op::Exp,
                _    => panic!("unknown infix operator"),
            };
            let rhs = parse_expr(pair.next().unwrap());
            AstNode::Arithmetic(Box::new(lhs), op, Box::new(rhs))
        }
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
