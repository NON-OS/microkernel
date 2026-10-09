#include "cproc.h"

void part_memory(void) {
    const char *p = "memory";
    /* 4 MiB, so the host's own traffic in Shmem cannot hide it. */
    static char chunk[4 << 20];
    memset(chunk, 'm', sizeof chunk);
    long n, before = field_of(slurp("/proc/meminfo", &n), "\nShmem:");
    int fd = open("/tmp/cproc-shm", O_RDWR | O_CREAT | O_TRUNC, 0600);
    CHECK(p, write(fd, chunk, sizeof chunk) == sizeof chunk, errno, 0);
    char *info = slurp("/proc/meminfo", &n);
    long after = field_of(info, "\nShmem:"), cached = field_of(info, "\nCached:");
    CHECK(p, after - before >= 3072 && cached >= after, after - before, cached);
    struct sysinfo si;
    sysinfo(&si);
    CHECK(p, (long)(si.sharedram * si.mem_unit / 1024) >= 3072, (long)si.sharedram, 0);
    close(fd);
    unlink("/tmp/cproc-shm");
    done(p, "a tmpfs file's bytes show as Shmem and Cached, and as sysinfo's sharedram");
}
