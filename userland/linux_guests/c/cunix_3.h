/* cunix, part 3 of 4: included once, by cunix.c. */

/* cunix: what a path leaves behind, abstract datagrams, and fork. */

static int left_behind(void) {
    struct sockaddr_un a, g;
    socklen_t l = name(&a, PATH, 0), gl = name(&g, GONE, 0);
    unlink(GONE);
    int c = socket(AF_UNIX, SOCK_STREAM, 0);
    int missing = connect(c, (void *)&g, gl) ? errno : 0;
    int s = socket(AF_UNIX, SOCK_STREAM, 0);
    bind(s, (void *)&a, l);
    int unheard = connect(c, (void *)&a, l) ? errno : 0;
    close(s);
    int s2 = socket(AF_UNIX, SOCK_STREAM, 0);
    int rebind = bind(s2, (void *)&a, l) ? errno : 0;
    int closed = connect(c, (void *)&a, l) ? errno : 0;
    unlink(PATH);
    int after = bind(s2, (void *)&a, l) ? errno : 0;
    close(s2);
    close(c);
    unlink(PATH);
    if (missing != ENOENT || unheard != ECONNREFUSED || rebind != EADDRINUSE ||
        closed != ECONNREFUSED || after != 0) {
        printf("[C] cunix left_behind got %d %d %d %d %d\n", missing, unheard, rebind, closed,
               after);
        return fail("left_behind: ENOENT, ECONNREFUSED, EADDRINUSE, ECONNREFUSED, 0", 0, 0);
    }
    ok("left_behind", "the path stays until unlink; rebind after it", after);
    return 0;
}

static int abstract_dgram(void) {
    struct sockaddr_un x, y, b, g;
    socklen_t xl = name(&x, "cunix-x", 1), yl = name(&y, "cunix-y", 1), gl = name(&g, GONE, 0);
    int rx = socket(AF_UNIX, SOCK_DGRAM, 0), tx = socket(AF_UNIX, SOCK_DGRAM, 0);
    char buf[8];
    socklen_t bl = sizeof b;
    if (bind(rx, (void *)&x, xl) || getsockname(rx, (void *)&b, &bl) || bl != xl) {
        return fail("abstract_dgram: bind and name", bl, xl);
    }
    sendto(tx, "a", 1, 0, (void *)&x, xl);
    bl = sizeof b;
    long got = recvfrom(rx, buf, 8, 0, (void *)&b, &bl);
    if (got != 1 || bl != 0) {
        return fail("abstract_dgram: an unnamed sender reports length 0", got, bl);
    }
    bind(tx, (void *)&y, yl);
    sendto(tx, "b", 1, 0, (void *)&x, xl);
    bl = sizeof b;
    recvfrom(rx, buf, 8, 0, (void *)&b, &bl);
    if (bl != yl || memcmp(b.sun_path, y.sun_path, yl - 2)) {
        return fail("abstract_dgram: a named sender reports its name", bl, yl);
    }
    int missing = sendto(tx, "c", 1, 0, (void *)&g, gl) < 0 ? errno : 0;
    int lone = socket(AF_UNIX, SOCK_DGRAM, 0);
    int nopeer = send(lone, "d", 1, 0) < 0 ? errno : 0;
    close(lone);
    close(rx);
    close(tx);
    if (missing != ENOENT || nopeer != ENOTCONN) {
        return fail("abstract_dgram: ENOENT, then ENOTCONN", missing, nopeer);
    }
    ok("abstract_dgram", "names reported, no peer errno", nopeer);
    return 0;
}
