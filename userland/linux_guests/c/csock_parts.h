// The parts of csock, one Linux behaviour each. Included once, by csock.c.

static int pair_stream(void) {
    int sv[2];
    char buf[8] = {0};
    if (socketpair(AF_UNIX, SOCK_STREAM, 0, sv)) {
        return fail("pair_stream: socketpair", -1, errno);
    }
    if (write(sv[0], "ping", 4) != 4 || read(sv[1], buf, 8) != 4 || memcmp(buf, "ping", 4)) {
        return fail("pair_stream: a to b", 0, errno);
    }
    if (write(sv[1], "pong!", 5) != 5 || read(sv[0], buf, 8) != 5 || memcmp(buf, "pong!", 5)) {
        return fail("pair_stream: b to a", 0, errno);
    }
    close(sv[0]);
    long got = read(sv[1], buf, 8);
    close(sv[1]);
    if (got != 0) {
        return fail("pair_stream: end of file after close", got, errno);
    }
    ok("pair_stream", "bytes each way, then eof", 9);
    return 0;
}

static int pair_dgram(void) {
    int sv[2];
    char buf[16];
    if (socketpair(AF_UNIX, SOCK_DGRAM, 0, sv)) {
        return fail("pair_dgram: socketpair", -1, errno);
    }
    write(sv[0], "one", 3);
    write(sv[0], "second", 6);
    long a = read(sv[1], buf, sizeof buf);
    long b = read(sv[1], buf, sizeof buf);
    close(sv[0]);
    close(sv[1]);
    if (a != 3 || b != 6) {
        return fail("pair_dgram: boundaries kept", a, b);
    }
    ok("pair_dgram", "two datagrams, sizes 3 and", b);
    return 0;
}

static int listen_accept(void) {
    int port, l = listener(&port, SOCK_NONBLOCK);
    if (l < 0) {
        return fail("listen_accept: listener", l, 0);
    }
    if (port == 0) {
        return fail("listen_accept: getsockname gave port 0", 0, 0);
    }
    if (accept4(l, 0, 0, 0) != -1 || errno != EAGAIN) {
        return fail("listen_accept: empty non-blocking accept is EAGAIN", errno, EAGAIN);
    }
    int c = dial(port);
    if (c < 0) {
        return fail("listen_accept: connect", c, 0);
    }
    struct sockaddr_in peer, mine;
    socklen_t plen = sizeof peer, mlen = sizeof mine;
    int s = accept4(l, (void *)&peer, &plen, SOCK_NONBLOCK | SOCK_CLOEXEC);
    if (s < 0) {
        return fail("listen_accept: accept4", -1, errno);
    }
    if (!(fcntl(s, F_GETFL) & O_NONBLOCK) || !(fcntl(s, F_GETFD) & FD_CLOEXEC)) {
        return fail("listen_accept: accept4 flags", fcntl(s, F_GETFL), fcntl(s, F_GETFD));
    }
    getsockname(c, (void *)&mine, &mlen);
    if (peer.sin_port != mine.sin_port || peer.sin_addr.s_addr != htonl(INADDR_LOOPBACK)) {
        return fail("listen_accept: accept's address is the client's", ntohs(peer.sin_port),
                    ntohs(mine.sin_port));
    }
    struct sockaddr_in back;
    socklen_t blen = sizeof back;
    getpeername(c, (void *)&back, &blen);
    if (ntohs(back.sin_port) != port || blen != sizeof back) {
        return fail("listen_accept: getpeername", ntohs(back.sin_port), port);
    }
    char buf[8];
    if (write(c, "hi", 2) != 2 || read(s, buf, 8) != 2) {
        return fail("listen_accept: bytes", 0, errno);
    }
    close(c);
    close(s);
    close(l);
    ok("listen_accept", "accept4 NONBLOCK|CLOEXEC on port", port > 0);
    return 0;
}

#include "csock_parts2.h"
