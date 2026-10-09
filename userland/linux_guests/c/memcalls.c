/*
 * The memory calls answered as Linux answers them: where mmap puts a hint, the
 * break giving pages back, unaligned addresses refused, mremap keeping a
 * mapping's protection, and mlock, msync and mincore with their errnos. Every
 * part runs and prints one line, so one boot names every part that fails.
 * Parts that must fault run in a forked child.
 */
#include <errno.h>
#include <stdio.h>
#include <sys/mman.h>
#include <sys/wait.h>
#include <unistd.h>

#include "memcalls.h"

volatile char *mc_p;

void check(const char *name, int ok, long got) {
    char detail[48];
    snprintf(detail, sizeof detail, "got %ld", got);
    part_line("memcalls", name, ok, detail);
}

void faults(const char *name, void (*fn)(void)) {
    pid_t c = fork();
    if (c == 0) {
        fn();
        _exit(0);
    }
    int st = 0;
    waitpid(c, &st, 0);
    check(name, segv(st), st);
}

long rc(long v) {
    return v == -1 ? -errno : v;
}

char *anon(long len, int prot) {
    return mmap(0, len, prot, MAP_PRIVATE | MAP_ANONYMOUS, -1, 0);
}

void mc_write(void) {
    mc_p[0] = 1;
}

void mc_read(void) {
    (void)mc_p[0];
}

int main(void) {
    placement();
    breaks();
    alignment();
    remaps();
    provenance();
    locks();
    syncs();
    cores();
    return finish("memcalls");
}
