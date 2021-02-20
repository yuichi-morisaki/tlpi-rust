use libc::EXIT_FAILURE;

use std::convert::TryFrom;
use std::ffi::CString;
use std::os::raw::{ c_char, c_int, c_uint };
use std::process;


extern {
    fn error_text(
        buf: *mut c_char,
        buf_size: c_uint,
        err_num: c_int
    ) -> c_int;
}


const BUF_SIZE: usize = 500;

pub fn error_text_rs(err_num: Option<i32>) -> Option<String> {
    let mut buf = vec![0; BUF_SIZE];
    let buf_size = match u32::try_from(BUF_SIZE) {
        Ok(size) => size,
        Err(err) => {
            eprintln!("Failed to convert usize to u32: {}", err);
            process::exit(EXIT_FAILURE);
        }
    };
    let err_num = err_num.unwrap_or(0);

    let result = unsafe {
        error_text(
            buf.as_mut_ptr() as *mut c_char,
            buf_size as c_uint,
            err_num as c_int,
        )
    };

    if result == -1 {
        eprintln!("Failed to get error text");
        process::exit(EXIT_FAILURE);
    }

    let num_written = match usize::try_from(result as i32) {
        Ok(num) => num,
        Err(err) => {
            eprintln!("Failed to convert i32 to usize: {}", err);
            process::exit(EXIT_FAILURE);
        }
    };

    if num_written == BUF_SIZE {
        eprintln!("Buffer size is too small for error text");
        process::exit(EXIT_FAILURE);
    } else if num_written == 0 {
        None
    } else {
        buf.truncate(num_written);
        let err_text = match CString::new(buf) {
            Ok(err_text) => err_text,
            Err(err) => {
                eprintln!("Failed to create CString: {}", err);
                process::exit(EXIT_FAILURE);
            }
        };
        let err_text = match err_text.into_string() {
            Ok(err_text) => err_text,
            Err(err) => {
                eprintln!("Failed to convert CString into String: {}", err);
                process::exit(EXIT_FAILURE);
            }
        };

        Some(err_text)
    }
}


#[cfg(test)]
mod tests {
    use super::error_text_rs as get_error_text;
    use std::ffi::CString;
    use std::os::raw::{ c_char, c_int };
    use libc::O_RDONLY;

    extern "C" {
        fn open(path: *const c_char, flags: c_int) -> c_int;
    }

    #[test]
    fn error_text_with_errno() {
        let fname = CString::new("not-exist-filename").unwrap();
        let fd = unsafe {
            open(fname.as_c_str().as_ptr(), O_RDONLY)
        };
        assert_eq!(fd, -1);

        let err_text = get_error_text(None).unwrap();
        println!("{}", err_text);
    }

    #[test]
    fn error_text_with() {
        let err_num = Some(2);
        let err_text = get_error_text(err_num).unwrap();
        println!("{}", err_text);
    }
}
