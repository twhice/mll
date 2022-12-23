/*
对PC的计量只能由高级可编译类型进行
避免重复计量PC导致错误

内部变量:
mll_ctcf_<FN_NAME>_<INDEX>  函数传参
mll_rtfr_<FN_NAME>          函数返回
mll_rtsw                    switch跳转
*/
use super::abi::Condition;
use super::code::Condition as CExpr;
use super::{
    abi::{LogicCode, Op},
    code::{CtrlDef, CtrlIf, CtrlReturn, CtrlSwitch, CtrlWhile, Expr, Set},
};
use crate::lang::vec_to_str;
use crate::{error::CTErr, DEBUG};
pub trait Complite
where
    Self: Debug,
{
    fn compliet(&self) -> Codes;
    // 我是傻逼
    fn is_ctrl_return(&self) -> bool {
        false
    }
}

use std::fmt::Debug;
/*
    代码生成部分
    先编译,再链接
    链接为局部链接,可能需要链接多次
*/

type Codes = Vec<LogicCode>;
type Name = Vec<char>;

struct Func {
    pub fn_name: Name,
    pub args_num: usize,
    pub have_ret: bool,
    pub call_self: bool,
}
impl Func {
    pub fn new(fn_name: &Name) -> Self {
        Self {
            fn_name: fn_name.clone(),
            args_num: 0,
            have_ret: false,
            call_self: false,
        }
    }
}

// 定义的函数/值
static mut DEFED_FNS: Vec<Func> = Vec::new();
static mut DEFED_VUL: Vec<Name> = Vec::new();

// 查询值是否已经定义
fn lookup_vul(name: &Vec<char>) -> bool {
    unsafe {
        for defed in &DEFED_VUL {
            if *defed == *name {
                return true;
            }
        }
    }
    false
}

// 别名系统 别名 实际名
static mut SHARD_VUL: Vec<(Name, Name)> = Vec::new();

impl Complite for CtrlDef {
    fn compliet(&self) -> Codes {
        todo!()
    }
}
impl Complite for CtrlReturn {
    fn compliet(&self) -> Codes {
        todo!()
    }
}
impl Complite for CtrlIf {
    fn compliet(&self) -> Codes {
        // 准备储存跳转行的一些tag,结束行的一些tag
        let mut codes = Codes::new();
        let mut jump_tags: Vec<usize> = Vec::new();
        let p_jump_tags = &mut jump_tags as *mut Vec<usize>;
        let mut finish_tags: Vec<usize> = Vec::new();

        // 重复代码
        let pop_jump_tags_first = || -> usize {
            unsafe {
                let tag = (*p_jump_tags)[0];
                (*p_jump_tags).remove(0);
                return tag;
            }
        };

        // 载入跳转行
        jump_tags.push(codes.link_cdtn(&self.if_condition));
        for elif in &self.elifs {
            jump_tags.push(codes.link_cdtn(&elif.0));
        }

        // 如果可能,编译else
        if self.else_statement.len() != 0 {
            codes.link_cmus(&self.else_statement);
        }

        // 跳转语句末尾的结束行
        finish_tags.push(codes.len());
        codes.push(jump_always());

        // 编译语句
        // 依次定向jump_tags的tag
        let codes_len = codes.len();
        codes[pop_jump_tags_first()].reset_target(codes_len);
        // 编译链接跳转指向的代码本体
        codes.link_cmus(&self.if_statement);
        finish_tags.push(codes.len());
        codes.push(jump_always());
        for elif in &self.elifs {
            let codes_len = codes.len();
            codes[pop_jump_tags_first()].reset_target(codes_len);
            codes.link_cmus(&elif.1);
            finish_tags.push(codes.len());
            codes.push(jump_always());
        }

        // 处理跳转到结束行的tag
        let codes_len = codes.len();
        for finish_tag in finish_tags {
            codes[finish_tag].reset_target(codes_len);
        }
        return codes;
    }
}
impl Complite for CtrlSwitch {
    fn compliet(&self) -> Codes {
        // 虚拟容器
        let mut codess: Vec<Codes> = Vec::new();

        // 编译同时计算出最长元素的长
        let mut max_len: usize = 0;
        for case in &self.cases {
            // 编译单个
            let mut codes = Codes::new();
            codes.link_cmus(case);
            if codes.len() > max_len {
                max_len = codes.len();
            }
            codess.push(codes);
        }

        // 补全长度不足部分,同时链接
        let mut cases_codes = Codes::new();
        for codes in &mut codess {
            for _ in 0..max_len - codes.len() {
                codes.push(empty_code())
            }
            cases_codes.link(codes);
        }

        // 创建跳转表
        let mut codes = Codes::new();
        let cdtn_name = codes.link_expr(&self.condition);

        // 跳转配置
        codes.push(LogicCode::Op(
            Op::Mul,
            create_data("mll_rtsw"),
            create_data(&format!("{}", max_len)),
            cdtn_name,
        ));

        // 进行跳转
        codes.push(LogicCode::Op(
            Op::Add,
            create_data("@counter"),
            create_data("@counter"),
            create_data("mll_ct_rtsw"),
        ));

        // 链接并返回
        codes.link(&mut cases_codes);
        return codes;
    }
}
impl Complite for CtrlWhile {
    fn compliet(&self) -> Codes {
        let mut codes = Codes::new();
        // 判断语句的位置
        let taga = codes.link_cdtn(&self.condition);
        // 编译循环体部分
        codes.link_cmus(&self.statements);
        codes.push(jump_always());
        // 避免冲突,获取长
        let codes_len = codes.len();
        // 最后一条语句指向判断语句
        codes[codes_len - 1].reset_target(taga);
        // 判断语句指空
        codes[taga].reset_target(codes_len);
        return codes;
    }
}
impl Complite for Set {
    fn compliet(&self) -> Codes {
        let mut codes = Codes::new();
        // 遍历set组合
        for set in &self.sets {
            // 求值
            let set_vul = codes.link_expr(&set.1);
            // 赋值
            codes.push(LogicCode::Set(set.0.clone(), set_vul));
            // 注册状态
            if !lookup_vul(&set.0) {
                unsafe { DEFED_VUL.push(set.0.clone()) }
            }
        }
        return codes;
    }
}
// 内建宏      函数名 参数长 实现
const MACROS: [(&str, usize, &dyn Fn(&Vec<Expr>) -> Codes); 1] = [(
    "sin",
    1,
    &(|args: &Vec<Expr>| -> Codes {
        let mut codes = Codes::new();
        let arg0 = codes.link_expr(&args[0]);
        codes.push(LogicCode::Op(Op::Sin, alloc_name(), arg0, create_data("0")));
        codes
    }),
)];

