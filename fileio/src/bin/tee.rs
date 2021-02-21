//! Practice 4-1.

use clap::{ Arg, App };
use common::constants::*;
use error::error_exit;
use fileio::{ open_rs, read_rs, write_rs, close_rs };
use std::process;

const BUF_SIZE: usize = 1024;


fn main() {
    let matches = App::new("tee")
        .arg(Arg::with_name("append")
            .short("a")
            .help("Set append flag on output file"))
        .arg(Arg::with_name("FILE")
            .required(true)
            .index(1)
            .help("Output file path"))
        .get_matches();

    let append = matches.is_present("append");
    let fname = matches.value_of("FILE").unwrap();

    run(fname, append);

    process::exit(EXIT_SUCCESS);
}


fn run(fname: &str, append: bool) {
    let open_flags =
        if append {
            O_WRONLY | O_CREAT | O_APPEND
        } else {
            O_WRONLY | O_CREAT | O_TRUNC
        };
    let file_perms = S_IRUSR | S_IWUSR | S_IRGRP | S_IROTH;

    let fd = match open_rs(fname, open_flags, Some(file_perms)) {
        Ok(fd) => fd,
        Err(_) => error_exit(&format!("opening file {}", fname)),
    };

    let mut buf = [0; BUF_SIZE];

    loop {
        let num_read = match read_rs(STDIN_FILENO, &mut buf) {
            Ok(num_bytes) => num_bytes,
            Err(_) => error_exit("read"),
        };
        if num_read == 0 {
            break;
        }

        match write_rs(STDOUT_FILENO, &buf[..num_read]) {
            Ok(num_bytes) => {
                if num_bytes < num_read {
                    error_exit("partial write to stdout");
                }
            }
            Err(_) => error_exit("write to stdout"),
        }

        match write_rs(fd, &buf[..num_read]) {
            Ok(num_bytes) => {
                if num_bytes < num_read {
                    error_exit("partial write to file");
                }
            }
            Err(_) => error_exit("write to file"),
        }
    }

    if let Err(_) = close_rs(fd) {
        error_exit("close");
    }
}
