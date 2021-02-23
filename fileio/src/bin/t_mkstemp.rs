use common::constants::*;
use common::data_types::*;
use error::{ error_exit, fatal };
use std::process;
use syscall::fs::{ close_rs, mkstemp_rs, unlink_rs };


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

    let tmp_fname = match CStr::from_bytes_with_nul(&template) {
        Err(err) => fatal(
            &format!("Failed to restore CStr from &[u8]: {}", err)
        ),
        Ok(cstr) => match cstr.to_str() {
            Err(err) => fatal(
                &format!("Failed to convert CStr into str: {}", err)
            ),
            Ok(s) => s,
        }
    };

    println!("Generated filename was: {}", tmp_fname);

    // Name disappears immediately, but the file is removed after close()
    if let Err(_) = unlink_rs(tmp_fname) {
        error_exit("unlink");
    }


    // Use file I?O system calls - read(), write(), and so on.


    if let Err(_) = close_rs(fd) {
        error_exit("close");
    }

    process::exit(EXIT_SUCCESS);
}
