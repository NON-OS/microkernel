/*
 * A caught signal sent to a thread that is running its own code, with no
 * call for it to arrive on: the handler has to run inside the spin, or the
 * spin never ends and the join never returns. The handler records the thread
 * it ran on, so one run on any other thread is a FAIL, not a pass.
 */
#define _GNU_SOURCE
#include <pthread.h>
#include <sched.h>
#include <signal.h>
#include <stdio.h>
#include <string.h>
#include <sys/syscall.h>
#include <time.h>
#include <unistd.h>

static volatile sig_atomic_t hit;
static volatile int spinning;
static volatile unsigned long spins;
static volatile long spin_tid, hit_tid;

static void on_usr1(int sig) {
    (void)sig;
    hit_tid = syscall(SYS_gettid);
    hit = 1;
}

static void *spin(void *arg) {
    (void)arg;
    spin_tid = syscall(SYS_gettid);
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
    if (hit_tid != spin_tid) {
        printf("[C] cpreempt FAIL: the handler ran on thread %ld, not the spinning %ld\n",
               hit_tid, spin_tid);
        fflush(stdout);
        return 1;
    }
    printf("[C] cpreempt PASS: the handler ran in a thread spinning with no call, after %ld ms\n",
           now_ms() - t0);
    fflush(stdout);
    return 0;
}
