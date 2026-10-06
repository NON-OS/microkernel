/* Attacks on where and how a guest may map: the kernel half, a wrapping
 * span, and a page that is both writable and executable. */
#include <errno.h>
#include <stdint.h>
#include <sys/mman.h>

#include "escape.h"

/* The first address of the kernel half, which no guest mapping may reach. */
#define KERNEL_HALF 0x0000800000000000UL

static int fixed_refused(uintptr_t at) {
    void *p = mmap((void *)at, PG, PROT_READ | PROT_WRITE,
                   MAP_PRIVATE | MAP_ANONYMOUS | MAP_FIXED, -1, 0);
    if (p == MAP_FAILED) {
        return 1;
    }
    /* It answered with an address; it must at least not be the one asked. */
    munmap(p, PG);
    return (uintptr_t)p != at;
}

void reach_kernel(void) {
    held("MAP_FIXED into the kernel half is refused", fixed_refused(KERNEL_HALF), 0);
    held("MAP_FIXED one page below the kernel half is refused", fixed_refused(KERNEL_HALF - PG), 0);
}

void wrap_span(void) {
    /* A length that wraps past the top of the address space. */
    void *p = mmap(0, (size_t)-PG, PROT_READ | PROT_WRITE, MAP_PRIVATE | MAP_ANONYMOUS, -1, 0);
    long got = p == MAP_FAILED ? -errno : 0;
    held("a mapping whose length wraps is refused", p == MAP_FAILED, got);
}

void wx_map(void) {
    /* Write and execute at once: NONOS refuses it, Linux allows it. */
    void *p = mmap(0, PG, PROT_READ | PROT_WRITE | PROT_EXEC, MAP_PRIVATE | MAP_ANONYMOUS, -1, 0);
    long got = p == MAP_FAILED ? -errno : 0;
    held("a write-and-execute mapping is refused", p == MAP_FAILED, got);
}
