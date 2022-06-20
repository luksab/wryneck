use colored::Colorize;
use std::fmt::{Debug, Display, Error};

use crate::formatter::{Format, Formatter};

#[derive(Debug)]
pub struct Program<'input> {
    pub functions: Vec<Function<'input>>,
}

impl Format for Program<'_> {
    fn format(&self, fmt: &mut Formatter) {
        for func in &self.functions {
            func.format(fmt);
        }
    }
}

// function -------------------------------------------------------------------

#[derive(Debug)]
pub struct Function<'input> {
    pub definition: FunctionDefinition<'input>,
    pub body: Box<Expression<'input>>,
    pub tests: Vec<Test<'input>>,
}

impl Format for Function<'_> {
    fn format(&self, fmt: &mut Formatter) {
        self.definition.format(fmt);
        self.body.format(fmt);
        if !self.tests.is_empty() {
            fmt.push_str_indented("[\n");
            fmt.indent();
            for test in &self.tests {
                fmt.push_str_indented("");
                test.format(fmt);
                fmt.push_str(",\n");
            }
            fmt.unindent();
            fmt.push_str_indented("]");
        }
        fmt.push_str("\n\n");
    }
}

#[derive(Debug)]
pub struct FunctionDefinition<'input> {
    pub name: &'input str,
    pub params: Vec<Parameter<'input>>,
}

impl Format for FunctionDefinition<'_> {
    fn format(&self, fmt: &mut Formatter) {
        if self.name == "hatch" {
            fmt.push_str_indented("🥚 🐣(");
        } else {
            fmt.push_str_indented("🥚 ");
            fmt.push_str(self.name);
            fmt.push_str("(");
        }
        fmt.push_string(
            self.params
                .iter()
                .map(|param| format!("{}", param))
                .collect::<Vec<_>>()
                .join(", "),
        );

        fmt.push_str(") ");
    }
}

#[derive(Debug)]
pub struct Parameter<'input> {
    pub name: &'input str,
}

impl Display for Parameter<'_> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), Error> {
        write!(fmt, "{}", self.name)
    }
}

#[derive(Debug)]
pub struct Test<'input> {
    pub input: Box<Expression<'input>>,
    pub output: Box<Expression<'input>>,
}

impl Format for Test<'_> {
    fn format(&self, fmt: &mut Formatter) {
        self.input.format(fmt);
        fmt.push_str(" = ");
        self.output.format(fmt);
    }
}

// statements -----------------------------------------------------------------

#[derive(Debug)]
pub enum Statement<'input> {
    Let(Let<'input>),
    Expression(Box<Expression<'input>>),
    Return(Box<Expression<'input>>),
    Error,
}

impl Format for Statement<'_> {
    fn format(&self, fmt: &mut Formatter) {
        match self {
            Statement::Let(let_) => let_.format(fmt),
            Statement::Expression(expr) => {
                fmt.push_str("\n");
                expr.format(fmt);
                fmt.push_str(";\n");
            }
            Statement::Return(expr) => {
                fmt.push_str("\n");
                fmt.push_str_indented("🐔 ");
                expr.format(fmt);
                fmt.push_str(";");
            }
            Statement::Error => fmt.push_string_indented("error".red().to_string()),
        }
    }
}

#[derive(Debug)]
pub struct Let<'input> {
    pub name: &'input str,
    pub value: Box<Expression<'input>>,
}

impl Format for Let<'_> {
    fn format(&self, fmt: &mut Formatter) {
        fmt.push_str("\n");
        fmt.push_str_indented("let ");
        fmt.push_str(self.name);
        fmt.push_str(" = ");
        self.value.format(fmt);
        fmt.push_str(";");
    }
}

#[derive(Debug)]
pub struct FunctionCall<'input> {
    pub name: &'input str,
    pub args: Vec<Box<Expression<'input>>>,
}

impl Format for FunctionCall<'_> {
    fn format(&self, fmt: &mut Formatter) {
        fmt.push_str(&format!("{}(", self.name));
        for (i, arg) in self.args.iter().enumerate() {
            if i > 0 {
                fmt.push_str(", ");
            }
            arg.format(fmt);
        }
        fmt.push_str(")");
    }
}

// expressions ----------------------------------------------------------------

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

impl Format for Expression<'_> {
    fn format(&self, fmt: &mut Formatter) {
        match self {
            Expression::Expression(expr) => expr.format(fmt),
            Expression::FunctionCall(func) => func.format(fmt),
            Expression::Block(block) => {
                fmt.push_str("{");
                fmt.indent();
                for stmt in block {
                    stmt.format(fmt);
                }
                fmt.unindent();
                fmt.push_str("\n");
                fmt.push_str_indented("}");
            }
            Expression::Variable(var) => fmt.push_string(var.to_string()),
            Expression::Number(num) => fmt.push_string(num.to_string()),
            Expression::Op(lhs, op, rhs) => {
                fmt.push_str("(");
                lhs.format(fmt);
                fmt.push_str(" ");
                fmt.push_string(op.to_string());
                fmt.push_str(" ");
                rhs.format(fmt);
                fmt.push_str(")");
            }
            Expression::Error => fmt.push_string("error".red().to_string()),
        }
    }
}

#[derive(Debug)]
pub struct Variable<'input> {
    pub name: &'input str,
}

impl Display for Variable<'_> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), Error> {
        write!(fmt, "{}", self.name)
    }
}

// math -----------------------------------------------------------------------

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
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> Result<(), Error> {
        use self::ExprSymbol::*;
        match *self {
            NumSymbol(n) => write!(fmt, "{:?}", n),
            Op(ref l, op, ref r) => write!(fmt, "({:?} {:?} {:?})", l, op, r),
            Error => write!(fmt, "{}", "error".red()),
        }
    }
}

impl Debug for Opcode {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> Result<(), Error> {
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
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> Result<(), Error> {
        use self::Opcode::*;
        match *self {
            Mul => write!(fmt, "*"),
            Div => write!(fmt, "/"),
            Add => write!(fmt, "+"),
            Sub => write!(fmt, "-"),
        }
    }
}
