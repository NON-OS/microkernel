#include "cwait.h"

int ioctls_fork(void) {
    int p[2];
    pipe(p);
    write(p[1], "abc", 3);
    int held = -1;
    ioctl(p[0], FIONREAD, &held);
    int one = 1;
    ioctl(p[0], FIONBIO, &one);
    char buf[4];
    ssize_t got = read(p[0], buf, sizeof buf);
    int drained = read(p[0], buf, 1) == -1 && errno == EAGAIN;
    ioctl(p[1], FIOCLEX);
    int cloexec = fcntl(p[1], F_GETFD) == FD_CLOEXEC;
    if (held != 3 || got != 3 || !drained || !cloexec) {
        return fail("descriptor ioctls", held, got * 10 + drained * 2 + cloexec);
    }
    int ep = epoll_create1(0);
    struct epoll_event ev = {.events = EPOLLIN, .data.u64 = 9};
    epoll_ctl(ep, EPOLL_CTL_ADD, p[0], &ev);
    write(p[1], "d", 1);
    pid_t child = fork();
    if (child == 0) {
        _exit(epoll_wait(ep, &ev, 1, 0) == 1 && ev.data.u64 == 9 ? 0 : 1);
    }
    int status = -1;
    waitpid(child, &status, 0);
    close(ep);
    close(p[0]);
    close(p[1]);
    if (child < 0 || !WIFEXITED(status) || WEXITSTATUS(status) != 0) {
        return fail("epoll list through fork", child, status);
    }
    ok("ioctl-fork", "FIONREAD, FIONBIO, FIOCLEX, and a forked child saw the epoll list; bytes held",
       held);
    return 0;
}

/* musl answers ENOSYS for the policy calls by design, so they are made raw.
 * SCHED_FIFO at priority 0 is EINVAL everywhere, the range being checked
 * before the privilege; at priority 1 it depends on privilege, and a CPU-1
 * mask on the CPU count, so those two are reported rather than checked: on
 * NONOS they are EPERM and EINVAL. */
