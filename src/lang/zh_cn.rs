use super::super::error::Err;
use super::super::io::Meg;
pub fn get_errmeg(error: &Err) -> &str {
    match error {
        Err::UnknownEscapeCharacter => "未知的转义字符",
        Err::UnknowSymbol => "未知运算符",
        Err::UnknowKeyword => "未知关键字",
        Err::Empty => "缺少...",
        Err::Unmatched => "不合语法",
        Err::NotVul => "它不可以作为值",
        Err::MissBra => "缺少括号",
        Err::MissVul => "缺少值",
        Err::IoMissArg | Err::IoNoArg => "缺少参数 \n输入参数-h以寻求帮助",
        Err::IoTooMuchArg => "过多参数 \n输入参数-h以寻求帮助",
        Err::IoUnknowArg => "未知参数 \n输入参数-h以寻求帮助",
    }
}

pub fn get_buildin_meg(meg: &Meg) -> &str {
    match meg {
        Meg::Help => MEG_HELP,
        Meg::Version => MEG_VERSION,
    }
}
const MEG_HELP: &str =
    "mll\t-f <输入文件> \t源文件路径\n\t[-o <输出文件>]\t输出文件路径\nmll\t -h \t获取帮助\n\t-v \t获取版本信息";
const MEG_VERSION: &str = "Minsustry-logic-language \nV0.0.0 By 异月(twhcie) \nLICENSE: GPLv3";
