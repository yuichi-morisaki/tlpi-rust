use common::constants::*;
use common::data_types::*;
use error::fatal;
use std::convert::TryFrom;

type Result<T> = std::result::Result<T, ()>;


extern "C" {
    fn open(
        path: *const c_char,
        flags: c_int,
        ...     // mode: mode_t
    ) -> c_int;
}

pub fn open_rs(
    path: &str,
    flags: c_int,
    mode: Option<mode_t>
) -> Result<c_int> {
    let path = match CString::new(path) {
        Ok(path) => path.into_bytes_with_nul(),
        Err(err) => fatal(
            &format!("File path is invalid for CString: {}", err)
        ),
    };
    let path: *const u8 = path.as_ptr();
    let path = path as *const i8 as *const c_char;

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


extern "C" {
    fn close(fd: c_int) -> c_int;
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


extern "C" {
    fn fcntl(
        fd: c_int,
        cmd: c_int,
        ...     // arg (see man 2 fcntl)
    ) -> c_int;

}

pub enum FcntlCmd {
    DupFd(c_int),   // the lowest file descriptor
    GetFl,
    SetFl(c_int),   // flags
}

pub fn fcntl_rs(fd: c_int, cmd: FcntlCmd) -> Result<c_int> {
    match cmd {
        FcntlCmd::DupFd(low_fd) => {
            let new_fd = unsafe {
                fcntl(fd, F_DUPFD, low_fd)
            };
            if new_fd == -1 {
                Err(())
            } else {
                Ok(new_fd)
            }
        }

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


extern "C" {
    fn mkstemp(template: *mut c_char) -> c_int;
}

pub fn mkstemp_rs(template: &mut [u8]) -> Result<c_int> {
    let template: *mut c_char = template.as_mut_ptr() as *mut i8;

    let fd = unsafe {
        mkstemp(template)
    };

    if fd == -1 {
        Err(())
    } else {
        Ok(fd)
    }
}


extern "C" {
    fn unlink(path: *const c_char) -> c_int;
}

pub fn unlink_rs(path: &str) -> Result<()> {
    let path = match CString::new(path) {
        Err(err) => fatal(
            &format!("Failed to create CString: {}", err)
        ),
        Ok(path) => path.into_bytes_with_nul(),
    };
    let path: *const u8 = path.as_ptr();
    let path = path as *const i8 as *const c_char;

    let result = unsafe {
        unlink(path)
    };

    if result == -1 {
        Err(())
    } else {
        Ok(())
    }
}


extern "C" {
    fn truncate(
        path: *const c_char,
        length: off_t,
    ) -> c_int;
}

pub fn truncate_rs(path: &str, length: usize) -> Result<()> {
    let path = match CString::new(path) {
        Err(err) => fatal(
            &format!("Failed to create CString: {}", err)
        ),
        Ok(path) => path.into_bytes_with_nul(),
    };
    let path: *const u8 = path.as_ptr();
    let path = path as *const i8 as *const c_char;

    let length = match off_t::try_from(length) {
        Err(err) => fatal(
            &format!("Failed to convert usize into off_t: {}", err)
        ),
        Ok(n) => n,
    };

    let result = unsafe {
        truncate(path, length)
    };

    if result == -1 {
        Err(())
    } else {
        Ok(())
    }
}


extern "C" {
    fn dup(old_fd: c_int) -> c_int;

    fn dup2(old_fd: c_int, new_fd: c_int) -> c_int;
}

pub fn dup_rs(old_fd: c_int) -> Result<c_int> {
    let new_fd = unsafe {
        dup(old_fd)
    };

    if new_fd == -1 {
        Err(())
    } else {
        Ok(new_fd)
    }
}

pub fn dup2_rs(old_fd: c_int, new_fd: c_int) -> Result<c_int> {
    let new_fd = unsafe {
        dup2(old_fd, new_fd)
    };

    if new_fd == -1 {
        Err(())
    } else {
        Ok(new_fd)
    }
}
