#include "cproc.h"

void part_dev(void) {
    const char *p = "dev";
    const struct { const char *name; int major, minor; } devs[] = {
        {"/dev/null", 1, 3}, {"/dev/zero", 1, 5}, {"/dev/full", 1, 7},
        {"/dev/random", 1, 8}, {"/dev/urandom", 1, 9}, {"/dev/tty", 5, 0},
    };
    for (unsigned i = 0; i < sizeof devs / sizeof devs[0]; i++) {
        struct stat st;
        CHECK(p, stat(devs[i].name, &st) == 0 && S_ISCHR(st.st_mode), i, errno);
        CHECK(p, major(st.st_rdev) == (unsigned)devs[i].major && minor(st.st_rdev) == (unsigned)devs[i].minor, i, st.st_rdev);
        CHECK(p, (st.st_mode & 0777) == 0666, i, st.st_mode);
    }
    char b[32];
    int fd = open("/dev/null", O_RDWR);
    CHECK(p, read(fd, b, sizeof b) == 0 && write(fd, "x", 1) == 1, fd, errno);
    close(fd);
    fd = open("/dev/zero", O_RDONLY);
    memset(b, 7, sizeof b);
    CHECK(p, read(fd, b, sizeof b) == 32 && b[0] == 0 && b[31] == 0, b[0], b[31]);
    close(fd);
    fd = open("/dev/full", O_WRONLY);
    errno = 0;
    CHECK(p, write(fd, "x", 1) == -1 && errno == ENOSPC, errno, ENOSPC);
    close(fd);
    char c[32];
    fd = open("/dev/urandom", O_RDONLY);
    CHECK(p, read(fd, b, 32) == 32 && read(fd, c, 32) == 32 && memcmp(b, c, 32) != 0, 0, 0);
    close(fd);
    errno = 0;
    CHECK(p, open("/dev/tty", O_RDWR) == -1 && errno == ENXIO, errno, ENXIO);
    char to[64] = {0};
    CHECK(p, readlink("/dev/stdin", to, sizeof to) == 15 && strcmp(to, "/proc/self/fd/0") == 0, 0, 0);
    memset(to, 0, sizeof to);
    CHECK(p, readlink("/dev/fd", to, sizeof to) == 13 && strcmp(to, "/proc/self/fd") == 0, 0, 0);
    done(p, "null zero full random urandom tty: numbers, modes, reads and writes");
}
