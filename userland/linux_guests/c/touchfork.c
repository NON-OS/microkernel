// Bytes a guest wrote into a reservation survive a fork. A reservation is a
// PROT_NONE mapping; a program opens part of it with mprotect, writes, and may
// close it again. Linux keeps the bytes through all of that and gives the child
// a copy. Touching a page never opened is SIGSEGV, in the child as anywhere.
// MAP_FIXED over a mapping replaces it: the new pages read zero, as Linux says.
#include <signal.h>
#include <stdio.h>
#include <string.h>
#include <sys/mman.h>
#include <sys/wait.h>
#include <unistd.h>

#define PG 4096
#define PAGES 16

static int failed, passed;
static volatile char *r;

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
    snprintf(line, sizeof line, "[C] touchfork %s: %s (%s, status 0x%x)\n", name,
             ok ? "ok" : "FAIL", must_fault ? "must fault" : "must not fault", st);
    say(line);
}

static void check_bytes(void) {
    for (int i = 4; i < 8; i++) {
        if (r[i * PG] != (char)(0x40 + i) || r[i * PG + PG - 1] != (char)(0x50 + i)) {
            _exit(2);
        }
    }
}
static void open_then_check(void) {
    mprotect((void *)(r + 4 * PG), 4 * PG, PROT_READ);
    check_bytes();
}
static void touch_unopened(void) {
    (void)r[0];
}
static void check_zero(void) {
    if (r[0] != 0) {
        _exit(2);
    }
}

int main(void) {
    char line[160];
    r = mmap(0, PAGES * PG, PROT_NONE, MAP_PRIVATE | MAP_ANONYMOUS, -1, 0);
    mprotect((void *)(r + 4 * PG), 4 * PG, PROT_READ | PROT_WRITE);
    for (int i = 4; i < 8; i++) {
        r[i * PG] = (char)(0x40 + i);
        r[i * PG + PG - 1] = (char)(0x50 + i);
    }
    part("opened part of a reservation, child reads the bytes", check_bytes, 0);
    mprotect((void *)(r + 4 * PG), 4 * PG, PROT_NONE);
    part("closed again, child opens it and reads the bytes", open_then_check, 0);
    part("child touches a page never opened", touch_unopened, 1);

    r = mmap(0, PG, PROT_READ | PROT_WRITE, MAP_PRIVATE | MAP_ANONYMOUS, -1, 0);
    r[0] = 0x77;
    mmap((void *)r, PG, PROT_NONE, MAP_PRIVATE | MAP_ANONYMOUS | MAP_FIXED, -1, 0);
    mprotect((void *)r, PG, PROT_READ);
    part("MAP_FIXED PROT_NONE over written page, child reads zero", check_zero, 0);

    snprintf(line, sizeof line, "[C] touchfork %s: %d parts ok, %d failed\n",
             failed ? "FAIL" : "PASS", passed, failed);
    say(line);
    return failed ? 1 : 0;
}
