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
    let unparsed_file = fs::read_to_string("src/files/tests/test_basic.leo").expect("cannot read file");
    println!("Unparsed file:\n{:?}\n", unparsed_file);

    // Create AST from file string.
    let file = parse(&unparsed_file).expect("unsuccessful parse");

    // Perform constant folding.
    let optimized_file = fold(file).expect("could not fold constants");

    // Write program to output.
    let mut w = fs::File::create("src/files/actual/test_basicActual.leo").unwrap();
    write!(&mut w, "{}", optimized_file).unwrap();
    println!("Resulting program:\n\n{}", optimized_file);
}

#[cfg(test)]
mod tests {
    use crate::{optimizer::*, parser::*};
    use std::fs;
    use std::io::Write;

    #[test]
    fn test_basic() {
        run_test("test_basic")
    }

    #[test]
    fn test_revert() {
        run_test("test_revert")
    }

    #[test]
    fn test_subtract_underflow() {
        test_failure("test_sub_underflow", CompilerError::Underflow)
    }

    #[test]
    fn test_add_overflow() {
        test_failure("test_add_overflow",CompilerError::Overflow)
    }

    #[test]
    fn test_div_zero() {
        test_failure("test_div_zero",CompilerError::DivByZero)
    }

    #[test]
    fn test_mul_overflow() {
        test_failure("test_mul_overflow",CompilerError::Overflow)
    }

    fn write_testfile(testname: &str) {
        let read_from = format!("src/files/tests/{}.leo", testname);
        let unparsed_file = fs::read_to_string(read_from).expect("cannot read file");

        let file = parse(&unparsed_file).expect("unsuccessful parse");
        let fold_result = fold(file);
        let optimized_file;
        if fold_result.is_ok() {
            optimized_file = fold_result.unwrap();
            let write_to = format!("src/files/actual/{}Actual.leo", testname);

            let mut w = fs::File::create(write_to).unwrap();
            write!(&mut w, "{}", optimized_file);
        } else {
            assert!(false, "could not write testfile due to error: {}", fold_result.err().unwrap())
        }

    }

    fn test_failure(testname: &str,expected_error: CompilerError) {
        let read_from = format!("src/files/tests/{}.leo", testname);
        let unparsed_file = fs::read_to_string(read_from).expect("cannot read file");

        let file = parse(&unparsed_file).expect("unsuccessful parse");
        let folding_err = fold(file).unwrap_err();
        assert_eq!(expected_error, folding_err)
    }

    fn compare_testfile(testname: &str) {
        let path_to_actual = format!("src/files/actual/{}Actual.leo", testname);
        let actual = fs::read_to_string(path_to_actual).expect("cannot read file");

        let path_to_expected = format!("src/files/expected/{}Expected.leo", testname);
        let expected = fs::read_to_string(path_to_expected).expect("cannot read file");

        assert_eq!(actual, expected);
    }

    fn run_test(testname: &str) {
        write_testfile(testname);
        compare_testfile(testname);
    }
}
