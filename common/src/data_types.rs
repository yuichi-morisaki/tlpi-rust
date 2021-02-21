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
    // #include <unistd.h>
    size_t,     // usize
    ssize_t,    // isize
    off_t,      // i64
};
