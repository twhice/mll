/*
对PC的计量只能由高级可编译类型进行
避免重复计量PC导致错误

内部变量:
mll_ctcf_<FN_NAME>_<INDEX>  函数传参
mll_rtfr_<FN_NAME>          函数返回
mll_rtsc_<FN_NAME>          执行函数前保存位置
mll_rtsw                    switch跳转
*/
use super::abi::{BuildingQuery, Condition, Sensor};
use super::code::Condition as CExpr;
use super::{
    abi::{LogicCode, Op},
    code::{CtrlDef, CtrlIf, CtrlReturn, CtrlSwitch, CtrlWhile, Expr, Set},
};
use crate::error::CTErr;
use crate::lang::vec_to_str;
pub trait Complie
where
    Self: Debug,
{
    fn complie(&self) -> Codes;
    // 我是傻逼
    fn is_def(&self) -> bool {
        false
    }
}

use std::fmt::Debug;
use std::ops::Deref;
/*
    代码生成部分
    先编译,再链接
    链接为局部链接,可能需要链接多次
*/

type Codes = Vec<LogicCode>;
type Name = Vec<char>;

#[derive(Debug, Clone)]
struct Func {
    pub fn_name: Name,
    pub args_num: usize,
    pub call_self: bool,
    pub address: usize,
}
impl Func {
    pub fn new(fn_name: &Name) -> Self {
        Self {
            fn_name: fn_name.clone(),
            args_num: 0,
            call_self: false,
            address: 0,
        }
    }
}

#[derive(Clone, Copy)]
enum VulType {
    Base,
    Building,
    Unit,
}

struct Vul {
    vul_name: Name,
    vul_type: VulType,
}
impl Deref for Vul {
    type Target = Name;

    fn deref(&self) -> &Self::Target {
        &self.vul_name
    }
}
impl Vul {
    fn new(vul_name: Name, vul_type: VulType) -> Self {
        Self { vul_name, vul_type }
    }
}

