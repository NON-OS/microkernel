/* csock, part 6 of 11: included once, by csock.c. */

/* csock: receiving without waiting, waiting, options, and fork. */

static int empty_recv(void) {
    int a, b;
    char buf[8];
    if (tcp_pair(&a, &b)) {
        return fail("empty_recv: pair", 0, errno);
    }
    if (recv(b, buf, 8, MSG_DONTWAIT) != -1 || errno != EAGAIN) {
        return fail("empty_recv: MSG_DONTWAIT", errno, EAGAIN);
    }
    fcntl(b, F_SETFL, O_NONBLOCK);
    if (read(b, buf, 8) != -1 || errno != EAGAIN) {
        return fail("empty_recv: O_NONBLOCK read", errno, EAGAIN);
    }
    close(a);
    close(b);
    ok("empty_recv", "EAGAIN, errno", EAGAIN);
    return 0;
}

static int peek(void) {
    int a, b;
    char buf[8] = {0};
    if (tcp_pair(&a, &b)) {
        return fail("peek: pair", 0, errno);
    }
    if (write(a, "abc", 3) != 3) {
        return fail("peek: write", -1, errno);
    }
    long p = recv(b, buf, 8, MSG_PEEK);
    long r = recv(b, buf, 8, 0);
    close(a);
    close(b);
    if (p != 3 || r != 3 || memcmp(buf, "abc", 3)) {
        return fail("peek: data stays", p, r);
    }
    ok("peek", "peeked then read", r);
    return 0;
}

static int epipe(void) {
    int a, b;
    if (tcp_pair(&a, &b)) {
        return fail("epipe: pair", 0, errno);
    }
    close(b);
    long first = send(a, "x", 1, MSG_NOSIGNAL);
    long second = send(a, "x", 1, MSG_NOSIGNAL);
    int e = errno;
    close(a);
    if (first != 1 || second != -1 || e != EPIPE) {
        return fail("epipe: first send taken, then EPIPE", first, e);
    }
    ok("epipe", "second send errno", e);
    return 0;
}

static int late_port;

static void *late_dial(void *arg) {
    (void)arg;
    nap_ms(100);
    return (void *)(long)dial(late_port);
}
