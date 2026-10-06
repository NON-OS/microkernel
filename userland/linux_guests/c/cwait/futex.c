#include "cwait.h"

static pthread_mutex_t lock = PTHREAD_MUTEX_INITIALIZER;
static pthread_cond_t cond = PTHREAD_COND_INITIALIZER;
static int go_flag;
int woke;

static void *cond_waiter(void *arg) {
    (void)arg;
    pthread_mutex_lock(&lock);
    while (!go_flag) {
        pthread_cond_wait(&cond, &lock);
    }
    woke++;
    pthread_mutex_unlock(&lock);
    return 0;
}


int timed_futex(void) {
    struct timespec at;
    clock_gettime(CLOCK_REALTIME, &at);
    at.tv_nsec += 100 * 1000000;
    if (at.tv_nsec >= 1000000000) {
        at.tv_sec++;
        at.tv_nsec -= 1000000000;
    }
    long t0 = now_ms();
    pthread_mutex_lock(&lock);
    int rc = pthread_cond_timedwait(&cond, &lock, &at);
    pthread_mutex_unlock(&lock);
    long took = now_ms() - t0;
    if (rc != ETIMEDOUT || took < 95 || took > 2000) {
        return fail("cond_timedwait 100ms", rc, took);
    }
    ok("futex-timeout", "cond_timedwait 100ms answered ETIMEDOUT after ms", took);
    return 0;
}

int broadcast(void) {
    pthread_t t[4];
    for (int i = 0; i < 4; i++) {
        pthread_create(&t[i], 0, cond_waiter, 0);
    }
    nap_ms(50);
    pthread_mutex_lock(&lock);
    go_flag = 1;
    pthread_cond_broadcast(&cond);
    pthread_mutex_unlock(&lock);
    for (int i = 0; i < 4; i++) {
        pthread_join(t[i], 0);
    }
    if (woke != 4) {
        return fail("broadcast woke", woke, 4);
    }
    ok("broadcast", "condition variable waiters woken and joined:", woke);
    return 0;
}
