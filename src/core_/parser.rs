/*
Box<dyn Complite> = Ctrl  / Set
Ctrl    = "if" Expr (Box<dyn Complite> / Block) [ 'else ' (Box<dyn Complite> / Block) ]
        = "for" Name Num "," Num[ "," Num] (Box<dyn Complite> / Block)
        = "switch" {case Num (Box<dyn Complite> / Block)}
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

use super::code::*;
use super::complier::Complite;
use super::{Pos, Token, TokenType};
use crate::error::{Err, ErrMeg};

pub fn parser(tokens: &mut Vec<Token>) -> Result<Box<dyn Complite>, ErrMeg> {
    match com_unit(tokens) {
        Ok(cu) => Ok(cu),
        Err(err) => return Err(err),
    }
}
fn com_unit(tokens: &mut Vec<Token>) -> Result<Box<dyn Complite>, ErrMeg> {
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
        let fn_name = keytoken.get_text().clone();
        tokens.remove(0);
        let args = match tokens_get_fnargs(tokens) {
            Ok(ok) => ok,
            Err(err) => return Err(ErrMeg::new(err.pos, Err::UnknowSyntax)),
        };
        return Ok(Box::new(Expr::CallFn(fn_name, args)));
    }
}
fn set(tokens: &mut Vec<Token>) -> Result<Box<dyn Complite>, ErrMeg> {
    tokens.remove(0);
    let mut sets = Vec::new();
    loop {
        let name = tokens_get_name(tokens)?;
        if let Err(err) = tokens_match_err(tokens, "=") {
            match err.err {
                Err::Empty => {
                    return Err(err);
                }
                _ => {
                    return Err(ErrMeg::new(err.pos, Err::UseSet));
                }
            }
        };
        let vul = tokens_build_expr(tokens)?;
        sets.push((name, vul));
        if !tokens_match_bool(tokens, ",") {
            break;
        }
    }

    return Ok(Box::new(Set::new(sets)));
}
fn ctrl_if(tokens: &mut Vec<Token>) -> Result<Box<dyn Complite>, ErrMeg> {
    tokens.remove(0);
    let condition = tokens_get_condition(tokens)?;

    let if_statements = tokens_get_block(tokens)?;
    let mut else_statements = Vec::new();
    let mut elifs = Vec::new();
    while tokens_match_bool(tokens, "elif") {
        tokens.remove(0);
        let condition = tokens_get_condition(tokens)?;
        let statements = tokens_get_block(tokens)?;
        elifs.push((condition, statements));
    }
    if tokens_match_bool(tokens, "else") {
        tokens.remove(0);
        else_statements = tokens_get_block(tokens)?;
    }
    return Ok(Box::new(CtrlIf::new(
        condition,
        if_statements,
        elifs,
        else_statements,
    )));
}
fn ctrl_while(tokens: &mut Vec<Token>) -> Result<Box<dyn Complite>, ErrMeg> {
    // tokens_match_err(tokens, "while")?;
    // skip sth
    tokens.remove(0);

    let condition = tokens_get_condition(tokens)?;

    let statements = tokens_get_block(tokens)?;
    return Ok(Box::new(CtrlWhile::new(condition, statements)));
}
fn ctrl_switch(tokens: &mut Vec<Token>) -> Result<Box<dyn Complite>, ErrMeg> {
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
    return Ok(Box::new(CtrlSwitch::new(condition, cases)));
}
fn ctrl_return(tokens: &mut Vec<Token>) -> Result<Box<dyn Complite>, ErrMeg> {
    tokens_match_err(tokens, "return")?;
    let expr = tokens_build_expr(tokens)?;
    return Ok(Box::new(CtrlReturn::new(expr)));
}
fn ctrl_def(tokens: &mut Vec<Token>) -> Result<Box<dyn Complite>, ErrMeg> {
    // tokens_match_err(tokens, "pg")?;
    tokens.remove(0);
    let fn_name = tokens_get_name(tokens)?;
    let mut get_fn_args = || -> Result<Vec<Expr>, ErrMeg> {
        tokens_match_err(tokens, "(")?;
        let mut args: Vec<Expr> = Vec::new();
        let mut can_break = true;
        while tokens.len() != 0 {
            if matches!(tokens[0].get_type(), TokenType::Name) {
                args.push((&tokens[0]).into());
                tokens.remove(0);
                can_break = true;
                if tokens_match_bool(tokens, ",") {
                    tokens.remove(0);
                    can_break = false
                }
            } else {
                break;
            }
        }
        if can_break == false {
            let err_pos;
            if tokens.len() != 0 {
                err_pos = tokens[0].pos.clone();
            } else {
                err_pos = Pos::new();
            }
            return Err(ErrMeg::new(err_pos, Err::MissName));
        }
        tokens_match_err(tokens, ")")?;
        return Ok(args);
    };
    let args = match get_fn_args() {
        Ok(ok) => ok,
        Err(err) => match err.err {
            Err::Empty | Err::Unmatched => return Err(ErrMeg::new(err.pos, Err::UseDef)),
            _ => return Err(err),
        },
    };
    let statements = tokens_get_block(tokens)?;
    return Ok(Box::new(CtrlDef::new(fn_name, args, statements)));
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
fn tokens_get_name(tokens: &mut Vec<Token>) -> Result<Vec<char>, ErrMeg> {
    let p_tokens = tokens as *mut Vec<Token>;
    let token = match tokens_get_first(tokens) {
        Ok(token) => {
            if matches!(token.get_type(), TokenType::Name) {
                token.clone()
            } else {
                return Err(ErrMeg::new(token.pos.clone(), Err::NotName));
            }
        }
        Err(_) => return Err(ErrMeg::new(Pos::new(), Err::MissName)),
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
        return Err(ErrMeg::new(Pos::new(), Err::Empty));
    }
}
fn tokens_get_block(tokens: &mut Vec<Token>) -> Result<Vec<Box<dyn Complite>>, ErrMeg> {
    if let Err(err) = tokens_match_err(tokens, "{") {
        match err.err {
            Err::Empty => return Err(err),
            _ => return Err(ErrMeg::new(err.pos, Err::UseBlock)),
        }
    };
    let mut ret = Vec::new();
    let mut have_end = false;
    while !tokens.is_empty() {
        if !tokens_match_bool(tokens, "}") {
            ret.push(com_unit(tokens)?);
        } else {
            tokens.remove(0);
            have_end = true;
            break;
        }
    }
    if !have_end {
        return Err(ErrMeg::new(Pos::new(), Err::UseBlock));
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
            exprs.pop();
            let fn_name = token_cache.get_text().clone();
            let fn_args = match tokens_get_fnargs(&mut fake_exprs) {
                Ok(ok) => ok,
                Err(err) => match err.err {
                    Err::Unmatched | Err::Empty => {
                        return Err(ErrMeg::new(err.pos, Err::UseCallFn))
                    }
                    _ => return Err(err),
                },
            };
            exprs.push(Expr::CallFn(fn_name, fn_args));
        }
        if fake_exprs.len() == 0 {
            break;
        }
        fake_exprs.remove(0);
        exprs.push(fake_expr.into());
    }

    // 调用上古屎山
    Ok(unsafe { build_exprs(&mut exprs) })
}
fn tokens_get_expr(tokens: &mut Vec<Token>) -> Result<Vec<Token>, ErrMeg> {
    let mut expect_vul = true;
    let mut ret: Vec<Token> = Vec::new();
    'l: loop {
        if tokens.len() == 0 {
            break 'l;
        }
        let token = tokens[0].clone();

        if expect_vul {
            match token.get_type() {
                TokenType::Name | TokenType::Num | TokenType::Str => {
                    tokens.remove(0);
                    ret.push(token);
                    expect_vul = false
                }
                TokenType::Symbol => {
                    if token.match_text("!") || token.match_text("-") {
                        tokens.remove(0);
                        ret.push(token)
                    } else if token.match_text("(") {
                        tokens.remove(0);
                        ret.append(&mut tokens_get_expr(tokens)?);
                        expect_vul = false;
                        if let Err(err) = tokens_match_err(tokens, ")") {
                            return Err(ErrMeg::new(err.pos, Err::MissBra));
                        }
                    } else {
                        return Err(ErrMeg::new(token.pos, Err::MissVul));
                    }
                }
                TokenType::Space => todo!(),
            }
        } else {
            match token.get_type() {
                TokenType::Symbol => {
                    if expr_priotity(&(&token).into()) != 0 {
                        if token.match_text("!") {
                            return Err(ErrMeg::new(token.pos, Err::NotVul));
                        }
                        tokens.remove(0);
                        ret.push(token);
                        expect_vul = true
                    } else {
                        break;
                    }
                }
                _ => break,
            }
        }
    }
    return Ok(ret);
}
fn tokens_get_fnargs(tokens: &mut Vec<Token>) -> Result<Vec<Expr>, ErrMeg> {
    // let fn_name = tokens_get_name(tokens)?;
    tokens_match_err(tokens, "(")?;
    let mut args = Vec::new();
    let arg0 = tokens_build_expr(tokens);

    if let Ok(arg) = arg0 {
        // let arg0 = arg0.get_text().clone();
        args.push(arg);
        while tokens_match_bool(tokens, ",") {
            tokens.remove(0);
            args.push(tokens_build_expr(tokens)?);
        }
    }
    tokens_match_err(tokens, ")")?;
    return Ok(args);
}
fn expr_priotity(expr: &Expr) -> usize {
    match expr {
        Expr::Op(op_text) => {
            let fuck = [
                vec![" "],
                // vec!["(", ""],
                vec!["!", "**"],
                vec!["*", "/"],
                vec!["+", "-"],
                vec!["<<", ">>"],
                vec![">", "<", ">=", "<="],
                vec!["!=", "=="],
                vec!["&", "&&", "|", "||", "^"],
                // vec![")"],
            ];
            for priotity in 0..fuck.len() {
                let sub_fuck = &fuck[priotity];
                for op in sub_fuck {
                    if *op_text == op.chars().collect::<Vec<char>>() {
                        return priotity;
                    }
                }
            }

            0
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
const COMPARE_OPS: [&str; 6] = [">", "<", "==", "!=", ">=", "<="];
fn tokens_get_condition(tokens: &mut Vec<Token>) -> Result<Condition, ErrMeg> {
    let lexpr = tokens_build_expr(tokens)?;
    match lexpr.clone() {
        Expr::Eoe(le, op, re) => match *op {
            Expr::Op(op) => {
                for compare in COMPARE_OPS {
                    if op == compare.chars().collect::<Vec<char>>() {
                        return Ok(Condition::new(*le, op, *re));
                    }
                }
            }
            // 不可能的分支
            _ => todo!(),
        },
        // 忽略
        _ => {}
    }
    if tokens.len() != 0 {
        for compare in COMPARE_OPS {
            if tokens[0].match_text(compare) {
                tokens.remove(0);
                let rexpr = tokens_build_expr(tokens)?;
                return Ok(Condition::new(lexpr, compare.chars().collect(), rexpr));
            }
        }
    }
    return Ok(Condition::new(
        lexpr.clone(),
        vec!['=', '='],
        Expr::Data(vec!['1']),
    ));
}
