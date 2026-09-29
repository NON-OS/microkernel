/*
 * signalfd, checked against Linux: a blocked SIGUSR1 raised at the process
 * reads back from a signalfd as a record with its number and the sender's
 * pid, a non-blocking signalfd with nothing pending answers EAGAIN, poll
 * reports it readable once a signal waits, and a sigqueue value arrives in
 * ssi_int through a blocking read.
 */
#define _GNU_SOURCE
#include <errno.h>
#include <poll.h>
#include <sys/signalfd.h>
#include <unistd.h>

#include "alarm.h"

void signal_fd(void) {
    sigset_t set;
    sigemptyset(&set);
    sigaddset(&set, SIGUSR1);
    sigprocmask(SIG_BLOCK, &set, 0);
    int fd = signalfd(-1, &set, SFD_NONBLOCK | SFD_CLOEXEC);
    struct signalfd_siginfo si;
    errno = 0;
    ssize_t n = read(fd, &si, sizeof si);
    part(fd >= 0 && n == -1 && errno == EAGAIN, "signalfd with nothing pending: EAGAIN", fd);
    struct pollfd p = {fd, POLLIN, 0};
    int before = poll(&p, 1, 0);
    kill(getpid(), SIGUSR1);
    int after = poll(&p, 1, 0);
    part(before == 0 && after == 1 && (p.revents & POLLIN), "poll: readable once SIGUSR1 waits",
         after);
    n = read(fd, &si, sizeof si);
    part(n == sizeof si && si.ssi_signo == SIGUSR1 && si.ssi_pid == (unsigned)getpid(),
         "signalfd reads SIGUSR1 with the sender's pid", si.ssi_signo);
    close(fd);
    sigaddset(&set, SIGUSR2);
    int blocking = signalfd(-1, &set, 0);
    union sigval v = {.sival_int = 4321};
    sigqueue(getpid(), SIGUSR2, v);
    n = read(blocking, &si, sizeof si);
    part(n == sizeof si && si.ssi_signo == SIGUSR2 && si.ssi_int == 4321,
         "a blocking signalfd read takes sigqueue's value", si.ssi_int);
    close(blocking);
}
