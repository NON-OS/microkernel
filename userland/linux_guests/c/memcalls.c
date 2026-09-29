// The memory calls answered as Linux answers them: where mmap puts a hint, the
// break giving pages back, unaligned addresses refused, mremap keeping a
// mapping's protection, and mlock, msync and mincore with their errnos. Every
// part runs and prints one line, so one boot names every part that fails.
// Parts that must fault run in a forked child; the personality reports a
// signal death as exit status 128+signo, counted as the same SIGSEGV.
#define _GNU_SOURCE
#include <errno.h>
#include <fcntl.h>
#include <signal.h>
#include <stdio.h>
#include <string.h>
#include <sys/mman.h>
#include <sys/syscall.h>
#include <sys/wait.h>
#include <unistd.h>

#define PG 4096
#define NOREPLACE 0x100000

static int failed, passed;
static volatile char *p;

static void say(const char *s) {
    write(1, s, strlen(s));
}

static void check(const char *name, int ok, long got) {
    char line[160];
    ok ? passed++ : failed++;
    snprintf(line, sizeof line, "[C] memcalls %s: %s (got %ld)\n", name, ok ? "ok" : "FAIL", got);
    say(line);
}

// -errno of a call that returned -1, or its value.
static long rc(long v) {
    return v == -1 ? -errno : v;
}

static void faults(const char *name, void (*fn)(void)) {
    pid_t c = fork();
    if (c == 0) {
        fn();
        _exit(0);
    }
    int st = 0;
    waitpid(c, &st, 0);
    int segv = (WIFSIGNALED(st) && WTERMSIG(st) == SIGSEGV) ||
               (WIFEXITED(st) && WEXITSTATUS(st) == 128 + SIGSEGV);
    check(name, segv, st);
}

static void write_p(void) {
    p[0] = 1;
}
static void read_p(void) {
    (void)p[0];
}

static char *anon(long len, int prot) {
    return mmap(0, len, prot, MAP_PRIVATE | MAP_ANONYMOUS, -1, 0);
}

static void placement(void) {
    char *m = anon(PG, PROT_READ | PROT_WRITE);
    m[0] = 0x11;
    char *n = mmap(m, PG, PROT_READ | PROT_WRITE, MAP_PRIVATE | MAP_ANONYMOUS, -1, 0);
    check("hint on a mapping lands elsewhere, zeroed", n != m && n[0] == 0 && m[0] == 0x11,
          (long)(n - m));
    void *q = mmap(m, PG, PROT_READ, MAP_PRIVATE | MAP_ANONYMOUS | NOREPLACE, -1, 0);
    long e = q == MAP_FAILED ? -errno : 0;
    check("MAP_FIXED_NOREPLACE on a mapping is EEXIST", e == -EEXIST && m[0] == 0x11, e);
    // A mapping placed just above the last one mmap chose: the next mmap that
    // leaves the choice to the system must not land on it. On a system that
    // already holds that address the probe cannot be placed, and says so.
    char *a = anon(PG, PROT_READ | PROT_WRITE);
    char *f = mmap(a + PG, PG, PROT_READ | PROT_WRITE, MAP_PRIVATE | MAP_ANONYMOUS | NOREPLACE, -1, 0);
    if (f == MAP_FAILED) {
        check("next mmap skips a mapping placed above the last (probe address taken)",
              errno == EEXIST, -errno);
        return;
    }
    f[0] = 0x44;
    char *b = anon(PG, PROT_READ | PROT_WRITE);
    check("next mmap skips a mapping placed above the last", b != f && b[0] == 0 && f[0] == 0x44,
          (long)(b - f));
}

