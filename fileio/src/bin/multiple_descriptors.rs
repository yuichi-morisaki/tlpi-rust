//! Exercise 5-6

use clap::{ Arg, App };
use common::constants::*;
use error::error_exit;
use std::io;
use std::io::Write;
use std::process;
use std::process::Command;
use syscall::fs::{ open_rs, close_rs, dup_rs };
use syscall::io::{ lseek_rs, write_rs };


fn main() {
    let matches = App::new("multiple_descriptors")
        .arg(Arg::with_name("file")
            .required(true)
            .index(1)
            .help("File path"))
        .get_matches();

    let fname = matches.value_of("file").unwrap();

    run(fname);

    process::exit(EXIT_SUCCESS);
}


fn run(fname: &str) {
    let flags = O_RDWR | O_CREAT | O_TRUNC;
    let perms = S_IRUSR | S_IWUSR;

    let fd1 = match open_rs(fname, flags, Some(perms)) {
        Err(_) => error_exit("open"),
        Ok(fd) => fd,
    };
    let fd2 = match dup_rs(fd1) {
        Err(_) => error_exit("dup"),
        Ok(fd) => fd,
    };
    let fd3 = match open_rs(fname, O_RDWR, None) {
        Err(_) => error_exit("open"),
        Ok(fd) => fd,
    };


    if let Err(_) = write_rs(fd1, "Hello,".as_bytes()) {
        error_exit("write");
    } else {
        cat(fname);
    }

    if let Err(_) = write_rs(fd2, " world".as_bytes()) {
        error_exit("write");
    } else {
        cat(fname);
    }

    if let Err(_) = lseek_rs(fd2, 0, SEEK_SET) {
        error_exit("lseek");
    }
    if let Err(_) = write_rs(fd1, "HELLO,".as_bytes()) {
        error_exit("write");
    } else {
        cat(fname);
    }

    if let Err(_) = write_rs(fd3, "Gidday".as_bytes()) {
        error_exit("write");
    } else {
        cat(fname);
    }


    if let Err(_) = close_rs(fd1) {
        error_exit("close");
    }
    if let Err(_) = close_rs(fd2) {
        error_exit("close");
    }
    if let Err(_) = close_rs(fd3) {
        error_exit("close");
    }
}


fn cat(fname: &str) {
    let output = Command::new("cat").arg(fname).output().unwrap();
    io::stdout().write_all(&output.stdout).unwrap();
    io::stderr().write_all(&output.stderr).unwrap();
    io::stdout().write_all(b"\n").unwrap();
}
