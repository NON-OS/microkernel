/* cunix, part 4 of 4: included once, by cunix.c. */

/* cunix: a connected datagram socket, a chosen name, and fork. */

static int connected_dgram(void) {
    struct sockaddr_un r, a;
    socklen_t rl = name(&r, "cunix-r", 1), al = name(&a, "cunix-a", 1);
    int rs = socket(AF_UNIX, SOCK_DGRAM, 0), as = socket(AF_UNIX, SOCK_DGRAM, 0);
    int stranger = socket(AF_UNIX, SOCK_DGRAM, 0);
    bind(rs, (void *)&r, rl);
    bind(as, (void *)&a, al);
    /* r talks only to a: a stranger's datagram to r is refused. */
    connect(rs, (void *)&a, al);
    int refused = sendto(stranger, "s", 1, 0, (void *)&r, rl) < 0 ? errno : 0;
    long sent = send(rs, "to-a", 4, 0);
    char buf[8];
    long got = recv(as, buf, 8, 0);
    close(rs);
    close(as);
    close(stranger);
    if (refused != EPERM || sent != 4 || got != 4) {
        return fail("connected_dgram: EPERM for a stranger, 4 bytes to the peer", refused, got);
    }
    ok("connected_dgram", "a stranger refused, errno", refused);
    return 0;
}

static int autobind(void) {
    struct sockaddr_un a, b;
    int s = socket(AF_UNIX, SOCK_DGRAM, 0);
    memset(&a, 0, sizeof a);
    a.sun_family = AF_UNIX;
    socklen_t bl = sizeof b;
    int rc = bind(s, (void *)&a, sizeof(sa_family_t));
    getsockname(s, (void *)&b, &bl);
    close(s);
    /* Linux chooses a NUL and five hex digits. */
    if (rc || bl != 8 || b.sun_path[0] != 0) {
        return fail("autobind: a NUL and five hex digits", rc, bl);
    }
    ok("autobind", "a chosen abstract name, length", bl);
    return 0;
}

static int across_fork(void) {
    struct sockaddr_un a;
    socklen_t l = name(&a, PATH, 0);
    unlink(PATH);
    int s = socket(AF_UNIX, SOCK_STREAM, 0);
    bind(s, (void *)&a, l);
    listen(s, 1);
    pid_t kid = fork();
    if (kid == 0) {
        int c = socket(AF_UNIX, SOCK_STREAM, 0);
        _exit(connect(c, (void *)&a, l) || write(c, "kid", 3) != 3);
    }
    int t = accept(s, 0, 0);
    char buf[8];
    long got = read(t, buf, 8);
    int status = -1;
    waitpid(kid, &status, 0);
    close(t);
    close(s);
    unlink(PATH);
    if (got != 3 || status != 0) {
        return fail("across_fork: the child's connection and bytes", got, status);
    }
    ok("across_fork", "bytes from the child", got);
    return 0;
}
