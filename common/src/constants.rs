pub use libc::{
    // #include <unistd.h>
    STDIN_FILENO,   // c_int 0
    STDOUT_FILENO,  // c_int 1
    STDERR_FILENO,  // c_int 2

// exit(3)
    // #include <stdlib.h>
    EXIT_SUCCESS,   // c_int 0
    EXIT_FAILURE,   // c_int 1

// ...
    // #include <errno.h>
    ENOENT,     // c_int 2      No such file or directory
    EBADF,      // c_int 9      Bad file descriptor

// open(2)
    // #include <fcntl.h>
        // -- file access flags
    O_RDONLY,   // c_int 0
    O_WRONLY,   // c_int 1
    O_RDWR,     // c_int 2
    O_ACCMODE,  // c_int 3
        // -- file creation flags
    O_CREAT,    // c_int 64     !must be with `mode`
    O_EXCL,     // c_int 128            See ch. 5.1
    O_NOCTTY,   // c_int 256            See ch. 34.4
    O_TRUNC,    // c_int 512
    O_DIRECTORY,    // c_int 0x10000    See ch. 18.8
    O_NOFOLLOW,     // c_int 0x20000
    O_CLOEXEC,      // c_int 0x80000    See ch. 27.4
    O_LARGEFILE,    // c_int 0          See ch. 5.10
        // -- open file status flags
    O_APPEND,   // c_int 1024           See ch. 5.1
    O_NONBLOCK, // c_int 2048           See ch. 5.9
    O_DSYNC,    // c_int 4096           See ch. 13.3
    O_ASYNC,    // c_int 0x2000         See ch. 5.3 & 63.3
    O_DIRECT,   // c_int 0x4000         See ch. 13.6
    O_NOATIME,  // c_int 0x40000
    O_SYNC,     // c_int 0x101000       See ch. 13.3

    O_PATH,     // c_int 0x200000       P.S. in p.82 (137/1605)

    // #include <sys/stat.h>
    S_IRUSR,    // mode_t 256
    S_IWUSR,    // mode_t 128
    S_IRGRP,    // mode_t 32
    S_IWGRP,    // mode_t 16
    S_IROTH,    // mode_t 4
    S_IWOTH,    // mode_t 2

// lseek(2)
    // #include <unistd.h>
    SEEK_SET,   // c_int 0
    SEEK_CUR,   // c_int 1
    SEEK_END,   // c_int 2
    SEEK_DATA,  // c_int 3
    SEEK_HOLE,  // c_int 4

// fcntl(2)
    // #include <fcntl.h>
    F_DUPFD,    // c_int 0          Duplicating a file descriptor
    F_GETFL,    // c_int 3          File status flag
    F_SETFL,    // c_int 4          File status flag
};
