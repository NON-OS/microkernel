/* csock, part 8 of 11: included once, by csock.c. */

static int options(void) {
    int port, l = listener(&port, 0);
    int one = 1;
    if (get_int(l, SOL_SOCKET, SO_TYPE) != SOCK_STREAM ||
        get_int(l, SOL_SOCKET, SO_DOMAIN) != AF_INET ||
        get_int(l, SOL_SOCKET, SO_ACCEPTCONN) != 1 ||
        get_int(l, SOL_SOCKET, SO_REUSEADDR) != 1) {
        return fail("options: type, domain, acceptconn, reuseaddr",
                    get_int(l, SOL_SOCKET, SO_TYPE), get_int(l, SOL_SOCKET, SO_ACCEPTCONN));
    }
    int c = dial(port);
    int sets[][2] = {
        {SOL_SOCKET, SO_KEEPALIVE}, {IPPROTO_TCP, TCP_NODELAY}, {SOL_SOCKET, SO_REUSEPORT},
        {SOL_SOCKET, SO_BROADCAST},
    };
    for (unsigned i = 0; i < sizeof sets / sizeof sets[0]; i++) {
        if (setsockopt(c, sets[i][0], sets[i][1], &one, sizeof one) ||
            get_int(c, sets[i][0], sets[i][1]) != 1) {
            return fail("options: set then read back", sets[i][1], errno);
        }
    }
    int idle = 15;
    if (setsockopt(c, IPPROTO_TCP, TCP_KEEPIDLE, &idle, sizeof idle) ||
        get_int(c, IPPROTO_TCP, TCP_KEEPIDLE) != 15) {
        return fail("options: TCP_KEEPIDLE", get_int(c, IPPROTO_TCP, TCP_KEEPIDLE), errno);
    }
    int buf = 65536;
    if (setsockopt(c, SOL_SOCKET, SO_RCVBUF, &buf, sizeof buf) ||
        get_int(c, SOL_SOCKET, SO_RCVBUF) != 2 * buf) {
        return fail("options: SO_RCVBUF doubled", get_int(c, SOL_SOCKET, SO_RCVBUF), 2 * buf);
    }
    struct linger lg = {1, 5}, back = {0, 0};
    socklen_t len = sizeof back;
    setsockopt(c, SOL_SOCKET, SO_LINGER, &lg, sizeof lg);
    getsockopt(c, SOL_SOCKET, SO_LINGER, &back, &len);
    struct timeval tv = {2, 500000}, tb = {0, 0};
    len = sizeof tb;
    setsockopt(c, SOL_SOCKET, SO_RCVTIMEO, &tv, sizeof tv);
    getsockopt(c, SOL_SOCKET, SO_RCVTIMEO, &tb, &len);
    int bogus = setsockopt(c, SOL_SOCKET, 9999, &one, sizeof one);
    int e = errno;
    close(c);
    close(l);
    if (back.l_onoff != 1 || back.l_linger != 5 || tb.tv_sec != 2 || tb.tv_usec != 500000) {
        return fail("options: SO_LINGER and SO_RCVTIMEO read back", back.l_linger, tb.tv_usec);
    }
    if (bogus != -1 || e != ENOPROTOOPT) {
        return fail("options: an unknown option is ENOPROTOOPT", bogus, e);
    }
    ok("options", "11 options, unknown errno", e);
    return 0;
}
