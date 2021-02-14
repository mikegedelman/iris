mod tok;

use logos::Logos;
use crate::ast::AstNode;

lalrpop_mod!(pub grammar); // synthesized by LALRPOP

pub fn parse(fname: &str) -> Vec<AstNode> {
    let unparsed_file = std::fs::read_to_string(&fname)
        .expect("cannot read iris file");

    // Logos' spanned() gives us a vector with type Iterator<Item, Range>
    // LALRPOP wants Iterator<Location, Item, Location> - transform to that
    let lexer = tok::Tok::lexer(&unparsed_file).spanned()
        .map(|(tok, range)| (range.start, tok, range.end));

    grammar::IrisParser::new().parse(&unparsed_file, lexer).unwrap()
}

// #[cfg(test)]
// mod tests {
//     use crate::ast::{AstNode,Term};

//     #[test]
//     fn iris() {
//         assert_eq!(grammar::IrisParser::new().parse("22;").unwrap()[0], AstNode::Term(Term::Integer(22)));
//     }
// }