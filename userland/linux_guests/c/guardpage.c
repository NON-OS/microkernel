// A pthread recurses until it walks off the bottom of its stack. musl reserves
// each thread stack with PROT_NONE and opens all but the lowest part with
// mprotect, so the part it leaves closed is the guard. On Linux the first touch
// of the guard is SIGSEGV, which ends the whole process with status 139. The
// recursion stops by itself 64 KiB below the guard, so a guard that guards
// nothing prints the FAIL line with how far it ran; no PASS line exists, since
// the only correct outcome is that the process does not get to print one.
#define _GNU_SOURCE
#include <pthread.h>
#include <stdint.h>
#include <stdio.h>
#include <string.h>
#include <unistd.h>

#define FRAME 1024
#define PAST (64 * 1024)

static uintptr_t lo;
static uintptr_t deepest;

static void say(const char *s) {
    write(1, s, strlen(s));
}

static unsigned dive(unsigned depth) {
    volatile char frame[FRAME];
    uintptr_t here = (uintptr_t)frame;
    frame[0] = (char)depth;
    frame[FRAME - 1] = (char)depth;
    deepest = here;
    if (here < lo - PAST) {
        return depth;
    }
    return dive(depth + 1) + frame[0] - frame[FRAME - 1];
}

static void *worker(void *arg) {
    (void)arg;
    pthread_attr_t a;
    void *base;
    size_t size, guard;
    char line[160];
    pthread_getattr_np(pthread_self(), &a);
    pthread_attr_getstack(&a, &base, &size);
    pthread_attr_getguardsize(&a, &guard);
    lo = (uintptr_t)base;
    snprintf(line, sizeof line, "[C] guardpage: stack %zu KiB, guard %zu KiB below 0x%lx; recursing\n",
             size / 1024, guard / 1024, (unsigned long)lo);
    say(line);
    unsigned depth = dive(0);
    snprintf(line, sizeof line,
             "[C] guardpage FAIL: %u frames, ran %lu KiB below the stack with no fault\n", depth,
             (unsigned long)((lo - deepest) / 1024));
    say(line);
    return 0;
}

int main(void) {
    pthread_t t;
    if (pthread_create(&t, 0, worker, 0) != 0) {
        say("[C] guardpage FAIL: no thread\n");
        return 1;
    }
    pthread_join(t, 0);
    return 1;
}
