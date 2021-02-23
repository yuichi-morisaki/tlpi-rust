use common::constants::*;
use common::data_types::*;
use error::fatal;

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
        Ok(path) => path.into_bytes(),
        Err(err) => {
            fatal(
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
