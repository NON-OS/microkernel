/*
 * An idle guest: main blocks in accept on 127.0.0.1 while nothing happens
 * for the number of seconds given (default 10), then a thread connects. It
 * is what the serve loop's wakeups are measured against: a family socket
 * changes only in an answer, so the loop has nothing to look at meanwhile.
 */

#include <netinet/in.h>
#include <pthread.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/socket.h>
#include <time.h>
#include <unistd.h>

#include "cidle_1.h"

int main(int argc, char **argv) {
    if (argc > 1) {
        idle_s = atoi(argv[1]);
    }
    int l = socket(AF_INET, SOCK_STREAM, 0);
    memset(&addr, 0, sizeof addr);
    addr.sin_family = AF_INET;
    addr.sin_addr.s_addr = htonl(INADDR_LOOPBACK);
    socklen_t len = sizeof addr;
    bind(l, (void *)&addr, sizeof addr);
    getsockname(l, (void *)&addr, &len);
    listen(l, 1);
    pthread_t t;
    long t0 = now_ms();
    pthread_create(&t, 0, late, 0);
    int s = accept(l, 0, 0);
    long waited = now_ms() - t0;
    pthread_join(t, 0);
    printf("[C] cidle %s: accept blocked %ld ms for a connect after %d s\n",
           s >= 0 && waited >= idle_s * 1000 ? "PASS" : "FAIL", waited, idle_s);
    fflush(stdout);
    return s >= 0 ? 0 : 1;
}
