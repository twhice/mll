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
    let keytoken = &tokens[0];
    if keytoken.match_text("set") {
        set(tokens)
    } else if keytoken.match_text("if") {
        ctrl_if(tokens)
    } else if keytoken.match_text("switch") {
        ctrl_switch(tokens)
    } else if keytoken.match_text("def") {
        ctrl_def(tokens)
    } else if keytoken.match_text("return") {
        ctrl_return(tokens)
    } else if keytoken.match_text("while") {
        ctrl_while(tokens)
    } else {
        todo!()
    }
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

    let if_statements = tokens_get_block(tokens)?;
    let mut else_statements = Vec::new();
    let mut elifs = Vec::new();
    while tokens_match_bool(tokens, "elif") {
        let condition = tokens_build_expr(tokens)?;
        let statements = tokens_get_block(tokens)?;
        elifs.push((condition, statements));
    }
    if tokens_match_bool(tokens, "else") {
        tokens.remove(0);
        else_statements = tokens_get_block(tokens)?;
    }
    return Ok(ComUnit::Ctrl(Ctrl::CtrlIf(CtrlIf::new(
        condition,
        if_statements,
        elifs,
        else_statements,
    ))));
}
fn ctrl_while(tokens: &mut Vec<Token>) -> Result<ComUnit, ErrMeg> {
    // tokens_match_err(tokens, "while")?;
    // skip sth
    tokens.remove(0);

    let condition = tokens_build_expr(tokens)?;

    let statements = tokens_get_block(tokens)?;
    return Ok(ComUnit::Ctrl(Ctrl::CtrlWhile(CtrlWhile::new(
        condition, statements,
    ))));
}
fn ctrl_switch(tokens: &mut Vec<Token>) -> Result<ComUnit, ErrMeg> {
    // skip sth
    tokens.remove(0);
    // "condition"
    let condition = tokens_build_expr(tokens)?;
    tokens_match_err(tokens, "{")?;
    // cases
    let mut cases = Vec::new();
    while tokens_match_bool(tokens, "{") {
        cases.push(tokens_get_block(tokens)?);
    }
    return Ok(ComUnit::Ctrl(Ctrl::CtrlSwitch(CtrlSwitch::new(
        condition, cases,
    ))));
}
fn ctrl_return(tokens: &mut Vec<Token>) -> Result<ComUnit, ErrMeg> {
    tokens_match_err(tokens, "return")?;
    let expr = tokens_build_expr(tokens)?;
    return Ok(ComUnit::Ctrl(Ctrl::CtrlReturn(CtrlReturn::new(expr))));
}
fn ctrl_def(tokens: &mut Vec<Token>) -> Result<ComUnit, ErrMeg> {
    // tokens_match_err(tokens, "pg")?;
    tokens.remove(0);
    let fn_name = tokens_get_name(tokens)?;
    tokens_match_err(tokens, "(")?;
    let mut args = vec![];
    let arg0 = tokens_tryget_name(tokens);
    if let Some(arg0) = arg0 {
        args.push(arg0.clone())
    }
    while tokens_match_bool(tokens, ",") {
        tokens.remove(0);
        args.push(tokens_get_name(tokens)?);
    }
    tokens_match_err(tokens, ")")?;
    let statements = tokens_get_block(tokens)?;
    return Ok(ComUnit::Ctrl(Ctrl::CtrlDef(CtrlDef::new(
        fn_name, args, statements,
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
                token.clone()
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
fn tokens_get_block(tokens: &mut Vec<Token>) -> Result<Vec<ComUnit>, ErrMeg> {
    tokens_match_err(tokens, "{")?;
    let mut ret = Vec::new();
    while !tokens.is_empty() {
        if !tokens_match_bool(tokens, "}") {
            ret.push(com_unit(tokens)?);
        } else {
            tokens.remove(0);
            break;
        }
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
    Ok(unsafe { build_exprs(&mut exprs) })
}
fn tokens_get_expr(tokens: &mut Vec<Token>) -> Result<Vec<Token>, ErrMeg> {
    let mut get_data = true;
    let mut deep: usize = 0;
    let mut ret = Vec::new();
    let p_tokens = tokens as *mut Vec<Token>;
    let mut token = tokens[0].clone();
    'l: loop {
        // println!(
        //     "\nBegin loop:\n\tTokens: {:?}\n\tExprs: {:?}\n\tDeep: {}\n",
        //     tokens, ret, deep
        // );
        if tokens.len() <= 0 {
            break 'l;
        }
        token = unsafe { (*p_tokens)[0].clone() };
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
                    if token.match_text("(") {
                        while tokens.len() > 0 {
                            let token = unsafe { &(*p_tokens)[0] };
                            tokens.remove(0);
                            ret.push(token.clone());
                            if token.match_text(")") {
                                break;
                            }
                        }
                    }
                    if token.match_text(")") {
                        deep -= 1;
                        tokens.remove(0);
                        ret.push(token.clone())
                    }
                    continue;
                }
                TokenType::Str | TokenType::Space | TokenType::Name | TokenType::Num => {
                    break;
                }
            }
        }
        // println!(
        //     "\nEnd loop:\n\tTokens: {:?}\n\tExprs: {:?}\n\tDeep: {}\n",
        //     tokens, ret, deep
        // );
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
                vec![" "],
                vec!["(", ""],
                vec!["!", "**"],
                vec!["*", "/"],
                vec!["+", "-"],
                vec!["<<", ">>"],
                vec![">", "<", ">=", "<="],
                vec!["!=", "=="],
                vec!["&", "&&", "|", "||", "^"],
                vec![")"],
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
unsafe fn build_exprs(exprs: &mut Vec<Expr>) -> Expr {
    let mut ret = Vec::new();
    let mut ops = Vec::new();
    // let mut exprs = exprs.clone();
    while exprs.len() > 0 {
        let expr = exprs[0].clone();
        exprs.remove(0);
        match expr {
            Expr::Op(_) => {
                if expr.is_not() {
                    let _expr = exprs[0].clone();
                    exprs.remove(0);
                    ret.push(Expr::Oe(Box::new(expr), Box::new(_expr)));
                } else if expr.is_left_part() {
                    ret.push(build_exprs(exprs))
                } else if expr.is_right_part() {
                    break;
                } else if ops.len() == 0 {
                    ops.push(expr)
                } else
                /* ops.len() == 1*/
                {
                    ops.push(expr);
                    // 尝试处理优先级问题
                    while ops.len() > 1 {
                        let last_p = expr_priotity(&ops[ops.len() - 2]);
                        let this_p = expr_priotity(&ops[ops.len() - 1]);
                        if last_p <= this_p {
                            let len = ret.len();
                            ret[len - 2] = Expr::Eoe(
                                Box::new(ret[ret.len() - 2].clone()),
                                Box::new(ops[ops.len() - 2].clone()),
                                Box::new(ret[ret.len() - 1].clone()),
                            );
                            ret.remove(len - 1);
                            ops.remove(ops.len() - 2);
                        } else {
                            break;
                        }
                    }
                }
            }
            _ => ret.push(expr),
        }
    }
    while ops.len() > 0 {
        let len = ret.len();
        ret[len - 2] = Expr::Eoe(
            Box::new(ret[ret.len() - 2].clone()),
            Box::new(ops[ops.len() - 1].clone()),
            Box::new(ret[ret.len() - 1].clone()),
        );
        ret.remove(len - 1);
        ops.remove(ops.len() - 1);
    }
    return ret[0].clone();
}
// 中文 aEoe
// aaaaa
