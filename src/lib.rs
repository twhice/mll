mod core;
mod error;
mod lang;
const LANGUAGE: lang::Language = lang::Language::Chinese;
pub fn test() {
    let src = "let src = \" \".to_owned()".to_owned();
    println!("{:?}", core::lexer(src, core::Pos::new()))
}
