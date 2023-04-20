use std::collections::HashMap;

use crate::{lex::*, stmt::*, Stmts};

#[derive(Debug, Clone)]
pub struct Error {
    pub err_index: usize,
    pub msg: String,
    pub after_token: bool,
}

impl Error {
    pub fn new(err_index: usize, msg: impl ToString) -> Self {
        Self {
            err_index,
            msg: msg.to_string(),
            after_token: false,
        }
    }
    pub fn after(mut self) -> Self {
        self.after_token = true;
        self
    }
}

pub type TResult<T> = Result<T, Error>;

#[derive(Debug, Clone)]
pub struct Parser {
    pub tokens: Vec<Token>,
    pub index: usize,
    pub begin: bool,
    /// 变量池 <变量名,(读,写)>
    pub var_pool: HashMap<String, (Vec<&'static Token>, Vec<&'static Token>)>,
    pub label_pool: HashMap<String, (Vec<&'static Token>, Vec<&'static Token>)>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            begin: true,
            index: 0,
            var_pool: HashMap::new(),
            label_pool: HashMap::new(),
        }
    }

    pub fn next_token(&self) -> TResult<&'static Token> {
        let p = self as *const Parser;

        let raw = self
            .tokens
            .get(self.index)
            .ok_or(Error::new(self.index, "end"))? as *const Token;

        unsafe {
            let mut_p = p as *mut Parser;
            (*mut_p).index += 1;
            Ok(&*raw)
        }
    }

    pub fn get_token(&self) -> TResult<&'static Token> {
        let raw = self
            .tokens
            .get(self.index - 1) // 因为next_token最后加一 idx-1 才是
            .ok_or(Error::new(self.index, "end"))? as *const Token;

        unsafe { Ok(&*raw) }
    }

    pub fn reset(&mut self, idx: usize) -> &mut Self {
        self.index = idx;
        self
    }

    pub fn edit_var(&mut self, k: &str) -> &mut (Vec<&'static Token>, Vec<&'static Token>) {
        if self.var_pool.get(k).is_some() {
            return self.var_pool.get_mut(k).unwrap();
        } else {
            self.var_pool.insert(k.to_owned(), (vec![], vec![]));
            return self.edit_var(k);
        }
    }

    pub fn edit_label(&mut self, k: &str) -> &mut (Vec<&'static Token>, Vec<&'static Token>) {
        if self.label_pool.get(k).is_some() {
            return self.label_pool.get_mut(k).unwrap();
        } else {
            self.label_pool.insert(k.to_owned(), (vec![], vec![]));
            return self.edit_label(k);
        }
    }
}

fn get_ident(parser: &mut Parser) -> TResult<String> {
    let ident = parser.next_token();
    let ident = ident
        .or(Err(Error::new(parser.index, "期待标识符").after()))?
        .vul
        .as_ident()
        .ok_or(Error::new(parser.index, "期待标识符"))?
        .clone();
    Ok(ident)
}

fn match_ident(parser: &mut Parser, rhs: &str) -> TResult<()> {
    let ident = get_ident(parser)?;
    match ident == rhs {
        true => Ok(()),
        false => Err(Error::new(
            parser.index,
            format!("期待字段 '{}' 而不是字段 '{}'", rhs, ident),
        )),
    }
}

fn get_symbol(parser: &mut Parser) -> TResult<Symbol> {
    let symbol = parser.next_token();
    let symbol = *symbol
        .or(Err(Error::new(parser.index, "期待符号").after()))?
        .vul
        .as_symbol()
        .ok_or(Error::new(parser.index, "期待符号"))?;
    Ok(symbol)
}

fn get_vul(parser: &mut Parser) -> TResult<f64> {
    let vul = parser.next_token();
    let vul = *vul
        .or(Err(Error::new(parser.index, "期待数字").after()))?
        .vul
        .as_vul()
        .ok_or(Error::new(parser.index, "期待数字"))?;
    Ok(vul)
}

fn match_symbols(parser: &mut Parser, rhs: Symbol) -> TResult<()> {
    let symbol = get_symbol(parser)?;
    if symbol == rhs {
        Ok(())
    } else {
        Err(Error::new(
            parser.index,
            format!("期待符号 '{}' 而不是符号 '{}'", rhs, symbol),
        ))
    }
}

