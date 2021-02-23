//! Supplementary program for chapter 5
//!
//! Demonstrate the use of the truncate() system call to truncate the file
//! named in argv[1] to the length specified in argv[2]

use clap::{ Arg, App };
use common::constants::*;
use error::{ error_exit, fatal };
use std::process;
use syscall::fs::truncate_rs;


fn main() {
    let matches = App::new("t_truncate")
        .arg(Arg::with_name("FILE")
            .required(true)
            .index(1)
            .help("File path to be truncated"))
        .arg(Arg::with_name("SIZE")
            .required(true)
            .index(2)
            .help("New file size"))
        .get_matches();

    let fname = matches.value_of("FILE").unwrap();
    let new_size = matches.value_of("SIZE").unwrap();
    let new_size = match new_size.parse::<usize>() {
        Err(err) => fatal(
            &format!("Failed to parse '{}' as usize: {}", new_size, err)
        ),
        Ok(n) => n,
    };


    if let Err(_) = truncate_rs(fname, new_size) {
        error_exit("truncate");
    }


    process::exit(EXIT_SUCCESS);
}
