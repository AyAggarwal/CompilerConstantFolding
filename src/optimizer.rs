use crate::ast::*;
use crate::error::CompilerError;
use crate::Value::Integer;
use crate::{Expression, Program};
use std::collections::HashMap;
use std::u8;

type Result<T> = std::result::Result<T, CompilerError>;

//takes in a given Program AST and returns a new AST with constants, expressions, and booleans folded.
pub fn fold(program: Program) -> Result<Program> {
    //initialize memory
    let mut memory = HashMap::new();

    //grab name and inputs
    let name = program.name;
    let inputs = program.inputs;

    //iterate through statements and attempt evaluation
    let mut statements = program.statements;
    for statement in statements.iter_mut() {
        match statement {
            //assign statement "let a = 1u8 + 2u8"
            Statement::Assign {
                //a
                variable,
                //1u8 + 2u8
                expression,
            } => {
                //recursive evaluation of the expression tree, with memory provided. memory is not mutated by evaluate fn.
                let opt = evaluate(expression.clone(), &mut memory);
                match opt {
                    Some(eval_result) => match eval_result {
                        //fold current expression if Ok result
                        Ok(val) => {
                            *expression = Expression::Value(Box::new(Integer(val)));
                            memory.insert(variable.clone(), val);
                        }
                        //return given error from evaluation
                        Err(e) => return Err(e),
                    },
                    //cannot be folded due to unkown identifier, move on
                    None => continue,
                }
            }
        }
    }
    Ok(Program {
        name,
        inputs,
        statements,
    })
}

//has to be able to run a pass
fn evaluate(exp: Expression, memory: &mut HashMap<String, u8>) -> Option<Result<u8>> {
    match exp {
        Expression::Binary {
            left,
            operator,
            right,
        } => {
            let lv = match left {
                // 1u8
                Integer(l) => l,
                // a
                Value::Identifier(iden) => {
                    //check for value of identifier in memory
                    let poll = memory.get(&iden);
                    match poll {
                        //update left value
                        Some(val) => *val,
                        //continue traversal
                        None => return None,
                    }
                }
                Value::Expression(_) => return None,
            };
            let rv;
            //attempt evaluation of right side
            let eval_valid = evaluate(*right, memory);
            match eval_valid {
                Some(eval_result) => match eval_result {
                    //fold right value
                    Ok(int) => {
                        rv = int;
                    }
                    //return evaluation error back to caller
                    Err(e) => return Some(Err(e)),
                },
                //could not fold, move on
                None => return None,
            }

            //evaluation
            match operator {
                Operator::Add => Some(add_u8(lv, rv)),
                Operator::Subtract => Some(sub_u8(lv, rv)),
                Operator::Multiply => Some(mul_u8(lv, rv)),
                Operator::Divide => Some(div_u8(lv, rv)),
            }
        }
        //hit the end of the expression
        Expression::Value(x) => match *x {
            Integer(val) => Some(Ok(val)),
            //check if iden has been seen before
            Value::Identifier(iden) => {
                let poll = memory.get(&iden);
                match poll {
                    Some(val) => Some(Ok(*val)),
                    None => return None,
                }
            }
            Value::Expression(_) => None,
        },
    }
}

//helper function to attempt addition and handle errors
fn add_u8(v1: u8, v2: u8) -> Result<u8> {
    let (val, overflow) = v1.overflowing_add(v2);
    if overflow {
        Err(CompilerError::Overflow)
    } else {
        Ok(val)
    }
}

//helper function to attempt subtraction and handle errors
fn sub_u8(v1: u8, v2: u8) -> Result<u8> {
    let (val, underflow) = v1.overflowing_sub(v2);
    if underflow {
        Err(CompilerError::Underflow)
    } else {
        Ok(val)
    }
}

//helper function to attempt multiplication and handle errors
fn mul_u8(v1: u8, v2: u8) -> Result<u8> {
    let (val, overflow) = v1.overflowing_mul(v2);
    if overflow {
        Err(CompilerError::Overflow)
    } else {
        Ok(val)
    }
}

//helper function to attempt division and handle errors
fn div_u8(v1: u8, v2: u8) -> Result<u8> {
    if v2 == 0 {
        return Err(CompilerError::DivByZero);
    }
    let (val, overflow) = v1.overflowing_div(v2);
    if overflow {
        Err(CompilerError::Overflow)
    } else {
        Ok(val)
    }
}
