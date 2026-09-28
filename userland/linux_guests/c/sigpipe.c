/*
 * A write to a pipe no one can read raises SIGPIPE on the writing thread, as
 * Linux does. Caught, the handler runs and the write returns EPIPE; ignored,
 * the write returns EPIPE and nothing else happens; left at its default, it
 * ends the process, which a parent sees as a death by signal 13.
 */
#include <errno.h>
#include <signal.h>
#include <stdio.h>
#include <string.h>
#include <sys/wait.h>
#include <unistd.h>

static volatile sig_atomic_t caught;
static int failed, parts;

static void on_pipe(int sig) { caught = sig; }

static void part(int ok, const char *what) {
    printf("[C] sigpipe %s: %s\n", ok ? "ok" : "FAIL", what);
    fflush(stdout);
    failed += !ok;
    parts++;
}

/* A pipe whose read end is already closed; the write end is returned. */
static int widowed(void) {
    int p[2];
    if (pipe(p) != 0) {
        return -1;
    }
    close(p[0]);
    return p[1];
}

int main(void) {
    struct sigaction sa;
    memset(&sa, 0, sizeof sa);
    sa.sa_handler = on_pipe;
    sigaction(SIGPIPE, &sa, 0);
    int w = widowed();
    errno = 0;
    ssize_t n = write(w, "x", 1);
    int e = errno;
    part(caught == SIGPIPE && n == -1 && e == EPIPE, "caught: the handler ran, write gave EPIPE");
    close(w);

    signal(SIGPIPE, SIG_IGN);
    caught = 0;
    w = widowed();
    errno = 0;
    n = write(w, "x", 1);
    e = errno;
    part(caught == 0 && n == -1 && e == EPIPE, "ignored: write gave EPIPE, no handler");
    close(w);

    pid_t c = fork();
    if (c == 0) {
        signal(SIGPIPE, SIG_DFL);
        int cw = widowed();
        write(cw, "x", 1);
        _exit(0);
    }
    int st = 0;
    pid_t got = waitpid(c, &st, 0);
    part(got == c && WIFSIGNALED(st) && WTERMSIG(st) == SIGPIPE,
         "default: the writer died of signal 13");

    printf("[C] sigpipe %s: %d of %d parts held\n", failed ? "FAIL" : "PASS", parts - failed,
           parts);
    fflush(stdout);
    return failed != 0;
}
