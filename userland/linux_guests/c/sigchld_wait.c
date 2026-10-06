/*
 * wait4 and waitid against Linux: WNOHANG on a running child, waitid WNOWAIT
 * looking without reaping, WUNTRACED, the status word of an exit and of a
 * death by signal, and EINVAL for an option Linux does not know.
 */
#include <errno.h>
#include <signal.h>
#include <string.h>
#include <sys/wait.h>
#include <unistd.h>
#include "sigchld.h"

void waits(void) {
    siginfo_t si;
    pid_t b = child_exiting(3, 300);
    int st = -1;
    part(wait4(b, &st, WNOHANG, 0) == 0, "wait4 WNOHANG on a running child answers 0");
    memset(&si, 0, sizeof si);
    si.si_pid = 1;
    int rc = waitid(P_PID, b, &si, WEXITED | WNOHANG);
    part(rc == 0 && si.si_pid == 0, "waitid WNOHANG on a running child answers 0, si_pid 0");
    memset(&si, 0, sizeof si);
    rc = waitid(P_PID, b, &si, WEXITED | WNOWAIT);
    part(rc == 0 && si.si_pid == b && si.si_status == 3, "waitid WNOWAIT reports and leaves it");
    rc = wait4(b, &st, WUNTRACED, 0);
    part(rc == b && WIFEXITED(st) && WEXITSTATUS(st) == 3,
         "wait4 WUNTRACED then reaps it: WIFEXITED, status 3");
    errno = 0;
    part(wait4(-1, &st, 0x100, 0) == -1 && errno == EINVAL, "wait4 with an unknown option: EINVAL");
    pid_t c = fork();
    if (c == 0) {
        signal(SIGTERM, SIG_DFL);
        kill(getpid(), SIGTERM);
        for (;;) pause();
    }
    rc = wait4(c, &st, 0, 0);
    part(rc == c && WIFSIGNALED(st) && WTERMSIG(st) == SIGTERM && !WIFEXITED(st),
         "a child ended by SIGTERM: WIFSIGNALED, WTERMSIG 15");
}
