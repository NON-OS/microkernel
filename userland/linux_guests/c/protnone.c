/*
 * PROT_NONE means no access. Each part runs in a forked child and the parent
 * reads how the child ended: a part that must fault passes only when the child
 * dies of SIGSEGV, a part that must not fault passes only when it exits 0.
 * Every part runs, so one boot names every part that fails.
 */
#include <sys/mman.h>
#include <unistd.h>

#include "memproof.h"

static volatile char *p;

static void read_it(void) {
    if (p[0] != 0x5a) {
        _exit(2);
    }
}
static void write_it(void) {
    p[0] = 1;
}
static void read_below(void) {
    (void)p[-1];
}

int main(void) {
    const char *me = "protnone";
    p = mmap(0, PG, PROT_NONE, MAP_PRIVATE | MAP_ANONYMOUS, -1, 0);
    part(me, "read of an mmap PROT_NONE page", read_it, 1);
    part(me, "write to an mmap PROT_NONE page", write_it, 1);

    p = mmap(0, PG, PROT_READ | PROT_WRITE, MAP_PRIVATE | MAP_ANONYMOUS, -1, 0);
    p[0] = 0x5a;
    mprotect((void *)p, PG, PROT_NONE);
    part(me, "read after mprotect RW to PROT_NONE", read_it, 1);
    mprotect((void *)p, PG, PROT_READ);
    part(me, "read after PROT_NONE back to R keeps the byte", read_it, 0);
    part(me, "write to a PROT_READ page", write_it, 1);

    char *r = mmap(0, 3 * PG, PROT_NONE, MAP_PRIVATE | MAP_ANONYMOUS, -1, 0);
    mprotect(r + PG, PG, PROT_READ | PROT_WRITE);
    p = (volatile char *)(r + PG);
    p[0] = 0x5a;
    part(me, "read of the opened page of a reservation", read_it, 0);
    part(me, "read of the closed page below it", read_below, 1);
    return finish(me);
}
