use colored::*;
use std::fmt::{Debug, Display, Error, Formatter};

#[derive(Debug)]
pub struct Program<'input> {
    pub functions: Vec<Function<'input>>,
}

impl Display for Program<'_> {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result<(), Error> {
        for func in &self.functions {
            writeln!(fmt, "{}", func)?;
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct Function<'input> {
    pub definition: FunctionDefinition<'input>,
    pub body: Box<Expression<'input>>,
    pub tests: Vec<Test<'input>>,
}

impl Display for Function<'_> {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result<(), Error> {
        write!(fmt, "{}", self.definition)?;
        writeln!(fmt, "{}", self.body)?;
        if !self.tests.is_empty() {
            writeln!(fmt, "[")?;
            for test in &self.tests {
                writeln!(fmt, "   {},", test)?;
            }
            writeln!(fmt, "]")?;
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct Test<'input> {
    pub input: Box<Expression<'input>>,
    pub output: Box<Expression<'input>>,
}

impl Display for Test<'_> {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result<(), Error> {
        write!(fmt, "{} = {}", self.input, self.output)?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct FunctionDefinition<'input> {
    pub name: &'input str,
    pub params: Vec<Parameter<'input>>,
}

impl Display for FunctionDefinition<'_> {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result<(), Error> {
        if self.name == "hatch" {
            write!(fmt, "🥚 🐣(")?;
        } else {
            write!(fmt, "🥚 {}(", self.name)?;
        }
        write!(
            fmt,
            "{}",
            self.params
                .iter()
                .map(|param| format!("{}", param))
                .collect::<Vec<_>>()
                .join(", ")
        )?;
        write!(fmt, ") ")?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct Parameter<'input> {
    pub name: &'input str,
}

impl Display for Parameter<'_> {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result<(), Error> {
        write!(fmt, "{}", self.name)
    }
}

#[derive(Debug)]
pub enum Expression<'input> {
    Expression(Box<Expression<'input>>),
    Block(Vec<Statement<'input>>),
    FunctionCall(FunctionCall<'input>),
    Variable(Variable<'input>),
    Number(i32),
    Op(Box<Expression<'input>>, Opcode, Box<Expression<'input>>),
    Error,
}

impl Display for Expression<'_> {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result<(), Error> {
        match self {
            Expression::Expression(expr) => write!(fmt, "{}", expr),
            Expression::FunctionCall(func) => write!(fmt, "{}", func),
            Expression::Block(block) => {
                write!(fmt, "{{\n")?;
                for stmt in block {
                    writeln!(fmt, "   {};", stmt)?;
                }
                write!(fmt, "}}")
            }
            Expression::Variable(var) => write!(fmt, "{}", var),
            Expression::Number(num) => write!(fmt, "{}", num),
            Expression::Op(lhs, op, rhs) => write!(fmt, "({} {} {})", lhs, op, rhs),
            Expression::Error => write!(fmt, "error"),
        }
    }
}

#[derive(Debug)]
pub enum Statement<'input> {
    Let(Let<'input>),
    Expression(Box<Expression<'input>>),
    Return(Box<Expression<'input>>),
    Error,
}

impl Display for Statement<'_> {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result<(), Error> {
        match self {
            Statement::Let(let_) => write!(fmt, "{}", let_),
            Statement::Expression(expr) => write!(fmt, "{}", expr),
            Statement::Return(expr) => write!(fmt, "🐔 {}", expr),
            Statement::Error => write!(fmt, "{}", "error".red()),
        }
    }
}

#[derive(Debug)]
pub struct Let<'input> {
    pub name: &'input str,
    pub value: Box<Expression<'input>>,
}

impl Display for Let<'_> {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result<(), Error> {
        write!(fmt, "let {} = {}", self.name, self.value)
    }
}

#[derive(Debug)]
pub struct FunctionCall<'input> {
    pub name: &'input str,
    pub args: Vec<Box<Expression<'input>>>,
}

impl Display for FunctionCall<'_> {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result<(), Error> {
        write!(fmt, "{}(", self.name)?;
        write!(
            fmt,
            "{}",
            self.args
                .iter()
                .map(|arg| format!("{}", arg))
                .collect::<Vec<_>>()
                .join(", ")
        )?;
        write!(fmt, ")")
    }
}

#[derive(Debug)]
pub struct Variable<'input> {
    pub name: &'input str,
}

impl Display for Variable<'_> {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result<(), Error> {
        write!(fmt, "{}", self.name)
    }
}

pub enum ExprSymbol<'input> {
    NumSymbol(&'input str),
    Op(Box<ExprSymbol<'input>>, Opcode, Box<ExprSymbol<'input>>),
    Error,
}

#[derive(Copy, Clone)]
pub enum Opcode {
    Mul,
    Div,
    Add,
    Sub,
}

impl<'input> Debug for ExprSymbol<'input> {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::ExprSymbol::*;
        match *self {
            NumSymbol(n) => write!(fmt, "{:?}", n),
            Op(ref l, op, ref r) => write!(fmt, "({:?} {:?} {:?})", l, op, r),
            Error => write!(fmt, "{}", "error".red()),
        }
    }
}

impl Debug for Opcode {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::Opcode::*;
        match *self {
            Mul => write!(fmt, "*"),
            Div => write!(fmt, "/"),
            Add => write!(fmt, "+"),
            Sub => write!(fmt, "-"),
        }
    }
}

impl Display for Opcode {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::Opcode::*;
        match *self {
            Mul => write!(fmt, "*"),
            Div => write!(fmt, "/"),
            Add => write!(fmt, "+"),
            Sub => write!(fmt, "-"),
        }
    }
}
