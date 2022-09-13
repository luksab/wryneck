use colored::Colorize;
use id_collections::id_type;
use id_collections::IdVec;
use std::fmt::{Debug, Display, Error};

use crate::formatter::{Format, Formatter};

#[id_type]
pub struct FunctionId(usize);

#[derive(Debug)]
pub struct Program<'input> {
    pub things: Vec<TopLevel<'input>>,
    pub functions: IdVec<FunctionId, Function<'input>>,
}

impl Format for Program<'_> {
    fn format(&self, fmt: &mut Formatter) {
        for func in &self.things {
            match func {
                TopLevel::Function(func) => self.functions[*func].format(fmt),
                TopLevel::Comment(comment) => comment.format(fmt),
            }
        }
    }
}

// convert from base_ast::Program to Program
impl<'input> From<crate::base_ast::Program<'input>> for Program<'input> {
    fn from(ast: crate::base_ast::Program<'input>) -> Self {
        let mut functions = IdVec::new();
        Program {
            things: ast
                .things
                .into_iter()
                .map(|thing| match thing {
                    crate::base_ast::TopLevel::Function(func) => {
                        let id = functions.push(Function::from(func));
                        TopLevel::Function(id)
                    }
                    crate::base_ast::TopLevel::Comment(comment) => {
                        TopLevel::Comment(comment.into())
                    }
                })
                .collect::<Vec<_>>(),
            functions,
        }
    }
}

#[derive(Debug)]
pub enum TopLevel<'input> {
    Function(FunctionId),
    Comment(Comment<'input>),
}

#[derive(Debug)]
pub struct Comment<'input> {
    pub text: &'input str,
}

impl<'input> Comment<'input> {
    pub fn new(text: &'input str) -> Self {
        Self { text }
    }
}

impl Format for Comment<'_> {
    fn format(&self, fmt: &mut Formatter) {
        fmt.push_str_indented("// ");
        fmt.push_str(self.text);
        fmt.push_str("\n");
    }
}

impl<'input> From<crate::base_ast::Comment<'input>> for Comment<'input> {
    fn from(ast: crate::base_ast::Comment<'input>) -> Self {
        Self::new(ast.text)
    }
}

// function -------------------------------------------------------------------

#[derive(Debug)]
pub struct Function<'input> {
    pub definition: FunctionDefinition<'input>,
    pub body: Box<Expression<'input>>,
    pub tests: Vec<Test<'input>>,
}

