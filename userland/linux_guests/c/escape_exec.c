/* Attacks on what a guest may run and call: adding execute to a file it never
 * proved, a syscall number that is not served, and a step past a mapping. */
#define _GNU_SOURCE
#include <errno.h>
#include <fcntl.h>
#include <sys/mman.h>
#include <sys/syscall.h>
#include <unistd.h>

#include "escape.h"

static volatile char *edge;

void exec_escalate(void) {
    /* Map this program read-only, then try to make it executable. It was
     * mapped without execute, so it was never proved; NONOS refuses. */
    int fd = open("/bin/memproof", O_RDONLY);
    if (fd < 0) {
        fd = open("/proc/self/exe", O_RDONLY);
    }
    if (fd < 0) {
        held("exec added to an unproven file mapping is refused", 0, -errno);
        return;
    }
    void *p = mmap(0, PG, PROT_READ, MAP_PRIVATE, fd, 0);
    close(fd);
    if (p == MAP_FAILED) {
        held("exec added to an unproven file mapping is refused", 0, -errno);
        return;
    }
    long r = erc(mprotect(p, PG, PROT_READ | PROT_EXEC));
    held("exec added to an unproven file mapping is refused", r == -EPERM, r);
}

void forged_call(void) {
    /* A syscall number the personality does not serve. */
    long r = erc(syscall(0x462));
    held("an unserved syscall number answers ENOSYS", r == -ENOSYS, r);
}

static void step_past(void) {
    edge[PG] = 1;
}

void past_end(void) {
    /* Two pages, second given back, so the page written is a certain hole. */
    edge = mmap(0, 2 * PG, PROT_READ | PROT_WRITE, MAP_PRIVATE | MAP_ANONYMOUS, -1, 0);
    munmap((void *)(edge + PG), PG);
    part("escape", "a write into an unmapped hole faults", step_past, 1);
}
