//! Exercise 4-1

use clap::{ Arg, App };
use common::constants::*;
use error::{ error_exit, fatal };
use std::process;
use syscall::fs::{ open_rs, close_rs };
use syscall::io::{ read_rs, write_rs };

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

        let num_written = match write_rs(STDOUT_FILENO, &buf[..num_read]) {
            Ok(num_bytes) => num_bytes,
            Err(_) => error_exit("write to stdout"),
        };
        if num_written < num_read {
            fatal("partial write to stdout");
        }

        let num_written = match write_rs(fd, &buf[..num_read]) {
            Ok(num_bytes) => num_bytes,
            Err(_) => error_exit("write to file"),
        };
        if num_written < num_read {
            fatal("partial write to file");
        }
    }

    if let Err(_) = close_rs(fd) {
        error_exit("close");
    }
}
