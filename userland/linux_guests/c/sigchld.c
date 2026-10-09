/*
 * A parent told about its children as Linux tells it. A child's exit raises
 * SIGCHLD at a parent that catches it, with the child's pid, CLD_EXITED and
 * its code in the siginfo; waitid(P_ALL, WEXITED) then reports the same
 * child. The wait4 and waitid checks follow in sigchld_wait.c and musl's ways
 * to start a program in sigchld_spawn.c. Every part runs and prints its line;
 * the last line is PASS only if all of them held.
 */
#include <signal.h>
#include <stdio.h>
#include <string.h>
#include <sys/wait.h>
#include <time.h>
#include <unistd.h>
#include "sigchld.h"

static volatile sig_atomic_t got, got_code, got_status;
static volatile pid_t got_pid;
static int failed, parts;

static void on_chld(int sig, siginfo_t *info, void *uc) {
    (void)uc;
    got = sig;
    got_pid = info->si_pid;
    got_code = info->si_code;
    got_status = info->si_status;
}

void part(int ok, const char *what) {
    printf("[C] sigchld %s: %s\n", ok ? "ok" : "FAIL", what);
    fflush(stdout);
    failed += !ok;
    parts++;
}

pid_t child_exiting(int code, unsigned delay_ms) {
    pid_t pid = fork();
    if (pid == 0) {
        struct timespec t = {delay_ms / 1000, (delay_ms % 1000) * 1000000L};
        nanosleep(&t, 0);
        _exit(code);
    }
    return pid;
}

int main(void) {
    struct sigaction sa;
    memset(&sa, 0, sizeof sa);
    sa.sa_sigaction = on_chld;
    sa.sa_flags = SA_SIGINFO | SA_RESTART;
    sigaction(SIGCHLD, &sa, 0);
    pid_t a = child_exiting(7, 0);
    /* Sleep in short steps until the handler has run or three seconds pass. */
    for (int i = 0; i < 300 && !got; i++) {
        struct timespec t = {0, 10 * 1000 * 1000};
        nanosleep(&t, 0);
    }
    part(got == SIGCHLD && got_pid == a && got_code == CLD_EXITED && got_status == 7,
         "SIGCHLD caught with the child's pid, CLD_EXITED and status 7");
    siginfo_t si;
    memset(&si, 0, sizeof si);
    int rc = waitid(P_ALL, 0, &si, WEXITED);
    part(rc == 0 && si.si_pid == a && si.si_signo == SIGCHLD && si.si_code == CLD_EXITED &&
             si.si_status == 7,
         "waitid(P_ALL, WEXITED) reports the child, CLD_EXITED, status 7");
    waits();
    spawns();
    printf("[C] sigchld %s: %d of %d parts held\n", failed ? "FAIL" : "PASS", parts - failed,
           parts);
    fflush(stdout);
    return failed != 0;
}
