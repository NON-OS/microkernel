#include "cfiles.h"

#ifndef SYS_close_range
#define SYS_close_range 436
#endif

#define CLOSE_RANGE_CLOEXEC (1U << 2)

void part_close_range(void) {
    const char *p = "close_range";
    int base = open("/", O_RDONLY);
    int a = dup(base), b = dup(base);
    CHECK(p, syscall(SYS_close_range, a, b, CLOSE_RANGE_CLOEXEC) == 0, errno, 0);
    CHECK(p, fcntl(a, F_GETFD) == FD_CLOEXEC && fcntl(b, F_GETFD) == FD_CLOEXEC, a, b);
    CHECK(p, fcntl(base, F_GETFD) == 0, base, 0);
    CHECK(p, syscall(SYS_close_range, a, ~0U, 0) == 0, errno, 0);
    ERR(p, fcntl(a, F_GETFD), EBADF);
    ERR(p, fcntl(b, F_GETFD), EBADF);
    ERR(p, syscall(SYS_close_range, 5, 4, 0), EINVAL);
    ERR(p, syscall(SYS_close_range, 3, 4, 0x80), EINVAL);
    close(base);
    done(p, "marks close-on-exec, closes a range, refuses a bad one");
}
