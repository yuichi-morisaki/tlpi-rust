use common::data_types::*;
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
        let num_read = usize::try_from(result)
            .expect("Failed to convert ssize_t to usize in read_rs");
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
        let num_written = usize::try_from(result)
            .expect("Failed to convert ssize to usize in write_rs");
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
