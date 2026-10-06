/*
 * musl's own ways to start a program, which clone a vfork child onto a stack
 * of its own: posix_spawn, system through the shell, and popen reading the
 * child's output through a pipe. Then, with every child reaped, wait4 and
 * waitid answer ECHILD.
 */
#include <errno.h>
#include <signal.h>
#include <spawn.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/wait.h>
#include "sigchld.h"

extern char **environ;

void spawns(void) {
    pid_t d = -1;
    int st = -1;
    char *argv[] = {"/bin/cthreads", 0};
    int rc = posix_spawn(&d, "/bin/cthreads", 0, 0, argv, environ);
    part(rc == 0 && waitpid(d, &st, 0) == d && WIFEXITED(st) && WEXITSTATUS(st) == 0,
         "posix_spawn runs cthreads, status 0");
    st = system("/bin/leaderexit");
    part(WIFEXITED(st) && WEXITSTATUS(st) == 42, "system runs leaderexit through sh, status 42");
    char line[128] = {0};
    FILE *f = popen("/bin/cthreads", "r");
    int got_line = f && fgets(line, sizeof line, f) != 0;
    int closed = f ? pclose(f) : -1;
    part(got_line && strstr(line, "cthreads PASS") && WIFEXITED(closed) && WEXITSTATUS(closed) == 0,
         "popen reads cthreads' PASS line through a pipe");
    siginfo_t si;
    errno = 0;
    part(wait4(-1, &st, 0, 0) == -1 && errno == ECHILD, "wait4 with no child left: ECHILD");
    errno = 0;
    part(waitid(P_ALL, 0, &si, WEXITED) == -1 && errno == ECHILD,
         "waitid with no child left: ECHILD");
}
