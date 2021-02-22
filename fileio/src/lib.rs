mod syscalls;

pub use syscalls::open_rs;
pub use syscalls::read_rs;
pub use syscalls::write_rs;
pub use syscalls::close_rs;
pub use syscalls::lseek_rs;
pub use syscalls::fcntl_rs;
pub use syscalls::FcntlCmd;
