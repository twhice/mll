mod core_;
mod error;
mod io;
mod lang;
pub struct Argument {
    pub input_file_path: String,
    pub output_file_path: String,
    pub show_debug_meg: bool,
    pub get_help: bool,
    pub get_version: bool,
}

impl Argument {
    pub fn new() -> Self {
        Self {
            input_file_path: String::new(),
            output_file_path: String::new(),
            show_debug_meg: false,
            get_help: false,
            get_version: false,
        }
    }
}
pub static mut DEBUG: bool = false;
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
pub use core_::complie;
pub use io::build_args;
pub use io::Meg;
pub use lang::get_buildin_meg;
pub use lang::get_errmeg;
