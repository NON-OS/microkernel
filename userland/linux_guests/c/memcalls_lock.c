/* memcalls: mlock, msync and mincore with Linux's errnos. */
#define _GNU_SOURCE
#include <errno.h>
#include <fcntl.h>
#include <sys/mman.h>
#include <sys/syscall.h>
#include <unistd.h>

#include "memcalls.h"

void locks(void) {
    char *m = anon(3 * PG, PROT_READ | PROT_WRITE);
    check("mlock of a mapping", rc(mlock(m, 3 * PG)) == 0, rc(mlock(m, 3 * PG)));
    check("munlock of a mapping", rc(munlock(m, 3 * PG)) == 0, rc(munlock(m, 3 * PG)));
    munmap(m + PG, PG);
    check("mlock over a hole is ENOMEM", rc(mlock(m, 3 * PG)) == -ENOMEM, rc(mlock(m, 3 * PG)));
    check("mlock2 with an unknown flag is EINVAL", rc(syscall(SYS_mlock2, m, PG, 2)) == -EINVAL,
          rc(syscall(SYS_mlock2, m, PG, 2)));
    check("mlock2 MLOCK_ONFAULT", rc(syscall(SYS_mlock2, m, PG, 1)) == 0,
          rc(syscall(SYS_mlock2, m, PG, 1)));
    check("mlockall(0) is EINVAL", rc(mlockall(0)) == -EINVAL, rc(mlockall(0)));
    check("mlockall(MCL_ONFAULT) alone is EINVAL", rc(mlockall(MCL_ONFAULT)) == -EINVAL,
          rc(mlockall(MCL_ONFAULT)));
    check("mlockall(MCL_CURRENT)", rc(mlockall(MCL_CURRENT)) == 0, rc(mlockall(MCL_CURRENT)));
    check("munlockall", rc(munlockall()) == 0, rc(munlockall()));
}

void syncs(void) {
    char *m = anon(3 * PG, PROT_READ | PROT_WRITE);
    check("msync MS_SYNC", rc(msync(m, 3 * PG, MS_SYNC)) == 0, rc(msync(m, 3 * PG, MS_SYNC)));
    check("msync unaligned is EINVAL", rc(msync(m + 1, PG, MS_SYNC)) == -EINVAL,
          rc(msync(m + 1, PG, MS_SYNC)));
    check("msync MS_ASYNC|MS_SYNC is EINVAL", rc(msync(m, PG, MS_ASYNC | MS_SYNC)) == -EINVAL,
          rc(msync(m, PG, MS_ASYNC | MS_SYNC)));
    check("msync unknown flag is EINVAL", rc(msync(m, PG, 8)) == -EINVAL, rc(msync(m, PG, 8)));
    munmap(m + PG, PG);
    check("msync over a hole is ENOMEM", rc(msync(m, 3 * PG, MS_SYNC)) == -ENOMEM,
          rc(msync(m, 3 * PG, MS_SYNC)));
}

void cores(void) {
    char *r = anon(4 * PG, PROT_NONE);
    mprotect(r + PG, PG, PROT_READ | PROT_WRITE);
    r[PG] = 1;
    unsigned char v[4] = { 9, 9, 9, 9 };
    long got = rc(mincore(r, 4 * PG, v));
    check("mincore of a reservation with one page opened is 0,1,0,0",
          got == 0 && v[0] == 0 && v[1] == 1 && v[2] == 0 && v[3] == 0,
          got ? got : v[0] | v[1] << 8 | v[2] << 16 | (long)v[3] << 24);
    check("mincore unaligned is EINVAL", rc(mincore(r + 1, PG, v)) == -EINVAL,
          rc(mincore(r + 1, PG, v)));
    munmap(r + 2 * PG, PG);
    check("mincore over a hole is ENOMEM", rc(mincore(r, 4 * PG, v)) == -ENOMEM,
          rc(mincore(r, 4 * PG, v)));
}
