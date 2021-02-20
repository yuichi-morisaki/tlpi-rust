use error::error_exit;
use std::env;
use std::os::raw::c_int;

extern {
    fn set_errno(err_num: c_int);
}

fn main() {
    let argv: Vec<_> = env::args().collect();
    let argc = argv.len();

    if argc > 1 {
        if let Ok(err_num) = argv[1].parse::<i32>() {
            unsafe {
                set_errno(err_num as c_int);
            }
        }
    }

    error_exit("This is demo of `error_exit`");
}
