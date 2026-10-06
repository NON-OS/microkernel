/* csock, part 9 of 11: included once, by csock.c. */

/*
 * The child holds the accepted end after the parent closes its own: the
 * client still reads the child's bytes, then end of file when the child exits.
 */
static int fork_share(void) {
    int a, b;
    char buf[8];
    if (tcp_pair(&a, &b)) {
        return fail("fork_share: pair", 0, errno);
    }
    pid_t kid = fork();
    if (kid == 0) {
        close(a);
        nap_ms(50);
        _exit(write(b, "kid", 3) == 3 ? 0 : 1);
    }
    close(b);
    long got = read(a, buf, 8);
    int status = 0;
    waitpid(kid, &status, 0);
    long then = read(a, buf, 8);
    close(a);
    if (got != 3 || then != 0 || status != 0) {
        return fail("fork_share: child's bytes, then eof", got, then);
    }
    ok("fork_share", "3 bytes from the child, then", then);
    return 0;
}

/*
 * A listener with a backlog of 0 queues one connect; the next non-blocking
 * one answers EINPROGRESS, is not writable, and a second connect on it is
 * EALREADY, until an accept makes room and it completes.
 */
static int backlog(void) {
    int s = socket(AF_INET, SOCK_STREAM, 0);
    struct sockaddr_in sa = loopback(0);
    socklen_t len = sizeof sa;
    bind(s, (void *)&sa, sizeof sa);
    getsockname(s, (void *)&sa, &len);
    listen(s, 0);
    int c0 = socket(AF_INET, SOCK_STREAM | SOCK_NONBLOCK, 0);
    int c1 = socket(AF_INET, SOCK_STREAM | SOCK_NONBLOCK, 0);
    connect(c0, (void *)&sa, sizeof sa);
    int r1 = connect(c1, (void *)&sa, sizeof sa);
    int e1 = errno;
    struct pollfd p = {c1, POLLOUT, 0};
    int early = poll(&p, 1, 100);
    int again = connect(c1, (void *)&sa, sizeof sa) ? errno : 0;
    int a = accept(s, 0, 0);
    p.revents = 0;
    int later = poll(&p, 1, 3000);
    int err = -1;
    socklen_t el = sizeof err;
    getsockopt(c1, SOL_SOCKET, SO_ERROR, &err, &el);
    struct pollfd q = {s, POLLIN, 0};
    int b = poll(&q, 1, 3000) == 1 ? accept(s, 0, 0) : -1;
    close(a);
    close(b);
    close(c0);
    close(c1);
    close(s);
    if (r1 != -1 || e1 != EINPROGRESS || early != 0 || again != EALREADY || later != 1 ||
        err != 0 || b < 0) {
        printf("[C] csock backlog got %d %d %d %d %d %d %d\n", r1, e1, early, again, later, err, b);
        return fail("backlog: EINPROGRESS, waits, EALREADY, then connected", e1, again);
    }
    ok("backlog", "a full queue's connect completed after accept; EALREADY", again);
    return 0;
}
