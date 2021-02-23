use clap::{ Arg, App };
use common::constants::*;
use error::error_exit;
use std::process;
use syscall::fileio::{ open_rs, close_rs, fcntl_rs, FcntlCmd };


fn main() {
    let matches = App::new("t_fcntl")
        .arg(Arg::with_name("FILE")
            .required(true)
            .index(1)
            .help("File path"))
        .get_matches();

    let fname = matches.value_of("FILE").unwrap();

    run(fname);

    process::exit(EXIT_SUCCESS);
}


fn run(fname: &str) {
    let fd = match open_rs(fname, O_RDWR, None) {
        Ok(fd) => fd,
        Err(_) => error_exit("open"),
    };


    let mut flags = match fcntl_rs(fd, FcntlCmd::GetFl) {
        Err(_) => error_exit("fcntl get flags"),
        Ok(flags) => flags,
    };

    // Check file open flags
    if (flags & O_NOATIME) > 0 {
        println!("This file is set not to update the last access time");
    } else {
        println!("This file is set to update the last access time");
    }


    // Check file access permissions
    let mode = flags & O_ACCMODE;
    if mode == O_RDONLY {
        println!("file is readable");
    } else if mode == O_WRONLY {
        println!("file is writable");
    } else if mode == O_RDWR {
        println!("file is readable and writable");
    }


    // Change and set file open flags
    flags |= O_NOATIME;
    if let Err(_) = fcntl_rs(fd, FcntlCmd::SetFl(flags)) {
        error_exit("fcntl set flags");
    } else {
        println!("Set O_NOATIME flag");
    }

    // And, confirm the changed flags
    let flags = match fcntl_rs(fd, FcntlCmd::GetFl) {
        Err(_) => error_exit("fcntl get flags"),
        Ok(flags) => flags,
    };
    if (flags & O_NOATIME) > 0 {
        println!("This file is set not to update the last access time");
    } else {
        println!("This file is set to update the last access time");
    }


    if let Err(_) = close_rs(fd) {
        error_exit("close");
    }
}
