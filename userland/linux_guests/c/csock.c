// Sockets, as Linux has them: a socketpair each way, a listener on 127.0.0.1
// with its name and accept4's flags, a non-blocking connect that answers
// EINPROGRESS and then SO_ERROR 0, a refused port, shutdown(SHUT_WR) giving
// the peer end of file while the other way still flows, epoll readiness on a
// listener, end of file, EAGAIN on an empty non-blocking receive, MSG_PEEK
// and MSG_DONTWAIT, EPIPE after the peer is gone, an accept and a receive
// that wait for another thread, the options Go and a C server set, a
// socket a forked child shares, a connect a full listener holds, the
// options a loopback connection cannot tell apart, and listeners that share
// a port with SO_REUSEPORT. Each part prints as it passes and every part
// runs, so one run names each part that fails.
#define _GNU_SOURCE
#include <errno.h>
#include <fcntl.h>
#include <netinet/in.h>
#include <netinet/tcp.h>
#include <poll.h>
#include <pthread.h>
#include <signal.h>
#include <stdio.h>
#include <string.h>
#include <sys/epoll.h>
#include <sys/socket.h>
#include <sys/wait.h>
#include <time.h>
#include <unistd.h>

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

// A listener on 127.0.0.1 at a port the kernel picks; its port through *port.
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

// A connected pair over 127.0.0.1: *a is the client, *b the accepted end.
static int tcp_pair(int *a, int *b) {
    int port, l = listener(&port, 0);
    if (l < 0) {
        return l;
    }
    *a = dial(port);
    *b = *a < 0 ? -1 : accept(l, 0, 0);
    close(l);
    return *a < 0 ? *a : *b < 0 ? -errno : 0;
}

#include "csock_parts.h"

int main(void) {
    signal(SIGPIPE, SIG_IGN);
    int (*const part[])(void) = {
        pair_stream, pair_dgram, listen_accept, nb_connect, refused, half_close,
        epoll_listener, eof,     empty_recv,    peek,       epipe,   blocking_accept,
        blocking_recv, options,  fork_share,     backlog, quiet_options, reuseport,
    };
    const int count = sizeof part / sizeof part[0];
    long t0 = now_ms();
    int failed = 0;
    for (int i = 0; i < count; i++) {
        failed += part[i]();
    }
    if (failed) {
        printf("[C] csock FAIL: %d parts failed, %d passed\n", failed, parts);
        fflush(stdout);
        return 1;
    }
    printf("[C] csock PASS: %d parts in %ld ms\n", parts, now_ms() - t0);
    fflush(stdout);
    return 0;
}
