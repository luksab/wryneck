use crate::ast::*;

pub struct Formatter {
    pub indent_level: usize,
    pub string: String,
}

impl Formatter {
    pub fn new() -> Self {
        Self {
            indent_level: 0,
            string: String::new(),
        }
    }

    pub fn indent(&mut self) {
        self.indent_level += 1;
    }

    pub fn unindent(&mut self) {
        self.indent_level -= 1;
    }

    pub fn push_indent(&mut self) {
        self.string.push_str(&" ".repeat(self.indent_level * 4));
    }

    pub fn push_str_indented(&mut self, s: &str) {
        self.push_indent();
        self.string.push_str(s);
    }

    pub fn push_string_indented(&mut self, s: String) {
        self.push_indent();
        self.string.push_str(&s);
    }

    pub fn push_str(&mut self, s: &str) {
        self.string.push_str(s);
    }
    pub fn push_string(&mut self, s: String) {
        self.string.push_str(&s);
    }
}

pub trait Format {
    fn format(&self, fmt: &mut Formatter);
}

pub fn format(input: &Program) -> String {
    let mut formatter = Formatter {
        indent_level: 0,
        string: String::new(),
    };
    input.format(&mut formatter);
    formatter.string
}
