/*
ComUnit = Ctrl  / Set
Ctrl    = "if" Expr (ComUnit / Block) [ 'else ' (ComUnit / Block) ]
        = "for" Name Num "," Num[ "," Num] (ComUnit / Block)
        = "switch" {case Num (ComUnit / Block)}
        = "return" Expr
        = "pg" FnName "{" Block "}"
Set     = "set" Name "=" Expr { "," Name "=" Expr}
Expr    = Expr (+ - * / % > < >= <= == != && || ** in ) Expr
        = ( ! ) Expr44                                                                                                                                          `````````````````````````````````````````````````
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
    match set(_tokens) {
        Ok(ret) => Ok(ret),
        Err(_err) => {
            println!("{_err}");
            ctrl(tokens)
        }
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
        let vul = tokens_build_expr(tokens)?;
        sets.push(Set::new(name, vul));
        if !tokens_match_bool(tokens, ",") {
            break;
        }
    }
    return Ok(ComUnit::Set(sets));
}
fn ctrl_if(tokens: &mut Vec<Token>) -> Result<ComUnit, ErrMeg> {
    tokens_match_err(tokens, "if")?;
    let condition = tokens_build_expr(tokens)?;
    tokens_match_err(tokens, "{")?;
    let if_statements = tokens_get_eus(tokens)?;
    let mut else_statements = Vec::new();
    if tokens_match_bool(tokens, "else") {
        tokens.remove(0);
        tokens_match_err(tokens, "{")?;
        else_statements = tokens_get_eus(tokens)?;
    }
    return Ok(ComUnit::Ctrl(Ctrl::Ctrl_if(CtrlIf::new(
        condition,
        if_statements,
        else_statements,
    ))));
}
fn ctrl_for(tokens: &mut Vec<Token>) -> Result<ComUnit, ErrMeg> {
    tokens_match_err(tokens, "for");
    let name = tokens_get_name(tokens)?;
    tokens_match_err(tokens, ",")?;
    let condition = tokens_build_expr(tokens)?;
    tokens_match_err(tokens, ",")?;
    let work = tokens_build_expr(tokens)?;
    todo!()
}
fn ctrl_switch(tokens: &mut Vec<Token>) -> Result<ComUnit, ErrMeg> {
    todo!()
}
fn ctrl_return(tokens: &mut Vec<Token>) -> Result<ComUnit, ErrMeg> {
    tokens_match_err(tokens, "return")?;
    let expr = tokens_build_expr(tokens)?;
    return Ok(ComUnit::Ctrl(Ctrl::Ctrl_return(CtrlReturn::new(expr))));
}
fn ctrl_pg(tokens: &mut Vec<Token>) -> Result<ComUnit, ErrMeg> {
    tokens_match_err(tokens, "pg")?;
    let fn_name = tokens_tryget_name(tokens);
    tokens_match_err(tokens, "{")?;
    let statements = tokens_get_eus(tokens)?;
    return Ok(ComUnit::Ctrl(Ctrl::Ctrl_pg(CtrlPg::new(
        fn_name, statements,
    ))));
}
fn tokens_match_err<'a>(tokens: &'a mut Vec<Token>, text: &'a str) -> Result<(), ErrMeg> {
    let token = tokens_get_first(tokens)?;
    if token.match_text(text) {
        tokens.remove(0);
        Ok(())
    } else {
        return Err(ErrMeg::new(token.pos.clone(), crate::error::Err::Unmatched));
    }
}
fn tokens_match_bool<'a>(tokens: &'a mut Vec<Token>, text: &'a str) -> bool {
    let token = match tokens_get_first(tokens) {
        Ok(token) => token,
        Err(_) => return false,
    };
    token.match_text(text)
}
fn tokens_tryget_name(tokens: &mut Vec<Token>) -> Option<Vec<char>> {
    let p_tokens = tokens as *mut Vec<Token>;
    let token = match tokens_get_first(tokens) {
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
    let token = match tokens_get_first(tokens) {
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
fn tokens_get_first(tokens: &mut Vec<Token>) -> Result<&Token, ErrMeg> {
    if let Some(token) = tokens.first() {
        return Ok(token);
    } else {
        return Err(ErrMeg::new(Pos::new(), crate::error::Err::Empty));
    }
}
fn get_space_size(tokens: &mut Vec<Token>) -> Result<usize, ErrMeg> {
    let p_tokens = tokens as *mut Vec<Token>;
    let token = tokens_get_first(tokens)?;
    if matches!(token.get_type(), TokenType::Space) {
        unsafe {
            (*p_tokens).remove(0);
        }
        return Ok(token.get_text().len());
    } else {
        Ok(0)
    }
}
fn tokens_get_eus(tokens: &mut Vec<Token>) -> Result<Vec<ComUnit>, ErrMeg> {
    let mut ret = Vec::new();
    while !tokens.is_empty() && !tokens_match_bool(tokens, "}") {
        ret.push(com_unit(tokens)?);
    }
    return Ok(ret);
}
fn tokens_build_expr(tokens: &mut Vec<Token>) -> Result<Expr, ErrMeg> {
    // 构建表达式表
    let mut exprs: Vec<Expr> = Vec::new();

    // 从tokens中构建exprs
    let mut may_fun = false;
    let mut fake_exprs = tokens_get_expr(tokens)?;
    let mut token_cache: Token = Token {
        text: Vec::new(),
        pos: Pos::new(),
        ttype: TokenType::Space,
    };
    while fake_exprs.len() > 0 {
        let fake_expr = &fake_exprs[0].clone();
        if !may_fun && matches!(fake_expr.get_type(), TokenType::Name) {
            token_cache = fake_expr.clone();
            may_fun = true;
        } else if may_fun && fake_expr.match_text("(") {
            let fn_name = token_cache.get_text().clone();
            let fn_args = tokens_get_fnargs(tokens)?;
            exprs.push(Expr::CallFn(fn_name, fn_args));
        }
        fake_exprs.remove(0);
        exprs.push(fake_expr.into());
    }

    // 调用上古屎山
    Ok(unsafe { build_exprs(&exprs) })
}
fn tokens_get_expr(tokens: &mut Vec<Token>) -> Result<Vec<Token>, ErrMeg> {
    let mut get_data = true;
    let mut deep: usize = 0;
    let mut ret = Vec::new();
    let p_tokens = tokens as *mut Vec<Token>;
    let mut token = &tokens[0];
    'l: loop {
        if tokens.len() <= 0 {
            break 'l;
        }
        token = unsafe { &(*p_tokens)[0] };
        if get_data {
            match token.get_type() {
                TokenType::Name | TokenType::Num => {
                    ret.push(token.clone());
                    tokens.remove(0);
                    get_data = false;
                }
                TokenType::Symbol => {
                    // 括号
                    if token.match_text("(") {
                        ret.push(token.clone());
                        tokens.remove(0);
                        deep += 1;
                    } else if token.match_text("!") {
                        ret.push(token.clone());
                        tokens.remove(0);
                    } else {
                        return Err(ErrMeg::new(token.pos.clone(), Err::NotVul));
                    }
                }
                TokenType::Str | TokenType::Space => {
                    return Err(ErrMeg::new(token.pos.clone(), Err::NotVul))
                }
            }
        } else {
            match token.get_type() {
                TokenType::Symbol => {
                    if token.match_text("(") {
                        deep += 1;
                        continue 'l;
                    }
                    for op in [
                        "+", "-", "*", "/", ">", "<", ">=", "<=", "==", "!=", "!", "**", "&&", "||",
                    ] {
                        if token.match_text(op) {
                            ret.push(token.clone());
                            tokens.remove(0);
                            get_data = true;
                            continue 'l;
                        }
                    }
                    if token.match_text(")") {
                        while tokens.len() > 0 {
                            let token = unsafe { &(*p_tokens)[0] };
                            tokens.remove(0);
                            ret.push(token.clone());
                            if token.match_text(")") {
                                break;
                            }
                        }
                    }
                    continue;
                }
                TokenType::Str | TokenType::Space | TokenType::Name | TokenType::Num => {
                    break;
                }
            }
        }
    }
    if get_data {
        return Err(ErrMeg::new(token.pos.clone(), Err::MissVul));
    }
    if deep != 0 {
        return Err(ErrMeg::new(token.pos.clone(), Err::MissBra));
    }

    return Ok(ret);
}
fn tokens_get_fnargs(tokens: &mut Vec<Token>) -> Result<Vec<Vec<char>>, ErrMeg> {
    // let fn_name = tokens_get_name(tokens)?;
    tokens_match_err(tokens, "(")?;
    let mut args = Vec::new();
    let arg0 = tokens_tryget_name(tokens);
    if let Some(arg0) = arg0 {
        args.push(arg0);
        while tokens_match_bool(tokens, ",") {
            tokens.remove(0);
            args.push(tokens_get_name(tokens)?)
        }
    }
    return Ok(args);
}
fn expr_priotity(expr: &Expr) -> usize {
    match expr {
        Expr::Op(op_text) => {
            let fuck = [
                vec!["(", ")"],
                vec!["!", "**"],
                vec!["*", "/"],
                vec!["+", "-"],
                vec!["<<", ">>"],
                vec![">", "<", ">=", "<="],
                vec!["!=", "=="],
                vec!["&", "&&", "|", "||", "^"],
            ];
            for priotity in 0..fuck.len() {
                let sub_fuck = &fuck[priotity];
                for op in sub_fuck {
                    if *op_text == op.chars().collect::<Vec<char>>() {
                        return priotity;
                    }
                }
            }
            todo!()
        }
        _ => 0,
    }
}
unsafe fn build_exprs(exprs: &Vec<Expr>) -> Expr {
    let mut priority_s = Vec::new();
    for expr in exprs {
        priority_s.push(expr_priotity(&expr));
    }
    // 解决该死的所有权
    let mut exprs = exprs.clone();
    let prio_ptr = &mut priority_s as *mut Vec<usize>;
    let expr_ptr = &mut exprs as *mut Vec<Expr>;
    // 闭包:提取表达式
    let build_op2 = |atsp: usize| {
        (*expr_ptr)[atsp - 1] = Expr::Eoe(
            Box::new((*expr_ptr)[atsp - 1].clone()),
            Box::new((*expr_ptr)[atsp].clone()),
            Box::new((*expr_ptr)[atsp + 1].clone()),
        );
        (*expr_ptr).remove(atsp);
        (*prio_ptr).remove(atsp);
        (*expr_ptr).remove(atsp);
        (*prio_ptr).remove(atsp);
    };
    let build_op1 = |atsp: usize| {
        (*expr_ptr)[atsp - 1] = Expr::Oe(
            Box::new((*expr_ptr)[atsp - 1].clone()),
            Box::new((*expr_ptr)[atsp].clone()),
        );
        (*expr_ptr).remove(atsp);
        (*prio_ptr).remove(atsp);
    };
    let mut bra_begin = 0;
    // let mut bra_end = 0;
    // 从低到高优先级遍历
    for priority_index in 0..8 {
        let mut index = 0;
        let mut bra_deep = 0;

        while index < (*priority_s).len() {
            let priority = priority_s[index];

            if priority == priority_index {
                if exprs[index].is_left_part() {
                    if bra_deep == 0 {
                        // DANGERIOUS
                        let more = exprs[index + 1..].to_vec();
                        if more[0].is_right_part() {
                            exprs[index] = Expr::Data(Vec::new())
                        } else {
                            exprs[index] = build_exprs(&more);
                        }
                        priority_s[index] = 0;
                        bra_begin = index;
                    }
                    bra_deep += 1;
                } else if exprs[0].is_right_part() {
                    if bra_deep == 0 {
                        return exprs[0].clone();
                    }
                    //括号刚结束
                    else if bra_deep == 1 {
                        // 大回退
                        while index != bra_begin {
                            index -= 1;
                            (*expr_ptr).remove(bra_begin + 1);
                            (*prio_ptr).remove(bra_begin + 1);
                        }
                    }
                    bra_deep -= 1;
                } else if exprs[0].is_not() && bra_deep == 0 {
                    build_op1(index);
                    index -= 1;
                } else if bra_deep == 0 {
                    build_op2(index);
                    index -= 1;
                }
            }
            index += 1;
        }
    }
    return exprs[0].clone();
}
// 中文 a
// aaaaa