static void breaks(void) {
    long cur = syscall(SYS_brk, 0);
    long up = syscall(SYS_brk, cur + 2 * PG);
    ((volatile char *)cur)[PG] = 0x22;
    syscall(SYS_brk, cur);
    syscall(SYS_brk, cur + 2 * PG);
    char b = ((volatile char *)cur)[PG];
    check("brk down and up again reads zero", up == cur + 2 * PG && b == 0, b);
    syscall(SYS_brk, cur);
    long top = (cur + PG - 1) & ~(long)(PG - 1);
    void *in = mmap((void *)(top + PG), PG, PROT_READ, MAP_PRIVATE | MAP_ANONYMOUS | NOREPLACE, -1, 0);
    long got = syscall(SYS_brk, top + 4 * PG);
    check("brk into a mapping is refused", in != MAP_FAILED && got == cur, got - cur);
    munmap(in, PG);
}

static void alignment(void) {
    char *m = anon(2 * PG, PROT_READ | PROT_WRITE);
    check("munmap unaligned is EINVAL", rc(munmap(m + 1, PG)) == -EINVAL, rc(munmap(m + 1, PG)));
    // musl's mprotect rounds the address down itself; the kernel's does not.
    long e = rc(syscall(SYS_mprotect, m + 1, PG, PROT_READ));
    check("mprotect unaligned is EINVAL", e == -EINVAL, e);
    void *q = mmap(m + 1, PG, PROT_READ, MAP_PRIVATE | MAP_ANONYMOUS | MAP_FIXED, -1, 0);
    e = q == MAP_FAILED ? -errno : 0;
    check("MAP_FIXED unaligned is EINVAL", e == -EINVAL, e);
    // musl's mmap refuses an unaligned offset itself; the kernel's must too.
    e = rc(syscall(SYS_mmap, 0, PG, PROT_READ, MAP_PRIVATE | MAP_ANONYMOUS, -1, 1));
    check("mmap offset unaligned is EINVAL", e == -EINVAL, e);
}

static void remaps(void) {
    char *m = anon(PG, PROT_READ | PROT_WRITE);
    m[0] = 0x33;
    mprotect(m, PG, PROT_READ);
    char *g = mremap(m, PG, 2 * PG, MREMAP_MAYMOVE);
    check("mremap of a read-only page keeps the byte", g != MAP_FAILED && g[0] == 0x33, g[0]);
    p = g + PG;
    faults("mremap grown part of a read-only page faults on write", write_p);
    char *r = anon(2 * PG, PROT_NONE);
    anon(PG, PROT_NONE);
    char *q = mremap(r, 2 * PG, 4 * PG, MREMAP_MAYMOVE);
    check("mremap of a reservation succeeds", q != MAP_FAILED, (long)(q == MAP_FAILED));
    p = q + 3 * PG;
    faults("mremap grown reservation still faults on read", read_p);
    char *two = anon(2 * PG, PROT_READ | PROT_WRITE);
    mprotect(two + PG, PG, PROT_READ);
    void *x = mremap(two, 2 * PG, 3 * PG, MREMAP_MAYMOVE);
    long e = x == MAP_FAILED ? -errno : 0;
    check("mremap across two mappings is EFAULT", e == -EFAULT, e);
}

// A file mapped without exec was never proved; where mprotect refuses to make
// it executable, it must refuse the copy mremap moved too. Host Linux allows
// both, and the part checks only that the two answers agree.
static void provenance(void) {
    // This program's own file: /bin/memproof in the store, itself on a host.
    int fd = open("/bin/memproof", O_RDONLY);
    if (fd < 0) {
        fd = open("/proc/self/exe", O_RDONLY);
    }
    char *m = mmap(0, PG, PROT_READ, MAP_PRIVATE, fd, 0);
    if (m == MAP_FAILED) {
        check("file mapping for the provenance part", 0, -errno);
        return;
    }
    long first = rc(mprotect(m, PG, PROT_READ | PROT_EXEC));
    mprotect(m, PG, PROT_READ);
    mmap(m + PG, PG, PROT_READ, MAP_PRIVATE | MAP_ANONYMOUS | NOREPLACE, -1, 0);
    char *n = mremap(m, PG, 2 * PG, MREMAP_MAYMOVE);
    long moved = n == MAP_FAILED ? -1000 : rc(mprotect(n, PG, PROT_READ | PROT_EXEC));
    check("moved unproven file bytes refused exec as before the move", n != m && moved == first,
          moved);
    close(fd);
}

