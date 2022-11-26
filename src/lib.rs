mod core;
mod error;
mod lang;
const LANGUAGE: lang::Language = lang::Language::Chinese;
pub fn test() {
    let src = "pg { set awasome}";
    println!("{:?}", core::lexer(src, &mut core::Pos::new()))
}
