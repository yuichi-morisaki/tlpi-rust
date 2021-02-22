use common::constants::*;
use common::data_types::*;
use error::error_exit;
use std::convert::TryFrom;


type Result<T> = std::result::Result<T, ()>;


extern "C" {
    fn open(
        path: *const c_char,
        flags: c_int,
        ...     // mode: mode_t
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

    fn lseek(
        fd: c_int,
        offset: off_t,
        whence: c_int
    ) -> off_t;

    fn fcntl(
        fd: c_int,
        cmd: c_int,
        ...     // arg (see man 2 fcntl)
    ) -> c_int;
}


pub fn open_rs(
    path: &str,
    flags: c_int,
    mode: Option<mode_t>
) -> Result<c_int> {
    let path = match CString::new(path) {
        Ok(path) => path.into_bytes(),
        Err(err) => {
            error_exit(
                &format!("File path is invalid for CString: {}", err)
            );
        }
    };
    let path = path.as_ptr() as *const c_char;

    let fd = unsafe {
        if let Some(mode) = mode {
            open(path, flags, mode)
        } else {
            open(path, flags)
        }
    };

    if fd == -1 {
        Err(())
    } else {
        Ok(fd)
    }
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


pub fn close_rs(fd: c_int) -> Result<()> {
    let result = unsafe {
        close(fd)
    };

    if result == -1 {
        Err(())
    } else {
        Ok(())
    }
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


pub enum FcntlCmd {
    GetFl,
    SetFl(c_int),   // flags
}

pub fn fcntl_rs(fd: c_int, cmd: FcntlCmd) -> Result<c_int> {
    match cmd {
        FcntlCmd::GetFl => {
            let result = unsafe {
                fcntl(fd, F_GETFL)
            };
            if result == -1 {
                Err(())
            } else {
                Ok(result)
            }
        }

        FcntlCmd::SetFl(flags) => {
            let result = unsafe {
                fcntl(fd, F_SETFL, flags)
            };
            if result == -1 {
                Err(())
            } else {
                Ok(result)
            }
        }
    }
}
