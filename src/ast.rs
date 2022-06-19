use colored::*;
use std::fmt::{Debug, Error, Formatter};

#[derive(Debug)]
pub struct Function<'input> {
    pub definition: FunctionDefinition<'input>,
    pub body: Box<Expression<'input>>,
    pub tests: Vec<Test<'input>>,
}

#[derive(Debug)]
pub struct Test<'input> {
    pub input: Box<Expression<'input>>,
    pub output: Box<Expression<'input>>,
}

#[derive(Debug)]
pub struct FunctionDefinition<'input> {
    pub name: &'input str,
    pub params: Vec<Parameter<'input>>,
}

#[derive(Debug)]
pub struct Parameter<'input> {
    pub name: &'input str,
}

#[derive(Debug)]
pub enum Expression<'input> {
    Expression(Box<Expression<'input>>),
    Block(Vec<Statement<'input>>),
    Number(i32),
    Op(Box<Expression<'input>>, Opcode, Box<Expression<'input>>),
    Error,
}

#[derive(Debug)]
pub enum Statement<'input> {
    Let(Let<'input>),
    Return(Box<Expression<'input>>),
}

#[derive(Debug)]
pub struct Let<'input> {
    pub name: &'input str,
    pub value: Box<Expression<'input>>,
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

// impl Debug for MathExpr {
//     fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
//         use self::MathExpr::*;
//         match *self {
//             Number(n) => write!(fmt, "{:?}", n),
//             Op(ref l, op, ref r) => write!(fmt, "({:?} {:?} {:?})", l, op, r),
//             Error => write!(fmt, "{}", "error".red()),
//         }
//     }
// }

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
