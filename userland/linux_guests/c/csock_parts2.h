// csock: connecting, and closing one way or both.

static int nb_connect(void) {
    int port, l = listener(&port, 0);
    int c = socket(AF_INET, SOCK_STREAM | SOCK_NONBLOCK, 0);
    struct sockaddr_in sa = loopback(port);
    int rc = connect(c, (void *)&sa, sizeof sa);
    int e = errno;
    if (rc != -1 || e != EINPROGRESS) {
        return fail("nb_connect: EINPROGRESS", rc, e);
    }
    struct pollfd p = {c, POLLOUT, 0};
    if (poll(&p, 1, 5000) != 1 || !(p.revents & POLLOUT)) {
        return fail("nb_connect: POLLOUT", p.revents, 0);
    }
    int err = -1;
    socklen_t len = sizeof err;
    if (getsockopt(c, SOL_SOCKET, SO_ERROR, &err, &len) || err != 0) {
        return fail("nb_connect: SO_ERROR", err, errno);
    }
    close(c);
    close(l);
    ok("nb_connect", "EINPROGRESS, POLLOUT, SO_ERROR", err);
    return 0;
}

// A port with no listener: a listener is opened and closed to find one.
static int refused(void) {
    int port, l = listener(&port, 0);
    close(l);
    int c = dial(port);
    if (c != -ECONNREFUSED) {
        return fail("refused: blocking connect", c, -ECONNREFUSED);
    }
    int n = socket(AF_INET, SOCK_STREAM | SOCK_NONBLOCK, 0);
    struct sockaddr_in sa = loopback(port);
    int rc = connect(n, (void *)&sa, sizeof sa);
    int e = errno;
    struct pollfd p = {n, POLLOUT, 0};
    poll(&p, 1, 5000);
    int err = 0;
    socklen_t len = sizeof err;
    getsockopt(n, SOL_SOCKET, SO_ERROR, &err, &len);
    close(n);
    if (rc != -1 || e != EINPROGRESS || err != ECONNREFUSED || !(p.revents & POLLERR)) {
        return fail("refused: non-blocking connect then SO_ERROR", e, err);
    }
    ok("refused", "ECONNREFUSED both ways, errno", ECONNREFUSED);
    return 0;
}

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
    write(a, "last", 4);
    close(a);
    long first = read(b, buf, 8), then = read(b, buf, 8);
    close(b);
    if (first != 4 || then != 0) {
        return fail("eof: data then end of file", first, then);
    }
    ok("eof", "4 bytes then", then);
    return 0;
}

#include "csock_parts3.h"
