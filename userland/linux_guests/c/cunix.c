/*
 * Unix sockets with names, as Linux has them: a listener on a path, its
 * name and its client's, a connect that completes at once; ENOENT for a path
 * with nothing there and ECONNREFUSED for one with no listener; a path that
 * stays after its socket closes until it is unlinked; abstract names and the
 * sender a datagram reports; a connected datagram socket that refuses
 * strangers; a name bind chooses; and a connection across fork. Each part
 * prints as it passes and every part runs.
 */

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

#include "cunix_1.h"
#include "cunix_2.h"
#include "cunix_3.h"
#include "cunix_4.h"

int main(void) {
    int (*const part[])(void) = {path_stream, left_behind, abstract_dgram,
                                 connected_dgram, autobind, across_fork};
    const int count = sizeof part / sizeof part[0];
    int failed = 0;
    for (int i = 0; i < count; i++) {
        failed += part[i]();
    }
    if (failed) {
        printf("[C] cunix FAIL: %d parts failed, %d passed\n", failed, parts);
        fflush(stdout);
        return 1;
    }
    printf("[C] cunix PASS: %d parts\n", parts);
    fflush(stdout);
    return 0;
}
