/* cudp, part 1 of 3: included once, by cudp.c. */

static int parts;

static int fail(const char *what, long a, long b) {
    printf("[C] cudp FAIL: %s (%ld, %ld)\n", what, a, b);
    fflush(stdout);
    return 1;
}

static void ok(const char *part, const char *detail, long n) {
    parts++;
    printf("[C] cudp %s ok: %s %ld\n", part, detail, n);
    fflush(stdout);
}

/* A datagram socket bound to 127.0.0.1 at a port the kernel picks. */
static int bound(struct sockaddr_in *at) {
    int s = socket(AF_INET, SOCK_DGRAM, 0);
    memset(at, 0, sizeof *at);
    at->sin_family = AF_INET;
    at->sin_addr.s_addr = htonl(INADDR_LOOPBACK);
    socklen_t len = sizeof *at;
    if (s < 0 || bind(s, (void *)at, sizeof *at) || getsockname(s, (void *)at, &len)) {
        return -1;
    }
    return s;
}

static int echo(void) {
    struct sockaddr_in srv, from, back;
    int s = bound(&srv), c = socket(AF_INET, SOCK_DGRAM, 0);
    char buf[32];
    socklen_t flen = sizeof from, blen = sizeof back;
    if (s < 0 || sendto(c, "ping", 4, 0, (void *)&srv, sizeof srv) != 4) {
        return fail("echo: send", s, errno);
    }
    long got = recvfrom(s, buf, sizeof buf, 0, (void *)&from, &flen);
    if (got != 4 || from.sin_port == 0 || flen != sizeof from) {
        return fail("echo: server heard the client and its address", got, ntohs(from.sin_port));
    }
    sendto(s, "pong!", 5, 0, (void *)&from, flen);
    got = recvfrom(c, buf, sizeof buf, 0, (void *)&back, &blen);
    close(s);
    close(c);
    if (got != 5 || back.sin_port != srv.sin_port || memcmp(buf, "pong!", 5)) {
        return fail("echo: client heard the server", got, ntohs(back.sin_port));
    }
    ok("echo", "4 bytes there, back from the server's port", ntohs(back.sin_port) > 0);
    return 0;
}
