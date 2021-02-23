//! Exercise 5-2

use clap::{ Arg, App };
use common::constants::*;
use error::{ error_exit, fatal };
use std::process;
use syscall::fs::{ open_rs, close_rs };
use syscall::io::{ lseek_rs, write_rs };


fn main() {
    let matches = App::new("o_append")
        .arg(Arg::with_name("FILE")
            .required(true)
            .index(1)
            .help("File path"))
        .arg(Arg::with_name("TEXT")
            .required(true)
            .index(2)
            .help("Text appended to the file"))
        .get_matches();

    let fname = matches.value_of("FILE").unwrap();
    let text = matches.value_of("TEXT").unwrap();

    run(fname, text);

    process::exit(EXIT_SUCCESS);
}


fn run(fname: &str, text: &str) {
    let fd = match open_rs(fname, O_WRONLY | O_APPEND, None) {
        Err(_) => error_exit("open"),
        Ok(fd) => fd,
    };

    if let Err(_) = lseek_rs(fd, 0, SEEK_SET) {
        error_exit("lseek");
    }

    match write_rs(fd, text.as_bytes()) {
        Err(_) => error_exit("write"),
        Ok(num_written) => {
            if num_written < text.len() {
                fatal("partial write");
            }
        }
    }

    if let Err(_) = close_rs(fd) {
        error_exit("close");
    }
}
