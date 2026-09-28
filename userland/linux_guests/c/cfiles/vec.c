#include "cfiles.h"

/* musl has no wrappers for the v2 forms; the offset goes as low and high words. */
static long preadv2_(int fd, const struct iovec *v, int n, long off, int flags) {
    return syscall(SYS_preadv2, fd, v, n, off, 0, flags);
}

static long pwritev2_(int fd, const struct iovec *v, int n, long off, int flags) {
    return syscall(SYS_pwritev2, fd, v, n, off, 0, flags);
}

void part_vec(void) {
    const char *p = "preadv";
    int fd = mk("vec", "abcdefghij");
    char x[3], y[4];
    struct iovec iv[2] = {{x, 3}, {y, 4}};
    CHECK(p, preadv(fd, iv, 2, 2) == 7 && memcmp(x, "cde", 3) == 0 && memcmp(y, "fghi", 4) == 0, x[0], y[0]);
    CHECK(p, lseek(fd, 0, SEEK_CUR) == 10, lseek(fd, 0, SEEK_CUR), 10);
    struct iovec ow[2] = {{"12", 2}, {"345", 3}};
    CHECK(p, pwritev(fd, ow, 2, 1) == 5, 0, 0);
    char b[11] = {0};
    pread(fd, b, 10, 0);
    CHECK(p, memcmp(b, "a12345ghij", 10) == 0, b[1], b[5]);
    lseek(fd, 3, SEEK_SET);
    CHECK(p, preadv2_(fd, iv, 1, -1, 0) == 3 && memcmp(x, "345", 3) == 0, x[0], 0);
    CHECK(p, lseek(fd, 0, SEEK_CUR) == 6, lseek(fd, 0, SEEK_CUR), 6);
    CHECK(p, pwritev2_(fd, ow, 1, 8, 0) == 2, 0, 0);
    CHECK(p, lseek(fd, 0, SEEK_CUR) == 6, lseek(fd, 0, SEEK_CUR), 6);
    ERR(p, preadv(fd, iv, 2, -2), EINVAL);
    close(fd);
    done(p, "preadv, pwritev and the v2 forms at an offset and at -1");
}
