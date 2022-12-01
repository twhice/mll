mod core_;
mod error;
mod lang;
pub use core_::run;
pub const DEBUG: bool = true;
// pub use core::lexer;
const LANGUAGE: lang::Language = lang::Language::Chinese;
pub fn old_lexer_test() {
    println!(" 语法分析测试");
    println!("\n成功: ");
    let src = "pg { set awasome}";
    println!("{:?}", core_::lexer(src, &mut core_::Pos::new()));

    println!("\n失败: ");
    let src = "pg { / }";
    println!("{:?}", core_::lexer(src, &mut core_::Pos::new()));
    println!("\n");
}
pub fn old_repl() {
    println!("语法分析尚未完成...\n输入一行代码,输出语法分析结果!\nCtrl+C退出");
    loop {
        let mut input = String::new();
        match std::io::stdin().read_line(&mut input) {
            Ok(_) => println!("{:?}", core_::lexer(&input, &mut core_::Pos::new())),
            Err(_) => todo!("终止"),
        }
    }
}
pub fn old_parser_test() {
    let src = "set a=foo()";
    let mut base_pos = core_::Pos::new();
    base_pos.set_filename("parser-test".to_owned());
    println!("{}", base_pos);
    let mut tokens = core_::lexer(src, &mut base_pos).unwrap();
    let com_unit = core_::parser(&mut tokens);
    println!("{:?}", com_unit)
}
pub fn old_err_test() {
    let mut pos = core_::Pos::new();
    pos.set_line(114514);
    pos.set_filename("filename".to_owned());
    let fake_err = error::ErrMeg::new(pos, error::Err::UnknowKeyword);
    println!("{}", fake_err)
}
