//! Exercise 5-4

use common::constants::*;
use common::data_types::*;
use error::{ error_exit, get_errno_in_c, set_errno_in_c };
use syscall::fs::{ close_rs, fcntl_rs, FcntlCmd };


pub fn dup(old_fd: c_int) -> c_int {
    match fcntl_rs(old_fd, FcntlCmd::DupFd(0)) {
        Err(_) => -1,
        Ok(new_fd) => new_fd
    }
}


pub fn dup2(old_fd: c_int, new_fd: c_int) -> c_int {
    if old_fd == new_fd {
        match fcntl_rs(old_fd, FcntlCmd::GetFl) {
            Err(_) => {
                set_errno_in_c(EBADF);
                -1
            }
            Ok(_) => new_fd,
        }
    } else {
        if let Err(_) = close_rs(new_fd) {
            if get_errno_in_c() != EBADF {
                error_exit("close");
            }
        }
        match fcntl_rs(old_fd, FcntlCmd::DupFd(new_fd)) {
            Err(_) => -1,
            Ok(fd) => fd, /* may not be new_fd
                             because of non-atomic operations */
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dup_with_invalid_fd() {
        assert_eq!(dup(1000), -1);
        assert_eq!(get_errno_in_c(), EBADF);
    }

    #[test]
    fn dup_with_valid_fd() {
        assert_eq!(dup(0), 3);
    }

    #[test]
    fn dup2_with_invalid_fd() {
        assert_eq!(dup2(1000, 100), -1);
        assert_eq!(get_errno_in_c(), EBADF);
    }

    #[test]
    fn dup2_with_invalid_fd_same() {
        assert_eq!(dup2(1000, 1000), -1);
        assert_eq!(get_errno_in_c(), EBADF);
    }

    #[test]
    fn dup2_with_valid_fd() {
        assert_eq!(dup2(0, 100), 100);
    }

    #[test]
    fn dup2_with_valid_fd_same() {
        assert_eq!(dup2(0, 0), 0);
    }
}
