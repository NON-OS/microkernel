/* csock, part 5 of 11: included once, by csock.c. */

static int half_close(void) {
    int a, b;
    char buf[8];
    if (tcp_pair(&a, &b)) {
        return fail("half_close: pair", 0, errno);
    }
    if (shutdown(a, SHUT_WR)) {
        return fail("half_close: shutdown", -1, errno);
    }
    long got = read(b, buf, 8);
    if (got != 0) {
        return fail("half_close: peer reads end of file", got, errno);
    }
    if (write(b, "back", 4) != 4 || read(a, buf, 8) != 4) {
        return fail("half_close: the other way still flows", 0, errno);
    }
    if (write(a, "x", 1) != -1 || errno != EPIPE) {
        return fail("half_close: write after SHUT_WR is EPIPE", errno, EPIPE);
    }
    if (fcntl(a, F_GETFD) < 0) {
        return fail("half_close: shutdown closed the descriptor", errno, 0);
    }
    close(a);
    close(b);
    ok("half_close", "eof one way, 4 bytes back", 4);
    return 0;
}

static int epoll_listener(void) {
    int port, l = listener(&port, SOCK_NONBLOCK);
    int ep = epoll_create1(0);
    struct epoll_event ev = {.events = EPOLLIN, .data.fd = l}, out;
    epoll_ctl(ep, EPOLL_CTL_ADD, l, &ev);
    int before = epoll_wait(ep, &out, 1, 0);
    int c = dial(port);
    int after = epoll_wait(ep, &out, 1, 5000);
    int s = accept(l, 0, 0);
    int drained = epoll_wait(ep, &out, 1, 0);
    close(s);
    close(c);
    close(ep);
    close(l);
    if (before != 0 || after != 1 || !(out.events & EPOLLIN) || s < 0 || drained != 0) {
        return fail("epoll_listener: EPOLLIN only while pending", before * 10 + after, drained);
    }
    ok("epoll_listener", "EPOLLIN once pending, then", drained);
    return 0;
}

static int eof(void) {
    int a, b;
    char buf[8];
    if (tcp_pair(&a, &b)) {
        return fail("eof: pair", 0, errno);
    }
    if (write(a, "last", 4) != 4) {
        return fail("eof: write", -1, errno);
    }
    close(a);
    long first = read(b, buf, 8), then = read(b, buf, 8);
    close(b);
    if (first != 4 || then != 0) {
        return fail("eof: data then end of file", first, then);
    }
    ok("eof", "4 bytes then", then);
    return 0;
}
