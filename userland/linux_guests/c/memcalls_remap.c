/* memcalls: mremap keeping a mapping's protection, backing and provenance. */
#define _GNU_SOURCE
#include <errno.h>
#include <fcntl.h>
#include <sys/mman.h>
#include <sys/syscall.h>
#include <unistd.h>

#include "memcalls.h"

void remaps(void) {
    char *m = anon(PG, PROT_READ | PROT_WRITE);
    m[0] = 0x33;
    mprotect(m, PG, PROT_READ);
    char *g = mremap(m, PG, 2 * PG, MREMAP_MAYMOVE);
    check("mremap of a read-only page keeps the byte", g != MAP_FAILED && g[0] == 0x33, g[0]);
    mc_p = g + PG;
    faults("mremap grown part of a read-only page faults on write", mc_write);
    char *r = anon(2 * PG, PROT_NONE);
    anon(PG, PROT_NONE);
    char *q = mremap(r, 2 * PG, 4 * PG, MREMAP_MAYMOVE);
    check("mremap of a reservation succeeds", q != MAP_FAILED, (long)(q == MAP_FAILED));
    mc_p = q + 3 * PG;
    faults("mremap grown reservation still faults on read", mc_read);
    char *two = anon(2 * PG, PROT_READ | PROT_WRITE);
    mprotect(two + PG, PG, PROT_READ);
    void *x = mremap(two, 2 * PG, 3 * PG, MREMAP_MAYMOVE);
    long e = x == MAP_FAILED ? -errno : 0;
    check("mremap across two mappings is EFAULT", e == -EFAULT, e);
}

/*
 * A file mapped without exec was never proved; where mprotect refuses to make
 * it executable, it must refuse the copy mremap moved too. Host Linux allows
 * both, and the part checks only that the two answers agree.
 */
void provenance(void) {
    /* This program's own file: /bin/memproof in the store, itself on a host. */
    int fd = open("/bin/memproof", O_RDONLY);
    if (fd < 0) {
        fd = open("/proc/self/exe", O_RDONLY);
    }
    char *m = mmap(0, PG, PROT_READ, MAP_PRIVATE, fd, 0);
    if (m == MAP_FAILED) {
        check("file mapping for the provenance part", 0, -errno);
        return;
    }
    long first = rc(mprotect(m, PG, PROT_READ | PROT_EXEC));
    mprotect(m, PG, PROT_READ);
    mmap(m + PG, PG, PROT_READ, MAP_PRIVATE | MAP_ANONYMOUS | NOREPLACE, -1, 0);
    char *n = mremap(m, PG, 2 * PG, MREMAP_MAYMOVE);
    long moved = n == MAP_FAILED ? -1000 : rc(mprotect(n, PG, PROT_READ | PROT_EXEC));
    check("moved unproven file bytes refused exec as before the move", n != m && moved == first,
          moved);
    close(fd);
}
