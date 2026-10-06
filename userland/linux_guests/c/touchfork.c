/*
 * Bytes a guest wrote into a reservation survive a fork. A reservation is a
 * PROT_NONE mapping; a program opens part of it with mprotect, writes, and may
 * close it again. Linux keeps the bytes through all of that and gives the child
 * a copy. Touching a page never opened is SIGSEGV, in the child as anywhere.
 * MAP_FIXED over a mapping replaces it: the new pages read zero, as Linux says.
 */
#include <sys/mman.h>
#include <unistd.h>

#include "memproof.h"

#define PAGES 16

static volatile char *r;

static void check_bytes(void) {
    for (int i = 4; i < 8; i++) {
        if (r[i * PG] != (char)(0x40 + i) || r[i * PG + PG - 1] != (char)(0x50 + i)) {
            _exit(2);
        }
    }
}
static void open_then_check(void) {
    mprotect((void *)(r + 4 * PG), 4 * PG, PROT_READ);
    check_bytes();
}
static void touch_unopened(void) {
    (void)r[0];
}
static void check_zero(void) {
    if (r[0] != 0) {
        _exit(2);
    }
}

int main(void) {
    const char *me = "touchfork";
    int anon = MAP_PRIVATE | MAP_ANONYMOUS;
    r = mmap(0, PAGES * PG, PROT_NONE, anon, -1, 0);
    mprotect((void *)(r + 4 * PG), 4 * PG, PROT_READ | PROT_WRITE);
    for (int i = 4; i < 8; i++) {
        r[i * PG] = (char)(0x40 + i);
        r[i * PG + PG - 1] = (char)(0x50 + i);
    }
    part(me, "opened part of a reservation, child reads the bytes", check_bytes, 0);
    mprotect((void *)(r + 4 * PG), 4 * PG, PROT_NONE);
    part(me, "closed again, child opens it and reads the bytes", open_then_check, 0);
    part(me, "child touches a page never opened", touch_unopened, 1);

    r = mmap(0, PG, PROT_READ | PROT_WRITE, anon, -1, 0);
    r[0] = 0x77;
    mmap((void *)r, PG, PROT_NONE, anon | MAP_FIXED, -1, 0);
    mprotect((void *)r, PG, PROT_READ);
    part(me, "MAP_FIXED PROT_NONE over written page, child reads zero", check_zero, 0);
    return finish(me);
}
