use std::{fmt::Display, str::FromStr};

pub struct Lexer {
    src: Vec<char>,
    index: usize,
    pos: [usize; 2],
}

impl Lexer {
    pub fn new(src: &str) -> Self {
        Self {
            src: src.chars().collect(),
            index: 0,
            pos: [1, 1],
        }
    }

    pub fn next_char(&mut self) -> Option<char> {
        self.index += 1;
        let char = *self.src.get(self.index)?;
        if char == '\n' {
            self.pos[0] += 1;
            self.pos[1] = 0;
        } else {
            self.pos[1] += 1;
        }
        Some(char)
    }
}

impl Iterator for Lexer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let mut this_char = *self.src.get(self.index)?;

        // 跳过空格 直到this_char 不是空格
        while this_char.is_whitespace() {
            this_char = self.next_char()?;
        }

        let pos = self.pos;

        // 注释
        if this_char == '#' {
            while self.next_char()? != '\n' {} // 直到index索引指向\n
            return self.next();
        }

        let mut temp = String::from(this_char);

        // 标识符
        if this_char.is_alphabetic() {
            while let Some(ch) = self.next_char() {
                if ch.is_alphanumeric() || ch == '_' {
                    temp.push(ch)
                } else {
                    break;
                }
            }
            return Some(Token::new(pos, TokenVul::Ident(temp)));
        }
        // 数字
        if this_char.is_ascii_digit() {
            let mut doted = false;
            while let Some(ch) = self.next_char() {
                if ch.is_ascii_digit() {
                    temp.push(ch)
                } else if !doted && ch == '.' {
                    temp.push(ch);
                    doted = false;
                } else {
                    break;
                }
            }
            return Some(Token::new(pos, TokenVul::Vul(temp.parse().ok()?)));
        }
        // 符号
        if temp.parse::<Symbol>().is_ok() {
            while let Some(ch) = self.next_char() {
                temp.push(ch);
                match temp.parse::<Symbol>() {
                    Err(_) => break, // break后 next_char指向正确
                    _ => {}
                }
            }
            temp.pop(); // 弹出导致错误的char
            return Some(Token::new(pos, TokenVul::Symbol(temp.parse().ok()?))); // ? unwarp均可
        }
        // 其他
        self.next_char();
        Some(Token::new(pos, TokenVul::Unknow(this_char)))
    }
}

#[derive(Debug, Clone)]
pub struct Token {
    pub pos: [usize; 2],
    pub vul: TokenVul,
}

impl Token {
    pub fn new(pos: [usize; 2], vul: TokenVul) -> Self {
        Self { pos, vul }
    }
}

#[derive(Debug, Clone)]
pub enum TokenVul {
    Ident(String),
    Vul(f64),
    Symbol(Symbol),
    Unknow(char),
}

impl TokenVul {
    pub fn as_ident(&self) -> Option<&String> {
        if let Self::Ident(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_vul(&self) -> Option<&f64> {
        if let Self::Vul(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_symbol(&self) -> Option<&Symbol> {
        if let Self::Symbol(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_unknow(&self) -> Option<&char> {
        if let Self::Unknow(v) = self {
            Some(v)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Symbol {
    Add,
    Min,
    Mul,
    Div,
    Mod,
    Pow,
    // Comment,
    Ass,
    BracketL,
    BracketR,
    BLockL,
    BLockR,
    LabelL,
    LabelR,
    Nothing,
}

impl FromStr for Symbol {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Self::Add),
            "-" => Ok(Self::Min),
            "*" => Ok(Self::Mul),
            "/" => Ok(Self::Div),
            "%" => Ok(Self::Mod),
            "^" => Ok(Self::Pow),
            // "#" => Ok(Self::Comment),
            "=" => Ok(Self::Ass),
            "(" => Ok(Self::BracketL),
            ")" => Ok(Self::BracketR),
            "{" => Ok(Self::BLockL),
            "}" => Ok(Self::BLockR),
            "<" => Ok(Self::LabelL),
            ">" => Ok(Self::LabelR),
            "_" => Ok(Self::Nothing),
            _ => Err(()),
        }
    }
}

impl Symbol {
    pub fn is_ass_op(&self) -> bool {
        self == &Self::Ass
    }

    pub fn is_binary(&self) -> bool {
        match self {
            Symbol::Add | Symbol::Min | Symbol::Mul | Symbol::Div | Symbol::Mod | Symbol::Pow => {
                true
            }
            _ => false,
        }
    }

    pub fn is_unary(&self) -> bool {
        match self {
            // todo
            _ => false,
        }
    }

    pub fn priority(&self) -> usize {
        match self {
            Symbol::Add => 5,
            Symbol::Min => 5,
            Symbol::Mul => 6,
            Symbol::Div => 6,
            Symbol::Mod => 6,
            Symbol::Pow => 7,
            Symbol::Ass => 1,
            _ => 0,
        }
    }
}

impl Display for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Symbol::Add => "+",
            Symbol::Min => "-",
            Symbol::Mul => "*",
            Symbol::Div => "/",
            Symbol::Mod => "%",
            Symbol::Pow => "^",
            Symbol::Ass => "=",
            Symbol::BracketL => "(",
            Symbol::BracketR => ")",
            Symbol::BLockL => "{",
            Symbol::BLockR => "}",
            Symbol::LabelL => "<",
            Symbol::LabelR => ">",
            Symbol::Nothing => "_",
        };
        write!(f, "{}", str)
    }
}

impl Display for TokenVul {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenVul::Ident(id) => write!(f, "{}", id),
            TokenVul::Vul(fl) => write!(f, "{}", fl),
            TokenVul::Symbol(op) => write!(f, "{}", op),
            TokenVul::Unknow(ch) => write!(f, "{}", ch),
        }
    }
}
