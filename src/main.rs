extern crate pest;
#[macro_use]
extern crate pest_derive;

mod parse;
mod run;


fn main() {
    let unparsed_file = std::fs::read_to_string("main.iris")
        .expect("cannot read iris file");

    let ast = parse::parse(&unparsed_file).expect("unsuccessful parse");
    // println!("{:?}", ast);

    run::run(ast);
}
