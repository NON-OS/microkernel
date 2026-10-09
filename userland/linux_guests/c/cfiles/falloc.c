#include "cfiles.h"

#ifndef FALLOC_FL_COLLAPSE_RANGE
#define FALLOC_FL_COLLAPSE_RANGE 0x08
#endif

void part_falloc(void) {
    const char *p = "fallocate";
    int fd = mk("fa", "abcdef");
    CHECK(p, fallocate(fd, 0, 4, 10) == 0, 0, 0);
    struct stat st;
    fstat(fd, &st);
    CHECK(p, st.st_size == 14, st.st_size, 14);
    CHECK(p, fallocate(fd, FALLOC_FL_KEEP_SIZE, 0, 100) == 0, 0, 0);
    fstat(fd, &st);
    CHECK(p, st.st_size == 14, st.st_size, 14);
    CHECK(p, fallocate(fd, FALLOC_FL_PUNCH_HOLE | FALLOC_FL_KEEP_SIZE, 1, 2) == 0, 0, 0);
    char b[4];
    pread(fd, b, 4, 0);
    CHECK(p, b[0] == 'a' && b[1] == 0 && b[2] == 0 && b[3] == 'd', b[1], b[3]);
    ERR(p, fallocate(fd, FALLOC_FL_PUNCH_HOLE, 0, 1), EOPNOTSUPP);
    ERR(p, fallocate(fd, FALLOC_FL_COLLAPSE_RANGE, 0, 4096), EOPNOTSUPP);
    ERR(p, fallocate(fd, 0, 0, 0), EINVAL);
    CHECK(p, posix_fadvise(fd, 0, 0, POSIX_FADV_SEQUENTIAL) == 0, 0, 0);
    CHECK(p, syscall(SYS_fadvise64, fd, 0, 0, 99) == -1 && errno == EINVAL, errno, EINVAL);
    close(fd);
    done(p, "mode 0 grows, KEEP_SIZE keeps, PUNCH_HOLE zeroes, the rest refused");
}
