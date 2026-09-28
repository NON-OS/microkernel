#include "cwait.h"

int efd_late;
void *late_write(void *arg) {
    nap_ms(100);
    uint64_t v = (uint64_t)(uintptr_t)arg;
    write(efd_late, &v, 8);
    return 0;
}


int eventfd_semaphore(void) {
    int fd = eventfd(3, EFD_NONBLOCK | EFD_SEMAPHORE);
    uint64_t v = 0;
    for (int i = 0; i < 3; i++) {
        if (read(fd, &v, 8) != 8 || v != 1) {
            return fail("semaphore read", i, (long)v);
        }
    }
    if (read(fd, &v, 8) != -1 || errno != EAGAIN) {
        return fail("empty non-blocking eventfd read", (long)v, errno);
    }
    int plain = eventfd(0, EFD_NONBLOCK);
    v = 5;
    write(plain, &v, 8);
    write(plain, &v, 8);
    if (read(plain, &v, 8) != 8 || v != 10) {
        return fail("eventfd count", (long)v, 10);
    }
    close(fd);
    close(plain);
    ok("eventfd", "3 semaphore reads of 1, EAGAIN at zero, then a count of", 10);
    return 0;
}

int eventfd_blocking(void) {
    efd_late = eventfd(0, 0);
    pthread_t t;
    long t0 = now_ms();
    pthread_create(&t, 0, late_write, (void *)(uintptr_t)7);
    uint64_t v = 0;
    ssize_t n = read(efd_late, &v, 8);
    long took = now_ms() - t0;
    pthread_join(t, 0);
    if (n != 8 || v != 7 || took < 90) {
        return fail("blocking eventfd read", (long)v, took);
    }
    ok("eventfd-wait", "a read waited for another thread's write of 7, ms", took);
    return 0;
}
