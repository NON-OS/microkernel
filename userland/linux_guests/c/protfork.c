// A fork gives the child the parent's mappings with the protection they have
// now, not the one they were made with. Each part changes a protection with
// mprotect, forks, and the child tries one access; the parent reads how the
// child ended. The personality reports a signal death as exit status
// 128+signo, which is counted as the same SIGSEGV; the raw status is printed.
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
    snprintf(line, sizeof line, "[C] protfork %s: %s (%s, status 0x%x)\n", name,
             ok ? "ok" : "FAIL", must_fault ? "must fault" : "must not fault", st);
    say(line);
}

static void write_it(void) {
    p[0] = 1;
    if (p[0] != 1) {
        _exit(2);
    }
}
static void read_it(void) {
    if (p[0] != 0x5a) {
        _exit(2);
    }
}

static volatile char *fresh(int pages) {
    volatile char *m =
        mmap(0, pages * PG, PROT_READ | PROT_WRITE, MAP_PRIVATE | MAP_ANONYMOUS, -1, 0);
    for (int i = 0; i < pages; i++) {
        m[i * PG] = 0x5a;
    }
    return m;
}

int main(void) {
    char line[160];
    p = fresh(1);
    mprotect((void *)p, PG, PROT_READ);
    part("RW to R, child writes", write_it, 1);
    part("RW to R, child reads the byte", read_it, 0);

    p = fresh(1);
    mprotect((void *)p, PG, PROT_NONE);
    part("RW to NONE, child reads", read_it, 1);

    p = fresh(1);
    mprotect((void *)p, PG, PROT_READ);
    mprotect((void *)p, PG, PROT_READ | PROT_WRITE);
    part("RW to R to RW, child writes", write_it, 0);

    volatile char *m = fresh(3);
    mprotect((void *)(m + PG), PG, PROT_READ);
    p = m;
    part("middle page R, child writes the first", write_it, 0);
    p = m + PG;
    part("middle page R, child writes the middle", write_it, 1);
    p = m + 2 * PG;
    part("middle page R, child writes the last", write_it, 0);

    snprintf(line, sizeof line, "[C] protfork %s: %d parts ok, %d failed\n",
             failed ? "FAIL" : "PASS", passed, failed);
    say(line);
    return failed ? 1 : 0;
}
