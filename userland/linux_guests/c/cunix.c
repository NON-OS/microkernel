// Unix sockets with names, as Linux has them: a listener on a path, its
// name and its client's, a connect that completes at once; ENOENT for a path
// with nothing there and ECONNREFUSED for one with no listener; a path that
// stays after its socket closes until it is unlinked; abstract names and the
// sender a datagram reports; a connected datagram socket that refuses
// strangers; a name bind chooses; and a connection across fork. Each part
// prints as it passes and every part runs.
#define _GNU_SOURCE
#include <errno.h>
#include <stddef.h>
#include <stdio.h>
#include <string.h>
#include <sys/socket.h>
#include <sys/un.h>
#include <sys/wait.h>
#include <unistd.h>

#define PATH "/tmp/cunix.sock"
#define GONE "/tmp/cunix.none"

static int parts;

static int fail(const char *what, long a, long b) {
    printf("[C] cunix FAIL: %s (%ld, %ld)\n", what, a, b);
    fflush(stdout);
    return 1;
}

static void ok(const char *part, const char *detail, long n) {
    parts++;
    printf("[C] cunix %s ok: %s %ld\n", part, detail, n);
    fflush(stdout);
}

// A sockaddr_un for a path, or for an abstract name when `abs` is set.
static socklen_t name(struct sockaddr_un *a, const char *p, int abs) {
    memset(a, 0, sizeof *a);
    a->sun_family = AF_UNIX;
    if (abs) {
        memcpy(a->sun_path + 1, p, strlen(p));
        return offsetof(struct sockaddr_un, sun_path) + 1 + strlen(p);
    }
    strcpy(a->sun_path, p);
    return offsetof(struct sockaddr_un, sun_path) + strlen(p) + 1;
}

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

#include "cunix_parts.h"
