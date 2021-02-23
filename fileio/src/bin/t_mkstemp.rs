use common::data_types::*;
use error::{ error_exit, fatal };
use syscall::fs::{ close_rs, mkstemp_rs };


fn main() {
    let mut template = match CString::new("/tmp/somestringXXXXXX") {
        Err(err) => fatal(
            &format!("Failed to create CString: {}", err)
        ),
        Ok(cstring) => cstring.into_bytes_with_nul(),
    };

    let fd = match mkstemp_rs(&mut template) {
        Err(_) => error_exit("mkstemp"),
        Ok(fd) => fd,
    };

    let temp_fname = match std::str::from_utf8(&template) {
        Err(err) => fatal(
            &format!("Failed to create &str: {}", err)
        ),
        Ok(s) => s,
    };

    println!("Generated filename was: {}", temp_fname);

    // unlink(template);    // TODO

    // Use file I?O system calls - read(), write(), and so on.

    if let Err(_) = close_rs(fd) {
        error_exit("close");
    }
}
