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
use std::io::Write;

fn main() {
    // Fetch file string.
    let unparsed_file = fs::read_to_string("src/files/test.leo").expect("cannot read file");
    println!("Unparsed file:\n{:?}\n", unparsed_file);

    // Create AST from file string.
    let file = parse(&unparsed_file).expect("unsuccessful parse");

    // Perform constant folding.
    let optimized_file = fold(file).expect("could not fold constants");

    // Write program to output.
    let mut w = fs::File::create("src/files/target/testActual.leo").unwrap();
    write!(&mut w, "{}", optimized_file).unwrap();
    println!("Resulting program:\n\n{}", optimized_file);
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::io::Write;
    use crate::{ast, parser::*, optimizer::*};
    fn write_testfile(testname: &str) {
        let read_from = format!("src/files/tests/{}.leo",testname);
        let unparsed_file = fs::read_to_string(read_from).expect("cannot read file");

        let file = parse(&unparsed_file).expect("unsuccessful parse");
        let optimized_file = fold(file).expect("could not fold constants");

        let write_to = format!("src/files/target/{}Actual.leo",testname);
        let mut w = fs::File::create(write_to).unwrap();
        write!(&mut w, "{}", optimized_file).unwrap();
    }
    fn compare_testfile(testname: &str) {
        let path_to_actual = format!("src/files/target/{}Actual.leo",testname);
        let actual = fs::read_to_string(path_to_actual).expect("cannot read file");

        let path_to_expected = format!("src/files/expected/{}Expected.leo",testname);
        let expected = fs::read_to_string(path_to_expected).expect("cannot read file");

        assert_eq!(actual,expected);
    }


    #[test]
    fn test_basic() {
        write_testfile("test");
        compare_testfile("test");
    }

    #[test]
    #[should_panic]
    fn test_subtract_underflow() {
        write_testfile("test2");
        compare_testfile("test2");
    }
}