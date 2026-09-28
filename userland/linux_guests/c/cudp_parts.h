// cudp: truncation, a refused port, several messages at once, and no peer.

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

// A port with no socket: one is bound and closed to find it.
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

static int mmsg(void) {
    struct sockaddr_in a;
    int s = bound(&a), c = socket(AF_INET, SOCK_DGRAM, 0);
    connect(c, (void *)&a, sizeof a);
    char out[3][4] = {"one", "two", "six"}, in[3][8];
    struct iovec oi[3], ii[3];
    struct mmsghdr om[3], im[3];
    memset(om, 0, sizeof om);
    memset(im, 0, sizeof im);
    for (int i = 0; i < 3; i++) {
        oi[i] = (struct iovec){out[i], 3};
        ii[i] = (struct iovec){in[i], 8};
        om[i].msg_hdr.msg_iov = &oi[i];
        om[i].msg_hdr.msg_iovlen = 1;
        im[i].msg_hdr.msg_iov = &ii[i];
        im[i].msg_hdr.msg_iovlen = 1;
    }
    int sent = sendmmsg(c, om, 3, 0);
    int got = recvmmsg(s, im, 3, MSG_WAITFORONE, 0);
    close(s);
    close(c);
    if (sent != 3 || got != 3 || im[2].msg_len != 3 || memcmp(in[2], "six", 3)) {
        return fail("mmsg: three out, three in", sent, got);
    }
    ok("mmsg", "sendmmsg and recvmmsg moved", got);
    return 0;
}

static int nameless(void) {
    int c = socket(AF_INET, SOCK_DGRAM, 0);
    long sent = send(c, "x", 1, 0);
    int e = errno;
    close(c);
    if (sent != -1 || e != EDESTADDRREQ) {
        return fail("nameless: no peer is EDESTADDRREQ", sent, e);
    }
    ok("nameless", "errno", e);
    return 0;
}
