//! Exercise 5-5

use clap::{ Arg, App };
use common::constants::*;
use common::data_types::*;
use error::error_exit;
use fileio::{ dup, dup2 };  // Exercise 5-4
use std::process;
use syscall::fs::{ open_rs, close_rs, fcntl_rs, FcntlCmd };
use syscall::io::lseek_rs;


fn main() {
    let matches = App::new("shared_ofd")
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
    let fd1 = match open_rs(fname, O_RDONLY, None) {
        Err(_) => error_exit("open"),
        Ok(fd) => fd,
    };

    let fd2 = dup(fd1);
    if fd2 == -1 {
        error_exit("dup");
    }

    println!("---- dup: offset ----");
    confirm_offset_shared(fd1, fd2);
    println!("---- dup: flags ----");
    confirm_flags_shared(fd1, fd2);

    let fd2 = dup2(fd1, fd2);
    if fd2 == -1 {
        error_exit("dup2");
    }

    println!("---- dup2: offset ----");
    confirm_offset_shared(fd1, fd2);
    println!("---- dup2: flags ----");
    confirm_flags_shared(fd1, fd2);

    if let Err(_) = close_rs(fd1) {
        error_exit("close fd1");
    }
    if let Err(_) = close_rs(fd2) {
        error_exit("close fd2");
    }
}


fn confirm_offset_shared(fd1: c_int, fd2: c_int) {
    // Can `fd1` detect that `fd2` moved the offset?
    match lseek_rs(fd1, 0, SEEK_CUR) {
        Err(_) => error_exit("lseek"),
        Ok(offset) => println!("[fd1] offset: {}", offset),
    }
    match lseek_rs(fd2, 0, SEEK_END) {
        Err(_) => error_exit("lseek"),
        Ok(offset) => println!("[fd2] seek end - offset: {}", offset),
    }
    match lseek_rs(fd1, 0, SEEK_CUR) {
        Err(_) => error_exit("lseek"),
        Ok(offset) => println!("[fd1] offset: {}", offset),
    }

    // Can `fd2` detect that `fd1` moved the offset?
    match lseek_rs(fd1, 0, SEEK_SET) {
        Err(_) => error_exit("lseek"),
        Ok(offset) => println!("[fd1] seek top - offset: {}", offset),
    }
    match lseek_rs(fd2, 0, SEEK_CUR) {
        Err(_) => error_exit("lseek"),
        Ok(offset) => println!("[fd2] offset: {}", offset),
    }
}


fn confirm_flags_shared(fd1: c_int, fd2: c_int) {
    // Can `fd1` detect that `fd2` changed a flag?
    match fcntl_rs(fd1, FcntlCmd::GetFl) {
        Err(_) => error_exit("fcntl"),
        Ok(flags) => {
            if (flags & O_NOATIME) == 0 {
                println!("[fd1] flag O_NOATIME is off");
            }
        }
    }
    let mut flags = match fcntl_rs(fd2, FcntlCmd::GetFl) {
        Err(_) => error_exit("fcntl"),
        Ok(flags) => flags,
    };
    flags |= O_NOATIME;
    if let Err(_) = fcntl_rs(fd2, FcntlCmd::SetFl(flags)) {
        error_exit("fcntl");
    } else {
        println!("[fd2] set flag O_NOATIME on");
    }
    match fcntl_rs(fd1, FcntlCmd::GetFl) {
        Err(_) => error_exit("fcntl"),
        Ok(flags) => {
            if (flags & O_NOATIME) > 0 {
                println!("[fd1] flag O_NOATIME is on");
            }
        }
    }

    // Can `fd2` detect that `fd1` changed a flag?
    match fcntl_rs(fd2, FcntlCmd::GetFl) {
        Err(_) => error_exit("fcntl"),
        Ok(flags) => {
            if (flags & O_NOATIME) > 0 {
                println!("[fd2] flag O_NOATIME is on");
            }
        }
    }
    let mut flags = match fcntl_rs(fd2, FcntlCmd::GetFl) {
        Err(_) => error_exit("fcntl"),
        Ok(flags) => flags,
    };
    flags &= !O_NOATIME;
    if let Err(_) = fcntl_rs(fd1, FcntlCmd::SetFl(flags)) {
        error_exit("fcntl");
    } else {
        println!("[fd1] set flag O_NOATIME off");
    }
    match fcntl_rs(fd2, FcntlCmd::GetFl) {
        Err(_) => error_exit("fcntl"),
        Ok(flags) => {
            if (flags & O_NOATIME) == 0 {
                println!("[fd2] flag O_NOATIME is off");
            }
        }
    }
}
