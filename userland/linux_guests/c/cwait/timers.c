#include "cwait.h"

int timers(void) {
    int tf = timerfd_create(CLOCK_MONOTONIC, TFD_NONBLOCK | TFD_CLOEXEC);
    uint64_t n = 0;
    if (read(tf, &n, 8) != -1 || errno != EAGAIN) {
        return fail("unarmed timer read", (long)n, errno);
    }
    struct itimerspec every = {{0, 50 * 1000000}, {0, 50 * 1000000}};
    long armed = now_ms();
    timerfd_settime(tf, 0, &every, 0);
    long set = now_ms();
    nap_ms(180);
    struct itimerspec now;
    timerfd_gettime(tf, &now);
    long before = now_ms();
    ssize_t got = read(tf, &n, 8);
    long after = now_ms();
    /* Every 50 ms that passed between arming and the read, however late the
     * nap woke: bounds from the clock, a millisecond either side. */
    long least = (before - set - 1) / 50, most = (after - armed + 1) / 50;
    if (got != 8 || least < 3 || (long)n < least || (long)n > most ||
        now.it_interval.tv_nsec != 50 * 1000000) {
        return fail("periodic timer count", (long)n, after - armed);
    }
    long periodic = (long)n, span = after - armed;
    close(tf);
    tf = timerfd_create(CLOCK_MONOTONIC, 0);
    struct itimerspec once = {{0, 0}, {0, 100 * 1000000}};
    timerfd_settime(tf, 0, &once, 0);
    long t0 = now_ms();
    got = read(tf, &n, 8);
    long waited = now_ms() - t0;
    if (got != 8 || n != 1 || waited < 90) {
        return fail("blocking timer read", (long)n, waited);
    }
    struct timespec at;
    clock_gettime(CLOCK_MONOTONIC, &at);
    at.tv_nsec += 100 * 1000000;
    if (at.tv_nsec >= 1000000000) {
        at.tv_sec++;
        at.tv_nsec -= 1000000000;
    }
    struct itimerspec abs = {{0, 0}, at};
    timerfd_settime(tf, TFD_TIMER_ABSTIME, &abs, 0);
    int ep = epoll_create1(0);
    struct epoll_event ev = {.events = EPOLLIN, .data.u64 = 3};
    epoll_ctl(ep, EPOLL_CTL_ADD, tf, &ev);
    t0 = now_ms();
    int ready = epoll_wait(ep, &ev, 1, -1);
    long absolute = now_ms() - t0;
    close(ep);
    close(tf);
    if (ready != 1 || absolute < 90 || absolute > 2000) {
        return fail("absolute timer through epoll", ready, absolute);
    }
    printf("[C] cwait timerfd detail: periodic 50ms fired %ld times in %ld ms, one-shot read waited "
           "%ld ms, absolute +100ms woke epoll after %ld ms\n",
           periodic, span, waited, absolute);
    ok("timerfd", "periodic, one-shot blocking and absolute timers; periodic count", periodic);
    return 0;
}
