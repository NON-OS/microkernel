/* cudp, part 2 of 3: included once, by cudp.c. */

static int connected(void) {
    struct sockaddr_in a, b, x;
    int sa = bound(&a), sb = bound(&b), sx = bound(&x);
    char buf[8];
    connect(sb, (void *)&a, sizeof a);
    if (send(sb, "to-a", 4, 0) != 4 || recv(sa, buf, 8, 0) != 4) {
        return fail("connected: send without an address", 0, errno);
    }
    /* sb keeps only a's datagrams: x's is dropped, a's arrives. */
    sendto(sx, "noise", 5, 0, (void *)&b, sizeof b);
    sendto(sa, "real", 4, 0, (void *)&b, sizeof b);
    long got = recv(sb, buf, 8, 0);
    long more = recv(sb, buf, 8, MSG_DONTWAIT);
    int e = errno;
    close(sa);
    close(sb);
    close(sx);
    if (got != 4 || more != -1 || e != EAGAIN) {
        return fail("connected: only the peer's datagrams kept", got, more);
    }
    ok("connected", "a stranger's datagram dropped, peer's bytes", got);
    return 0;
}

/* cudp: truncation, a refused port, several messages at once, and no peer. */

static int cut(void) {
    struct sockaddr_in a;
    int s = bound(&a), c = socket(AF_INET, SOCK_DGRAM, 0);
    char big[100], buf[4];
    memset(big, 'z', sizeof big);
    sendto(c, big, sizeof big, 0, (void *)&a, sizeof a);
    long whole = recv(s, buf, sizeof buf, MSG_TRUNC);
    long rest = recv(s, buf, sizeof buf, MSG_DONTWAIT);
    sendto(c, big, sizeof big, 0, (void *)&a, sizeof a);
    struct iovec iov = {buf, sizeof buf};
    struct msghdr m = {0};
    m.msg_iov = &iov;
    m.msg_iovlen = 1;
    long cut = recvmsg(s, &m, 0);
    close(s);
    close(c);
    if (whole != 100 || rest != -1 || cut != 4 || !(m.msg_flags & MSG_TRUNC)) {
        return fail("trunc: cut to 4, whole length 100, flag set", whole, cut);
    }
    ok("trunc", "MSG_TRUNC gave the whole length", whole);
    return 0;
}

/* A port with no socket: one is bound and closed to find it. */
static int refused(void) {
    struct sockaddr_in gone;
    close(bound(&gone));
    int c = socket(AF_INET, SOCK_DGRAM, 0);
    char buf[4];
    connect(c, (void *)&gone, sizeof gone);
    long sent = send(c, "x", 1, 0);
    long got = recv(c, buf, sizeof buf, MSG_DONTWAIT);
    int e = errno;
    close(c);
    if (sent != 1 || got != -1 || e != ECONNREFUSED) {
        return fail("refused: send taken, then ECONNREFUSED", sent, e);
    }
    ok("refused", "connected to a closed port, errno", e);
    return 0;
}
