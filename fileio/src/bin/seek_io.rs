//! Listing 4-3
//!
//! Demonstrate the use of lseek() and file I/O system calls.
//!
//! Usage: seek_io file {r<length>|R<length>|w<string>|s<offset>}...
//!
//! This program opens the file named on its command line, and then performs
//! the file I/O operations specified by its remaining command-line arguments:
//!
//!     r<length>   Read `length` bytes from the file at current file offset,
//!                 displaying them as text.
//!
//!     R<length>   Read `length` bytes from the file at current file offset,
//!                 displaying them as hex.
//!
//!     w<string>   Write `string` at current file offset.
//!
//!     s<offset>   Set the file offset to `offset`.
//!
//! Example:
//!
//!     seek_io myfile wxyz s1 r2

use common::constants::*;
use common::data_types::*;
use error::{ cmdline_error, error_exit, fatal, usage_error };
use fileio::{ open_rs, read_rs, write_rs, lseek_rs, close_rs };
use std::env;
use std::fmt;
use std::process;
use std::str::FromStr;


fn main() {
    let argv: Vec<_> = env::args().collect();
    let argc = argv.len();

    if argc < 3 || argv[1] == "--help" {
        usage_error(&format!(
            "{} file {{r<length>|R<length>|w<string>|s<offset>}}...",
            &argv[0]
        ));
    }

    let open_flags = O_RDWR | O_CREAT;
    let file_perms =
        S_IRUSR | S_IWUSR | S_IRGRP | S_IWGRP | S_IROTH | S_IWOTH;
    let fd = match open_rs(&argv[1], open_flags, Some(file_perms)) {
        Ok(fd) => fd,
        Err(_) => error_exit("open"),
    };

    for ap in 2..argc {
        match argv[ap].as_bytes()[0] {
            b'r' |    // Display bytes at current offset, as text
            b'R' => { // Display bytes at current offset, as hex
                let len: usize = parse(&argv[ap].as_bytes()[1..]);
                let mut buf = vec![0; len];
                let num_read = match read_rs(fd, &mut buf) {
                    Ok(n) => n,
                    Err(_) => error_exit("read"),
                };
                if num_read == 0 {
                    println!("{}: end-of-file", &argv[ap]);
                } else {
                    buf.truncate(num_read);
                    if argv[ap].as_bytes()[0] == b'r' {
                        println!("{}: {}", &argv[ap], read_as_text(&buf));
                    } else {
                        println!("{}: {}", &argv[ap], read_as_hex(&buf));
                    }
                }
            }

            b'w' => { // Write string at current offset
                let arg = &argv[ap].as_bytes()[1..];
                match write_rs(fd, arg) {
                    Ok(num_written) => println!(
                        "{}: wrote {} bytes", &argv[ap], num_written
                    ),
                    Err(_) => error_exit("write"),
                };
            }

            b's' => { // Change file offset
                let offset: off_t = parse(&argv[ap].as_bytes()[1..]);
                match lseek_rs(fd, offset, SEEK_SET) {
                    Ok(_) => println!(
                        "{}: seek succeeded", &argv[ap]
                    ),
                    Err(_) => error_exit("lseek"),
                };
            }

            _ => cmdline_error(&format!(
                "Argument must start with [rRws]: {}", &argv[ap]
            )),
        }
    }

    if let Err(_) = close_rs(fd) {
        error_exit("close");
    }

    process::exit(EXIT_SUCCESS);
}


fn parse<T: FromStr>(arg: &[u8]) -> T
where
    T::Err: fmt::Display,
{
    let arg = std::str::from_utf8(arg)
        .expect("Failed to make &str as utf-8");
    match arg.parse::<T>() {
        Ok(n) => n,
        Err(err) => fatal(&format!("Failed to parse '{}': {}", arg, err)),
    }
}


fn read_as_text(buf: &[u8]) -> &str {
    match std::str::from_utf8(buf) {
        Ok(s) => s,
        Err(err) => fatal(&format!("Failed to make &str as utf8: {}", err)),
    }
}


fn read_as_hex(buf: &[u8]) -> String {
    buf.iter()
        .map(|x| format!("{:02x}", x))
        .collect::<Vec<_>>()
        .join(" ")
}
