#include "cwait.h"

int epoll_timeout(void) {
    int ep = epoll_create1(EPOLL_CLOEXEC);
    int fd = eventfd(0, EFD_NONBLOCK);
    struct epoll_event ev = {.events = EPOLLIN, .data.u64 = 0xfeed};
    epoll_ctl(ep, EPOLL_CTL_ADD, fd, &ev);
    long t0 = now_ms();
    int n = epoll_wait(ep, &ev, 1, 100);
    long took = now_ms() - t0;
    if (n != 0 || took < 95 || took > 2000) {
        return fail("epoll_wait 100ms", n, took);
    }
    ok("epoll-timeout", "epoll_wait 100ms with nothing ready answered 0 after ms", took);
    efd_late = fd;
    pthread_t t;
    t0 = now_ms();
    pthread_create(&t, 0, late_write, (void *)(uintptr_t)1);
    n = epoll_wait(ep, &ev, 1, -1);
    took = now_ms() - t0;
    pthread_join(t, 0);
    if (n != 1 || ev.events != EPOLLIN || ev.data.u64 != 0xfeed || took < 90) {
        return fail("epoll_wait woken by a write", n, took);
    }
    ok("epoll-wake", "epoll_wait with no timeout woken by another thread's write, ms", took);
    close(fd);
    close(ep);
    return 0;
}


int edge(void) {
    int p[2];
    pipe2(p, O_NONBLOCK);
    int ep = epoll_create1(0);
    struct epoll_event ev = {.events = EPOLLIN | EPOLLET, .data.u64 = 1};
    epoll_ctl(ep, EPOLL_CTL_ADD, p[0], &ev);
    struct epoll_event lt = {.events = EPOLLOUT, .data.u64 = 2};
    epoll_ctl(ep, EPOLL_CTL_ADD, p[1], &lt);
    struct epoll_event got[2];
    char c;
    write(p[1], "x", 1);
    int first = epoll_wait(ep, got, 2, 0);
    int again = epoll_wait(ep, got, 2, 0);
    read(p[0], &c, 1);
    int drained = read(p[0], &c, 1) == -1 && errno == EAGAIN;
    write(p[1], "y", 1);
    int rearmed = epoll_wait(ep, got, 2, 0);
    /* The write end is level-triggered and always writable: counted each time. */
    if (first != 2 || again != 1 || !drained || rearmed != 2) {
        return fail("edge-triggered counts", first * 100 + again * 10 + rearmed, drained);
    }
    if (epoll_ctl(ep, EPOLL_CTL_ADD, p[0], &ev) != -1 || errno != EEXIST) {
        return fail("adding twice", errno, EEXIST);
    }
    close(p[0]);
    close(p[1]);
    close(ep);
    ok("edge", "EPOLLET reported once per rise, again after EAGAIN; events", first + again + rearmed);
    return 0;
}
