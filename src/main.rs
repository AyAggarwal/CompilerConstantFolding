extern crate pest;
#[macro_use]
extern crate pest_derive;

pub mod ast;
pub use ast::*;

pub mod parser;
pub use parser::*;

pub mod optimizer;
pub use optimizer::*;

mod error;
use error::*;

use clap::Parser as P;
use std::fs;
use std::io::Write;
use std::path::PathBuf;

// clap parsing struct for CLI commands
#[derive(P)]
#[command(author, version, about, long_about = None)]
struct Cli {
    //optional name to parse
    command: Option<String>,

    //input file path including name, requires output
    #[arg(short, long, requires = "output")]
    input: PathBuf,

    //output file path including name, requires input
    #[arg(short, long)]
    output: PathBuf,
}

type Result<T> = std::result::Result<T, GenerationError>;

fn main() {
    let cli = Cli::parse();
    //check for cli options
    if cli.command == Some(String::from("generate")) {
        let input = cli.input;
        let output = cli.output;
        let res = generate(input, output);
        match res {
            //success
            Ok(_) => return,
            //report error
            Err(e) => println!("error generating file: {}", e),
        }
    } else {
        //replicating the behavior of fn write_testfile, but printing it out to the console
        let read_from = format!("src/files/tests/test_basic.leo");
        let write_to = format!("src/files/actual/test_basicActual.leo");
        //generate file
        let result = generate(PathBuf::from(read_from), PathBuf::from(write_to));
        match result {
            Ok(_) => {
                //print to console
                let file = fs::read_to_string("src/files/actual/test_basicActual.leo").unwrap();
                println!("{}", file)
            }
            Err(e) => {
                //report error
                println!("error generating file: {}", e);
            }
        };
    }
}

//code generation function which takes an input and output path
pub fn generate(input: PathBuf, output: PathBuf) -> Result<()> {
    match input.to_str() {
        Some(path) => {
            let unparsed_file =
                fs::read_to_string(path).map_err(|_| GenerationError::FileReadError)?;
            //parse file
            let file = parse(&unparsed_file)?;
            //perform constant folding
            let optimized_file = fold(file)?;
            //report errors or write out to path
            if let Some(out) = output.to_str() {
                if let Ok(mut w) = fs::File::create(out) {
                    write!(&mut w, "{}", optimized_file).unwrap();
                    Ok(())
                } else {
                    Err(GenerationError::FileWriteError)
                }
            } else {
                Err(GenerationError::FileWriteError)
            }
        }
        None => {
            return Err(GenerationError::FileReadError);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::error::{CompilerError, GenerationError};
    use crate::generate;
    use std::fs;
    use std::path::PathBuf;

    #[test]
    fn test_basic() {
        run_test("test_basic")
    }

    #[test]
    fn test_revert() {
        run_test("test_revert")
    }

    #[test]
    fn test_ident() {
        run_test("test_ident")
    }

    #[test]
    fn test_subtract_underflow() {
        test_failure(
            "test_sub_underflow",
            GenerationError::CompilerError(CompilerError::Underflow),
        )
    }

    #[test]
    fn test_add_overflow() {
        test_failure(
            "test_add_overflow",
            GenerationError::CompilerError(CompilerError::Overflow),
        )
    }

    #[test]
    fn test_div_zero() {
        test_failure(
            "test_div_zero",
            GenerationError::CompilerError(CompilerError::DivByZero),
        )
    }

    #[test]
    fn test_mul_overflow() {
        test_failure(
            "test_mul_overflow",
            GenerationError::CompilerError(CompilerError::Overflow),
        )
    }

    //Writes a testfile to the /src/files/actual directory based on the filename
    //which must exist in the /src/files/tests directory.
    fn write_testfile(testname: &str) {
        let read_from = format!("src/files/tests/{}.leo", testname);
        let write_to = format!("src/files/actual/{}Actual.leo", testname);
        let result = generate(PathBuf::from(read_from), PathBuf::from(write_to));
        match result {
            Ok(_) => return,
            Err(e) => {
                println!("{}", e);
                assert!(false)
            }
        };
    }

    //attempts to compile a testfile and expects an error of provided type
    fn test_failure(testname: &str, expected_error: GenerationError) {
        let read_from = format!("src/files/tests/{}.leo", testname);
        let write_to = format!("src/files/actual/{}Actual.leo", testname);
        let result = generate(PathBuf::from(read_from), PathBuf::from(write_to)).unwrap_err();
        assert_eq!(expected_error, result)
    }

    //compares the generated testfile to the exepcted based on the testname
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