fn get_expr(parser: &mut Parser) -> TResult<Expr> {
    let mut _expect_vul = true;
    let mut next_expect_negative = false;
    let mut last_is_unary = false;

    let mut temp_exprs: Vec<Expr> = vec![];
    let mut temp_ops: Vec<Symbol> = vec![];

    // 收集表达式
    loop {
        let idx = parser.index;
        // 如果下一个期待值
        // 可以是
        // 带上了符号的值
        // 带上一元运算符的字段/值
        // 一个括号
        if _expect_vul {
            // 是标识符?
            if let Ok(ident) = get_ident(parser) {
                // 如果前面有负数号
                if !next_expect_negative {
                    // 增加"读"
                    let ref_token = parser.get_token()?;
                    parser.edit_var(&ident).0.push(ref_token);
                    // 处理一元运算符
                    if last_is_unary {
                        last_is_unary = false;
                        temp_exprs.push(Expr::WithOp1(
                            temp_ops.pop().unwrap(),
                            Box::new(ident.into()),
                        ));
                    } else {
                        temp_exprs.push(ident.into())
                    }
                    _expect_vul = false
                } else {
                    return Err(match parser.tokens.get(idx + 1) {
                        Some(token) => {
                            Error::new(idx + 1, format!("期待 数字 而不是 '{}'", token.vul))
                        }
                        None => Error::new(idx, "期待 数字").after(),
                    });
                }
                continue;
            }
            // 是数字?
            else if let Ok(vul) = get_vul(parser.reset(idx)) {
                // 处理负号
                let temp: Expr = if next_expect_negative {
                    next_expect_negative = false;
                    (vul * -1.0).into()
                } else {
                    vul.into()
                };
                // 处理一元运算符
                if last_is_unary {
                    last_is_unary = false;
                    temp_exprs.push(Expr::WithOp1(temp_ops.pop().unwrap(), Box::new(temp)));
                } else {
                    temp_exprs.push(temp)
                }
                _expect_vul = false;
                continue;
            }
            // 是符号?
            else if let Ok(op) = get_symbol(parser.reset(idx)) {
                // 是负号?
                if op == Symbol::Min {
                    next_expect_negative = true;
                    continue;
                }
                // 是括号?
                else if op == Symbol::BracketL {
                    // 保证碰见右括号就直接返回
                    // 通过deep变量计算括号深度
                    // 如果深度不达标(括号不完整)
                    // 就报错
                    // 此时index指向Symbol::BracketL

                    // 获取括号内的表达式
                    // 此时index应该指向Symbol::BracketR
                    temp_exprs.push(get_expr(parser)?);
                    // 因为碰见反括号会回溯
                    // 提取括号结束后的下一个token应该是反括号
                    // 不然只好报错咯
                    match_symbols(parser, Symbol::BracketR).or(Err(
                        match parser.tokens.get(idx + 1) {
                            Some(token) => Error::new(
                                idx + 1,
                                format!("期待 '{}' 而不是 '{}'", Symbol::BracketR, token.vul),
                            ),
                            None => Error::new(idx, format!("期待 '{}'", Symbol::BracketR)).after(),
                        },
                    ))?;

                    // 因为刚刚的括号算是一个表达式
                    // 所以下一循环期待符号
                    _expect_vul = false;
                    continue;
                }
                // 是一元运算符?
                else if op.is_unary() {
                    // 提前处理一元运算符
                    temp_ops.push(op);
                    last_is_unary = true;
                    continue;
                }
            }
            // 都不是: 报错
            return Err(match parser.tokens.get(idx + 1) {
                Some(token) => {
                    Error::new(idx + 1, format!("期待 字段/数字 而不是 '{}'", token.vul))
                }
                None => Error::new(idx, "期待 字段/数字").after(),
            });
        }
        // 如果下一个不期待值
        // 可以是
        // 一个二元运算符
        // 反括号向前回溯并且立即返回
        // 其他情况,回溯并且返回
        else if !_expect_vul {
            if let Ok(op) = get_symbol(parser) {
                if op.is_binary() {
                    temp_ops.push(op);
                    _expect_vul = true;
                } else if op == Symbol::BracketR {
                    parser.reset(idx);
                    break; // 退出循环 准备收集exprs归一
                }
                // todo: 函数调用语法
                // tode: 下标运算符
                // 其他情况: 回溯 退出循环
                else {
                    _expect_vul = true;
                    parser.reset(idx);
                    break;
                }
            }
            // 回溯 退出循环
            else {
                parser.reset(idx);
                break;
            }
        }
    }
    // 组合exprs/ops
    // temp_exprs.reverse();
    // temp_ops.reverse();
    // 会发生改变的index
    // 已经保证所有符号都是二元运算符
    while temp_ops.len() != 0 {
        let mut deep = 1;
        let mut _opa = temp_ops[0];
        let mut _opb = temp_ops[0];
        while temp_ops.len() > deep {
            _opa = temp_ops[deep - 1];
            _opb = temp_ops[deep];
            if _opa.priority() >= _opb.priority() {
                break;
            } else {
                deep += 1;
            }
        }
        let expb = temp_exprs[deep].clone();
        let expa = temp_exprs[deep - 1].clone();
        temp_exprs.remove(deep);
        temp_ops.remove(deep - 1);
        temp_exprs[deep - 1] = Expr::WithOp2(_opb, Box::new(expa), Box::new(expb));
    }

    assert!(temp_exprs.len() == 1);
    return Ok(temp_exprs.pop().unwrap());
}

