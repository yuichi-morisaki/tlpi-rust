pub use std::os::raw::{
    c_void,
    c_char,
    c_schar,
    c_uchar,
    c_double,
    c_float,
    c_int,
    c_uint,
    c_long,
    c_ulong,
    c_longlong,
    c_ulonglong,
    c_short,
    c_ushort,
};

pub use std::ffi::{
    CString,
    CStr,
};

pub use libc::{
    // #include <sys/types.h>
    mode_t,     // u32
    pid_t,      // i32
    // #include <unistd.h>
    off_t,      // i64
    size_t,     // usize
    ssize_t,    // isize
};
