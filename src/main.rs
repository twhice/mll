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
    let argument = if true {
        match build_args() {
            Ok(ok) => ok,
            Err(err) => {
                println!("{err}");
                return ExitCode::FAILURE;
            }
        }
    } else {
        let mut fake_argument = Argument::new();
        fake_argument.show_debug_meg = true;
        fake_argument.input_file_path =
            "/home/twhicer/code/mindustry-logic-language/src/io/unit_auto_ammo.mll".to_owned();
        fake_argument
    };
    if argument.get_help {
        println!("{}", get_buildin_meg(&Meg::Help))
    } else if argument.get_version {
        println!("{}", get_buildin_meg(&Meg::Version))
    } else {
        let inf = argument.input_file_path.clone();
        // let outf = argument.output_file_path.clone();
        unsafe {
            DEBUG = argument.show_debug_meg;
        }
        let src = match std::fs::read_to_string(argument.input_file_path.clone()) {
            Ok(src) => src,
            Err(err) => {
                println!("{}", err);
                return ExitCode::FAILURE;
            }
        };
        // let mut sentens: Vec<Vec<String>> = Vec::new();
        match complie(src, &inf) {
            Ok(result) => {
                println!("编译结束!结果如下");
                println!("====================");
                for mdt_code in result {
                    println!("{}", mdt_code)
                }
                println!("====================");
            }
            Err(err) => {
                println!("{}", err);
                return ExitCode::FAILURE;
            }
        };
    }
    return ExitCode::SUCCESS;
}
// run对ERR::EMPTY的处理 OK
// expr和def对fn_args函数错误返回的处理 OK
// ERR::UNMATCH 的转化 OK
// 表达式解析Debug
// 代码生成
