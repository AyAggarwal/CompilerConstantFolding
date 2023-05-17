extern crate pest;
#[macro_use]
extern crate pest_derive;

pub mod ast;
pub use ast::*;

pub mod parser;
pub use parser::*;

pub mod optimizer;
pub use optimizer::*;

use std::fs;

fn main() {
    // Fetch file string.
    let unparsed_file = fs::read_to_string("src/files/test.leo").expect("cannot read file");
    println!("Unparsed file:\n{:?}\n", unparsed_file);

    // Create AST from file string.
    let file = parse(&unparsed_file).expect("unsuccessful parse");

    // Perform constant folding.
    let optimized_file = fold(file).expect("rust is lit");
    // Write program to output.
    println!("Resulting program:\n\n{}", optimized_file);
}
