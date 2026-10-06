/*
 * csig: a caught signal raised for a thread not running its own code must
 * reach it and run its handler there. In each part the thread, once answered,
 * spins with no call, so only a signal delivered with that answer, or at its
 * next tick, ends the spin: one parked in a futex and then woken, one asleep
 * whose sleep then ends, and one whose second signal lands just before its
 * first handler returns. A part not done in 3 s prints FAIL, exits non-zero.
 */
#define _GNU_SOURCE
#include <pthread.h>
#include <signal.h>
#include <stdio.h>
#include <sys/syscall.h>
#include <time.h>
#include <unistd.h>

#define FUTEX_WAIT 0
#define FUTEX_WAKE 1

static volatile int got1, got2, sent2, word, ready, part;
static volatile long wtid, h1tid, h2tid;

static void on1(int s) {
    (void)s;
    h1tid = syscall(SYS_gettid);
    got1 = 1;
    while (part == 3 && !sent2 && h1tid == wtid) {}
}
static void on2(int s) { (void)s; h2tid = syscall(SYS_gettid); got2 = 1; }

static void *worker(void *arg) {
    (void)arg;
    wtid = syscall(SYS_gettid);
    ready = 1;
    if (part == 1) syscall(SYS_futex, &word, FUTEX_WAIT, 0, 0);
    if (part == 2) syscall(SYS_nanosleep, &(struct timespec){0, 300000000}, 0);
    while (part == 3 ? !got2 : !got1) {}
    return 0;
}

static long ms(void) {
    struct timespec t;
    clock_gettime(CLOCK_MONOTONIC, &t);
    return t.tv_sec * 1000 + t.tv_nsec / 1000000;
}

static int run(int which, const char *name) {
    pthread_t t;
    got1 = got2 = sent2 = word = ready = 0;
    part = which;
    pthread_create(&t, 0, worker, 0);
    while (!ready) sched_yield();
    usleep(50000);
    long t0 = ms();
    pthread_kill(t, SIGUSR1);
    if (which == 1) { word = 1; syscall(SYS_futex, &word, FUTEX_WAKE, 1); }
    if (which == 3) { while (!got1) sched_yield(); pthread_kill(t, SIGUSR2); sent2 = 1; }
    while ((which == 3 ? !got2 : !got1) && ms() - t0 < 3000) sched_yield();
    int got = which == 3 ? got2 : got1, done = got && h1tid == wtid && (which < 3 || h2tid == wtid);
    const char *why = got ? "FAIL: ran on another thread" : "FAIL: not delivered";
    printf("[C] csig %s %s after %ld ms\n", name, done ? "ok" : why, ms() - t0);
    if (!done) return 1;
    pthread_join(t, 0);
    return 0;
}

int main(void) {
    struct sigaction a = {.sa_handler = on1}, b = {.sa_handler = on2};
    sigaction(SIGUSR1, &a, 0);
    sigaction(SIGUSR2, &b, 0);
    int bad = run(1, "futex-woken") + run(2, "sleep-ended") + run(3, "after-return");
    printf("[C] csig %s: %d of 3 parts failed\n", bad ? "FAIL" : "PASS", bad);
    fflush(stdout);
    return bad ? 1 : 0;
}
