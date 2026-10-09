#include "cwait.h"

int close_forgets(void) {
    int ep = epoll_create1(0);
    int p[2];
    pipe(p);
    write(p[1], "x", 1);
    struct epoll_event ev = {.events = EPOLLIN, .data.u64 = 7};
    epoll_ctl(ep, EPOLL_CTL_ADD, p[0], &ev);
    int number = p[0];
    close(p[0]);
    close(p[1]);
    int q[2];
    pipe(q);
    int reused = q[0] == number;
    int added = epoll_ctl(ep, EPOLL_CTL_ADD, q[0], &ev);
    struct epoll_event got;
    int stale = epoll_wait(ep, &got, 1, 0);
    int far = dup2(0, 100000) == -1 && errno == EBADF;
    close(q[0]);
    close(q[1]);
    close(ep);
    if (!reused || added != 0 || stale != 0 || !far) {
        return fail("close leaves epoll", added * 10 + stale, reused * 10 + far);
    }
    ok("close-forget", "a closed descriptor left epoll and its number was added again; stale events",
       stale);
    return 0;
}
