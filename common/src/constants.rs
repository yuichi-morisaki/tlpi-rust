pub use libc::{
// exit(3)
    // #include <stdlib.h>
    EXIT_SUCCESS,   // c_int 0
    EXIT_FAILURE,   // c_int 1

// open(2)
    // #include <fcntl.h>
        // -- file access flags
    O_RDONLY,   // c_int 0
    O_WRONLY,   // c_int 1
    O_RDWR,     // c_int 2
        // -- file creation flags
    O_CREAT,    // c_int 64
    O_EXCL,     // c_int 128
    O_NOCTTY,   // c_int 256
    O_TRUNC,    // c_int 512
    O_DIRECTORY,    // c_int 0x10000
    O_NOFOLLOW,     // c_int 0x20000
    O_CLOEXEC,      // c_int 0x80000
    O_LARGEFILE,    // c_int 0
        // -- open file status flags
    O_APPEND,   // c_int 1024
    O_NONBLOCK, // c_int 2048
    O_DSYNC,    // c_int 4096
    O_ASYNC,    // c_int 0x2000
    O_DIRECT,   // c_int 0x4000
    O_NOATIME,  // c_int 0x40000
    O_SYNC,     // c_int 0x101000
    // #include <sys/stat.h>
    S_IRUSR,    // mode_t 256
    S_IWUSR,    // mode_t 128
    S_IRGRP,    // mode_t 32
    S_IWGRP,    // mode_t 16
    S_IROTH,    // mode_t 4
    S_IWOTH,    // mode_t 2
};
