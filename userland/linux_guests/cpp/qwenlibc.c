/*
 * String functions that never read past a string's page.
 *
 * zig 0.16's libc implements strnlen(s, max) as a vector search over
 * s[0..max]: unaligned 32-byte loads, two at a time, up to max, whatever
 * the terminator says. musl's printf asks strnlen(s, INT_MAX) for every
 * %s, so a string in the last bytes of a mapping reads into the page after
 * it. On Linux a heap is one run of pages and that page is mapped; on NONOS
 * each allocator slab is its own mapping and the next page is often not,
 * so the guest died formatting a log line it never printed. zig's own
 * symbols are weak; these strong ones replace them for this program, and
 * each scan stops at the end of the page it is in.
 */
#include <stddef.h>
#include <stdint.h>
#include <string.h>

enum { PAGE = 4096 };

size_t strnlen(const char *s, size_t max) {
    size_t n = 0;
    while (n < max) {
        size_t room = PAGE - ((uintptr_t)(s + n) & (PAGE - 1));
        if (room > max - n) room = max - n;
        const char *z = memchr(s + n, 0, room);
        if (z) return (size_t)(z - s);
        n += room;
    }
    return max;
}

char *stpncpy(char *restrict d, const char *restrict s, size_t n) {
    size_t len = strnlen(s, n);
    memcpy(d, s, len);
    memset(d + len, 0, n - len);
    return d + len;
}

char *strncpy(char *restrict d, const char *restrict s, size_t n) {
    stpncpy(d, s, n);
    return d;
}

char *strncat(char *restrict d, const char *restrict s, size_t n) {
    char *end = d + strlen(d);
    size_t len = strnlen(s, n);
    memcpy(end, s, len);
    end[len] = 0;
    return d;
}
