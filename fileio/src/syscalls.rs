use error::error_exit;
use libc::{ mode_t, size_t, ssize_t };
use std::ffi::CString;
use std::os::raw::{ c_char, c_int, c_void };


extern "C" {
    fn open(
        path: *const c_char,
        flags: c_int,
        ... /* mode: mode_t */
    ) -> c_int;

    fn read(
        fd: c_int,
        buf: *mut c_void,
        count: size_t,
    ) -> ssize_t;

    fn write(
        fd: c_int,
        buf: *const c_void,
        count: size_t,
    ) -> ssize_t;

    fn close(fd: c_int) -> c_int;
}


pub fn open_rs(path: &str, flags: i32, mode: Option<u32>) -> Result<i32, ()> {
    let flags = flags as c_int;
    let mode = mode.map(|x| x as mode_t);

    let path = match CString::new(path) {
        Ok(path) => path.into_bytes(),
        Err(err) => {
            error_exit(
                &format!("File path is invalid for CString: {}", err)
            );
        }
    };
    let path = path.as_ptr() as *const c_char;

    let result =
        if let Some(mode) = mode {
            unsafe {
                open(path, flags, mode)
            }
        } else {
            unsafe {
                open(path, flags)
            }
        };

    if result == -1 {
        Err(())
    } else {
        let fd = result as i32;
        Ok(fd)
    }
}


pub fn read_rs(fd: i32, buf: &mut [u8]) -> Result<usize, ()> {
    let fd = fd as c_int;
    let count = buf.len() as size_t;
    let buf = buf.as_mut_ptr() as *mut c_char as *mut c_void;

    let result = unsafe {
        read(fd, buf, count)
    };

    if result == -1 {
        Err(())
    } else {
        let num_read = result as usize;
        Ok(num_read)
    }
}


pub fn write_rs(fd: i32, buf: &[u8]) -> Result<usize, ()> {
    let fd = fd as c_int;
    let count = buf.len() as size_t;
    let buf = buf.as_ptr() as *const c_char as *const c_void;

    let result = unsafe {
        write(fd, buf, count)
    };

    if result == -1 {
        Err(())
    } else {
        let num_written = result as usize;
        Ok(num_written)
    }
}


pub fn close_rs(fd: i32) -> Result<(), ()> {
    let fd = fd as c_int;

    let result = unsafe {
        close(fd)
    };

    if result == -1 {
        Err(())
    } else {
        Ok(())
    }
}
