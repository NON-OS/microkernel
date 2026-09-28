// csock: the options a server sets, and a socket a forked child shares.

static int get_int(int s, int level, int opt) {
    int v = -1;
    socklen_t len = sizeof v;
    return getsockopt(s, level, opt, &v, &len) ? -errno : v;
}

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

// The child holds the accepted end after the parent closes its own: the
// client still reads the child's bytes, then end of file when the child exits.
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
        write(b, "kid", 3);
        _exit(0);
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

// A listener with a backlog of 0 queues one connect; the next non-blocking
// one answers EINPROGRESS, is not writable, and a second connect on it is
// EALREADY, until an accept makes room and it completes.
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
    int b = accept(s, 0, 0);
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

// The options Linux keeps that a loopback connection cannot tell apart,
// read back as Linux reads them, and the ones a socket's kind refuses.
static int quiet_options(void) {
    int t = socket(AF_INET, SOCK_STREAM, 0), u = socket(AF_INET, SOCK_DGRAM, 0);
    int x = socket(AF_UNIX, SOCK_STREAM, 0);
    int tos = 0x13, ttl = 32, zero = 0, ut = 5000, prio = 6;
    setsockopt(t, IPPROTO_IP, IP_TOS, &tos, sizeof tos);
    setsockopt(t, IPPROTO_IP, IP_TTL, &ttl, sizeof ttl);
    int bad_ttl = setsockopt(t, IPPROTO_IP, IP_TTL, &zero, sizeof zero) ? errno : 0;
    setsockopt(t, IPPROTO_TCP, TCP_USER_TIMEOUT, &ut, sizeof ut);
    setsockopt(t, SOL_SOCKET, SO_PRIORITY, &prio, sizeof prio);
    int got_tos = get_int(t, IPPROTO_IP, IP_TOS), got_ttl = get_int(t, IPPROTO_IP, IP_TTL);
    int got_ut = get_int(t, IPPROTO_TCP, TCP_USER_TIMEOUT);
    int got_qa = get_int(t, IPPROTO_TCP, TCP_QUICKACK), got_prio = get_int(t, SOL_SOCKET, SO_PRIORITY);
    int udp_tcp = setsockopt(u, IPPROTO_TCP, TCP_USER_TIMEOUT, &ut, sizeof ut) ? errno : 0;
    int unix_tcp = setsockopt(x, IPPROTO_TCP, TCP_NODELAY, &prio, sizeof prio) ? errno : 0;
    close(t);
    close(u);
    close(x);
    // A TCP socket drops the two ECN bits of the type of service.
    if (got_tos != 0x10 || got_ttl != 32 || bad_ttl != EINVAL || got_ut != 5000 || got_qa != 1 ||
        got_prio != 6 || udp_tcp != ENOPROTOOPT || unix_tcp != EOPNOTSUPP) {
        printf("[C] csock quiet_options got %d %d %d %d %d %d %d %d\n", got_tos, got_ttl, bad_ttl,
               got_ut, got_qa, got_prio, udp_tcp, unix_tcp);
        return fail("quiet_options: kept and read back as Linux does", got_tos, got_ttl);
    }
    ok("quiet_options", "6 kept and read back; refusals", unix_tcp);
    return 0;
}

// Listeners that set SO_REUSEPORT share a port and between them take every
// connection; a socket without it cannot bind there, and when one listener
// closes the other takes what comes next.
static int reuseport(void) {
    struct sockaddr_in sa = loopback(0);
    socklen_t len = sizeof sa;
    int one = 1, l[2], taken[2] = {0, 0};
    for (int i = 0; i < 2; i++) {
        l[i] = socket(AF_INET, SOCK_STREAM | SOCK_NONBLOCK, 0);
        setsockopt(l[i], SOL_SOCKET, SO_REUSEPORT, &one, sizeof one);
        if (bind(l[i], (void *)&sa, sizeof sa) || listen(l[i], 16)) {
            return fail("reuseport: two listeners on one port", i, errno);
        }
        getsockname(l[i], (void *)&sa, &len);
    }
    int plain = socket(AF_INET, SOCK_STREAM, 0);
    int refused = bind(plain, (void *)&sa, sizeof sa) ? errno : 0;
    close(plain);
    int c[16];
    for (int i = 0; i < 16; i++) {
        c[i] = dial(ntohs(sa.sin_port));
    }
    for (int i = 0; i < 2; i++) {
        int a;
        while ((a = accept(l[i], 0, 0)) >= 0) {
            taken[i]++;
            close(a);
        }
    }
    close(l[0]);
    int late = dial(ntohs(sa.sin_port));
    struct pollfd p = {l[1], POLLIN, 0};
    int ready = poll(&p, 1, 3000);
    int last = accept(l[1], 0, 0);
    for (int i = 0; i < 16; i++) {
        close(c[i]);
    }
    close(late);
    close(last);
    close(l[1]);
    if (refused != EADDRINUSE || taken[0] + taken[1] != 16 || !taken[0] || !taken[1] ||
        ready != 1 || last < 0) {
        printf("[C] csock reuseport got %d %d+%d %d %d\n", refused, taken[0], taken[1], ready, last);
        return fail("reuseport: shared, spread, and the survivor takes the rest", taken[0], taken[1]);
    }
    ok("reuseport", "16 connections spread over 2 listeners; without it errno", refused);
    return 0;
}
