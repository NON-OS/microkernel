// musl pthreads end to end: pthread_create reserves each stack with PROT_NONE
// and commits it with mprotect, the workers sum known ranges under a mutex,
// and pthread_join waits for each worker's exit, which musl learns from the
// clear-tid word the personality zeroes and wakes. A create that fails, a
// wrong total, or a join that never returns is a broken thread runtime.
#include <pthread.h>
#include <stdio.h>

#define WORKERS 8
#define EACH 10000UL

static pthread_mutex_t lock = PTHREAD_MUTEX_INITIALIZER;
static unsigned long long total;

static void *work(void *arg) {
    unsigned long base = (unsigned long)arg;
    unsigned long long sum = 0;
    for (unsigned long i = 0; i < EACH; i++) {
        sum += base + i;
    }
    pthread_mutex_lock(&lock);
    total += sum;
    pthread_mutex_unlock(&lock);
    return 0;
}

int main(void) {
    pthread_t t[WORKERS];
    for (unsigned long w = 0; w < WORKERS; w++) {
        if (pthread_create(&t[w], 0, work, (void *)(w * EACH)) != 0) {
            printf("[C] cthreads FAIL: create %lu\n", w);
            fflush(stdout);
            return 1;
        }
    }
    for (int w = 0; w < WORKERS; w++) {
        pthread_join(t[w], 0);
    }
    unsigned long long n = WORKERS * EACH;
    unsigned long long want = (n - 1) * n / 2;
    if (total != want) {
        printf("[C] cthreads FAIL: total=%llu want=%llu\n", total, want);
        fflush(stdout);
        return 1;
    }
    printf("[C] cthreads PASS: %d pthreads joined, summed %llu\n", WORKERS, total);
    fflush(stdout);
    return 0;
}
