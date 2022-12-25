use std::fmt::{Debug, Display};

/*

*/
mod abi;
mod code;
mod complier;
mod lexer;
mod linker;
mod parser;

pub use lexer::lexer;
pub use parser::parser;

use crate::error::ErrMeg;

use self::complier::{jump_always, Link};
#[derive(Clone)]
pub struct Pos {
    filename: String,
    line: usize,
    row: usize,
}
impl Pos {
    pub fn new() -> Self {
        Self {
            filename: "未知位置".to_owned(),
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
pub enum TokenType {
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
impl PartialEq<Pos> for Pos {
    fn eq(&self, other: &Pos) -> bool {
        self.filename == other.filename && self.line == other.line && self.row == other.row
    }
}

pub fn complie(src: String, filename: &str) -> Result<Vec<String>, ErrMeg> {
    let debug_meg = unsafe { super::DEBUG };
    let mut tokens = Vec::new();
    let mut base_pos = Pos {
        filename: filename.to_owned(),
        line: 1,
        row: 1,
    };
    let lines = src.lines().collect::<Vec<&str>>();
    for line in src.lines() {
        let mut token_ = lexer::lexer(line, &mut base_pos)?;
        tokens.append(&mut token_);
        base_pos.new_line();
    }
    if debug_meg {
        println!("Tokens: {:?}", tokens);
    }
    let mut com_units = Vec::new();
    while tokens.len() > 0 {
        com_units.push(match parser(&mut tokens) {
            Ok(ok) => ok,
            Err(err) => {
                let mut err_pos = Pos::new();
                if err.pos == Pos::new() {
                    err_pos.line = lines.len();
                    err_pos.row = lines[lines.len() - 1].len();
                    err_pos.filename = filename.to_owned();
                } else {
                    err_pos = err.pos;
                }
                return Err(ErrMeg::new(err_pos, err.err));
            }
        })
    }
    if debug_meg {
        println!("ComUnits: {:?}", com_units);
    }

    // 编译链接
    let mut codes = Vec::new();
    let mut indexs_isdef: Vec<bool> = Vec::new();
    let mut codes_withdef = false;
    codes.push(jump_always());

    // 先定义所有函数
    for i in 0..com_units.len() {
        if com_units[i].is_def() {
            codes.link(&mut com_units[i].complie());
            indexs_isdef.push(false);
            codes_withdef = true;
            continue;
        }
        indexs_isdef.push(true)
    }
    // 如果没有def就去掉跳过函数定义的行
    if codes_withdef {
        let codes_len = codes.len();
        codes[0].reset_target(codes_len);
    } else {
        codes.remove(0);
    }
    for i in 0..com_units.len() {
        if indexs_isdef[i] {
            codes.link(&mut com_units[i].complie());
        }
    }

    let mut mdt_codes: Vec<String> = Vec::new();
    for code in codes {
        mdt_codes.push(code.to_string());
    }
    mdt_codes.push("end".to_owned());
    if mdt_codes.len() > 999 {
        crate::error::CTErr::ProcessTooLong.solve()
    }

    Ok(mdt_codes)
}
