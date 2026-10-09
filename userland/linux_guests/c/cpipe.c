/*
 * cpipe: bytes through a pipe as Linux moves them. A child writes 1 MiB with
 * writev and 200000 bytes with one write, both to a blocking pipe, and each
 * must answer its whole length. The parent reads everything back with readv
 * in uneven pieces and checks every byte. A non-blocking writer to a pipe
 * with no reader draining it gets a short count, then EAGAIN. Any difference
 * prints a FAIL line and exits non-zero.
 */
#define _GNU_SOURCE
#include <errno.h>
#include <fcntl.h>
#include <stdio.h>
#include <sys/uio.h>
#include <sys/wait.h>
#include <unistd.h>

#define BIG (1 << 20)
#define ONE 200000
static unsigned char out[BIG], in[7777];

static int fail(const char *what, long a, long b) {
    printf("[C] cpipe FAIL: %s (%ld, %ld)\n", what, a, b);
    return 1;
}

static long check(long got, long n, long bad) {
    for (long i = 0; i < n && bad < 0; i++) {
        long at = got + i;
        if (in[i] != out[at < BIG ? at : at - BIG]) bad = at;
    }
    return bad;
}

int main(void) {
    for (long i = 0; i < BIG; i++) out[i] = (unsigned char)(i * 131 + (i >> 9));
    int p[2];
    if (pipe(p)) return fail("pipe", errno, 0);
    pid_t child = fork();
    if (child < 0) return fail("fork", errno, 0);
    if (child == 0) {
        close(p[0]);
        struct iovec v[3] = {{out, 1000}, {out + 1000, 300000}, {out + 301000, BIG - 301000}};
        ssize_t a = writev(p[1], v, 3);
        ssize_t b = write(p[1], out, ONE);
        _exit(a == BIG && b == ONE ? 0 : 3);
    }
    close(p[1]);
    long got = 0, bad = -1;
    for (;;) {
        struct iovec v[2] = {{in, 1234}, {in + 1234, sizeof in - 1234}};
        ssize_t n = readv(p[0], v, 2);
        if (n < 0) return fail("readv", got, errno);
        if (n == 0) break;
        bad = check(got, n, bad);
        got += n;
    }
    int st = 0;
    waitpid(child, &st, 0);
    if (got != BIG + ONE || bad >= 0) return fail("bytes read back", got, bad);
    if (!WIFEXITED(st) || WEXITSTATUS(st) != 0) return fail("writer's counts", st, 0);
    int q[2];
    if (pipe2(q, O_NONBLOCK)) return fail("pipe2", errno, 0);
    ssize_t first = write(q[1], out, BIG);
    ssize_t again = write(q[1], out, 10);
    if (first <= 0 || first >= BIG || again != -1 || errno != EAGAIN) {
        return fail("non-blocking writer", first, again);
    }
    printf("[C] cpipe PASS: %ld bytes through writev, write and readv; non-blocking took %ld, "
           "then EAGAIN\n", got, (long)first);
    return 0;
}
