//! Practice 4-2.

use clap::{ Arg, App };
use common::constants::*;
use common::data_types::*;
use error::{ error_exit, fatal };
use std::cmp;
use std::convert::TryFrom;
use std::process;
use syscall::fileio::{ open_rs, close_rs, read_rs, write_rs, lseek_rs };


const BUF_SIZE: usize = 4096;

fn main() {
    let matches = App::new("cp_hole")
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
        Err(_) => error_exit(&format!("opening file: {}", src_fname)),
    };

    let open_flags = O_WRONLY | O_CREAT | O_TRUNC;
    let file_perms = S_IRUSR | S_IWUSR | S_IRGRP | S_IROTH;
    let output_fd = match open_rs(dst_fname, open_flags, Some(file_perms)) {
        Ok(fd) => fd,
        Err(_) => error_exit(&format!("opening file: {}", dst_fname)),
    };

    let mut buf = [0; BUF_SIZE];
    let mut current = 0;

    loop {
        let start = seek(
            input_fd, current, SEEK_DATA, "lseek data input");
        seek(output_fd, start, SEEK_SET, "lseek start output");

        let end = seek(
            input_fd, start, SEEK_HOLE, "lseek hole input");
        seek(input_fd, start, SEEK_SET, "lseek start input");

        let num_bytes = usize::try_from(end - start).unwrap();
        copy(input_fd, output_fd, num_bytes, &mut buf);

        let eof = seek(input_fd, 0, SEEK_END, "lseek eof input");
        if end == eof {
            break;
        }
        seek(input_fd, end, SEEK_SET, "lseek end input");

        current = end;
    }

    if let Err(_) = close_rs(input_fd) {
        error_exit("close input");
    }
    if let Err(_) = close_rs(output_fd) {
        error_exit("close output");
    }
}


fn copy(
    input_fd: c_int,
    output_fd: c_int,
    mut num_bytes: usize,
    buf: &mut [u8],
) {
    while num_bytes > 0 {
        let buf_size = cmp::min(BUF_SIZE, num_bytes);
        let num_read = match read_rs(input_fd, &mut buf[..buf_size]) {
            Ok(num_read) => num_read,
            Err(_) => error_exit("read"),
        };

        let num_written = match write_rs(output_fd, &buf[..num_read]) {
            Ok(num_written) => num_written,
            Err(_) => error_exit("write"),
        };

        if num_written < num_read {
            fatal("partial write");
        }

        num_bytes -= num_written;
    }
}


fn seek(fd: c_int, offset: off_t, whence: c_int, err_msg: &str) -> off_t {
    match lseek_rs(fd, offset, whence) {
        Ok(new_offset) => new_offset,
        Err(_) => error_exit(err_msg),
    }
}
