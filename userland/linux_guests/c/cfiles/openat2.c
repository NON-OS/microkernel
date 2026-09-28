#include "cfiles.h"

/* Linux's uapi, which musl's headers do not carry. */
struct open_how {
    uint64_t flags, mode, resolve;
};

#define RESOLVE_NO_MAGICLINKS 0x02

#define RESOLVE_NO_SYMLINKS 0x04

#define RESOLVE_BENEATH 0x08

#ifndef SYS_openat2
#define SYS_openat2 437
#endif

static long oa2(int dirfd, const char *path, uint64_t flags, uint64_t resolve) {
    struct open_how how;
    memset(&how, 0, sizeof how);
    how.flags = flags;
    how.resolve = resolve;
    return syscall(SYS_openat2, dirfd, path, &how, sizeof how);
}

void part_openat2(void) {
    const char *p = "openat2";
    mkdir(DIR "/o2", 0755);
    close(mk("o2/f", "x"));
    symlink("f", DIR "/o2/ln");
    int d = open(DIR "/o2", O_RDONLY | O_DIRECTORY);
    long fd = oa2(d, "f", O_RDONLY, 0);
    CHECK(p, fd >= 0, fd, errno);
    close(fd);
    fd = oa2(d, "ln", O_RDONLY, RESOLVE_BENEATH);
    CHECK(p, fd >= 0, fd, errno);
    close(fd);
    ERR(p, oa2(d, "../o2/f", O_RDONLY, RESOLVE_BENEATH), EXDEV);
    ERR(p, oa2(d, "/tmp", O_RDONLY, RESOLVE_BENEATH), EXDEV);
    ERR(p, oa2(d, "ln", O_RDONLY, RESOLVE_NO_SYMLINKS), ELOOP);
    ERR(p, oa2(d, "/proc/self/cwd", O_RDONLY, RESOLVE_NO_MAGICLINKS), ELOOP);
    fd = oa2(AT_FDCWD, "/proc/self/status", O_RDONLY, RESOLVE_NO_MAGICLINKS);
    CHECK(p, fd >= 0, fd, errno);
    close(fd);
    struct open_how how = {.flags = O_RDONLY, .mode = 0644};
    ERR(p, syscall(SYS_openat2, d, "f", &how, sizeof how), EINVAL);
    ERR(p, syscall(SYS_openat2, d, "f", &how, 8), EINVAL);
    close(d);
    done(p, "BENEATH, NO_SYMLINKS and NO_MAGICLINKS walk as Linux walks");
}