impl<'input> Function<'input> {
    pub fn comment(content: &'input str) -> Self {
        Self {
            definition: FunctionDefinition {
                name: content,
                params: Vec::new(),
            },
            body: Box::new(Expression::Error),
            tests: Vec::new(),
        }
    }
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

impl<'input> From<crate::base_ast::Function<'input>> for Function<'input> {
    fn from(ast: crate::base_ast::Function<'input>) -> Self {
        Self {
            definition: ast.definition.into(),
            body: Box::new(ast.body.into()),
            tests: ast
                .tests
                .into_iter()
                .map(|test| test.into())
                .collect::<Vec<_>>(),
        }
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

impl<'input> From<crate::base_ast::FunctionDefinition<'input>> for FunctionDefinition<'input> {
    fn from(ast: crate::base_ast::FunctionDefinition<'input>) -> Self {
        Self {
            name: ast.name,
            params: ast
                .params
                .into_iter()
                .map(|param| param.into())
                .collect::<Vec<_>>(),
        }
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

impl<'input> From<crate::base_ast::Parameter<'input>> for Parameter<'input> {
    fn from(ast: crate::base_ast::Parameter<'input>) -> Self {
        Self { name: ast.name }
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

impl<'input> From<crate::base_ast::Test<'input>> for Test<'input> {
    fn from(ast: crate::base_ast::Test<'input>) -> Self {
        Self {
            input: Box::new(ast.input.into()),
            output: Box::new(ast.output.into()),
        }
    }
}

// statements -----------------------------------------------------------------

#[derive(Debug)]
pub enum Statement<'input> {
    Let(Let<'input>),
    Expression(Box<Expression<'input>>),
    Return(Box<Expression<'input>>),
    Comment(Comment<'input>),
    Error,
}

impl Format for Statement<'_> {
    fn format(&self, fmt: &mut Formatter) {
        match self {
            Statement::Let(let_) => let_.format(fmt),
            Statement::Expression(expr) => {
                fmt.push_str_indented("");
                expr.format(fmt);
                fmt.push_str(";\n");
            }
            Statement::Return(expr) => {
                fmt.push_str_indented("🐔 ");
                expr.format(fmt);
                fmt.push_str(";\n");
            }
            Statement::Comment(comment) => comment.format(fmt),
            Statement::Error => fmt.push_string_indented("error!\n".red().to_string()),
        }
    }
}

impl<'input> From<crate::base_ast::Statement<'input>> for Statement<'input> {
    fn from(ast: crate::base_ast::Statement<'input>) -> Self {
        match ast {
            crate::base_ast::Statement::Let(let_) => Self::Let(let_.into()),
            crate::base_ast::Statement::Expression(expr) => Self::Expression(Box::new(expr.into())),
            crate::base_ast::Statement::Return(expr) => Self::Return(Box::new(expr.into())),
            crate::base_ast::Statement::Comment(comment) => Self::Comment(comment.into()),
            crate::base_ast::Statement::Error => Self::Error,
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
        fmt.push_str_indented("let ");
        fmt.push_str(self.name);
        fmt.push_str(" = ");
        self.value.format(fmt);
        fmt.push_str(";\n");
    }
}

impl<'input> From<crate::base_ast::Let<'input>> for Let<'input> {
    fn from(ast: crate::base_ast::Let<'input>) -> Self {
        Self {
            name: ast.name,
            value: Box::new(ast.value.into()),
        }
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

impl<'input> From<crate::base_ast::FunctionCall<'input>> for FunctionCall<'input> {
    fn from(ast: crate::base_ast::FunctionCall<'input>) -> Self {
        Self {
            name: ast.name,
            args: ast
                .args
                .into_iter()
                .map(|arg| Box::new(arg.into()))
                .collect::<Vec<_>>(),
        }
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
    String(ASTString<'input>),
    If(If<'input>),
    Op(Box<Expression<'input>>, Opcode, Box<Expression<'input>>),
    ExpressionComment((Box<Expression<'input>>, Comment<'input>)),
    Error,
}

impl Format for Expression<'_> {
    fn format(&self, fmt: &mut Formatter) {
        match self {
            Expression::Expression(expr) => expr.format(fmt),
            Expression::FunctionCall(func) => func.format(fmt),
            Expression::Block(block) => {
                fmt.push_str("{\n");
                fmt.indent();
                for stmt in block {
                    stmt.format(fmt);
                }
                fmt.unindent();
                fmt.push_str_indented("}");
            }
            Expression::Variable(var) => fmt.push_string(var.to_string()),
            Expression::Number(num) => fmt.push_string(num.to_string()),
            Expression::String(str) => fmt.push_string(str.to_string()),
            Expression::Op(lhs, op, rhs) => {
                fmt.push_str("(");
                lhs.format(fmt);
                fmt.push_str(" ");
                fmt.push_string(op.to_string());
                fmt.push_str(" ");
                rhs.format(fmt);
                fmt.push_str(")");
            }
            Expression::If(if_) => if_.format(fmt),
            Expression::ExpressionComment((expr, comment)) => {
                expr.format(fmt);
                comment.format(fmt);
            }
            Expression::Error => fmt.push_string("error".red().to_string()),
        }
    }
}

impl<'input> From<crate::base_ast::Expression<'input>> for Expression<'input> {
    fn from(ast: crate::base_ast::Expression<'input>) -> Self {
        match ast {
            crate::base_ast::Expression::Expression(expr) => {
                Self::Expression(Box::new(expr.into()))
            }
            crate::base_ast::Expression::Block(block) => Self::Block(
                block
                    .into_iter()
                    .map(|stmt| stmt.into())
                    .collect::<Vec<_>>(),
            ),
            crate::base_ast::Expression::FunctionCall(func) => Self::FunctionCall(func.into()),
            crate::base_ast::Expression::Variable(var) => Self::Variable(var.into()),
            crate::base_ast::Expression::Number(num) => Self::Number(num),
            crate::base_ast::Expression::String(str) => Self::String(str.into()),
            crate::base_ast::Expression::If(if_) => Self::If(if_.into()),
            crate::base_ast::Expression::Op(lhs, op, rhs) => {
                Self::Op(Box::new(lhs.into()), op.into(), Box::new(rhs.into()))
            }
            crate::base_ast::Expression::ExpressionComment((expr, comment)) => {
                Self::ExpressionComment((Box::new(expr.into()), comment.into()))
            }
            crate::base_ast::Expression::Error => Self::Error,
        }
    }
}

impl<'input> From<Box<crate::base_ast::Expression<'input>>> for Expression<'input> {
    fn from(ast: Box<crate::base_ast::Expression<'input>>) -> Self {
        Self::from(*ast)
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

impl<'input> From<crate::base_ast::Variable<'input>> for Variable<'input> {
    fn from(ast: crate::base_ast::Variable<'input>) -> Self {
        Self { name: ast.name }
    }
}

#[derive(Debug)]
pub struct ASTString<'input> {
    pub value: &'input str,
}

impl<'input> Display for ASTString<'input> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), Error> {
        write!(fmt, "{}", self.value)
    }
}

impl<'input> From<crate::base_ast::ASTString<'input>> for ASTString<'input> {
    fn from(ast: crate::base_ast::ASTString<'input>) -> Self {
        Self { value: ast.value }
    }
}

#[derive(Debug)]
pub struct If<'input> {
    pub condition: Box<Expression<'input>>,
    pub body: Box<Expression<'input>>,
    pub else_body: Option<Box<Expression<'input>>>,
}

impl Format for If<'_> {
    fn format(&self, fmt: &mut Formatter) {
        fmt.push_str("if ");
        self.condition.format(fmt);
        fmt.push_str(" ");
        self.body.format(fmt);
        if let Some(else_) = &self.else_body {
            else_.format(fmt);
        }
    }
}