fn get_if(parser: &mut Parser) -> TResult<If> {
    match_ident(parser, "if")?;
    let mut cond_blocks = vec![(get_expr(parser)?, get_block(parser)?)];
    let mut else_block = None;
    let mut idx = parser.index;
    while let Ok(_) = match_ident(parser, "else") {
        // else if
        if let Ok(_) = match_ident(parser, "if") {
            cond_blocks.push((get_expr(parser)?, get_block(parser)?));
            idx = parser.index;
            continue;
        }
        // else
        else {
            parser.reset(idx + 1);
            else_block = Some(get_block(parser)?);
            idx = parser.index;
            break;
        }
    }
    parser.reset(idx);
    Ok(If {
        cond_blocks,
        else_block,
    })
}

fn get_set(parser: &mut Parser) -> TResult<Set> {
    match_ident(parser, "set")?;
    let l = get_ident(parser).or(Err(Error::new(parser.index, "期待标识符").after()))?;
    // 增加"写"
    let ref_token = parser.get_token()?;
    parser.edit_var(&l).1.push(ref_token);
    let op = get_symbol(parser).or(Err(Error::new(parser.index, "期待赋值运算符").after()))?;
    if !op.is_ass_op() {
        return Err(Error::new(
            parser.index,
            format!("期待赋值运算符 而不是 '{}'", op),
        ));
    }
    let r = get_expr(parser)?;
    Ok(Set { l, op, r })
}

fn get_goto(parser: &mut Parser) -> TResult<Goto> {
    match_ident(parser, "goto")?;
    let idx = parser.index;
    if let Ok(label) = get_ident(parser) {
        let token = parser.get_token()?;
        parser.edit_label(&label).0.push(token);
        return Ok(Goto { label });
    }
    parser.reset(idx);
    match_symbols(parser, Symbol::Nothing).or(Err(Error::new(
        parser.index,
        format!("期待 标识符/'{}'", Symbol::Nothing),
    )))?;
    let token = parser.get_token()?;
    parser.edit_label("_").0.push(token);
    Ok(Goto {
        label: String::from("_"),
    })
}

fn get_label(parser: &mut Parser) -> TResult<Label> {
    match_symbols(parser, Symbol::LabelL)?;
    let name = get_ident(parser)?;
    let idx = parser.index;
    match_symbols(parser, Symbol::LabelR).or(Err(Error::new(
        parser.index,
        format!("期待符号 '{}'", Symbol::LabelR,),
    )))?;
    let idx2 = parser.index;
    parser.reset(idx);
    let token = parser.get_token()?;
    parser.edit_label(&name).1.push(token);
    parser.reset(idx2);
    // println!("{} {}", idx, idx2);
    Ok(Label { name })
}

fn get_while(parser: &mut Parser) -> TResult<While> {
    match_ident(parser, "while")?;
    Ok(While {
        cond: get_expr(parser)?,
        block: get_block(parser)?,
    })
}

fn get_repeat_until(parser: &mut Parser) -> TResult<RepeatUntil> {
    match_ident(parser, "repeat")?;
    let block = get_block(parser)?;
    match_ident(parser, "until")?;
    Ok(RepeatUntil {
        cond: get_expr(parser)?,
        block,
    })
}

pub fn get_stmt(parser: &mut Parser) -> TResult<Box<dyn Statement>> {
    let mut err = Error::new(0, "");
    macro_rules! try_get {
        ($func:expr) => {{
            let idx = parser.index;
            if let Ok(stmt) = ($func)(parser).map_err(|e| err = e) {
                return Ok(Box::new(stmt));
            }
            if parser.index - idx > 1 {
                return Err(err);
            }
            parser.reset(idx);
        }};
    }

    try_get!(get_if);
    try_get!(get_set);
    try_get!(get_goto);
    try_get!(get_label);
    try_get!(get_while);
    try_get!(get_repeat_until);

    Err(err)
}

fn get_block(parser: &mut Parser) -> TResult<Stmts> {
    let idx = parser.index;
    if let Ok(stmt) = get_stmt(parser) {
        return Ok(vec![stmt]);
    }
    parser.reset(idx);

    match_symbols(parser, Symbol::BLockL).or(Err(Error::new(
        parser.index,
        format!("期待符号 '{}' 或一条语句", Symbol::BLockL),
    )))?;

    let mut stmts = vec![];

    let mut idx = parser.index;
    while let Err(_) = match_symbols(parser, Symbol::BLockR) {
        parser.reset(idx);
        stmts.push(get_stmt(parser)?);
        idx = parser.index;
    }
    Ok(stmts)
}
