//! Exercise 5-3

use clap::{ Arg, App };
use common::constants::*;
use error::{ error_exit, fatal };
use std::process;
use syscall::fs::{ open_rs, close_rs };
use syscall::io::{ lseek_rs, write_rs };


fn main() {
    let matches = App::new("atomic_append")
        .arg(Arg::with_name("file")
            .required(true)
            .index(1)
            .help("File path"))
        .arg(Arg::with_name("num_bytes")
            .required(true)
            .index(2)
            .help("Number of bytes to write"))
        .arg(Arg::with_name("x")
            .index(3)
            .help("Use seek and write instead of append"))
        .get_matches();

    let fname = matches.value_of("file").unwrap();
    let num_bytes = matches.value_of("num_bytes").unwrap();
    let append = matches.value_of("x").is_none();

    let num_bytes = match num_bytes.parse::<usize>() {
        Err(err) => fatal(
            &format!("Failed to parse '{}' as usize: {}", num_bytes, err)
        ),
        Ok(n) => n,
    };

    run(fname, num_bytes, append);

    process::exit(EXIT_SUCCESS);
}


fn run(fname: &str, num_bytes: usize, append: bool) {
    let flags =
        if append {
            O_WRONLY | O_CREAT | O_APPEND
        } else {
            O_WRONLY | O_CREAT
        };
    let perms = S_IRUSR | S_IWUSR | S_IRGRP | S_IROTH;

    let fd = match open_rs(fname, flags, Some(perms)) {
        Err(_) => error_exit("open"),
        Ok(fd) => fd,
    };

    let buf = [b'x'; 1];

    for _ in 0..num_bytes {
        if !append {
            if let Err(_) = lseek_rs(fd, 0, SEEK_END) {
                error_exit("lseek");
            }
        }
        if let Err(_) = write_rs(fd, &buf) {
            error_exit("write");
        }
    }

    if let Err(_) = close_rs(fd) {
        error_exit("close");
    }
}
