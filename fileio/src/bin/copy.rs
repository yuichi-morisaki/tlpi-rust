//! Listing 4-1
//!
//! Copy the file named argv[1] to a new file named in argv[2].

use common::constants::*;
use error::{ error_exit, usage_error };
use fileio::{ open_rs, read_rs, write_rs, close_rs };
use std::env;
use std::process;


const BUF_SIZE: usize = 1024;

fn main() {
    let argv: Vec<_> = env::args().collect();
    let argc = argv.len();

    if argc != 3 || argv[1] == "--help" {
        usage_error(
            &format!("{} old-file new-file", &argv[0])
        );
    }

    let input_fd = match open_rs(&argv[1], O_RDONLY, None) {
        Ok(fd) => fd,
        Err(_) => error_exit(&format!("opening file {}", &argv[1])),
    };

    let open_flags = O_CREAT | O_WRONLY | O_TRUNC;
    let file_perms =
        S_IRUSR | S_IWUSR | S_IRGRP | S_IWGRP | S_IROTH | S_IWOTH;
    let output_fd = match open_rs(&argv[2], open_flags, Some(file_perms)) {
        Ok(fd) => fd,
        Err(_) => error_exit(&format!("opening file {}", &argv[2])),
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

        match write_rs(output_fd, &buf[..num_read]) {
            Ok(num_bytes) => {
                if num_bytes < num_read {
                    error_exit("partial write");
                }
            }
            Err(_) => error_exit("write"),
        }
    }

    if let Err(_) = close_rs(input_fd) {
        error_exit("close input");
    }
    if let Err(_) = close_rs(output_fd) {
        error_exit("close output");
    }

    process::exit(EXIT_SUCCESS);
}
