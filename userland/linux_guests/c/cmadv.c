/*
 * cmadv: madvise as Linux keeps it. MADV_DONTNEED leaves private anonymous
 * pages reading zero and their neighbours as they were, in a fresh mapping, a
 * read-only one, and a PROT_NONE reservation made readable, and a child forked
 * afterwards sees the same bytes. A hint changes nothing. Unknown advice and a
 * misaligned address are EINVAL, an unmapped span ENOMEM. On a file mapping
 * Linux reloads the file's bytes; refusing with EINVAL is the other honest
 * answer. Any difference prints a FAIL line and exits non-zero.
 */
#include <errno.h>
#include <fcntl.h>
#include <stdio.h>
#include <string.h>
#include <sys/mman.h>
#include <sys/wait.h>
#include <unistd.h>

#define PG 4096L
static int bad;

static void check(const char *what, int ok, long a, long b) {
    printf("[C] cmadv %s %s (%ld, %ld)\n", what, ok ? "ok" : "FAIL", a, b);
    bad += !ok;
}

static long zeros(const unsigned char *p, long n) {
    long z = 0;
    for (long i = 0; i < n; i++) z += p[i] == 0;
    return z;
}

int main(void) {
    unsigned char *m = mmap(0, 16 * PG, PROT_READ | PROT_WRITE, MAP_PRIVATE | MAP_ANONYMOUS, -1, 0);
    memset(m, 0xAB, 16 * PG);
    int rc = madvise(m + 4 * PG, 8 * PG, MADV_DONTNEED);
    check("dontneed", rc == 0 && zeros(m + 4 * PG, 8 * PG) == 8 * PG && m[0] == 0xAB &&
          m[4 * PG - 1] == 0xAB && m[12 * PG] == 0xAB, rc, zeros(m + 4 * PG, 8 * PG));
    m[5 * PG] = 7;
    pid_t c = fork();
    if (c == 0) _exit(m[5 * PG] == 7 && m[6 * PG] == 0 && m[0] == 0xAB ? 0 : 1);
    int st = -1;
    waitpid(c, &st, 0);
    check("fork after", WIFEXITED(st) && WEXITSTATUS(st) == 0, st, 0);
    mprotect(m, 4 * PG, PROT_READ);
    rc = madvise(m, PG, MADV_DONTNEED);
    check("read-only", rc == 0 && m[0] == 0 && m[PG] == 0xAB, rc, m[0]);
    unsigned char *r = mmap(0, 8 * PG, PROT_NONE, MAP_PRIVATE | MAP_ANONYMOUS, -1, 0);
    mprotect(r, 8 * PG, PROT_READ | PROT_WRITE);
    memset(r, 0x5A, 8 * PG);
    rc = madvise(r, 8 * PG, MADV_DONTNEED);
    check("reserved", rc == 0 && zeros(r, 8 * PG) == 8 * PG, rc, zeros(r, 8 * PG));
    rc = madvise(m + 12 * PG, 4 * PG, MADV_WILLNEED);
    check("hint", rc == 0 && m[12 * PG] == 0xAB, rc, m[12 * PG]);
    long e1 = madvise(m, PG, 12345) == -1 ? errno : 0;
    long e2 = madvise(m + 1, PG, MADV_DONTNEED) == -1 ? errno : 0;
    check("einval", e1 == EINVAL && e2 == EINVAL, e1, e2);
    munmap(m + 14 * PG, 2 * PG);
    long e3 = madvise(m + 12 * PG, 4 * PG, MADV_DONTNEED) == -1 ? errno : 0;
    check("enomem", e3 == ENOMEM, e3, 0);
    int fd = open("/bin/busybox", O_RDONLY);
    if (fd < 0) fd = open("/bin/sh", O_RDONLY);
    unsigned char *f = mmap(0, PG, PROT_READ, MAP_PRIVATE, fd, 0);
    rc = madvise(f, PG, MADV_DONTNEED);
    check("file", (rc == 0 && f[0] == 0x7f) || (rc == -1 && errno == EINVAL), rc, f[0]);
    printf("[C] cmadv %s: %d failed\n", bad ? "FAIL" : "PASS", bad);
    return bad != 0;
}
