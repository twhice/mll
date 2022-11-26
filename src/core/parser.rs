/*
ComUnit = Ctrl  / Set
Ctrl    = "if" Expr (ComUnit / Block) [ 'else ' (ComUnit / Block) ]
        = "for" Name Num "," Num[ "," Num] (ComUnit / Block)
        = "switch" {case Num (ComUnit / Block)}
        = "return" Expr
        = "pg" FnName "{" Block "}"
Set     = "set" Name "=" Expr { "," Name "=" Expr}
Expr    = Expr (+ - * / % > < >= <= == != && || ** in ) Expr
        = ( ! ) Expr
        = Num
        = Name
Name    = ..
Num     = ..
*/

use crate::error::{Err, ErrMeg};

use super::code::*;
use super::{Pos, Token, TokenType};

pub fn parser(tokens: &mut Vec<Token>) -> Result<ComUnit, ErrMeg> {
    match com_unit(tokens) {
        Ok(cu) => Ok(cu),
        Err(_) => todo!(),
    }
}
fn com_unit(tokens: &mut Vec<Token>) -> Result<ComUnit, ErrMeg> {
    let _tokens = &mut tokens.clone();
    match ctrl(tokens) {
        Ok(ret) => Ok(ret),
        Err(_) => set(_tokens),
    }
}
fn ctrl(tokens: &mut Vec<Token>) -> Result<ComUnit, ErrMeg> {
    let _tokens = &mut tokens.clone();
    if let Ok(ret) = ctrl_if(_tokens) {
        return Ok(ret);
    }
    let _tokens = &mut tokens.clone();
    if let Ok(ret) = ctrl_pg(_tokens) {
        return Ok(ret);
    }
    let _tokens = &mut tokens.clone();
    if let Ok(ret) = ctrl_for(_tokens) {
        return Ok(ret);
    }
    let _tokens = &mut tokens.clone();
    if let Ok(ret) = ctrl_switch(_tokens) {
        return Ok(ret);
    }
    ctrl_return(tokens)
}
fn set(tokens: &mut Vec<Token>) -> Result<ComUnit, ErrMeg> {
    if tokens_match_bool(tokens, "set") {
        tokens.remove(0);
    } else {
        tokens_match_err(tokens, "let")?;
    }
    let mut sets = Vec::new();
    loop {
        let name = tokens_get_name(tokens)?;
        tokens_match_err(tokens, "=")?;
        let vul = tokens_get_expr(tokens)?;
        sets.push(Set::new(name, vul));
        if !tokens_match_bool(tokens, ",") {
            break;
        }
    }
    return Ok(ComUnit::Set(sets));
}
fn expr(tokens: &mut Vec<Token>) -> Result<ComUnit, ErrMeg> {
    todo!()
}
fn ctrl_if(tokens: &mut Vec<Token>) -> Result<ComUnit, ErrMeg> {
    tokens_match_err(tokens, "if")?;
    let condition = tokens_get_expr(tokens)?;
    tokens_match_err(tokens, "{")?;
    let if_statements = get_cus(tokens)?;
    let mut else_statements = Vec::new();
    if tokens_match_bool(tokens, "else") {
        tokens.remove(0);
        tokens_match_err(tokens, "{")?;
        else_statements = get_cus(tokens)?;
    }
    return Ok(ComUnit::Ctrl(Ctrl::Ctrl_if(CtrlIf::new(
        condition,
        if_statements,
        else_statements,
    ))));
}
fn ctrl_for(tokens: &mut Vec<Token>) -> Result<ComUnit, ErrMeg> {
    todo!()
}
fn ctrl_switch(tokens: &mut Vec<Token>) -> Result<ComUnit, ErrMeg> {
    todo!()
}
fn ctrl_return(tokens: &mut Vec<Token>) -> Result<ComUnit, ErrMeg> {
    tokens_match_err(tokens, "return")?;
    let expr = tokens_get_expr(tokens)?;
    return Ok(ComUnit::Ctrl(Ctrl::Ctrl_return(CtrlReturn::new(expr))));
}
fn ctrl_pg(tokens: &mut Vec<Token>) -> Result<ComUnit, ErrMeg> {
    tokens_match_err(tokens, "pg")?;
    let fn_name = tokens_tryget_name(tokens);
    tokens_match_err(tokens, "{")?;
    let statements = get_cus(tokens)?;
    return Ok(ComUnit::Ctrl(Ctrl::Ctrl_pg(CtrlPg::new(
        fn_name, statements,
    ))));
}
fn tokens_get_expr(tokens: &mut Vec<Token>) -> Result<Expr, ErrMeg> {
    todo!()
}
fn tokens_match_err<'a>(tokens: &'a mut Vec<Token>, text: &'a str) -> Result<(), ErrMeg> {
    let token = get_tokens_first(tokens)?;
    if token.compare_text(text) {
        tokens.remove(0);
        Ok(())
    } else {
        return Err(ErrMeg::new(token.pos.clone(), crate::error::Err::Unmatched));
    }
}
fn tokens_match_bool<'a>(tokens: &'a mut Vec<Token>, text: &'a str) -> bool {
    let token = match get_tokens_first(tokens) {
        Ok(token) => token,
        Err(_) => return false,
    };
    token.compare_text(text)
}
fn tokens_tryget_name(tokens: &mut Vec<Token>) -> Option<Vec<char>> {
    let p_tokens = tokens as *mut Vec<Token>;
    let token = match get_tokens_first(tokens) {
        Ok(token) => {
            if matches!(token.get_type(), TokenType::Name) {
                token
            } else {
                return None;
            }
        }
        Err(_) => return None,
    };
    unsafe {
        (*p_tokens).remove(0);
    }
    Some(token.get_text().clone())
}
fn tokens_get_name(tokens: &mut Vec<Token>) -> Result<Vec<char>, ErrMeg> {
    let p_tokens = tokens as *mut Vec<Token>;
    let token = match get_tokens_first(tokens) {
        Ok(token) => {
            if matches!(token.get_type(), TokenType::Name) {
                token
            } else {
                return Err(ErrMeg::new(token.pos.clone(), Err::Unmatched));
            }
        }
        Err(_) => return Err(ErrMeg::new(Pos::new(), Err::Empty)),
    };
    unsafe {
        (*p_tokens).remove(0);
    }
    Ok(token.get_text().clone())
}
fn get_tokens_first(tokens: &mut Vec<Token>) -> Result<&Token, ErrMeg> {
    if let Some(token) = tokens.first() {
        return Ok(token);
    } else {
        return Err(ErrMeg::new(Pos::new(), crate::error::Err::Empty));
    }
}
fn get_space_size(tokens: &mut Vec<Token>) -> Result<usize, ErrMeg> {
    let p_tokens = tokens as *mut Vec<Token>;
    let token = get_tokens_first(tokens)?;
    if matches!(token.get_type(), TokenType::Space) {
        unsafe {
            (*p_tokens).remove(0);
        }
        return Ok(token.get_text().len());
    } else {
        Ok(0)
    }
}
fn get_cus(tokens: &mut Vec<Token>) -> Result<Vec<ComUnit>, ErrMeg> {
    let mut ret = Vec::new();
    while !tokens.is_empty() && !tokens_match_bool(tokens, "}") {
        ret.push(com_unit(tokens)?);
    }
    return Ok(ret);
}
