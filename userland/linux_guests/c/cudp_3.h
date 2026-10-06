/* cudp, part 3 of 3: included once, by cudp.c. */

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
