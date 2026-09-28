/*
 * Sockets, as Linux has them: a socketpair each way, a listener on 127.0.0.1
 * with its name and accept4's flags, a non-blocking connect that answers
 * EINPROGRESS and then SO_ERROR 0, a refused port, shutdown(SHUT_WR) giving
 * the peer end of file while the other way still flows, epoll readiness on a
 * listener, end of file, EAGAIN on an empty non-blocking receive, MSG_PEEK
 * and MSG_DONTWAIT, EPIPE after the peer is gone, an accept and a receive
 * that wait for another thread, the options Go and a C server set, a
 * socket a forked child shares, a connect a full listener holds, the
 * options a loopback connection cannot tell apart, and listeners that share
 * a port with SO_REUSEPORT. Each part prints as it passes and every part
 * runs, so one run names each part that fails.
 */

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

#include "csock_1.h"
#include "csock_2.h"
#include "csock_3.h"
#include "csock_4.h"
#include "csock_5.h"
#include "csock_6.h"
#include "csock_7.h"
#include "csock_8.h"
#include "csock_9.h"
#include "csock_10.h"
#include "csock_11.h"

int main(void) {
    signal(SIGPIPE, SIG_IGN);
    int (*const part[])(void) = {
        pair_stream,  pair_dgram, listen_accept,  nb_connect,     refused,       half_close,
        epoll_listener, eof,      empty_recv,     peek,           epipe,         blocking_accept,
        blocking_recv, options,   fork_share,     backlog,        quiet_options, reuseport,
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
