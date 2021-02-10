
mod parse;
mod run;

#[macro_use] extern crate lalrpop_util;

lalrpop_mod!(pub grammar); // synthesized by LALRPOP

fn main() {
    // let unparsed_file = std::fs::read_to_string("main.iris")
    //     .expect("cannot read iris file");

    // let ast = parse::parse(&unparsed_file).expect("unsuccessful parse");
    // // println!("{:?}", ast);

    // run::run(ast);
    // assert!(grammar::TermParser::new().parse("22").is_ok());
    println!("hi");
}
