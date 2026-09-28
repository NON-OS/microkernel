/* cunix, part 2 of 4: included once, by cunix.c. */

static int path_stream(void) {
    struct sockaddr_un a, b;
    socklen_t l = name(&a, PATH, 0), bl = sizeof b;
    unlink(PATH);
    int s = socket(AF_UNIX, SOCK_STREAM, 0);
    if (listen(s, 1) != -1 || errno != EINVAL) {
        return fail("path_stream: listen before bind is EINVAL", errno, EINVAL);
    }
    if (bind(s, (void *)&a, l) || listen(s, 4)) {
        return fail("path_stream: bind and listen", -1, errno);
    }
    int twice = socket(AF_UNIX, SOCK_STREAM, 0);
    if (bind(twice, (void *)&a, l) != -1 || errno != EADDRINUSE) {
        return fail("path_stream: a second bind is EADDRINUSE", errno, EADDRINUSE);
    }
    close(twice);
    int c = socket(AF_UNIX, SOCK_STREAM | SOCK_NONBLOCK, 0);
    if (connect(c, (void *)&a, l)) {
        return fail("path_stream: a non-blocking connect completes at once", -1, errno);
    }
    int t = accept(s, (void *)&b, &bl);
    if (t < 0 || bl != 2) {
        return fail("path_stream: accept names an unnamed client with 2 bytes", t, bl);
    }
    bl = sizeof b;
    getsockname(s, (void *)&b, &bl);
    if (bl != l || strcmp(b.sun_path, PATH)) {
        return fail("path_stream: getsockname", bl, l);
    }
    bl = sizeof b;
    getpeername(c, (void *)&b, &bl);
    if (bl != l || strcmp(b.sun_path, PATH)) {
        return fail("path_stream: the client's peer is the path", bl, l);
    }
    char buf[8];
    if (write(c, "unix", 4) != 4 || read(t, buf, 8) != 4) {
        return fail("path_stream: bytes", 0, errno);
    }
    close(c);
    long eof = read(t, buf, 8);
    close(t);
    close(s);
    if (eof != 0) {
        return fail("path_stream: end of file", eof, 0);
    }
    ok("path_stream", "bound, named, connected, 4 bytes, eof; name length", l);
    return 0;
}
