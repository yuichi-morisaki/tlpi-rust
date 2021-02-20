use error::{ error_exit_en, usage_error };
use libc::EXIT_FAILURE;
use std::env;
use std::process;

fn main() {
    let argv: Vec<_> = env::args().collect();
    let argc = argv.len();

    if argc != 2 || argv[1] == "--help" {
        usage_error(&format!("{} errnum", &argv[0]));
    }

    let err_num = match argv[1].parse::<i32>() {
        Ok(err_num) => err_num,
        Err(err) => {
            eprintln!("Failed to parse arg[1] as i32: {}", err);
            process::exit(EXIT_FAILURE);
        }
    };

    error_exit_en(err_num, "This is demo of `error_exit_en`")
}
