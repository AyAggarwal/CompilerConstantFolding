# Aleo Constant Folidng Implemetation
This repo contains an example optimizer for the basic leo subset language. 

To generate compiler optimized code, the command `generate` must be used. Arguments to Input and Output are paths to filenames and are required.
```
cargo run generate  --input <INPUT> --output <OUTPUT>
```

as an example, try creating an after.leo file based on the given before in the files directory!
```
cargo run generate --input src/files/before.leo --output src/files/after.leo 

```
`after.leo` should appear in `src/files`

The main program should work out of the box without any configuration. To test the basic usage run the following in the root directory. 
```
cargo run
```

To test the advanced robustness features run the following command in the root directory.
```
cargo test
```
## Feature List
- Constant Folding in Assign statements
- Identifier based memory and folding
- Boolean folding in assingn statements
- If statements
- Folding of If statements, and nested code
- code generation in a simple CLI command
- Error handleing and reporting

# Implementation Overview
### Optimizer
The LeoParser derived from the leo subset grammar creates an AST with a list of statements. There is only one type of statement which is an assignment of a variable to an expression. The parser creates an expression tree right-recursively. Thus, the optimizer evaluates the expressions recursively, bubbling up the evaluated expressions to the root of the parsed AST. 

The optimizer catches overflow, underflow, and division by zero by using helper functions to perform evaluation of the integer expressions. These errors cause panic if caught, because the program would not run as intended. 

### Testing
The testing suite uses cargo, and the help of directory strucures. The helper function `write_testfile` takes in a filename and looks in the `src/files/tests` directory for the corresponding filename. It writes out the optimized file to `src/files/target`. Next, the `compare_testfile` function will read the generated target as well as the solution file in `src/files/expected` and compare in an assert statement. 

This functionality is wrapped in a clean `run_test` function so the developer can create test files manually and easily add them to the testing suite. 

### Code Generation
The provided `fmt::Display` imeplemetations work exactly as how I would go about implementing code generation, essentially reconstructing the program using the provided grammar and AST. The only implementation needed was to have the Display for `Value` read the type of the integer and print it out. Currently, this uses Rust's `u8` type but in the future the `Value` from the AST can contian an `Integer(Type(U8))` instead to have the type wrapped into the AST. The benefit of this would be including types that don't exist in Rust natively. 

### Order of Operations
`test_order_of_operations.leo` in the tests directory contains expressions that will fail on right-recursive evalutation, but would be evaluate correctly left-to-right.

A genuine attempt at implementing Pratt Parsing is listed on the `pratt` branch. Reached a point where it was not worth refactoring the code to work with Pest's pratt parser, nor for the purposes of this challenge did it make sense to implement it from scratch. 