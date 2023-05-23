use crate::ast::*;
use crate::error::CompilerError;
use crate::Value::*;
use crate::{Expression, Program};
use std::collections::HashMap;

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
    let mut new_statements = Vec::new();
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
                            //*expression = Expression::Value(Box::new(val.clone()));
                            memory.insert(variable.clone(), val.clone());
                            new_statements.push(Statement::Assign {
                                variable: variable.clone(),
                                expression: Expression::Value(Box::new(val.clone())),
                            })
                        }
                        //return given error from evaluation
                        Err(e) => return Err(e),
                    },
                    //cannot be folded due to unkown identifier, move on
                    None => new_statements.push(statement.clone()),
                }
            }
            Statement::If {
                expression,
                statements_a,
                statements_b,
            } => {
                // evaluate expression, if yes insert A if no insert B if none just continue
                let opt = evaluate(expression.clone(), &mut memory);
                match opt {
                    Some(result) => match result {
                        Ok(boolean) => {
                            if let Boolean(x) = boolean {
                                if x {
                                    new_statements.append(statements_a);
                                } else {
                                    new_statements.append(statements_b);
                                }
                            } else {
                                new_statements.push(statement.clone());
                            }
                        }
                        Err(e) => return Err(e),
                    },
                    None => new_statements.push(statement.clone()),
                }
            }
        }
    }
    Ok(Program {
        name,
        inputs,
        statements: new_statements,
    })
}

//has to be able to run a pass
fn evaluate(exp: Expression, memory: &mut HashMap<String, Value>) -> Option<Result<Value>> {
    match exp {
        Expression::Binary {
            left,
            operator,
            right,
        } => {
            let lv;
            let lv_valid = evaluate(Expression::Value(Box::new(left)), memory);
            match lv_valid {
                Some(lv_result) => match lv_result {
                    //fold right value
                    Ok(val) => {
                        lv = val;
                    }
                    //return evaluation error back to caller
                    Err(e) => return Some(Err(e)),
                },
                //could not fold, move on
                None => return None,
            }
            let rv;
            //attempt evaluation of right side
            let rv_valid = evaluate(*right, memory);
            match rv_valid {
                Some(rv_result) => match rv_result {
                    //fold right value
                    Ok(val) => {
                        rv = val;
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
                Operator::GreaterThan => Some(gt_bool(lv, rv)),
                Operator::LessThan => Some(lt_bool(lv, rv)),
                Operator::Equal => Some(eq_bool(lv, rv)),
            }
        }
        //hit the end of the expression
        Expression::Value(x) => match *x {
            Integer(_) => Some(Ok(*x)),
            //check if iden has been seen before
            Identifier(iden) => {
                let poll = memory.get(&iden);
                match poll {
                    Some(val) => Some(Ok(val.clone())),
                    None => return None,
                }
            }
            Value::Expression(_) => None,
            Boolean(_) => Some(Ok(*x)),
        },
    }
}

//helper function to attempt addition and handle errors
fn add_u8(v1: Value, v2: Value) -> Result<Value> {
    match v1 {
        Integer(x) => match v2 {
            Integer(y) => {
                let (val, overflow) = x.overflowing_add(y);
                if overflow {
                    Err(CompilerError::Overflow)
                } else {
                    Ok(Integer(val))
                }
            }
            _ => return Err(CompilerError::MismatchType),
        },
        _ => return Err(CompilerError::MismatchType),
    }
}

//helper function to attempt subtraction and handle errors
fn sub_u8(v1: Value, v2: Value) -> Result<Value> {
    match v1 {
        Integer(x) => match v2 {
            Integer(y) => {
                let (val, overflow) = x.overflowing_sub(y);
                if overflow {
                    Err(CompilerError::Underflow)
                } else {
                    Ok(Integer(val))
                }
            }
            _ => return Err(CompilerError::MismatchType),
        },
        _ => return Err(CompilerError::MismatchType),
    }
}

//helper function to attempt multiplication and handle errors
fn mul_u8(v1: Value, v2: Value) -> Result<Value> {
    match v1 {
        Integer(x) => match v2 {
            Integer(y) => {
                let (val, overflow) = x.overflowing_mul(y);
                if overflow {
                    Err(CompilerError::Overflow)
                } else {
                    Ok(Integer(val))
                }
            }
            _ => return Err(CompilerError::MismatchType),
        },
        _ => return Err(CompilerError::MismatchType),
    }
}

//helper function to attempt division and handle errors
fn div_u8(v1: Value, v2: Value) -> Result<Value> {
    match v1 {
        Integer(x) => match v2 {
            Integer(y) => {
                if y == 0 {
                    return Err(CompilerError::DivByZero);
                }
                let (val, overflow) = x.overflowing_div(y);
                if overflow {
                    return Err(CompilerError::Overflow);
                } else {
                    return Ok(Integer(val));
                }
            }
            _ => return Err(CompilerError::MismatchType),
        },
        _ => return Err(CompilerError::MismatchType),
    }
}

//helper function to attempt multiplication and handle errors
fn gt_bool(v1: Value, v2: Value) -> Result<Value> {
    match v1 {
        Integer(x) => match v2 {
            Integer(y) => {
                if x > y {
                    Ok(Boolean(true))
                } else {
                    Ok(Boolean(false))
                }
            }
            _ => return Err(CompilerError::MismatchType),
        },
        _ => return Err(CompilerError::MismatchType),
    }
}

//helper function to attempt multiplication and handle errors
fn lt_bool(v1: Value, v2: Value) -> Result<Value> {
    match v1 {
        Integer(x) => match v2 {
            Integer(y) => {
                if x < y {
                    Ok(Boolean(true))
                } else {
                    Ok(Boolean(false))
                }
            }
            _ => return Err(CompilerError::MismatchType),
        },
        _ => return Err(CompilerError::MismatchType),
    }
}

//helper function to attempt multiplication and handle errors
fn eq_bool(v1: Value, v2: Value) -> Result<Value> {
    match v1 {
        Integer(x) => match v2 {
            Integer(y) => {
                if x == y {
                    Ok(Boolean(true))
                } else {
                    Ok(Boolean(false))
                }
            }
            _ => return Err(CompilerError::MismatchType),
        },
        _ => return Err(CompilerError::MismatchType),
    }
}
