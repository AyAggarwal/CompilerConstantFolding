use std::error::Error;
use crate::ast::*;
use crate::{Program, Rule,Expression,};
use crate::Value::Integer;

pub fn fold(program: Program) -> Result<Program,Box<dyn Error>> {
    let name = program.name;
    let inputs = program.inputs;
    let mut statements = program.statements;
    for statement in statements.iter_mut() {
        match statement {
            Statement::Assign { variable: _, expression } => {
                let opt = evaluate(expression.clone());
                match opt {
                    Some(val) => {
                        *expression = Expression::Value(Box::new(Integer(val)));
                    },
                    None => continue,
                }
            },
        }
    }
    Ok(Program {
        name, inputs, statements
    })
}

//has to be able to run a pass
pub fn evaluate(exp: Expression) -> Option<u8> {
    match exp {
        Expression::Binary { left, operator, right } => {
            let lv = match left {
                Integer(l) => l,
                _ => return None
            };
            let rv = evaluate(*right).unwrap();
            match operator {
                Operator::Add => {
                    return Some(add_u8(lv,rv))
                },
                Operator::Subtract => {
                    return Some(sub_u8(lv, rv))
                },
                Operator::Multiply => {
                    return Some(mul_u8(lv,rv))
                },
                Operator::Divide => {
                    return Some(div_u8(lv,rv))
                },
            }
        },
        Expression::Value(x) => {
            return match *x {
                Integer(val) => Some(val),
                Value::Identifier(_) => None,
                Value::Expression(_) => None,
            }
        },
    }
}

pub fn add_u8(v1: u8, v2: u8) -> u8 {
    let (val, overflow) = v1.overflowing_add(v2);
    if overflow {
        panic!("addition of values {} and {} caused integer overflow",v1,v2)
    } else {
        val
    }
}

pub fn sub_u8(v1: u8, v2: u8) -> u8 {
    let (val, overflow) = v1.overflowing_sub(v2);
    if overflow {
        panic!("subtraction of values {} and {} caused integer overflow",v1,v2)
    } else {
        val
    }
}

pub fn mul_u8(v1: u8, v2: u8) -> u8 {
    let (val, overflow) = v1.overflowing_mul(v2);
    if overflow {
        panic!("multiplication of values {} and {} caused integer overflow",v1,v2)
    } else {
        val
    }
}

pub fn div_u8(v1: u8, v2: u8) -> u8 {
    if v2 == 0 {
        panic!("division by zero")
    }
    let (val, overflow) = v1.overflowing_div(v2);
    if overflow {
        panic!("addition of values {} and {} caused integer overflow",v1,v2)
    } else {
        val
    }
}

