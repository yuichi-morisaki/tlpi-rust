//! Listing 5-2
//!
//! Demonstrate the use of the readv() system call to perform "gather I/O".
//!
//! (This program is merely intended to provide a code snippet for the book;
//! unless you construct a suitably formatted input file, it can't be
//! usually executed.)

use clap::{ Arg, App };
use common::constants::*;
use error::error_exit;
use std::process;
use syscall::fs::{ open_rs, close_rs };
use syscall::io::readv_rs;


fn main() {
    let matches = App::new("t_readv")
        .arg(Arg::with_name("FILE")
            .required(true)
            .index(1)
            .help("File path for input"))
        .get_matches();

    let fname = matches.value_of("FILE").unwrap();

    run(fname);

    process::exit(EXIT_SUCCESS);
}


fn run(fname: &str) {
    let fd = match open_rs(fname, O_RDONLY, None) {
        Err(_) => error_exit("open"),
        Ok(fd) => fd,
    };

    let mut buf1 = [0; 1];
    let mut buf2 = [0; 3];
    let mut buf3 = [0; 2];
    let mut buf4 = [0; 5];

    let mut buf: [&mut [u8]; 4] =
        [&mut buf1, &mut buf2, &mut buf3, &mut buf4];

    let num_read = match readv_rs(fd, &mut buf) {
        Err(_) => error_exit("readv"),
        Ok(n) => n,
    };

    println!("Total {} bytes read.", num_read);
    println!("    buf1: '{}'", std::str::from_utf8(&buf1).unwrap());
    println!("    buf2: '{}'", std::str::from_utf8(&buf2).unwrap());
    println!("    buf3: '{}'", std::str::from_utf8(&buf3).unwrap());
    println!("    buf4: '{}'", std::str::from_utf8(&buf4).unwrap());

    if let Err(_) = close_rs(fd) {
        error_exit("close");
    }
}
