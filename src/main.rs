use std::{fmt::Error, process::ExitCode};

fn main() -> Result<ExitCode, Error> {
    if false {
        mindustry_logic_language::old_err_test();
        mindustry_logic_language::old_lexer_test();
        mindustry_logic_language::old_repl();
        mindustry_logic_language::old_parser_test();
    }
    let mut src = String::new();
    src += "if 1 {\n";
    src += "    set a = 1\n";
    src += "    set b = a\n";
    src += "}elif a!=1{\n";
    src += "    while a != 1{\n";
    src += "       set a = a + 1/10\n";
    src += "    }\n";
    src += "}else{\n";
    src += "    set a = 1\n";
    src += "}\n";
    mindustry_logic_language::run(src, "main.mrh");
    return Ok(ExitCode::SUCCESS);
}
