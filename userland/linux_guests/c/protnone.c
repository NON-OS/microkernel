// PROT_NONE means no access. Each part runs in a forked child and the parent
// reads how the child ended: a part that must fault passes only when the child
// dies of SIGSEGV, a part that must not fault passes only when it exits 0.
// Every part runs, so one boot names every part that fails. The personality
// reports a signal death as exit status 128+signo, which is counted as the
// same SIGSEGV; the raw status is printed either way.
#include <signal.h>
#include <stdio.h>
#include <string.h>
#include <sys/mman.h>
#include <sys/wait.h>
#include <unistd.h>

#define PG 4096

static int failed, passed;
static volatile char *p;

static void say(const char *s) {
    write(1, s, strlen(s));
}

static void part(const char *name, void (*fn)(void), int must_fault) {
    char line[160];
    pid_t c = fork();
    if (c == 0) {
        fn();
        _exit(0);
    }
    int st = 0;
    waitpid(c, &st, 0);
    int segv = (WIFSIGNALED(st) && WTERMSIG(st) == SIGSEGV) ||
               (WIFEXITED(st) && WEXITSTATUS(st) == 128 + SIGSEGV);
    int clean = WIFEXITED(st) && WEXITSTATUS(st) == 0;
    int ok = must_fault ? segv : clean;
    ok ? passed++ : failed++;
    snprintf(line, sizeof line, "[C] protnone %s: %s (%s, status 0x%x)\n", name,
             ok ? "ok" : "FAIL", must_fault ? "must fault" : "must not fault", st);
    say(line);
}

static void read_it(void) {
    if (p[0] != 0x5a) {
        _exit(2);
    }
}
static void write_it(void) {
    p[0] = 1;
}
static void read_below(void) {
    (void)p[-1];
}

int main(void) {
    char line[160];
    p = mmap(0, PG, PROT_NONE, MAP_PRIVATE | MAP_ANONYMOUS, -1, 0);
    part("read of an mmap PROT_NONE page", read_it, 1);
    part("write to an mmap PROT_NONE page", write_it, 1);

    p = mmap(0, PG, PROT_READ | PROT_WRITE, MAP_PRIVATE | MAP_ANONYMOUS, -1, 0);
    p[0] = 0x5a;
    mprotect((void *)p, PG, PROT_NONE);
    part("read after mprotect RW to PROT_NONE", read_it, 1);
    mprotect((void *)p, PG, PROT_READ);
    part("read after PROT_NONE back to R keeps the byte", read_it, 0);
    part("write to a PROT_READ page", write_it, 1);

    char *r = mmap(0, 3 * PG, PROT_NONE, MAP_PRIVATE | MAP_ANONYMOUS, -1, 0);
    mprotect(r + PG, PG, PROT_READ | PROT_WRITE);
    p = (volatile char *)(r + PG);
    p[0] = 0x5a;
    part("read of the opened page of a reservation", read_it, 0);
    part("read of the closed page below it", read_below, 1);

    snprintf(line, sizeof line, "[C] protnone %s: %d parts ok, %d failed\n",
             failed ? "FAIL" : "PASS", passed, failed);
    say(line);
    return failed ? 1 : 0;
}
