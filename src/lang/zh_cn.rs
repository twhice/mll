use super::super::error::Err;
use super::super::io::Meg;
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
    })
    .to_owned();
}

pub fn get_buildin_meg(meg: &Meg) -> String {
    match meg {
        Meg::Help => format!(
            "{}\n{}\n{}\n{}\n{}",
            "mll\t-f <输入文件>\t传入输出文件的路径",
            "\t-o <输出文件>\t传入输出文件的位置,默认为./output.mdtc",
            "\t-d\t展示DEBUG信息(观感极差)",
            "mll\t-v\t获取版本信息",
            "\t-h\t获取帮助"
        ),
        Meg::Version => format!(
            "{}\n{}\n{}",
            "Mindustry-logic-language V0.0.0", "By 异月(twhice)", "LICENSE: GPLv3"
        ),
    }
}