// 原版常量
// const MDT_CONSTS : [&str;2]=["@counter","links"]
impl Complite for Expr {
    fn compliet(&self) -> Codes {
        match self {
            Expr::Eoe(l_expr, op, r_expr) => {
                let mut codes = Codes::new();
                // 求取符号两边
                let lv = codes.link_expr(&l_expr);
                let rv = codes.link_expr(&r_expr);
                // 运算行
                codes.push(LogicCode::Op(Op::from(&(**op)), alloc_name(), lv, rv));
                codes
            }
            Expr::Oe(op, vul) => {
                let mut codes = Codes::new();
                // 求取值
                let vul = codes.link_expr(vul);
                // 运算行
                codes.push(LogicCode::Op(
                    Op::from(&(**op)),
                    alloc_name(),
                    vul,
                    // 空白值
                    create_data("0"),
                ));
                codes
            }
            Expr::Data(vul) => {
                // 数字开头 @或者_开头直接跳
                let currten = vul[0];
                if currten.is_ascii_digit() || currten == '@' || currten == '_' {
                    return vec![LogicCode::Set(alloc_name(), vul.clone())];
                }

                unsafe {
                    // 查看名称是否为别名
                    for shard in &SHARD_VUL {
                        if *vul == shard.0 {
                            return vec![LogicCode::Set(alloc_name(), shard.1.clone())];
                        }
                    }

                    // 查看值是否已经定义
                    // 未定义的给予提醒
                    if !lookup_vul(vul) {
                        CTErr::UnDefVul(vul.clone()).solve();
                    }

                    return vec![LogicCode::Set(alloc_name(), vul.clone())];
                }
            }
            Expr::Op(_) => todo!(),
            Expr::CallFn(fn_name, args) => {
                // 闭包函数简化代码
                let match_fn = |match_fn_name: &Vec<char>, args_num: usize| -> bool {
                    (args.len() == args_num)
                        && (fn_name.len() == match_fn_name.len())
                        && (*fn_name == *match_fn_name)
                };

                let mut codes = Codes::new();

                // 在内建宏函数中查询是否为已有函数
                for r#macro in &MACROS {
                    if match_fn(&r#macro.0.chars().collect(), r#macro.1) {
                        codes.link(&mut r#macro.2(args));
                        return codes;
                    }
                }

                unsafe {
                    // 在注册函数中查询是否为已有函数
                    for func in &DEFED_FNS {
                        if match_fn(&func.fn_name, func.args_num) {
                            // 传参
                            for arg_id in 0..args.len() {
                                // 链接参数
                                let arg_vul = codes.link_expr(&args[arg_id]);
                                // 传参语句
                                codes.push(LogicCode::Set(
                                    format!("mll_ctcf_{}_{}", vec_to_str(&fn_name), arg_id)
                                        .chars()
                                        .collect(),
                                    arg_vul,
                                ));
                            }

                            // 如果有必要, 收取返回值
                            if func.have_ret {
                                codes.push(LogicCode::Set(
                                    alloc_name(),
                                    format!("mll_rtfr_{}", vec_to_str(&fn_name))
                                        .chars()
                                        .collect(),
                                ))
                            }

                            return codes;
                        }
                    }
                }

                // 仍未定位到函数 :报错
                CTErr::UnknowFn(fn_name.clone()).solve();
                todo!()
            }
        }
    }
}
trait Link {
    fn link(&mut self, add_codes: &mut Self);
    fn link_expr(&mut self, expr: &Expr) -> Name;
    fn link_cdtn(&mut self, condition: &CExpr) -> usize;
    fn link_cmus(&mut self, complier_units: &Vec<Box<dyn Complite>>);
}
impl Link for Codes {
    fn link(&mut self, add_codes: &mut Self) {
        let base_codes = self;
        let base_len = base_codes.len();
        for i in 0..add_codes.len() {
            if let LogicCode::Jump(..) = &add_codes[i] {
                let old_target = add_codes[i].get_target();
                add_codes[i].reset_target(base_len + old_target)
            }
        }
        (*base_codes).append(add_codes);
    }

