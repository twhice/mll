use crate::{error::Err, Argument};

pub enum Meg {
    Help,
    Version,
}
pub fn build_args() -> Result<Argument, Err> {
    let mut args = std::env::args().collect::<Vec<String>>();
    let p_args = &mut args as *mut Vec<String>;
    let mut ret = Argument::new();
    let p_ret = &mut ret as *mut Argument;
    // 去除可执行文件名
    args.remove(0);
    if args.len() == 0 {
        return Err(Err::IoNoArg);
    }
    let mut try_lex = || -> Result<(), Err> {
        let mut args = unsafe { (*p_args).clone() };
        while args.len() != 0 {
            let arg = args[0].clone();
            args.remove(0);

            let match_text = |arg: &String, text: &str| -> bool { arg == text };
            if match_text(&arg, "-v") {
                ret.get_version = true;
                if args.len() == 0 {
                    break;
                } else {
                    return Err(Err::IoTooMuchArg);
                }
            } else if match_text(&arg, "-h") {
                ret.get_help = true;
                if args.len() == 0 {
                    break;
                } else {
                    return Err(Err::IoTooMuchArg);
                }
            } else if match_text(&arg, "-f") {
                if args.len() != 0 {
                    ret.input_file_path = args[0].clone();
                    args.remove(0);
                } else {
                    return Err(Err::IoMissArg);
                }
            } else if match_text(&arg, "-o") {
                if args.len() != 0 {
                    ret.output_file_path = args[0].clone();
                    args.remove(0);
                } else {
                    return Err(Err::IoMissArg);
                }
            } else if match_text(&arg, "-d") {
                ret.show_debug_meg = true;
            } else if match_text(&arg, "-w") {
                ret.show_warn_meg = true;
            } else if match_text(&arg, "-p") {
                ret.print_to_stdout = true;
            } else {
                return Err(Err::IoUnknowArg);
            }
        }
        return Ok(());
    };
    return match try_lex() {
        Ok(_) => Ok(ret),
        Err(_) => {
            unsafe {
                (*p_ret) = Argument::new();
                (*p_ret).input_file_path = args[0].clone();
            }
            args.remove(0);
            match try_lex() {
                Ok(_) => return Ok(ret),
                Err(err) => return Err(err),
            }
        }
    };
}
