use std::{fmt::Error, process::ExitCode};

fn main() -> Result<ExitCode, Error> {
    if false {
        mindustry_rogic_hardening::err_test();
        mindustry_rogic_hardening::lexer_test();
        mindustry_rogic_hardening::r#loop();
    }
    mindustry_rogic_hardening::parser_test();

    return Ok(ExitCode::SUCCESS);
}
