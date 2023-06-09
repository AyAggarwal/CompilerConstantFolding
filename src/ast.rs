// The Abstract Syntax Tree (AST) for Leo

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Program {
    // function main(a: u8) {
    //     let b = a + 1u8;
    // }
    pub name: String,
    pub inputs: Vec<Input>,
    pub statements: Vec<Statement>,
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Input {
    pub name: String,
    pub input_type: Type,
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Type {
    U8,
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Statement {
    // let a = 1u8;
    Assign {
        variable: String,
        expression: Expression,
    },
    If {
        expression: Expression,
        statements_a: Vec<Statement>,
        statements_b: Vec<Statement>,
    },
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Expression {
    // 1u8 + 2u8
    Binary {
        left: Value,
        operator: Operator,
        right: Box<Expression>,
    },
    // 1u8
    Value(Box<Value>),
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Value {
    // 1u8
    Integer(u8),
    // a
    Identifier(String),
    // true
    Boolean(bool),
    // (1u8 + a)
    Expression(Box<Expression>),
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
    GreaterThan,
    LessThan,
    Equal,
}

//trait to get the type of a variable for code generation
//Value::integer(Type) enum in AST could be used instead
trait TypeInfo {
    fn type_of(&self) -> &'static str;
}

impl TypeInfo for u8 {
    fn type_of(&self) -> &'static str {
        "u8"
    }
}

impl std::fmt::Display for Program {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let inputs = self
            .inputs
            .iter()
            .map(|input| format!("{:?}: {:?}", input.name, input.input_type))
            .collect::<Vec<String>>()
            .join(", ");

        let statements = self
            .statements
            .iter()
            .map(|statement| format!("    {}", statement))
            .collect::<Vec<String>>()
            .join("\n");

        write!(
            f,
            "function {}({}) {{\n{}\n}}",
            self.name, inputs, statements
        )
    }
}

impl std::fmt::Display for Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Statement::Assign {
                variable,
                expression,
            } => {
                write!(f, "let {} = {};", variable, expression)
            }
            Statement::If {
                expression,
                statements_a,
                statements_b,
            } => {
                let branch_a = statements_a
                    .iter()
                    .map(|statement| format!("    {}", statement))
                    .collect::<Vec<String>>()
                    .join("\n");

                let branch_b = statements_b
                    .iter()
                    .map(|statement| format!("    {}", statement))
                    .collect::<Vec<String>>()
                    .join("\n");

                write!(
                    f,
                    "if {} {{\n{}\n}} else {{\n{}\n}}\n",
                    expression, branch_a, branch_b
                )
            }
        }
    }
}

impl std::fmt::Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Expression::Binary {
                left,
                operator,
                right,
            } => {
                write!(f, "{} {} {}", left, operator, right)
            }
            Expression::Value(value) => {
                write!(f, "{}", value)
            }
        }
    }
}

impl std::fmt::Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Operator::Add => {
                write!(f, "+")
            }
            Operator::Subtract => {
                write!(f, "-")
            }
            Operator::Multiply => {
                write!(f, "*")
            }
            Operator::Divide => {
                write!(f, "/")
            }
            Operator::GreaterThan => {
                write!(f, ">")
            }
            Operator::LessThan => {
                write!(f, "<")
            }
            Operator::Equal => {
                write!(f, "==")
            }
        }
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Value::Integer(integer) => {
                write!(f, "{}{}", integer, integer.type_of())
            }
            Value::Identifier(identifier) => {
                write!(f, "{}", identifier)
            }
            Value::Expression(expression) => {
                write!(f, "({})", expression)
            }
            Value::Boolean(boolean) => {
                write!(f, "{}", boolean)
            }
        }
    }
}
