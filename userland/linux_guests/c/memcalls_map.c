/* memcalls: where mmap puts a mapping, the break, and unaligned addresses. */
#define _GNU_SOURCE
#include <errno.h>
#include <fcntl.h>
#include <sys/mman.h>
#include <sys/syscall.h>
#include <unistd.h>

#include "memcalls.h"

void placement(void) {
    char *m = anon(PG, PROT_READ | PROT_WRITE);
    m[0] = 0x11;
    char *n = mmap(m, PG, PROT_READ | PROT_WRITE, MAP_PRIVATE | MAP_ANONYMOUS, -1, 0);
    check("hint on a mapping lands elsewhere, zeroed", n != m && n[0] == 0 && m[0] == 0x11,
          (long)(n - m));
    void *q = mmap(m, PG, PROT_READ, MAP_PRIVATE | MAP_ANONYMOUS | NOREPLACE, -1, 0);
    long e = q == MAP_FAILED ? -errno : 0;
    check("MAP_FIXED_NOREPLACE on a mapping is EEXIST", e == -EEXIST && m[0] == 0x11, e);
    /*
     * A mapping placed just above the last one mmap chose: the next mmap that
     * leaves the choice to the system must not land on it. On a system that
     * already holds that address the probe cannot be placed, and says so.
     */
    char *a = anon(PG, PROT_READ | PROT_WRITE);
    char *f = mmap(a + PG, PG, PROT_READ | PROT_WRITE, MAP_PRIVATE | MAP_ANONYMOUS | NOREPLACE, -1, 0);
    if (f == MAP_FAILED) {
        check("next mmap skips a mapping placed above the last (probe address taken)",
              errno == EEXIST, -errno);
        return;
    }
    f[0] = 0x44;
    char *b = anon(PG, PROT_READ | PROT_WRITE);
    check("next mmap skips a mapping placed above the last", b != f && b[0] == 0 && f[0] == 0x44,
          (long)(b - f));
}

void breaks(void) {
    long cur = syscall(SYS_brk, 0);
    long up = syscall(SYS_brk, cur + 2 * PG);
    ((volatile char *)cur)[PG] = 0x22;
    syscall(SYS_brk, cur);
    syscall(SYS_brk, cur + 2 * PG);
    char b = ((volatile char *)cur)[PG];
    check("brk down and up again reads zero", up == cur + 2 * PG && b == 0, b);
    syscall(SYS_brk, cur);
    long top = (cur + PG - 1) & ~(long)(PG - 1);
    void *in = mmap((void *)(top + PG), PG, PROT_READ, MAP_PRIVATE | MAP_ANONYMOUS | NOREPLACE, -1, 0);
    long got = syscall(SYS_brk, top + 4 * PG);
    check("brk into a mapping is refused", in != MAP_FAILED && got == cur, got - cur);
    munmap(in, PG);
}

void alignment(void) {
    char *m = anon(2 * PG, PROT_READ | PROT_WRITE);
    check("munmap unaligned is EINVAL", rc(munmap(m + 1, PG)) == -EINVAL, rc(munmap(m + 1, PG)));
    /* musl's mprotect rounds the address down itself; the kernel's does not. */
    long e = rc(syscall(SYS_mprotect, m + 1, PG, PROT_READ));
    check("mprotect unaligned is EINVAL", e == -EINVAL, e);
    void *q = mmap(m + 1, PG, PROT_READ, MAP_PRIVATE | MAP_ANONYMOUS | MAP_FIXED, -1, 0);
    e = q == MAP_FAILED ? -errno : 0;
    check("MAP_FIXED unaligned is EINVAL", e == -EINVAL, e);
    /* musl's mmap refuses an unaligned offset itself; the kernel's must too. */
    e = rc(syscall(SYS_mmap, 0, PG, PROT_READ, MAP_PRIVATE | MAP_ANONYMOUS, -1, 1));
    check("mmap offset unaligned is EINVAL", e == -EINVAL, e);
}
