// A caught signal sent to a thread that is running its own code, with no
// call for it to arrive on: the handler has to run inside the spin, or the
// spin never ends and the join never returns.
#define _GNU_SOURCE
#include <pthread.h>
#include <sched.h>
#include <signal.h>
#include <stdio.h>
#include <string.h>
#include <time.h>

static volatile sig_atomic_t hit;
static volatile int spinning;
static volatile unsigned long spins;

static void on_usr1(int sig) {
    (void)sig;
    hit = 1;
}

static void *spin(void *arg) {
    (void)arg;
    spinning = 1;
    while (!hit) {
        spins++;
    }
    return 0;
}

static long now_ms(void) {
    struct timespec ts;
    clock_gettime(CLOCK_MONOTONIC, &ts);
    return ts.tv_sec * 1000 + ts.tv_nsec / 1000000;
}

int main(void) {
    struct sigaction sa;
    memset(&sa, 0, sizeof sa);
    sa.sa_handler = on_usr1;
    sigaction(SIGUSR1, &sa, 0);
    pthread_t t;
    pthread_create(&t, 0, spin, 0);
    while (!spinning) {
        sched_yield();
    }
    long t0 = now_ms();
    pthread_kill(t, SIGUSR1);
    pthread_join(t, 0);
    printf("[C] cpreempt PASS: the handler ran in a thread spinning with no call, after %ld ms\n",
           now_ms() - t0);
    fflush(stdout);
    return 0;
}
