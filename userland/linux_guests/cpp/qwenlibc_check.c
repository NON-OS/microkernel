/*
 * qwenlibc_check: a string in the last bytes of a page, a page no one may
 * read after it, and every call that measures a string with a bound past
 * the page. With zig's own strnlen this dies; with qwenlibc.c it passes.
 */
#include <limits.h>
#include <stdio.h>
#include <string.h>
#include <sys/mman.h>

int main(void) {
    char *p = mmap(0, 8192, PROT_READ | PROT_WRITE, MAP_PRIVATE | MAP_ANONYMOUS, -1, 0);
    if (p == MAP_FAILED || mprotect(p + 4096, 4096, PROT_NONE) != 0) return 2;
    const char *text = "qwen2.attention.layer_norm_rms_epsilon";
    char *s = p + 4096 - 48;
    memcpy(s, text, strlen(text) + 1);
    char out[64];
    int ok = strnlen(s, INT_MAX) == 38 && snprintf(out, sizeof out, "%s", s) == 38;
    strncpy(out, s, sizeof out);
    ok = ok && strcmp(out, text) == 0;
    out[0] = 0;
    strncat(out, s, sizeof out - 1);
    ok = ok && strcmp(out, text) == 0;
    puts(ok ? "qwenlibc: page-safe" : "qwenlibc: WRONG");
    return ok ? 0 : 1;
}