// 现存语句数 1是因为恒定有一条跳过所有函数定义的语句
static mut CODES_LEN: usize = 1;
// 定义的函数/值
static mut DEFED_FNS: Vec<Func> = Vec::new();
static mut DEFED_VUL: Vec<Vul> = Vec::new();
// 正在定义的函数
static mut DEFING_FN: Option<Func> = None;
// 查询值是否已经定义
fn lookup_vul(name: &Vec<char>) -> bool {
    unsafe {
        for defed in &DEFED_VUL {
            if defed.vul_name == *name {
                return true;
            }
        }
    }
    false
}
fn lookup_vultype(name: &Vec<char>) -> VulType {
    unsafe {
        for defed in &DEFED_VUL {
            if defed.vul_name == *name {
                return defed.vul_type;
            }
        }
    }
    VulType::Base
}
fn get_ref_vul(name: &Vec<char>) -> &'static mut Vul {
    unsafe {
        for r#ref in &mut DEFED_VUL {
            if r#ref.vul_name == *name {
                return r#ref;
            }
        }
    }
    todo!()
}
fn update_vul(name: &Vec<char>, r#type: VulType) {
    // 截取自旧(2022-12-25前)impl Complie for Set
    let this = Vul::new(name.clone(), r#type);
    if !lookup_vul(&name) {
        unsafe { DEFED_VUL.push(this) }
    } else {
        // 已经注册,就进行覆盖
        *get_ref_vul(&name) = this;
    }
}
// 别名系统 别名 实际名
static mut SHARD_VUL: Vec<(Name, Name)> = Vec::new();
// 实验性功能:
// 外部传参: 值类型
// 仅CallFn设置为其他,其他函数只设置为Base
static mut SHARE_UVL_TYPE: VulType = VulType::Base;

impl Complie for CtrlDef {
    fn complie(&self) -> Codes {
        // 避免SHARD覆盖,不允许函数中定义函数
        unsafe {
            if let Some(defing_fn) = &DEFING_FN {
                CTErr::DefinDef(defing_fn.fn_name.clone(), self.fn_name.clone()).solve()
            }
        }

        // 注册函数
        let mut this = Func::new(&self.fn_name);
        this.args_num = self.args.len();
        this.call_self = false;
        this.address = unsafe { CODES_LEN };
        unsafe {
            DEFED_FNS.push(this.clone());
            DEFING_FN = Some(this);
        }

        // 设置映射
        for arg_id in 0..self.args.len() {
            let arg = &self.args[arg_id];
            unsafe {
                SHARD_VUL.push((
                    arg.clone(),
                    format!("mll_ctcf_{}_{}", vec_to_str(&self.fn_name), arg_id)
                        .chars()
                        .collect(),
                ))
            }
        }

        // 编译本体
        let mut codes = Codes::new();
        codes.link_cmus(&self.statement);

        // 刷新CODES_LEN和DEFING_FN
        unsafe {
            CODES_LEN += codes.len();
            DEFING_FN = None;
        }
        return codes;
    }
    fn is_def(&self) -> bool {
        true
    }
}
impl Complie for CtrlReturn {
    fn complie(&self) -> Codes {
        // 获取函数名,过滤意义不明的return
        let fn_name = vec_to_str(unsafe {
            match &DEFING_FN {
                Some(defing_fn) => &defing_fn.fn_name,
                None => {
                    CTErr::UnknowReturn.solve();
                    return Codes::new();
                }
            }
        });

        let fn_ret_name = format!("mll_rtfr_{}", fn_name)
            .chars()
            .collect::<Vec<char>>();

        // 编译表达式
        let mut codes = Codes::new();
        let possiable_vul = codes.link_expr(&self.return_vul);

        // 在表达式编译产物被全部优化时,取possiable_vul
        if codes.len() == 0 {
            codes.push(LogicCode::Set(fn_ret_name, possiable_vul))
        } else {
            let codes_len = codes.len();
            // 重命名: 改用重命名函数

            codes[codes_len].rename_result(fn_ret_name);

            // 旧代码
            // codes[codes_len - 1] = match codes[codes_len - 1].clone() {
            //     LogicCode::Op(op, _, lv, rv) => LogicCode::Op(op, fn_ret_name, lv, rv),
            //     _ => todo!("你发现了一个Bug,速速反馈!"),
            // };
        }

        // 跳转回去
        codes.push(LogicCode::Set(
            create_data("@counter"),
            format!("mll_rtsc_{}", fn_name).chars().collect(),
        ));
        return codes;
    }
}
impl Complie for CtrlIf {
    fn complie(&self) -> Codes {
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
        jump_tags.push(codes.link_cond(&self.if_condition));
        for elif in &self.elifs {
            jump_tags.push(codes.link_cond(&elif.0));
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
impl Complie for CtrlSwitch {
    fn complie(&self) -> Codes {
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
impl Complie for CtrlWhile {
    fn complie(&self) -> Codes {
        let mut codes = Codes::new();
        // 判断语句的位置
        let taga = codes.link_xcon(&self.condition);
        // 编译循环体部分
        codes.link_cmus(&self.statements);
        codes.push(jump_always());
        // 避免冲突,获取长
        let codes_len = codes.len();
        // 最后一条语句指向第一条语句
        codes[codes_len - 1].reset_target(0);
        // 判断语句指空
        codes[taga].reset_target(codes_len);
        return codes;
    }
}
impl Complie for Set {
    fn complie(&self) -> Codes {
        let mut codes = Codes::new();
        // 遍历set组合
        for set in &self.sets {
            // 求值
            let set_vul = codes.link_expr(&set.1);
            // 赋值 类似Return,尽可能采用Rename的方式
            if codes.len() == 0 {
                // 值是有属性的量,就篡改类型!
                let vul_type = lookup_vultype(&set_vul);

                if set_vul == "@unit".chars().collect::<Vec<char>>() {
                    unsafe { SHARE_UVL_TYPE = VulType::Unit }
                } else if !matches!(vul_type, VulType::Base) {
                    unsafe { SHARE_UVL_TYPE = vul_type }
                } else {
                    codes.push(LogicCode::Set(set.0.clone(), set_vul));
                }
            } else {
                let codes_len = codes.len();
                codes[codes_len - 1].rename_result(set.0.clone());
            }
            // 注册状态
            update_vul(&set.0.clone(), unsafe { SHARE_UVL_TYPE })
        }
        return codes;
    }
}
// 内建宏      函数名 参数长 实现
const MACROS: [(&str, usize, &dyn Fn(&Vec<Expr>) -> Codes); 12] = [
    (
        "sin",
        1,
        &(|args: &Vec<Expr>| -> Codes {
            let mut codes = Codes::new();
            let arg0 = codes.link_expr(&args[0]);
            codes.push(LogicCode::Op(Op::Sin, alloc_name(), arg0, create_data("0")));
            codes
        }),
    ),
    (
        "cos",
        1,
        &(|args: &Vec<Expr>| -> Codes {
            let mut codes = Codes::new();
            let arg0 = codes.link_expr(&args[0]);
            codes.push(LogicCode::Op(Op::Cos, alloc_name(), arg0, create_data("0")));
            codes
        }),
    ),
    (
        "tan",
        1,
        &(|args: &Vec<Expr>| -> Codes {
            let mut codes = Codes::new();
            let arg0 = codes.link_expr(&args[0]);
            codes.push(LogicCode::Op(Op::Tan, alloc_name(), arg0, create_data("0")));
            codes
        }),
    ),
    (
        "asin",
        1,
        &(|args: &Vec<Expr>| -> Codes {
            let mut codes = Codes::new();
            let arg0 = codes.link_expr(&args[0]);
            codes.push(LogicCode::Op(
                Op::Asin,
                alloc_name(),
                arg0,
                create_data("0"),
            ));
            codes
        }),
    ),
    (
        "acos",
        1,
        &(|args: &Vec<Expr>| -> Codes {
            let mut codes = Codes::new();
            let arg0 = codes.link_expr(&args[0]);
            codes.push(LogicCode::Op(
                Op::Acos,
                alloc_name(),
                arg0,
                create_data("0"),
            ));
            codes
        }),
    ),
    (
        "atan",
        1,
        &(|args: &Vec<Expr>| -> Codes {
            let mut codes = Codes::new();
            let arg0 = codes.link_expr(&args[0]);
            codes.push(LogicCode::Op(
                Op::Atan,
                alloc_name(),
                arg0,
                create_data("0"),
            ));
            codes
        }),
    ),
    (
        "get_link",
        1,
        &(|args: &Vec<Expr>| -> Codes {
            let mut codes = Codes::new();
            let arg0 = codes.link_expr(&args[0]);
            codes.push(LogicCode::GetLink(alloc_name(), arg0));
            codes
        }),
    ),
    // 简易语句(2022 12 24)
    (
        "ubind",
        1,
        &(|args: &Vec<Expr>| -> Codes {
            let mut codes = Codes::new();
            let arg0 = codes.link_expr(&args[0]);
            codes.push(LogicCode::UnitBind(arg0));
            codes
        }),
    ),
    (
        "umove",
        2,
        &(|args: &Vec<Expr>| -> Codes {
            let mut codes = Codes::new();
            let x = codes.link_expr(&args[0]);
            let y = codes.link_expr(&args[1]);
            codes.push(LogicCode::UnitControl(
                create_data("move"),
                x,
                y,
                create_data("0"),
                create_data("0"),
            ));
            codes
        }),
    ),
    (
        "ulocate",
        6,
        &(|args: &Vec<Expr>| -> Codes {
            let mut codes = Codes::new();
            let target = BuildingQuery::from(&args[0]);
            let is_enemy = codes.link_expr(&args[1]);
            let name_of_x = args[2].get_data();
            let name_of_y = args[3].get_data();
            let name_of_ifcound = args[4].get_data();
            let name_of_building = args[5].get_data();

            update_vul(&name_of_x, VulType::Base);
            update_vul(&name_of_y, VulType::Base);
            update_vul(&name_of_ifcound, VulType::Base);
            update_vul(&name_of_building, VulType::Building);

            codes.push(LogicCode::UnitLocate(
                target,
                is_enemy,
                name_of_x,
                name_of_y,
                name_of_ifcound,
                name_of_building,
            ));
            codes
        }),
    ),
    (
        "uidrop",
        2,
        &(|args: &Vec<Expr>| -> Codes {
            let mut codes = Codes::new();

            let target = codes.link_expr(&args[0]);
            let item_number = codes.link_expr(&args[1]);
            codes.push(LogicCode::UnitControl(
                create_data("itemDrop"),
                target,
                item_number,
                create_data("0"),
                create_data("0"),
            ));
            codes
        }),
    ),
    (
        "uitake",
        3,
        &(|args: &Vec<Expr>| -> Codes {
            let mut codes = Codes::new();
            let target = codes.link_expr(&args[0]);
            let arg1_data = args[1].get_data();
            let item_name = if arg1_data == Vec::new() {
                codes.link_expr(&args[1])
            } else {
                arg1_data
            };
            let item_number = codes.link_expr(&args[2]);
            codes.push(LogicCode::UnitControl(
                create_data("itemTake"),
                target,
                item_name,
                item_number,
                create_data("0"),
            ));
            codes
        }),
    ),
];

impl Complie for Expr {
    fn complie(&self) -> Codes {
        match self {
            Expr::Eoe(l_expr, op, r_expr) => {
                let op = Op::from(&(**op));
                let mut codes = Codes::new();

                if matches!(op, Op::Sensor) {
                    // 特殊情况:成员访问运算符(Sensor语句)
                    let lv = codes.link_expr(&l_expr);

                    // 错误处理
                    if matches!(lookup_vultype(&lv), VulType::Base) {
                        CTErr::SensorBasetype(lv.clone()).solve();
                    }

                    let sensor = Sensor::from(&(**r_expr));
                    codes.push(LogicCode::Sensor(alloc_name(), lv, sensor))
                } else {
                    // 普通情况
                    // 求取符号两边
                    let lv = codes.link_expr(&l_expr);
                    let rv = codes.link_expr(&r_expr);
                    // 运算行
                    codes.push(LogicCode::Op(op, alloc_name(), lv, rv));
                }
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
                    // 阻止递归
                    if let Some(definf_fn) = &DEFING_FN {
                        if match_fn(&definf_fn.fn_name, definf_fn.args_num) {
                            CTErr::CallFninDef(definf_fn.fn_name.clone()).solve();
                        }
                    }

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

                            // 保存位置
                            codes.push(LogicCode::Op(
                                Op::Add,
                                format!("mll_rtsc_{}", vec_to_str(fn_name))
                                    .chars()
                                    .collect(),
                                create_data("@counter"),
                                create_data("1"),
                            ));

                            // 跳转
                            codes.push(LogicCode::Set(
                                create_data("@counter"),
                                format!("{}", func.address).chars().collect(),
                            ));

                            // 如果有必要, 收取返回值 -> 强制全体函数有返回值
                            // if func.have_ret {
                            codes.push(LogicCode::Set(
                                alloc_name(),
                                format!("mll_rtfr_{}", vec_to_str(&fn_name))
                                    .chars()
                                    .collect(),
                            ));
                            // }

                            return codes;
                        }
                    }
                }

                // 仍未定位到函数 :报错
                CTErr::UnknowFn(fn_name.clone()).solve();
                return codes;
            }
        }
    }
}
pub trait Link {
    fn link(&mut self, add_codes: &mut Self);
    fn link_expr(&mut self, expr: &Expr) -> Name;
    fn link_xcon(&mut self, condition: &CExpr) -> usize; // link反转条件
    fn link_cond(&mut self, condition: &CExpr) -> usize; // link条件
    fn link_cmus(&mut self, complier_units: &Vec<Box<dyn Complie>>); // link未编译语句块

    // fn link_rnep(&mut self, new_name: Name); // link并且重命名结果
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
        let mut codes = expr.complie();
        self.link(&mut codes);
        let self_len = self.len();
        return match self[self_len - 1].clone() {
            LogicCode::Set(_, vul) => {
                self.remove(self_len - 1);
                unsafe { SHARE_UVL_TYPE = VulType::Base }
                vul.clone()
            }
            LogicCode::Op(_, vul, _, _) => {
                unsafe { SHARE_UVL_TYPE = VulType::Base }
                vul.clone()
            }
            LogicCode::GetLink(..) => {
                unsafe { SHARE_UVL_TYPE = VulType::Building }
                create_data("0")
            }
            LogicCode::Sensor(r, ..) => r.clone(),
            _ => create_data("0"),
        };
    }
    fn link_xcon(&mut self, condition: &CExpr) -> usize {
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
    fn link_cond(&mut self, condition: &CExpr) -> usize {
        // 链接两条件
        let le = self.link_expr(&condition.lexpr);
        let re = self.link_expr(&condition.rexpr);
        // 编译条件
        let op = Op::from(condition.op.clone());
        let cdtn = Condition::try_from(op);

        //  尽可能简化
        match cdtn {
            Ok(op) => {
                self.push(LogicCode::Jump(0, op, le, re));
                return self.len() - 1;
            }
            Err(_) => {
                let new_name = alloc_name();
                self.push(LogicCode::Op(op, new_name.clone(), le, re));
                self.push(LogicCode::Jump(
                    0,
                    Condition::NotEq,
                    new_name,
                    create_data("1"),
                ));
            }
        }
        return self.len() - 1;
    }
    fn link_cmus(&mut self, complie_units: &Vec<Box<dyn Complie>>) {
        for complie_unit in complie_units {
            self.link(&mut complie_unit.complie());
        }
    }
}

// 常用函数

static mut ALLOCED: usize = 0;
pub fn alloc_name() -> Name {
    unsafe {
        let name = format!("mll_ct_expr_temp_{}", ALLOCED).chars().collect();
        ALLOCED += 1;
        return name;
    }
}
pub fn create_data(name: &str) -> Name {
    name.chars().collect()
}
pub fn jump_always() -> LogicCode {
    LogicCode::Jump(
        0,
        super::abi::Condition::Always,
        create_data("114514"),
        create_data("1919810"),
    )
}
pub fn empty_code() -> LogicCode {
    LogicCode::Set(create_data("_"), create_data("_"))
}
