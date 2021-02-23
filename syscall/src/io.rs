use common::data_types::*;
use error::fatal;
use std::convert::TryFrom;

type Result<T> = std::result::Result<T, ()>;


extern "C" {
    fn read(
        fd: c_int,
        buf: *mut c_void,
        count: size_t,
    ) -> ssize_t;
}

pub fn read_rs(fd: c_int, buf: &mut [u8]) -> Result<usize> {
    let count = buf.len() as size_t;
    let buf = buf.as_mut_ptr() as *mut c_char as *mut c_void;

    let result = unsafe {
        read(fd, buf, count)
    };

    if result == -1 {
        Err(())
    } else {
        let num_read = match usize::try_from(result) {
            Err(err) => fatal(
                &format!("Failed to convert ssize_t to usize: {}", err)
            ),
            Ok(n) => n,
        };
        Ok(num_read)
    }
}


extern "C" {
    fn write(
        fd: c_int,
        buf: *const c_void,
        count: size_t,
    ) -> ssize_t;
}

pub fn write_rs(fd: c_int, buf: &[u8]) -> Result<usize> {
    let count = buf.len() as size_t;
    let buf = buf.as_ptr() as *const c_char as *const c_void;

    let result = unsafe {
        write(fd, buf, count)
    };

    if result == -1 {
        Err(())
    } else {
        let num_written = match usize::try_from(result) {
            Err(err) => fatal(
                &format!("Failed to convert ssize_t to usize: {}", err),
            ),
            Ok(n) => n,
        };
        Ok(num_written)
    }
}


extern "C" {
    fn lseek(
        fd: c_int,
        offset: off_t,
        whence: c_int
    ) -> off_t;
}

pub fn lseek_rs(fd: c_int, offset: off_t, whence: c_int) -> Result<off_t> {
    let new_offset = unsafe {
        lseek(fd, offset, whence)
    };

    if new_offset == -1 {
        Err(())
    } else {
        Ok(new_offset)
    }
}

#[allow(non_camel_case_types)]
#[repr(C)]
struct iovec {
    iov_base: *const c_void,
    iov_len: size_t,
}

extern "C" {
    fn readv(
        fd: c_int,
        iov: *mut iovec,
        iovcnt: c_int
    ) -> ssize_t;
}

pub fn readv_rs(fd: c_int, iov: &mut [&mut [u8]]) -> Result<usize> {
    let iovcnt = iov.len();
    let mut vec = Vec::with_capacity(iovcnt);
    for u8_slice in iov.iter_mut() {
        let base_ptr: *mut c_char = u8_slice.as_mut_ptr() as *mut i8;
        vec.push(
            iovec {
                iov_base: base_ptr as *mut c_void,
                iov_len: u8_slice.len(),
            }
        );
    }

    let result = unsafe {
        readv(fd, vec.as_mut_ptr(), iovcnt as c_int)
    };

    if result == -1 {
        Err(())
    } else {
        let num_read = match usize::try_from(result) {
            Err(err) => fatal(
                &format!("Failed to convert ssize_t to usize: {}", err)
            ),
            Ok(n) => n,
        };
        Ok(num_read)
    }
}
