// A guest whose worker thread takes a real fault the runtime does not catch,
// while main waits. On Linux an unhandled fault in any thread ends the whole
// process; this proves NONOS does the same for a foreign guest, and that a
// clone thread runs its function at all. The worker runs on a stack mapped
// read-write outright, so the proof does not depend on the reserve-then-
// mprotect path a pthread stack uses. If the process were left alive, main
// would finish its wait and print the survived line, which the log must never
// show.
#define _GNU_SOURCE
#include <sched.h>
#include <sys/mman.h>
#include <time.h>
#include <stdio.h>

static int boom(void *arg) {
    (void)arg;
    volatile int *p = (volatile int *)0;
    *p = 1; // write to the unmapped null page: a fault, not a caught signal
    return 0;
}

int main(void) {
    fputs("[C] threadfault: main spawns a worker that will fault\n", stdout);
    fflush(stdout);
    long sz = 1 << 16;
    void *stack = mmap(0, sz, PROT_READ | PROT_WRITE, MAP_PRIVATE | MAP_ANONYMOUS, -1, 0);
    if (stack == MAP_FAILED) {
        fputs("[C] threadfault FAIL: no stack\n", stdout);
        fflush(stdout);
        return 1;
    }
    int flags = CLONE_VM | CLONE_THREAD | CLONE_SIGHAND | CLONE_FS | CLONE_FILES;
    int tid = clone(boom, (char *)stack + sz, flags, 0);
    if (tid < 0) {
        fputs("[C] threadfault FAIL: no worker\n", stdout);
        fflush(stdout);
        return 1;
    }
    struct timespec ts = { 1, 0 };
    for (int i = 0; i < 30; i++) {
        nanosleep(&ts, 0);
    }
    fputs("[C] threadfault SURVIVED: the process outlived a faulting thread\n", stdout);
    fflush(stdout);
    return 0;
}
