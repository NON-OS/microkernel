/*
 * Datagrams on the loopback address, as Linux has them: an echo between an
 * unbound client and a bound server, each told the other's address; a
 * connected socket that sends without naming a peer and keeps only its
 * peer's datagrams; a datagram cut to the buffer with MSG_TRUNC, and
 * recvmsg's MSG_TRUNC flag; ECONNREFUSED on a connected socket whose peer
 * port is closed; sendmmsg and recvmmsg; and EDESTADDRREQ with no peer at
 * all. Each part prints as it passes and every part runs.
 */

#define _GNU_SOURCE
#include <errno.h>
#include <netinet/in.h>
#include <stdio.h>
#include <string.h>
#include <sys/socket.h>
#include <sys/uio.h>
#include <unistd.h>

#include "cudp_1.h"
#include "cudp_2.h"
#include "cudp_3.h"

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
