extern crate logos;
#[macro_use] extern crate lalrpop_util;

mod ast;
mod parse;
mod run;

use std::env;

fn main() {
    // Get filename from cli
    let args: Vec<_> = env::args().collect();
    if args.len() < 2 {
        panic!("No input files given");
    }
    let fname = &args[1];

    // Parse and run the program
    let ast_list = parse::parse(&fname);
    // println!("{:#?}", ast_list);
    run::run(ast_list);

}
