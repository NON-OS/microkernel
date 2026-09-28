// Datagrams on the loopback address, as Linux has them: an echo between an
// unbound client and a bound server, each told the other's address; a
// connected socket that sends without naming a peer and keeps only its
// peer's datagrams; a datagram cut to the buffer with MSG_TRUNC, and
// recvmsg's MSG_TRUNC flag; ECONNREFUSED on a connected socket whose peer
// port is closed; sendmmsg and recvmmsg; and EDESTADDRREQ with no peer at
// all. Each part prints as it passes and every part runs.
#define _GNU_SOURCE
#include <errno.h>
#include <netinet/in.h>
#include <stdio.h>
#include <string.h>
#include <sys/socket.h>
#include <sys/uio.h>
#include <unistd.h>

static int parts;

static int fail(const char *what, long a, long b) {
    printf("[C] cudp FAIL: %s (%ld, %ld)\n", what, a, b);
    fflush(stdout);
    return 1;
}

static void ok(const char *part, const char *detail, long n) {
    parts++;
    printf("[C] cudp %s ok: %s %ld\n", part, detail, n);
    fflush(stdout);
}

// A datagram socket bound to 127.0.0.1 at a port the kernel picks.
static int bound(struct sockaddr_in *at) {
    int s = socket(AF_INET, SOCK_DGRAM, 0);
    memset(at, 0, sizeof *at);
    at->sin_family = AF_INET;
    at->sin_addr.s_addr = htonl(INADDR_LOOPBACK);
    socklen_t len = sizeof *at;
    if (s < 0 || bind(s, (void *)at, sizeof *at) || getsockname(s, (void *)at, &len)) {
        return -1;
    }
    return s;
}

static int echo(void) {
    struct sockaddr_in srv, from, back;
    int s = bound(&srv), c = socket(AF_INET, SOCK_DGRAM, 0);
    char buf[32];
    socklen_t flen = sizeof from, blen = sizeof back;
    if (s < 0 || sendto(c, "ping", 4, 0, (void *)&srv, sizeof srv) != 4) {
        return fail("echo: send", s, errno);
    }
    long got = recvfrom(s, buf, sizeof buf, 0, (void *)&from, &flen);
    if (got != 4 || from.sin_port == 0 || flen != sizeof from) {
        return fail("echo: server heard the client and its address", got, ntohs(from.sin_port));
    }
    sendto(s, "pong!", 5, 0, (void *)&from, flen);
    got = recvfrom(c, buf, sizeof buf, 0, (void *)&back, &blen);
    close(s);
    close(c);
    if (got != 5 || back.sin_port != srv.sin_port || memcmp(buf, "pong!", 5)) {
        return fail("echo: client heard the server", got, ntohs(back.sin_port));
    }
    ok("echo", "4 bytes there, back from the server's port", ntohs(back.sin_port) > 0);
    return 0;
}

static int connected(void) {
    struct sockaddr_in a, b, x;
    int sa = bound(&a), sb = bound(&b), sx = bound(&x);
    char buf[8];
    connect(sb, (void *)&a, sizeof a);
    if (send(sb, "to-a", 4, 0) != 4 || recv(sa, buf, 8, 0) != 4) {
        return fail("connected: send without an address", 0, errno);
    }
    // sb keeps only a's datagrams: x's is dropped, a's arrives.
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

#include "cudp_parts.h"

int main(void) {
    int (*const part[])(void) = {echo, connected, cut, refused, mmsg, nameless};
    const int count = sizeof part / sizeof part[0];
    int failed = 0;
    for (int i = 0; i < count; i++) {
        failed += part[i]();
    }
    if (failed) {
        printf("[C] cudp FAIL: %d parts failed, %d passed\n", failed, parts);
        fflush(stdout);
        return 1;
    }
    printf("[C] cudp PASS: %d parts\n", parts);
    fflush(stdout);
    return 0;
}
