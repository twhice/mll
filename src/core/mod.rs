use std::fmt::{Debug, Display};

/*

*/
mod code;
mod complier;
mod lexer;
mod parser;

pub use lexer::lexer;
pub use parser::parser;
#[derive(Clone)]
pub struct Pos {
    filename: String,
    line: usize,
    row: usize,
}
impl Pos {
    pub fn new() -> Self {
        Self {
            filename: "114514.tl".to_owned(),
            line: 1,
            row: 1,
        }
    }
    pub fn pass(&mut self) {
        self.row += 1;
    }
    pub fn new_line(&mut self) {
        self.line += 1;
    }
    pub fn set_line(&mut self, line: usize) {
        self.line = line
    }
    pub fn set_filename(&mut self, filename: String) {
        self.filename = filename
    }
}

#[derive(Debug, Clone, Copy)]
pub enum TokenType {
    Name,
    Num,
    Symbol,
    Str,
    Space,
}
#[derive(Debug, Clone)]
pub struct Token {
    text: Vec<char>,
    pub pos: Pos,
    ttype: TokenType,
}
impl Token {
    pub fn match_text(&self, anstr: &str) -> bool {
        let anvec: Vec<char> = anstr.chars().collect();
        self.text.eq(&anvec)
    }
    pub fn get_text(&self) -> &Vec<char> {
        &self.text
    }
    pub fn get_type(&self) -> TokenType {
        self.ttype
    }
}
impl Display for Pos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}:{}", self.filename, self.line, self.row)
    }
}
impl Debug for Pos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}
