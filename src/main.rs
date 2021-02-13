extern crate logos;

mod tok;
mod ast;
mod run;
mod builtins;

use std::env;

use logos::Logos;
use crate::ast::AstNode;

#[macro_use] extern crate lalrpop_util;
lalrpop_mod!(pub grammar); // synthesized by LALRPOP

fn parse(fname: &str) -> Vec<AstNode> {
    let unparsed_file = std::fs::read_to_string(&fname)
        .expect("cannot read iris file");

    // Logos' spanned() gives us a vector with type Iterator<Item, Range>
    // LALRPOP wants Iterator<Location, Item, Location> - transform to that
    let lexer = crate::tok::Tok::lexer(&unparsed_file).spanned()
        .map(|(tok, range)| (range.start, tok, range.end));

    grammar::IrisParser::new().parse(&unparsed_file, lexer).unwrap()
}

fn main() {
    // Get filename from cli
    let args: Vec<_> = env::args().collect();
    if args.len() < 2 {
        panic!("No input files given");
    }
    let fname = &args[1];

    // Parse and run the program
    let ast_list = parse(&fname);
    run::run(ast_list);
}

#[cfg(test)]
mod tests {
    use crate::ast::{AstNode,Term};

#[test]
fn iris() {
    assert_eq!(grammar::IrisParser::new().parse("22;").unwrap()[0], AstNode::Term(Term::Integer(22)));
    assert_eq!(grammar::IrisParser::new().parse("main;").unwrap()[0], AstNode::Term(Term::Ident("main".to_string())));
    assert_eq!(
        grammar::IrisParser::new().parse("print();").unwrap()[0],
        AstNode::FnCall{ name: "print".to_string(), args: vec![] },
    );
    assert_eq!(
        grammar::IrisParser::new().parse("print(1);").unwrap()[0],
        AstNode::FnCall{ name: "print".to_string(), args: vec![AstNode::Term(Term::Integer(1))] },
    );
    assert_eq!(
        grammar::IrisParser::new().parse("print(1);\n2;\n").unwrap()[0],
        AstNode::FnCall{ name: "print".to_string(), args: vec![AstNode::Term(Term::Integer(1))] },
    );
    assert_eq!(
        grammar::IrisParser::new().parse("print (1, 2);").unwrap()[0],
        AstNode::FnCall{
            name: "print".to_string(),
            args: vec![AstNode::Term(Term::Integer(1)), AstNode::Term(Term::Integer(2))] },
    );
    assert_eq!(
        grammar::IrisParser::new().parse("fn myfn(x) { print(x); }").unwrap()[0],
        AstNode::FnDef{
            name: "myfn".to_string(),
            args: vec!["x".to_string()],
            body: vec![AstNode::FnCall{
                name: "print".to_string(),
                args: vec![AstNode::Term(Term::Ident("x".to_string()))]
            }],
        },
    );
    assert_eq!(
        grammar::IrisParser::new().parse("
          fn test(a, b) {
            othertest(a, b);
          }").unwrap()[0],
        AstNode::FnDef{
            name: "test".to_string(),
            args: vec!["a".to_string(), "b".to_string()],
            body: vec![AstNode::FnCall{
                name: "othertest".to_string(),
                args: vec![
                    AstNode::Term(Term::Ident("a".to_string())),
                    AstNode::Term(Term::Ident("b".to_string())),
                ]
            }],
        },
    );
    assert_eq!(
        grammar::IrisParser::new().parse("x = f(y);").unwrap()[0],
        AstNode::VarDeclaration(
            Term::Ident("x".to_string()),
            Box::new(AstNode::FnCall{
                name: "f".to_string(),
                args: vec![
                    AstNode::Term(Term::Ident("y".to_string())),
                ]
            })
        ),
    );
    assert_eq!(
        grammar::IrisParser::new().parse("fn x() { return 5; }").unwrap()[0],
        AstNode::FnDef{
            name: "x".to_string(),
            args: vec![],
            body: vec![AstNode::Return(
                Box::new(AstNode::Term(Term::Integer(5))),
            )],
        },
    );
}
}