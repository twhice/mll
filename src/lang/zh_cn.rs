use std::process::exit;

use crate::error::CTErr;

use super::super::error::Err;
use super::super::io::Meg;
use crate::{ToString, WARN_MEG};
pub fn get_errmeg(error: &Err) -> String {
    return (match error {
        Err::UnknownEscapeCharacter => "未知的转义字符",
        Err::UnknowSymbol => "未知运算符",
        Err::UnknowKeyword => "未知关键字",
        Err::UnknowSyntax => "未知的语法",
        Err::Empty => "FAIL",
        Err::UseSet => "错误的set语句 set语句的正确用法: set 标识符 = 表达式 { , 标识符 = 表达式}",
        Err::UseDef => "错误的函数定义",
        Err::UseBlock => "不合法的语句块 正确的语句块: { 语句... }",
        Err::UseCallFn => "错误的函数调用",
        Err::Unmatched => "不合语法",
        Err::NotVul => "它不可以作为值",
        Err::NotName => "他不可以作为标识符",
        Err::MissBra => "缺少括号",
        Err::MissVul => "缺少值",
        Err::MissName => "缺少标识符",
        Err::IoMissArg | Err::IoNoArg => "缺少参数 \n输入参数-h以寻求帮助",
        Err::IoTooMuchArg => "过多参数 \n输入参数-h以寻求帮助",
        Err::IoUnknowArg => "未知参数 \n输入参数-h以寻求帮助",
        Err::None => "嗯哼哼哼啊啊啊啊啊啊啊啊啊啊啊啊啊啊啊啊啊啊啊啊啊啊啊啊啊啊啊啊啊啊啊啊啊啊啊啊啊啊啊啊啊啊啊啊啊啊啊啊啊啊啊啊啊啊啊啊啊啊啊啊啊啊啊啊啊啊啊啊啊啊啊啊啊啊啊啊啊啊啊啊",
    })
    .to_owned();
}

pub fn get_buildin_meg(meg: &Meg) -> String {
    match meg {
        Meg::Help => format!(
            "{}\n{}\n{}\n{}\n{}\n{}\n{}",
            "mll\t   <输入文件>\t传入输出文件的路径",
            "\t-o <输出文件>\t传入输出文件的位置,默认为./output.mdtc",
            "\t-d 展示DEBUG信息(观感极差)",
            "\t-w 展示警告(极具误导性)",
            "\t-p 在命令行中输出结果",
            "mll\t-v 获取版本信息",
            "\t-h\t获取帮助"
        ),
        Meg::Version => format!(
            "{}\n{}\n{}",
            "Mindustry-logic-language V0.0.1be", "By 异月(twhice)", "LICENSE: GPLv3"
        ),
    }
}
pub fn cte_solve(err: &CTErr) {
    if unsafe { WARN_MEG } {
        match err {
            CTErr::UnknowFn(fn_name) => {
                println!("警告: 无法查询函数 \"{}\"的定义", fn_name.to_string());
            }
            CTErr::UnknowConst(const_name) => {
                println!(
                "警告: 你在使用自定义数据 \"{}\" 作为unit_bind,sensor或者其他宏的参数,这可能引起bug",
                const_name.to_string()
            );
            }
            CTErr::SensorTypeUnmatch(caller, caller_type, callled, arg_id, called_expect) => {
                println!(
                    "警告: 变量\"{}\"的类型是\"{}\",而\"{}\"的第 {} 个参数期望\"{}\"类型",
                    caller.to_string(),
                    caller_type,
                    callled.to_string(),
                    arg_id,
                    called_expect
                );
            }
            CTErr::ProcessTooLong => {
                println!("警告: 编译后的代码行数大于999,将无法执行!请重构代码");
            }
            CTErr::UnDefVul(na) => {
                println!("警告: 未定义的值{}", na.to_string())
            }
            CTErr::UnknowReturn => {
                println!("警告: 代码某处出现了意义不明的return,已经忽略")
            }
            _ => {}
        }
    }
    match err {
        CTErr::DefinDef(fnn1, fnn2) => {
            println!(
                "错误: 在函数{}中定义了函数{}",
                fnn1.to_string(),
                fnn2.to_string()
            );
            println!("暂不支持闭包,不可在函数中定义函数!");
            exit(1);
        }
        CTErr::CallFninDef(fnn) => {
            println!("错误: 在函数{}中调用了自身", fnn.to_string());
            println!("暂不支持递归,不可在函数中定义函数!");
            exit(1);
        }
        _ => {}
    }
}
