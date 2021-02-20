#include <errno.h>  /* errno */
#include <stdio.h>  /* snprintf */
#include <string.h> /* strerror */
#include "ename.c.inc"  /* ename and MAX_ENAME */


int
error_text(char *buf, unsigned int buf_size, int err_num)
{
    int saved_errno;
    int num_bytes;

    if (err_num == 0) {
        err_num = errno;
    }

    if (err_num <= 0 || MAX_ENAME < err_num) {
        return 0;
    }

    saved_errno = errno;

    num_bytes = snprintf(
            buf, buf_size, "[%s %s]", ename[err_num], strerror(err_num));

    errno = saved_errno;

    if (num_bytes < 0) {
        return -1;
    }

    return num_bytes;
}
