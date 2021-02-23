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

use clap::{ Arg, App, AppSettings, Values };
use common::constants::*;
use common::data_types::*;
use error::{ cmdline_error, error_exit, fatal };
use std::fmt;
use std::process;
use std::str::FromStr;
use syscall::fileio::{ open_rs, close_rs, read_rs, write_rs, lseek_rs };


fn main() {
    let matches = App::new("seek_io")
        .setting(AppSettings::TrailingVarArg)
        .arg(Arg::with_name("FILE")
            .required(true)
            .index(1)
            .help("File path"))
        .arg(Arg::with_name("OPERATION")
            .multiple(true)
            .required(true)
            .help("Operation: [r<length>|R<length>|w<string>|s<offset>]"))
        .get_matches();

    let fname = matches.value_of("FILE").unwrap();
    let operations = matches.values_of("OPERATION").unwrap();

    run(fname, operations);

    process::exit(EXIT_SUCCESS);
}


fn run(fname: &str, operations: Values) {
    let open_flags = O_RDWR | O_CREAT;
    let file_perms =
        S_IRUSR | S_IWUSR | S_IRGRP | S_IWGRP | S_IROTH | S_IWOTH;
    let fd = match open_rs(fname, open_flags, Some(file_perms)) {
        Ok(fd) => fd,
        Err(_) => error_exit("open"),
    };

    for op in operations {
        let kind = op.as_bytes()[0] as char;
        let value = std::str::from_utf8(&op.as_bytes()[1..]).unwrap();

        match kind {
            'r' |    // Display bytes at current offset, as text
            'R' => { // Display bytes at current offset, as hex
                let len: usize = parse(value);
                let mut buf = vec![0; len];
                let num_read = match read_rs(fd, &mut buf) {
                    Ok(n) => n,
                    Err(_) => error_exit("read"),
                };
                if num_read == 0 {
                    println!("{}: end-of-file", op);
                } else {
                    buf.truncate(num_read);
                    if kind == 'r' {
                        println!("{}: {}", op, read_as_text(&buf));
                    } else {
                        println!("{}: {}", op, read_as_hex(&buf));
                    }
                }
            }

            'w' => { // Write string as current offset
                match write_rs(fd, value.as_bytes()) {
                    Ok(num_written) =>
                        println!("{}: wrote {} bytes", op, num_written),
                    Err(_) => error_exit("write"),
                }
            }

            's' => { // Change file offset
                let offset: off_t = parse(value);
                match lseek_rs(fd, offset, SEEK_SET) {
                    Ok(_) => println!("{}: seek succeeded", op),
                    Err(_) => error_exit("lseek"),
                };
            }

            _ => cmdline_error(
                &format!("Argument must start with [rRws]: {}", op)
            ),
        }
    }

    if let Err(_) = close_rs(fd) {
        error_exit("close");
    }
}


fn parse<T: FromStr>(value: &str) -> T
where
    T::Err: fmt::Display,
{
    match value.parse::<T>() {
        Ok(n) => n,
        Err(err) => fatal(
            &format!("Failed to parse `{}`: {}", value, err)
        ),
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
