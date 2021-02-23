//! Listing 5-1
//!
//! The following code shows why we need the open() O_EXCL flag.
//!
//! This program to tries ensure that it is the one that creates the file
//! named in its command-line argument. It does this by trying to open()
//! the filename once without the O_CREAT flag (if this open() succeeds
//! then the program know it is not the creator of the file), and if
//! that open() fails, it calls open() a second time, with the O_CREAT flag.
//!
//! If the first open() fails, the program assumes that it is the creator
//! of the file. However this may not be true: some other process may have
//! created the file between the two calls to open().

use clap::{ Arg, App };
use common::constants::*;
use error::{ error_exit, get_errno_in_c };
use std::process;
use std::thread;
use std::time::Duration;
use syscall::fileio::{ open_rs, close_rs };
use syscall::process::getpid_rs;


fn main() {
    let matches = App::new("bad_exclusive_open")
        .arg(Arg::with_name("sleep")
            .short("s")
            .long("sleep")
            .help("Set sleep on. Default is off"))
        .arg(Arg::with_name("FILE")
            .required(true)
            .index(1)
            .help("File path to create exclusively"))
        .get_matches();

    let sleep = matches.is_present("sleep");
    let fname = matches.value_of("FILE").unwrap();

    run(fname, sleep);

    process::exit(EXIT_SUCCESS);
}


fn run(fname: &str, sleep: bool) {
    if let Ok(fd) = open_rs(fname, O_WRONLY, None) {
        println!("[PID {}] File {} already exists", getpid_rs(), fname);
        if let Err(_) = close_rs(fd) {
            error_exit("close");
        }
    } else {
        let errno = get_errno_in_c();
        if errno != ENOENT {    // Failed for unexpected reason
            error_exit("open");
        } else {
            println!(
                "[PID {}] File {} doesn't exist yet", getpid_rs(), fname);

            if sleep {
                thread::sleep(Duration::from_secs(5));
                println!("[PID {}] Done sleeping", getpid_rs())
            }

            let flags = O_WRONLY | O_CREAT;
            let perms = S_IRUSR | S_IWUSR;
            let fd = match open_rs(fname, flags, Some(perms)) {
                Ok(fd) => fd,
                Err(_) => error_exit("open"),
            };
            println!(
                "[PID {}] Created file {} exclusively", getpid_rs(), fname);

            if let Err(_) = close_rs(fd) {
                error_exit("close");
            }
        }
    }
}
