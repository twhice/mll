/*
Copyright (C) 2022  异月(twhice)

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/
use mindustry_logic_language::*;
use std::process::ExitCode;

fn main() -> ExitCode {
    if DEBUG {
        if false {
            old_err_test();
            old_lexer_test();
            old_repl();
            old_parser_test();
        }
        let src = match std::fs::read_to_string(
            "/home/twhicer/code/mindustry-logic-language/src/io/test.mll",
        ) {
            Ok(src) => src,
            Err(err) => {
                println!("{}", err);
                return ExitCode::FAILURE;
            }
        };

        match run(src, "test.mll") {
            Ok(_) => {}
            Err(err) => {
                println!("{}", err);
                return ExitCode::FAILURE;
            }
        };
    } else {
        let argument = match build_args() {
            Ok(ok) => ok,
            Err(err) => {
                println!("{err}");
                return ExitCode::FAILURE;
            }
        };
        if argument.get_help {
            println!("{}", get_buildin_meg(&Meg::Help))
        } else if argument.get_version {
            println!("{}", get_buildin_meg(&Meg::Version))
        } else {
            let inf = argument.input_file_path.clone();
            // let outf = argument.output_file_path.clone();
            let src = match std::fs::read_to_string(argument.input_file_path.clone()) {
                Ok(src) => src,
                Err(err) => {
                    println!("{}", err);
                    return ExitCode::FAILURE;
                }
            };
            // let mut sentens: Vec<Vec<String>> = Vec::new();
            match run(src, &inf) {
                Ok(_) => todo!(),
                Err(err) => {
                    println!("{}", err);
                    return ExitCode::FAILURE;
                }
            };
        }
    }

    return ExitCode::SUCCESS;
}