impl<'input> From<crate::base_ast::If<'input>> for If<'input> {
    fn from(ast: crate::base_ast::If<'input>) -> Self {
        Self {
            condition: Box::new(ast.condition.into()),
            body: Box::new(ast.body.into()),
            else_body: ast.else_body.map(|else_| Box::new(else_.into())),
        }
    }
}

// math -----------------------------------------------------------------------

pub enum ExprSymbol<'input> {
    NumSymbol(&'input str),
    Op(Box<ExprSymbol<'input>>, Opcode, Box<ExprSymbol<'input>>),
    Error,
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

impl<'input> Display for ExprSymbol<'input> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> Result<(), Error> {
        use self::ExprSymbol::*;
        match *self {
            NumSymbol(n) => write!(fmt, "{}", n),
            Op(ref l, op, ref r) => write!(fmt, "({} {} {})", l, op, r),
            Error => write!(fmt, "{}", "error".red()),
        }
    }
}

impl<'input> From<crate::base_ast::ExprSymbol<'input>> for ExprSymbol<'input> {
    fn from(ast: crate::base_ast::ExprSymbol<'input>) -> Self {
        match ast {
            crate::base_ast::ExprSymbol::NumSymbol(num) => Self::NumSymbol(num),
            crate::base_ast::ExprSymbol::Op(lhs, op, rhs) => {
                Self::Op(Box::new((*lhs).into()), op.into(), Box::new((*rhs).into()))
            }
            crate::base_ast::ExprSymbol::Error => Self::Error,
        }
    }
}

#[derive(Copy, Clone)]
pub enum Opcode {
    Mul,
    Div,
    Add,
    Sub,
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

impl<'input> From<crate::base_ast::Opcode> for Opcode {
    fn from(ast: crate::base_ast::Opcode) -> Self {
        match ast {
            crate::base_ast::Opcode::Mul => Self::Mul,
            crate::base_ast::Opcode::Div => Self::Div,
            crate::base_ast::Opcode::Add => Self::Add,
            crate::base_ast::Opcode::Sub => Self::Sub,
        }
    }
}
