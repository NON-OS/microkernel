/* csock, part 2 of 11: included once, by csock.c. */

/* A connected pair over 127.0.0.1: *a is the client, *b the accepted end. */
static int tcp_pair(int *a, int *b) {
    int port, l = listener(&port, 0);
    if (l < 0) {
        return l;
    }
    *a = dial(port);
    *b = *a < 0 ? -1 : accept(l, 0, 0);
    close(l);
    return *a < 0 ? *a : *b < 0 ? -errno : 0;
}

/* The parts of csock, one Linux behaviour each. Included once, by csock.c. */

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
    if (write(sv[0], "one", 3) != 3 || write(sv[0], "second", 6) != 6) {
        return fail("pair_dgram: write", -1, errno);
    }
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
