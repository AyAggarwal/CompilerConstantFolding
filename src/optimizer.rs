use crate::ast::*;
use crate::Value::Integer;
use crate::{Expression, Program};
use std::collections::HashMap;
use std::{fmt, u8};

type Result<T> = std::result::Result<T, CompilerError>;

#[derive(Debug, PartialEq)]
pub enum CompilerError {
    Underflow,
    Overflow,
    DivByZero,
}

impl fmt::Display for CompilerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CompilerError::Underflow => write!(f, "Integer underflow during evaluation"),
            CompilerError::Overflow => write!(f, "Integer overflow during evaluation"),
            CompilerError::DivByZero => write!(f, "Division by Zero during evaluation"),
        }
    }
}

pub fn fold(program: Program) -> Result<Program> {
    let mut memory = HashMap::new();
    let name = program.name;
    let inputs = program.inputs;
    let mut statements = program.statements;
    for statement in statements.iter_mut() {
        match statement {
            Statement::Assign {
                variable,
                expression,
            } => {
                let opt = evaluate(expression.clone(), &mut memory);
                match opt {
                    Some(eval_result) => match eval_result {
                        Ok(val) => {
                            *expression = Expression::Value(Box::new(Integer(val)));
                            memory.insert(variable.clone(), val);
                        }
                        Err(e) => return Err(e),
                    },
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
                Integer(l) => l,
                Value::Identifier(iden) => {
                    let poll = memory.get(&iden);
                    match poll {
                        Some(val) => *val,
                        None => return None,
                    }
                }
                Value::Expression(_) => return None,
            };
            let rv;
            let eval_valid = evaluate(*right, memory);
            match eval_valid {
                Some(eval_result) => match eval_result {
                    Ok(int) => {
                        rv = int;
                    }
                    Err(e) => return Some(Err(e)),
                },
                None => return None,
            }

            match operator {
                Operator::Add => Some(add_u8(lv, rv)),
                Operator::Subtract => Some(sub_u8(lv, rv)),
                Operator::Multiply => Some(mul_u8(lv, rv)),
                Operator::Divide => Some(div_u8(lv, rv)),
            }
        }
        Expression::Value(x) => match *x {
            Integer(val) => Some(Ok(val)),
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

fn add_u8(v1: u8, v2: u8) -> Result<u8> {
    let (val, overflow) = v1.overflowing_add(v2);
    if overflow {
        Err(CompilerError::Overflow)
    } else {
        Ok(val)
    }
}

fn sub_u8(v1: u8, v2: u8) -> Result<u8> {
    let (val, underflow) = v1.overflowing_sub(v2);
    if underflow {
        Err(CompilerError::Underflow)
    } else {
        Ok(val)
    }
}

fn mul_u8(v1: u8, v2: u8) -> Result<u8> {
    let (val, overflow) = v1.overflowing_mul(v2);
    if overflow {
        Err(CompilerError::Overflow)
    } else {
        Ok(val)
    }
}

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
