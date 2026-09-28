// csock: receiving without waiting, waiting, options, and fork.

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
    write(a, "abc", 3);
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

static int blocking_accept(void) {
    int l = listener(&late_port, 0);
    pthread_t t;
    long t0 = now_ms();
    pthread_create(&t, 0, late_dial, 0);
    int s = accept(l, 0, 0);
    long waited = now_ms() - t0;
    void *c;
    pthread_join(t, &c);
    close((int)(long)c);
    close(s);
    close(l);
    if (s < 0 || waited < 80) {
        return fail("blocking_accept: waited for the connect", s, waited);
    }
    ok("blocking_accept", "waited ms", waited >= 80);
    return 0;
}

static int late_fd;
static void *late_write(void *arg) {
    (void)arg;
    nap_ms(100);
    write(late_fd, "late", 4);
    return 0;
}

static int blocking_recv(void) {
    int a, b;
    char buf[8];
    if (tcp_pair(&a, &b)) {
        return fail("blocking_recv: pair", 0, errno);
    }
    late_fd = a;
    pthread_t t;
    long t0 = now_ms();
    pthread_create(&t, 0, late_write, 0);
    long got = recv(b, buf, 8, 0);
    long waited = now_ms() - t0;
    pthread_join(t, 0);
    close(a);
    close(b);
    if (got != 4 || waited < 80) {
        return fail("blocking_recv: waited for the bytes", got, waited);
    }
    ok("blocking_recv", "4 bytes after waiting", waited >= 80);
    return 0;
}

#include "csock_parts4.h"
