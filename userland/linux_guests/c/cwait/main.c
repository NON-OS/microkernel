#include "cwait.h"

int parts;

long now_ms(void) {
    struct timespec ts;
    clock_gettime(CLOCK_MONOTONIC, &ts);
    return ts.tv_sec * 1000 + ts.tv_nsec / 1000000;
}

void nap_ms(long ms) {
    struct timespec ts = {ms / 1000, (ms % 1000) * 1000000};
    nanosleep(&ts, 0);
}

int fail(const char *what, long a, long b) {
    printf("[C] cwait FAIL: %s (%ld, %ld)\n", what, a, b);
    fflush(stdout);
    return 1;
}

void ok(const char *part, const char *detail, long n) {
    parts++;
    printf("[C] cwait %s ok: %s %ld\n", part, detail, n);
    fflush(stdout);
}

int main(void) {
    int (*const part[])(void) = {
        timed_futex,  broadcast, eventfd_semaphore, eventfd_blocking, epoll_timeout,
        pipe_nonblock, pipe_full, edge,             two_readers,      poll_select,
        close_forgets, timers,   ioctls_fork,       scheduler,        thread_kill,
        alternate_stack, thread_signal,
    };
    const int count = sizeof part / sizeof part[0];
    long t0 = now_ms();
    int failed = 0;
    /*
     * Every part runs, so one run names every part that fails; a hang still
     * stops it, at the part it hangs in.
     */
    for (int i = 0; i < count; i++) {
        failed += part[i]();
    }
    if (failed) {
        printf("[C] cwait FAIL: %d parts failed, %d passed\n", failed, parts);
        fflush(stdout);
        return 1;
    }
    printf("[C] cwait PASS: %d parts in %ld ms\n", parts, now_ms() - t0);
    fflush(stdout);
    return 0;
}
