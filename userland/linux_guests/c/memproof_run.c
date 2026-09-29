/* The parts of memproof.h every proof shares. */
#include <signal.h>
#include <stdio.h>
#include <string.h>
#include <sys/wait.h>
#include <unistd.h>

#include "memproof.h"

static int passed, failed;

void say(const char *s) {
    write(1, s, strlen(s));
}

int segv(int st) {
    return (WIFSIGNALED(st) && WTERMSIG(st) == SIGSEGV) ||
           (WIFEXITED(st) && WEXITSTATUS(st) == 128 + SIGSEGV);
}

void part_line(const char *proof, const char *name, int ok, const char *detail) {
    char line[200];
    ok ? passed++ : failed++;
    snprintf(line, sizeof line, "[C] %s %s: %s (%s)\n", proof, name, ok ? "ok" : "FAIL", detail);
    say(line);
}

void part(const char *proof, const char *name, void (*fn)(void), int must_fault) {
    char detail[64];
    pid_t c = fork();
    if (c < 0) {
        part_line(proof, name, 0, "fork failed");
        return;
    }
    if (c == 0) {
        fn();
        _exit(0);
    }
    int st = 0;
    waitpid(c, &st, 0);
    int clean = WIFEXITED(st) && WEXITSTATUS(st) == 0;
    snprintf(detail, sizeof detail, "%s, status 0x%x", must_fault ? "must fault" : "must not fault",
             st);
    part_line(proof, name, must_fault ? segv(st) : clean, detail);
}

int finish(const char *proof) {
    char line[160];
    snprintf(line, sizeof line, "[C] %s %s: %d parts ok, %d failed\n", proof,
             failed ? "FAIL" : "PASS", passed, failed);
    say(line);
    return failed ? 1 : 0;
}
