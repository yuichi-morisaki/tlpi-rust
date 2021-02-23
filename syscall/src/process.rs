use common::data_types::*;


extern "C" {
    fn getpid() -> pid_t;
}

pub fn getpid_rs() -> pid_t {
    unsafe {
        getpid()
    }
}
