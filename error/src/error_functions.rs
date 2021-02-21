use common::constants::*;
use common::data_types::*;
use std::env;
use std::io::{ self, Write };
use std::process;

use crate::get_error_text;


pub fn error_msg(msg: &str) {
    output_error(true, None, true, msg);
}


pub fn error_exit(msg: &str) -> ! {
    output_error(true, None, true, msg);
    terminate(true);
}


pub fn error_exit2(msg: &str) -> ! {
    output_error(true, None, false, msg);
    terminate(false);
}


pub fn error_exit_en(err_num: c_int, msg: &str) -> ! {
    output_error(true, Some(err_num), true, msg);
    terminate(true);
}


pub fn fatal(msg: &str) -> ! {
    output_error(false, None, true, msg);
    terminate(true);
}


pub fn usage_error(msg: &str) -> ! {
    if let Err(err) = io::stdout().flush() {
        eprintln!("Failed to flush pending stdout: {}", err);
    }
    eprintln!("Usage: {}", msg);
    process::exit(EXIT_FAILURE);
}


pub fn cmdline_error(msg: &str) -> ! {
    if let Err(err) = io::stdout().flush() {
        eprintln!("Failed to flush pending stdout: {}", err);
    }
    eprintln!("Command-line usage error: {}", msg);
    process::exit(EXIT_FAILURE);
}


fn output_error(
    use_err_text: bool,
    err_num: Option<c_int>,
    flush_stdout: bool,
    user_msg: &str,
) {
    let default_err_text = String::from(":");
    let err_text =
        if use_err_text {
            get_error_text(err_num).unwrap_or(default_err_text)
        } else {
            default_err_text
        };

    if flush_stdout {
        if let Err(err) = io::stdout().flush() {
            eprintln!("Failed to flush pending stdout: {}", err);
        }
    }

    eprintln!("ERROR{} {}", err_text, user_msg);
}


fn terminate(use_exit3: bool) -> ! {
    if let Ok(value) = env::var("EF_DUMPCORE") {
        if value.len() > 0 {
            process::abort();
        }
    }

    if use_exit3 {
        process::exit(EXIT_FAILURE);
    } else {
        unimplemented!("_exit(2) or _Exit");
    }
}


#[cfg(test)]
mod tests {
    use super::error_msg;
    use common::data_types::*;

    extern {
        fn set_errno(err_num: c_int);
    }

    #[test]
    fn demo_error_msg() {
        unsafe {
            set_errno(1);
        }
        error_msg("This is demo of `error_msg`");
        println!("Hello, world!");
    }
}
