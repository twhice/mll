use lex::*;
use std::{env::args, io};
use stmt::*;

mod lex;
mod parse;
mod stmt;

type Stmts = Vec<Box<dyn Statement>>;
type Tokens = Vec<Token>;

fn main() -> io::Result<()> {
    let args = args().collect::<Vec<String>>();
    if args.len() == 2 {
        let mut program = Complier::new(&args[1])?;
        program.run_lex().run_parser();
        program.debug_tokens().check_tokens();
        program.check_var_pool().check_label_pool();
        let _stmts = program.get_stmts();
        let _tokens = program.get_tokens();
    } else {
        println!("使用方式: ./mll <源文件>")
    }

    Ok(())
}
