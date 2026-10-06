/* csock, part 4 of 11: included once, by csock.c. */

/* csock: connecting, and closing one way or both. */

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

/* A port with no listener: a listener is opened and closed to find one. */
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
