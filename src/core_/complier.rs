/*
对PC的计量只能由高级可编译类型进行
避免重复计量PC导致错误
*/
use super::{abi::*, code::*};
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

static mut DEFED_FNS: Vec<Func> = Vec::new();
static mut DEFED_VUL: Vec<Name> = Vec::new();

impl Complite for CtrlDef {
    fn compliet(&self) -> Codes {
        todo!()
    }
}
impl Complite for CtrlIf {
    fn compliet(&self) -> Codes {
        todo!()
    }
}
impl Complite for CtrlReturn {
    fn compliet(&self) -> Codes {
        todo!()
    }
}
impl Complite for CtrlSwitch {
    fn compliet(&self) -> Codes {
        todo!()
    }
}
impl Complite for CtrlWhile {
    fn compliet(&self) -> Codes {
        todo!()
    }
}
impl Complite for Set {
    fn compliet(&self) -> Codes {
        todo!()
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
            Expr::Data(_) => todo!(),
            Expr::Op(_) => todo!(),
            Expr::CallFn(fn_name, args) => {
                // 闭包函数简化代码
                let match_fn = |match_fn_name: &Vec<char>, args_num: usize| -> bool {
                    (args.len() == args_num)
                        && (fn_name.len() == match_fn_name.len())
                        && (*fn_name == *match_fn_name)
                };

                let mut codes = Codes::new();
                let mut located_fn = false;

                // 在内建宏函数中查询是否为已有函数
                for r#macro in &MACROS {
                    if match_fn(&r#macro.0.chars().collect(), r#macro.1) {
                        codes.link(&mut r#macro.2(args));
                        located_fn = true;
                        break;
                    }
                }

                // 在注册函数中查询是否为已有函数
                if !located_fn {
                    unsafe {
                        for func in &DEFED_FNS {
                            if match_fn(&func.fn_name, func.args_num) {
                                // 传参
                                for arg_id in 0..args.len() {
                                    // 链接参数
                                    let arg_vul = codes.link_expr(&args[arg_id]);
                                    // 传参语句
                                    codes.push(LogicCode::Set(
                                        format!("mll_ct_call_{}_{}", vec_to_str(&fn_name), arg_id)
                                            .chars()
                                            .collect(),
                                        arg_vul,
                                    ));
                                }
                                located_fn = true;
                                break;
                            }
                        }
                    }
                }

                // 仍未定位到函数 :报错
                if !located_fn {
                    CTErr::UnknowFn(fn_name.clone()).solve();
                }
                todo!()
            }
        }
    }
}
trait Link {
    fn link(&mut self, add_codes: &mut Self);
    fn link_expr(&mut self, expr: &Expr) -> Name;
}
impl Link for Codes {
    fn link(&mut self, add_codes: &mut Self) {
        let base_codes = self;
        let base_len = base_codes.len();
        for i in 0..add_codes.len() {
            if let LogicCode::Jump(..) = &add_codes[i] {
                add_codes[i].offset_target(base_len)
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
}
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
