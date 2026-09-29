/*
 * A fork gives the child the parent's mappings with the protection they have
 * now, not the one they were made with. Each part changes a protection with
 * mprotect, forks, and the child tries one access; the parent reads how the
 * child ended.
 */
#include <sys/mman.h>
#include <unistd.h>

#include "memproof.h"

static volatile char *p;

static void write_it(void) {
    p[0] = 1;
    if (p[0] != 1) {
        _exit(2);
    }
}
static void read_it(void) {
    if (p[0] != 0x5a) {
        _exit(2);
    }
}

static volatile char *fresh(int pages) {
    volatile char *m =
        mmap(0, pages * PG, PROT_READ | PROT_WRITE, MAP_PRIVATE | MAP_ANONYMOUS, -1, 0);
    for (int i = 0; i < pages; i++) {
        m[i * PG] = 0x5a;
    }
    return m;
}

int main(void) {
    const char *me = "protfork";
    p = fresh(1);
    mprotect((void *)p, PG, PROT_READ);
    part(me, "RW to R, child writes", write_it, 1);
    part(me, "RW to R, child reads the byte", read_it, 0);

    p = fresh(1);
    mprotect((void *)p, PG, PROT_NONE);
    part(me, "RW to NONE, child reads", read_it, 1);

    p = fresh(1);
    mprotect((void *)p, PG, PROT_READ);
    mprotect((void *)p, PG, PROT_READ | PROT_WRITE);
    part(me, "RW to R to RW, child writes", write_it, 0);

    volatile char *m = fresh(3);
    mprotect((void *)(m + PG), PG, PROT_READ);
    p = m;
    part(me, "middle page R, child writes the first", write_it, 0);
    p = m + PG;
    part(me, "middle page R, child writes the middle", write_it, 1);
    p = m + 2 * PG;
    part(me, "middle page R, child writes the last", write_it, 0);
    return finish(me);
}
