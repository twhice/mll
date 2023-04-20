use std::{fmt::Debug, fs, io, process::exit};

use crate::{lex::*, parse::*, Stmts, Tokens};

#[derive(Debug)]
pub struct Complier {
    path: String,
    src: String,
    parser: Parser,
    statements: Stmts,
}

impl Complier {
    pub fn new(path: impl ToString) -> io::Result<Self> {
        let path = path.to_string();
        // 因为一点小bug 必须这样做了
        let src = fs::read_to_string(&path)? + " ";
        Ok(Self {
            path,
            src,
            parser: Parser::new(vec![]),
            statements: vec![],
        })
    }

    pub fn run_lex(&mut self) -> &mut Self {
        self.parser.tokens = Lexer::new(&self.src).collect();
        self
    }

    pub fn debug_tokens(&mut self) -> &mut Self {
        for token in &self.parser.tokens {
            println!("{:?}", token)
        }

        self
    }

    pub fn check_tokens(&mut self) -> &mut Self {
        for token in &self.parser.tokens {
            if let Some(err) = token.vul.as_unknow() {
                let err = Error::new(0, format!("未知的字符 '{}'", err));
                self.resolve(err, token.clone(), true);
                return self;
            }
        }

        self
    }

    pub fn get_stmts(&self) -> &Stmts {
        &self.statements
    }

    pub fn get_tokens(&self) -> &Tokens {
        &self.parser.tokens
    }

    pub fn run_parser(&mut self) -> &mut Self {
        let len = self.parser.tokens.len();
        while self.parser.index != len {
            match get_stmt(&mut self.parser) {
                Ok(stmt) => self.statements.push(stmt),
                Err(err) => {
                    let err_token = match self.parser.tokens.get(err.err_index) {
                        Some(token) => token.clone(),
                        None => self.parser.tokens[err.err_index - 1].clone(),
                    };
                    self.resolve(err, err_token, true);
                }
            }
        }
        self
    }

    pub fn check_var_pool(&mut self) -> &mut Self {
        for (name, (read, write)) in &self.parser.var_pool {
            if write.len() == 0 {
                for err_token in read {
                    self.resolve(
                        Error::new(0, format!("变量 '{}' 没有定义", name)),
                        (*err_token).clone(),
                        false,
                    )
                }
            }
            if read.len() == 0 {
                for err_token in write {
                    self.resolve(
                        Error::new(0, format!("变量 '{}' 从未使用", name)),
                        (*err_token).clone(),
                        false,
                    )
                }
            }
        }
        self
    }

    pub fn check_label_pool(&mut self) -> &mut Self {
        for (name, (read, write)) in &self.parser.label_pool {
            // 忽略这个特殊label
            if name == "_" {
            } else if write.len() == 0 {
                for err_token in read {
                    self.resolve(
                        Error::new(0, format!("标签 '{}' 没有定义", name)),
                        (*err_token).clone(),
                        true,
                    )
                }
            } else if write.len() > 1 {
                for err_token in &write[1..] {
                    // println!("{:?}", err_token);
                    self.resolve(
                        Error::new(0, format!("标签 '{}' 重复定义", name)),
                        (*err_token).clone(),
                        true,
                    )
                }
            } else if read.len() == 0 {
                for err_token in write {
                    self.resolve(
                        Error::new(0, format!("标签 '{}' 从未使用", name)),
                        (*err_token).clone(),
                        false,
                    )
                }
            }
        }
        self
    }

