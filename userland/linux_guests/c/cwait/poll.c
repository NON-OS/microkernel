#include "cwait.h"

static int late_fd;
static void *late_byte(void *arg) {
    (void)arg;
    nap_ms(100);
    write(late_fd, "z", 1);
    return 0;
}


int poll_select(void) {
    int p[2];
    pipe(p);
    struct pollfd pf = {.fd = p[0], .events = POLLIN};
    long t0 = now_ms();
    int n = poll(&pf, 1, 100);
    long polled = now_ms() - t0;
    t0 = now_ms();
    poll(0, 0, 50);
    long slept = now_ms() - t0;
    struct timespec ts = {0, 100 * 1000000};
    t0 = now_ms();
    int m = ppoll(&pf, 1, &ts, 0);
    long ppolled = now_ms() - t0;
    if (n != 0 || polled < 95 || slept < 45 || m != 0 || ppolled < 95 || polled > 2000) {
        return fail("poll and ppoll timeouts", polled, ppolled);
    }
    fd_set rd;
    FD_ZERO(&rd);
    FD_SET(p[0], &rd);
    struct timeval tv = {0, 100 * 1000};
    t0 = now_ms();
    int s = select(p[0] + 1, &rd, 0, 0, &tv);
    long selected = now_ms() - t0;
    if (s != 0 || FD_ISSET(p[0], &rd) || selected < 95) {
        return fail("select timeout", s, selected);
    }
    FD_SET(p[0], &rd);
    late_fd = p[1];
    pthread_t t;
    t0 = now_ms();
    pthread_create(&t, 0, late_byte, 0);
    s = select(p[0] + 1, &rd, 0, 0, 0);
    long woken = now_ms() - t0;
    pthread_join(t, 0);
    close(p[0]);
    close(p[1]);
    if (s != 1 || !FD_ISSET(p[0], &rd) || woken < 90) {
        return fail("select woken by a write", s, woken);
    }
    ok("poll-select", "poll, ppoll and select waited their 100ms timeouts, select woken after ms",
       woken);
    return 0;
}
