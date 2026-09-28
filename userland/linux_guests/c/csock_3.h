/* csock, part 3 of 11: included once, by csock.c. */

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