    /// 屎山代码
    /// 谨慎修改
    fn resolve(&self, err: Error, err_token: Token, is_error: bool) {
        // 出错的行
        let line_number = format!("{} |", err_token.pos[0]);
        let lines = self.src.lines().collect::<Vec<&str>>();
        let error_line_full = lines[err_token.pos[0] - 1];

        // 闭包函数 字符串在展示上的大小(多少个空格)
        let str_len = |str: &str| {
            let mut len = 0;
            for ch in str.chars() {
                if ch.is_ascii() {
                    len += 1
                } else {
                    len += 2;
                }
            }
            return len;
        };

        // 出现错误之前的部分
        let chars = error_line_full.chars().collect::<Vec<char>>();
        let mut error_line_begin_len = 0;
        print!("[0..{} - 1]", err_token.pos[1]);

        let error_line_begin_chars = if err_token.pos[1] != 0 {
            &chars[0..err_token.pos[1] - 1]
            // &chars[0..err_token.pos[1]]
        } else {
            &[]
        };
        // println!("{:?}", error_line_begin_chars);
        error_line_begin_chars
            .into_iter()
            .for_each(|ch| error_line_begin_len += if ch.is_ascii() { 1 } else { 2 });

        // 计算 出现错误之前的部分的展示长度
        // 和   出现错误的部分的展示长度
        let mut length_of_space = str_len(&line_number) + error_line_begin_len;
        let mut length_of_error = str_len(&err_token.vul.to_string());

        let mut real_error_pos = err_token.pos;

        // 如果错误的位置是在当前token之后 空格长度 和 ^长度
        if err.after_token {
            length_of_space += length_of_error;
            length_of_error = 1;
            real_error_pos[1] += err_token.vul.to_string().len();
        }
        //  根据错误模式打印信息
        let info = if is_error {
            format!(
                "Error: 在 {}:{}:{} : ",
                self.path, real_error_pos[0], real_error_pos[1]
            )
        } else {
            format!(
                "Warn: 在 {}:{}:{} : ",
                self.path, real_error_pos[0], real_error_pos[1]
            )
        };

        let mut msg = String::new();
        for _ in 0..length_of_space {
            msg.push(' ')
        }
        for _ in 0..length_of_error {
            msg.push('^')
        }

        eprintln!(
            "{}\n{}{}\n{} {}",
            info, line_number, error_line_full, msg, err.msg
        );

        if is_error {
            exit(-1);
        }
    }
}

pub trait Statement: Debug {
    fn compile(self) -> Vec<String>;
}

#[derive(Debug, Clone)]
pub enum Expr {
    Vul(String),
    WithOp1(Symbol, Box<Expr>),
    WithOp2(Symbol, Box<Expr>, Box<Expr>),
}

impl<T: ToString> From<T> for Expr {
    fn from(s: T) -> Self {
        Self::Vul(s.to_string())
    }
}

// 语句
#[derive(Debug, Clone)]
pub struct Set {
    pub l: String,
    pub op: Symbol,
    pub r: Expr,
}

impl Statement for Set {
    fn compile(self) -> Vec<String> {
        todo!()
    }
}

#[derive(Debug)]
pub struct If {
    pub cond_blocks: Vec<(Expr, Stmts)>,
    pub else_block: Option<Stmts>,
}

impl Statement for If {
    fn compile(self) -> Vec<String> {
        todo!()
    }
}

#[derive(Debug)]
pub struct While {
    pub cond: Expr,
    pub block: Stmts,
}

impl Statement for While {
    fn compile(self) -> Vec<String> {
        todo!()
    }
}

#[derive(Debug)]
pub struct RepeatUntil {
    pub cond: Expr,
    pub block: Stmts,
}

impl Statement for RepeatUntil {
    fn compile(self) -> Vec<String> {
        todo!()
    }
}

/// 用来跳转的label
///
/// 有一特殊label _ 表示第0行
#[derive(Debug)]
pub struct Label {
    pub name: String,
}

impl Statement for Label {
    fn compile(self) -> Vec<String> {
        todo!()
    }
}

#[derive(Debug)]
pub struct Goto {
    pub label: String,
}

impl Statement for Goto {
    fn compile(self) -> Vec<String> {
        todo!()
    }
}

// 声明

#[derive(Debug)]
pub struct Func {
    /// 函数名
    pub name: String,
    // 参数数目
    pub nr_arg: usize,
    // 函数体
    pub block: Stmts,
    // todo:返回值数目
    // pub nr_return: usize,
}

impl Statement for Func {
    fn compile(self) -> Vec<String> {
        todo!()
    }
}
