use super::Pos;
use super::Token;
use super::TokenType;

use crate::error::{Err, ErrMeg};

// 只支持处理一行!
pub fn lexer(src: &str, base_pos: &mut Pos) -> Result<Vec<Token>, ErrMeg> {
    let mut src: Vec<char> = src.chars().collect();
    // let mut index = 0;
    // let mut currten: char = ' ';
    let pos = base_pos;
    let p_src: *mut Vec<char> = &mut src as *mut Vec<char>;
    let p_pos: *mut Pos = pos as *mut Pos;

    let mut ret = Vec::new();
    let mut line_begin = true;
    let collect_by_rule =
        |currten: char, tokentype: TokenType, rule: &dyn Fn(char) -> bool| -> Token {
            let _src = unsafe { &mut *p_src };
            let _pos = unsafe { &mut *p_pos };

            let b_pos = _pos.clone();
            let mut temp = vec![currten];
            while matches!(_src.first(), Some(..)) && rule(_src[0]) {
                temp.push(_src[0]);
                _src.remove(0);
                _pos.pass();
            }

            return Token {
                text: temp,
                pos: b_pos,
                ttype: tokentype,
            };
        };
    while matches!(src.first(), Some(..)) {
        let currten = src[0];

        src.remove(0);
        if line_begin && currten.is_whitespace() {
            while matches!(src.first(), Some(..)) && src[0].is_ascii_whitespace() {
                src.remove(0);
                pos.pass();
            }
        } else {
            line_begin = false
        }
        if currten == '/' {
            if matches!(src.first(), Some('/')) {
                src.clear();
            } else {
                return Err(ErrMeg::new(pos.to_owned(), Err::CommentforError));
            }
        } else if currten == '#' {
            return Err(ErrMeg::new(pos.to_owned(), Err::CommentforError));
        // Name
        } else if currten.is_alphabetic() || currten == '_' {
            ret.push(collect_by_rule(
                currten,
                TokenType::Name,
                &|c: char| -> bool { c.is_ascii_alphanumeric() || c == '_' },
            ))
        // Num
        } else if currten.is_ascii_digit() {
            ret.push(collect_by_rule(
                currten,
                TokenType::Num,
                &|c: char| -> bool { c.is_ascii_digit() },
            ))
        }
        // Str
        else if currten == '"' {
            let mut temp = Vec::new();
            let mut pre = false;
            let begin_pos = pos.clone();
            while matches!(src.first(), Some(..)) {
                pos.pass();
                let currten = src[0];
                src.remove(0);
                if pre {
                    match currten {
                        'n' => temp.push('\n'),
                        't' => temp.push('\t'),
                        '"' => temp.push('"'),
                        '/' => temp.push('/'),
                        'r' => {}
                        _ => return Err(ErrMeg::new(pos.to_owned(), Err::UnknownEscapeCharacter)),
                    };
                } else if currten == '\\' {
                    pre = true
                } else if currten == '"' {
                    break;
                } else {
                    temp.push(currten)
                }
            }
            ret.push(Token {
                text: temp,
                pos: begin_pos,
                ttype: TokenType::Str,
            })
        } else if currten.is_ascii_punctuation() {
            let c_next = src.first();
            let temp = match currten {
                '+' => {
                    if matches!(c_next, Some('=')) {
                        "+="
                    } else {
                        "+"
                    }
                }
                '-' => {
                    if matches!(c_next, Some('=')) {
                        "-="
                    } else {
                        "-"
                    }
                }
                '*' => {
                    if matches!(c_next, Some('=')) {
                        "*="
                    } else if matches!(c_next, Some('*')) {
                        "**" // 2 ** 3 = 8
                    } else {
                        "*"
                    }
                }
                '/' => {
                    if matches!(c_next, Some('=')) {
                        "/="
                    } else {
                        "/"
                    }
                }
                '%' => {
                    if matches!(c_next, Some('=')) {
                        "%="
                    } else {
                        "%"
                    }
                }
                '=' => {
                    if matches!(c_next, Some('=')) {
                        "=="
                    } else {
                        "="
                    }
                }
                '!' => {
                    if matches!(c_next, Some('=')) {
                        "!="
                    } else {
                        "!"
                    }
                }
                '&' => {
                    if matches!(c_next, Some('&')) {
                        "&&"
                    } else {
                        return Err(ErrMeg::new(pos.to_owned(), Err::UnknowSymbol));
                    }
                }
                '|' => {
                    if matches!(c_next, Some('|')) {
                        "||"
                    } else {
                        return Err(ErrMeg::new(pos.to_owned(), Err::UnknowSymbol));
                    }
                }
                '(' => "(",
                ')' => ")",
                '{' => "{",
                '}' => "}",
                '.' => ".",
                _ => return Err(ErrMeg::new(pos.to_owned(), Err::UnknowSymbol)),
            };
            if temp.len() == 2 {
                src.remove(0);
            }
            ret.push(Token {
                text: temp.chars().collect(),
                pos: pos.clone(),
                ttype: TokenType::Symbol,
            })
        }
        pos.pass();
    }
    return Ok(ret);
}
