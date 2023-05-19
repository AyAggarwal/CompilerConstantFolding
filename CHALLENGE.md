# Coding Challenge: Constant Folding
**Task: Implement a constant folding optimization in Rust.**


## Challenge Statement

Given the grammar rules for a small subset of the Leo programming language as described, your task is to implement a constant folding optimization in a compiler for this language.

Constant folding is a compiler optimization technique that identifies and evaluates constant expressions at compile-time rather than computing them at runtime. For example, in the Leo language, an expression such as `1u8 + 1u8` can be simplified to `2u8` during the compilation.

In the given repository the lexer and parser are already implemented

* Lexer: This will break down the Leo program into tokens. Tokens could be keywords (like "function", "let"), identifiers, operators, or literals.

* Parser: This will parse the tokens into an abstract syntax tree (AST) that represents the program's structure.

You are expected to:

1. Implement a constant folding optimization pass: This pass will traverse the AST, identify constant expressions (like the `1u8 + 1u8` in the assignment `let a = 1u8 + 1u8;`), evaluate them, and replace them with their results.

2. Implement code generation: Generate the equivalent Leo code from the optimized AST.

Your implementation should be able to handle input programs that conform to the provided grammar, and output an equivalent, but optimized, Leo program. For instance, the program:

```leo
function main() {
    let a = 1u8 + 1u8;
}
```

Should be optimized to:

```leo
function main() {
    let a = 2u8;
}
```

Remember to handle potential edge cases, like constant folding in nested expressions.

**Some considerations:**
- **Robustness.** Will your algorithm hold for more complex programs?
- **Testing.** How would you design a robust testing infrastructure to cover all edge cases?
- **Errors.** How do you handle errors?
- **Code cleanliness, organization, and documentation.**

### Resources

https://www.rust-lang.org/tools/install

https://github.com/pest-parser/pest

https://pest.rs/book/

https://en.wikipedia.org/wiki/Formal_grammar

https://craftinginterpreters.com/parsing-expressions.html

https://craftinginterpreters.com/

https://en.wikipedia.org/wiki/Constant_folding

https://en.wikipedia.org/wiki/Compile-time_function_execution




