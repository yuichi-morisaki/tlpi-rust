//! Listing 4-1
//!
//! Copy the file named argv[1] to a new file named in argv[2].

use clap::{ Arg, App };
use common::constants::*;
use error::{ error_exit, fatal };
use fileio::{ open_rs, read_rs, write_rs, close_rs };
use std::process;


const BUF_SIZE: usize = 1024;

fn main() {
    let matches = App::new("copy")
        .arg(Arg::with_name("SOURCE")
            .required(true)
            .index(1)
            .help("File path of source"))
        .arg(Arg::with_name("DESTINATION")
            .required(true)
            .index(2)
            .help("File path of destination"))
        .get_matches();

    let src_fname = matches.value_of("SOURCE").unwrap();
    let dst_fname = matches.value_of("DESTINATION").unwrap();

    run(src_fname, dst_fname);

    process::exit(EXIT_SUCCESS);
}


fn run(src_fname: &str, dst_fname: &str) {
    let input_fd = match open_rs(src_fname, O_RDONLY, None) {
        Ok(fd) => fd,
        Err(_) => error_exit(&format!("opening file {}", src_fname)),
    };

    let open_flags = O_CREAT | O_WRONLY | O_TRUNC;
    let file_perms =
        S_IRUSR | S_IWUSR | S_IRGRP | S_IWGRP | S_IROTH | S_IWOTH;
    let output_fd = match open_rs(dst_fname, open_flags, Some(file_perms)) {
        Ok(fd) => fd,
        Err(_) => error_exit(&format!("opening file {}", dst_fname)),
    };

    let mut buf = [0; BUF_SIZE];

    loop {
        let num_read = match read_rs(input_fd, &mut buf) {
            Ok(num_bytes) => num_bytes,
            Err(_) => error_exit("read"),
        };
        if num_read == 0 {
            break;
        }

        let num_written = match write_rs(output_fd, &buf[..num_read]) {
            Ok(num_bytes) => num_bytes,
            Err(_) => error_exit("write"),
        };

        if num_written < num_read {
            fatal("partial write");
        }
    }

    if let Err(_) = close_rs(input_fd) {
        error_exit("close input");
    }
    if let Err(_) = close_rs(output_fd) {
        error_exit("close output");
    }
}
