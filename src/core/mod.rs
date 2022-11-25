use std::fmt::{Debug, Display};

/*

Unit    ::= {Tab}Def / GiveV / Expr
Tab     ::= ' '/'\t'
Def     ::=
DefFn   ::= Name '(' [ { NaT [ ',' ] } ] ')' : [Type
Deftype ::= Name ':' {Type}
GiveV   ::= (Name / NaT) '=' Expr
Expr    ::= (Expr Op Expr) /
            (Op Expr) /
            (Expr Op) /
            Number /
            Str /
Op      ::= / + / - / * / / / % / == / != / > / < / <= / >= / && / / / ! / << / >> / ( '(' [ { Name [','] } ] ')')
NaT     ::= Name ':' Type
Type    ::= TypeName
Name    ::= String
TypeNAme::= String

*/
/*
x = expr
x:t=expr
x:es
x():rett
x():
*/
mod lexer;

pub use lexer::lexer;
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