    fn link_expr(&mut self, expr: &Expr) -> Name {
        let mut codes = expr.compliet();
        self.link(&mut codes);
        let self_len = self.len();
        return match self[self_len].clone() {
            LogicCode::Set(_, vul) => {
                self.remove(self_len);
                vul.clone()
            }
            LogicCode::Op(_, vul, _, _) => vul.clone(),
            _ => todo!(),
        };
    }

    fn link_cdtn(&mut self, condition: &CExpr) -> usize {
        // 链接两条件
        let le = self.link_expr(&condition.lexpr);
        let re = self.link_expr(&condition.rexpr);
        // 编译条件
        let op = Op::from(condition.op.clone());
        let cdtn = Condition::try_from(op);
        match cdtn {
            // 简化的符号
            Ok(cdtn) => {
                // 严格相等的情况
                if let Condition::Seq = cdtn {
                    // 退化为eq
                    let new_name = alloc_name();
                    self.push(LogicCode::Op(Op::StrictEqual, new_name.clone(), le, re));
                    self.push(LogicCode::Jump(
                        0,
                        Condition::Eq,
                        new_name,
                        create_data("0"),
                    ));
                    // 返回位置
                    return self.len() - 1;
                }

                // 默认情况
                // 条件取反
                let cdtn = match cdtn {
                    Condition::Eq => Condition::NotEq,
                    Condition::NotEq => Condition::Eq,
                    Condition::Greater => Condition::NotGreater,
                    Condition::NotGreater => Condition::Greater,
                    Condition::Less => Condition::NotLess,
                    Condition::NotLess => Condition::Less,
                    Condition::Always => Condition::Always,
                    _ => todo!(),
                };
                self.push(LogicCode::Jump(0, cdtn, le, re));
                return self.len() - 1;
            }
            Err(_) => {
                // 计算结果再取反
                let new_name = alloc_name();
                self.push(LogicCode::Op(op, new_name.clone(), le, re));
                self.push(LogicCode::Jump(
                    0,
                    Condition::Eq,
                    new_name,
                    create_data("0"),
                ));
                // 返回位置
                return self.len() - 1;
            }
        }
    }

    fn link_cmus(&mut self, complie_units: &Vec<Box<dyn Complite>>) {
        for complie_unit in complie_units {
            self.link(&mut complie_unit.compliet());
        }
    }
}

// 常用函数

static mut ALLOCED: usize = 0;
fn alloc_name() -> Name {
    unsafe {
        let name = format!("mll_ct_expr_temp_{}", ALLOCED).chars().collect();
        ALLOCED += 1;
        return name;
    }
}
fn create_data(name: &str) -> Name {
    name.chars().collect()
}
fn jump_always() -> LogicCode {
    LogicCode::Jump(
        0,
        super::abi::Condition::Always,
        create_data("114514"),
        create_data("1919810"),
    )
}
fn empty_code() -> LogicCode {
    LogicCode::Set(create_data("_"), create_data("_"))
}
