mod tok;

use logos::Logos;
use crate::ast::AstNode;

// This macro pulls in the grammar defined in grammar.lalrpop
lalrpop_mod!(pub grammar);

/// Read the given file and attempt to parse it
pub fn parse(fname: &str) -> Vec<AstNode> {
    let unparsed_file = std::fs::read_to_string(&fname)
        .expect("cannot read iris file");

    // Logos' spanned() gives us a vector with type Iterator<Item, Range>
    // LALRPOP wants Iterator<Location, Item, Location> - transform to that
    let mut lexer: Vec<(usize,tok::Tok,usize)> = tok::Tok::lexer(&unparsed_file).spanned()
        .map(|(tok, range)| (range.start, tok, range.end))
        .collect();

    // This next bit is really awful: if the last token isn't a Crlf, just append one
    // This is just a hack because I wanted to stop fighting the parser generator
    // to get inputs to work when there's no trailing newline.
    if lexer.len() == 0 {
        return vec![];
    }
    let (_,last,_) = lexer.last().unwrap();
    if last != &tok::Tok::Crlf {
        lexer.push((0, tok::Tok::Crlf, 0));
    }

    grammar::IrisParser::new().parse(&unparsed_file, lexer).unwrap()
}
