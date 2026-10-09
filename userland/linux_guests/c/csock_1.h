/* csock, part 1 of 11: included once, by csock.c. */

static int parts;

static long now_ms(void) {
    struct timespec ts;
    clock_gettime(CLOCK_MONOTONIC, &ts);
    return ts.tv_sec * 1000 + ts.tv_nsec / 1000000;
}

static void nap_ms(long ms) {
    struct timespec ts = {ms / 1000, (ms % 1000) * 1000000};
    nanosleep(&ts, 0);
}

static int fail(const char *what, long a, long b) {
    printf("[C] csock FAIL: %s (%ld, %ld)\n", what, a, b);
    fflush(stdout);
    return 1;
}

static void ok(const char *part, const char *detail, long n) {
    parts++;
    printf("[C] csock %s ok: %s %ld\n", part, detail, n);
    fflush(stdout);
}

static struct sockaddr_in loopback(int port) {
    struct sockaddr_in sa;
    memset(&sa, 0, sizeof sa);
    sa.sin_family = AF_INET;
    sa.sin_port = htons(port);
    sa.sin_addr.s_addr = htonl(INADDR_LOOPBACK);
    return sa;
}

/* A listener on 127.0.0.1 at a port the kernel picks; its port through *port. */
static int listener(int *port, int flags) {
    int s = socket(AF_INET, SOCK_STREAM | flags, 0);
    if (s < 0) {
        return -errno;
    }
    int one = 1;
    setsockopt(s, SOL_SOCKET, SO_REUSEADDR, &one, sizeof one);
    struct sockaddr_in sa = loopback(0);
    socklen_t len = sizeof sa;
    if (bind(s, (void *)&sa, sizeof sa) || getsockname(s, (void *)&sa, &len) || listen(s, 8)) {
        int e = errno;
        close(s);
        return -e;
    }
    *port = ntohs(sa.sin_port);
    return s;
}

static int dial(int port) {
    int c = socket(AF_INET, SOCK_STREAM, 0);
    struct sockaddr_in sa = loopback(port);
    if (c < 0 || connect(c, (void *)&sa, sizeof sa)) {
        int e = errno;
        if (c >= 0) {
            close(c);
        }
        return -e;
    }
    return c;
}
