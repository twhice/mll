use std::fmt::{Debug, Display};

/*

*/
mod code;
mod complier;
mod lexer;
mod parser;

pub use lexer::lexer;
pub use parser::parser;

use crate::error::ErrMeg;
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
        self.row = 1;
    }
    pub fn set_line(&mut self, line: usize) {
        self.line = line
    }
    pub fn set_filename(&mut self, filename: String) {
        self.filename = filename
    }
}

#[derive(Debug, Clone, Copy)]
enum TokenType {
    Name,
    Num,
    Symbol,
    Str,
    Space,
}
#[derive(Clone)]
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
impl Debug for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut all = String::from("{");
        all += &format!("{:?}", self.ttype);
        all += " \"";
        for c in &self.text {
            all += String::from(*c).as_str();
        }
        all += "\" ";
        all += &self.pos.to_string();
        all += "}";
        write!(f, "{all}")
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

pub fn run(src: String, filename: &str) {
    let mut tokens = Vec::new();
    let mut base_pos = Pos {
        filename: filename.to_owned(),
        line: 1,
        row: 1,
    };
    for line in src.lines() {
        let token_: Result<Vec<Token>, ErrMeg> = lexer::lexer(line, &mut base_pos);
        tokens.append(&mut token_.unwrap());
        base_pos.new_line();
    }
    if super::DEBUG {
        println!("Tokens: {:?}", tokens);
    }
    let mut com_units = Vec::new();
    while tokens.len() > 0 {
        com_units.push(parser(&mut tokens).unwrap())
    }
    if super::DEBUG {
        println!("ComUnits: {:?}", com_units);
    }
}
