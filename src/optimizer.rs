use pest::error::Error;
use crate::ast::*;
use crate::{Program, Rule,Expression,};
use crate::Value::Integer;

pub fn fold(program: Program) -> Result<Program,Error<Rule>> {
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

            match operator {
                Operator::Add => {
                    let rv = evaluate(*right).unwrap();
                    return Some(lv + rv)
                },
                Operator::Subtract => todo!(),
                Operator::Multiply => todo!(),
                Operator::Divide => todo!(),
            }
        },
        Expression::Value(x) => {
            match *x {
                Integer(val) => Some(val),
                Value::Identifier(_) => return None,
                Value::Expression(_) => return None,
            }
        },
    }
}