//! Listing 4-2

use common::constants::*;
use error::error_exit;
use fileio::open_rs;

fn main() {
    // Open existing file for reading
    let _fd = match open_rs("startup", O_RDONLY, None) {
        Ok(fd) => fd,
        Err(_) => error_exit("open"),
    };

    // Open new or existing file for reading and writing, truncating to zero
    // bytes; file permissions read+write for owner, nothing for all others.
    let _fd =
        match open_rs(
            "myfile",
            O_RDWR | O_CREAT | O_TRUNC,
            Some(S_IRUSR | S_IWUSR)
        ) {
            Ok(fd) => fd,
            Err(_) => error_exit("open"),
        };

    // Open new or existing file for writing; writes should always append to
    // end of file.
    let _fd =
        match open_rs(
            "w.log",
            O_WRONLY | O_CREAT | O_APPEND,
            Some(S_IRUSR | S_IWUSR)
        ) {
            Ok(fd) => fd,
            Err(_) => error_exit("open"),
        };
}
