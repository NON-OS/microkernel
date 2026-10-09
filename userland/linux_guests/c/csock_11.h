/* csock, part 11 of 11: included once, by csock.c. */

/*
 * Listeners that set SO_REUSEPORT share a port and between them take every
 * connection; a socket without it cannot bind there, and when one listener
 * closes the other takes what comes next.
 */
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