static void locks(void) {
    char *m = anon(3 * PG, PROT_READ | PROT_WRITE);
    check("mlock of a mapping", rc(mlock(m, 3 * PG)) == 0, rc(mlock(m, 3 * PG)));
    check("munlock of a mapping", rc(munlock(m, 3 * PG)) == 0, rc(munlock(m, 3 * PG)));
    munmap(m + PG, PG);
    check("mlock over a hole is ENOMEM", rc(mlock(m, 3 * PG)) == -ENOMEM, rc(mlock(m, 3 * PG)));
    check("mlock2 with an unknown flag is EINVAL", rc(syscall(SYS_mlock2, m, PG, 2)) == -EINVAL,
          rc(syscall(SYS_mlock2, m, PG, 2)));
    check("mlock2 MLOCK_ONFAULT", rc(syscall(SYS_mlock2, m, PG, 1)) == 0,
          rc(syscall(SYS_mlock2, m, PG, 1)));
    check("mlockall(0) is EINVAL", rc(mlockall(0)) == -EINVAL, rc(mlockall(0)));
    check("mlockall(MCL_ONFAULT) alone is EINVAL", rc(mlockall(MCL_ONFAULT)) == -EINVAL,
          rc(mlockall(MCL_ONFAULT)));
    check("mlockall(MCL_CURRENT)", rc(mlockall(MCL_CURRENT)) == 0, rc(mlockall(MCL_CURRENT)));
    check("munlockall", rc(munlockall()) == 0, rc(munlockall()));
}

static void syncs(void) {
    char *m = anon(3 * PG, PROT_READ | PROT_WRITE);
    check("msync MS_SYNC", rc(msync(m, 3 * PG, MS_SYNC)) == 0, rc(msync(m, 3 * PG, MS_SYNC)));
    check("msync unaligned is EINVAL", rc(msync(m + 1, PG, MS_SYNC)) == -EINVAL,
          rc(msync(m + 1, PG, MS_SYNC)));
    check("msync MS_ASYNC|MS_SYNC is EINVAL", rc(msync(m, PG, MS_ASYNC | MS_SYNC)) == -EINVAL,
          rc(msync(m, PG, MS_ASYNC | MS_SYNC)));
    check("msync unknown flag is EINVAL", rc(msync(m, PG, 8)) == -EINVAL, rc(msync(m, PG, 8)));
    munmap(m + PG, PG);
    check("msync over a hole is ENOMEM", rc(msync(m, 3 * PG, MS_SYNC)) == -ENOMEM,
          rc(msync(m, 3 * PG, MS_SYNC)));
}

static void cores(void) {
    char *r = anon(4 * PG, PROT_NONE);
    mprotect(r + PG, PG, PROT_READ | PROT_WRITE);
    r[PG] = 1;
    unsigned char v[4] = { 9, 9, 9, 9 };
    long got = rc(mincore(r, 4 * PG, v));
    check("mincore of a reservation with one page opened is 0,1,0,0",
          got == 0 && v[0] == 0 && v[1] == 1 && v[2] == 0 && v[3] == 0,
          got ? got : v[0] | v[1] << 8 | v[2] << 16 | (long)v[3] << 24);
    check("mincore unaligned is EINVAL", rc(mincore(r + 1, PG, v)) == -EINVAL,
          rc(mincore(r + 1, PG, v)));
    munmap(r + 2 * PG, PG);
    check("mincore over a hole is ENOMEM", rc(mincore(r, 4 * PG, v)) == -ENOMEM,
          rc(mincore(r, 4 * PG, v)));
}

int main(void) {
    char line[160];
    placement();
    breaks();
    alignment();
    remaps();
    provenance();
    locks();
    syncs();
    cores();
    snprintf(line, sizeof line, "[C] memcalls %s: %d parts ok, %d failed\n",
             failed ? "FAIL" : "PASS", passed, failed);
    say(line);
    return failed ? 1 : 0;
}
