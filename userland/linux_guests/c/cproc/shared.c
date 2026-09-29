#include "cproc.h"

/* The whole file, NUL-terminated; its length in *n. */
char *slurp(const char *path, long *n) {
    static char buf[1 << 16];
    int fd = open(path, O_RDONLY);
    *n = -1;
    if (fd < 0) {
        return 0;
    }
    long got = 0, r;
    while ((r = read(fd, buf + got, sizeof buf - 1 - got)) > 0) {
        got += r;
    }
    close(fd);
    buf[got] = 0;
    *n = got;
    return buf;
}

long field_of(const char *text, const char *key) {
    const char *at = strstr(text, key);
    return at ? strtol(at + strlen(key), 0, 10) : -1;
}

void spin_ms(long ms) {
    struct timespec t0, t;
    clock_gettime(CLOCK_MONOTONIC, &t0);
    volatile unsigned long x = 0;
    do {
        for (int i = 0; i < 100000; i++) {
            x += i;
        }
        clock_gettime(CLOCK_MONOTONIC, &t);
    } while ((t.tv_sec - t0.tv_sec) * 1000 + (t.tv_nsec - t0.tv_nsec) / 1000000 < ms);
}

int is_nonos(void) {
    struct utsname u;
    uname(&u);
    return strstr(u.version, "NONOS") != 0;
}
