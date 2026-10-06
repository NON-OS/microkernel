#include "cwait.h"

int scheduler(void) {
    struct sched_param prio = {0};
    long policy = syscall(SYS_sched_getscheduler, 0);
    int zero_fifo = syscall(SYS_sched_setscheduler, 0, SCHED_FIFO, &prio) == -1 && errno == EINVAL;
    struct sched_param one = {.sched_priority = 1};
    long fifo = syscall(SYS_sched_setscheduler, 0, SCHED_FIFO, &one) == -1 ? errno : 0;
    syscall(SYS_sched_setscheduler, 0, SCHED_OTHER, &prio);
    int other = syscall(SYS_sched_setscheduler, 0, SCHED_OTHER, &prio) == 0;
    int range = sched_get_priority_max(SCHED_FIFO) == 99 && sched_get_priority_min(SCHED_RR) == 1;
    cpu_set_t set;
    CPU_ZERO(&set);
    CPU_SET(0, &set);
    int pinned = sched_setaffinity(0, sizeof set, &set) == 0;
    CPU_ZERO(&set);
    CPU_SET(1, &set);
    long cpu1 = sched_setaffinity(0, sizeof set, &set) == -1 ? errno : 0;
    CPU_ZERO(&set);
    CPU_SET(0, &set);
    sched_setaffinity(0, sizeof set, &set);
    int old = epoll_create(1);
    int zero = epoll_create(0) == -1 && errno == EINVAL;
    struct epoll_event ev;
    struct timespec half = {0, 50 * 1000000};
    long t0 = now_ms();
    long n = syscall(SYS_epoll_pwait2, old, &ev, 1, &half, 0, 8);
    long waited = now_ms() - t0;
    close(old);
    int all = policy == SCHED_OTHER && zero_fifo && other && range && pinned && old >= 0 && zero &&
              n == 0 && waited >= 45;
    if (!all) {
        return fail("scheduler and epoll forms",
                    policy * 10000 + other * 1000 + range * 100 + pinned * 10 + zero,
                    (n == 0) * 1000 + waited);
    }
    printf("[C] cwait sched detail: SCHED_FIFO at 1 errno %ld, a CPU-1-only mask errno %ld\n", fifo,
           cpu1);
    ok("sched", "SCHED_OTHER, CPU 0 pinned, epoll_create, epoll_pwait2 waited ms", waited);
    return 0;
}
