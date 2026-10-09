#include "cwait.h"

static volatile sig_atomic_t usr1;
static void on_usr1(int sig) {
    (void)sig;
    usr1 = 1;
}

int thread_kill(void) {
    struct sigaction sa;
    memset(&sa, 0, sizeof sa);
    sa.sa_handler = on_usr1;
    sigaction(SIGUSR1, &sa, 0);
    long rc = syscall(SYS_tgkill, getpid(), gettid(), SIGUSR1);
    long miss = syscall(SYS_tgkill, getpid(), 99999, SIGUSR1) == -1 ? errno : 0;
    getpid();
    if (rc != 0 || !usr1 || miss != ESRCH) {
        return fail("tgkill to the caller's own thread", rc * 10 + usr1, miss);
    }
    ok("tgkill", "a caught SIGUSR1 reached the thread getpid and gettid name; a stranger is errno",
       miss);
    return 0;
}

static volatile sig_atomic_t usr1_other;
static volatile int stop_yielding;
static volatile long yield_tid, usr1_tid;

static void on_usr1_other(int sig) {
    (void)sig;
    usr1_tid = gettid();
    usr1_other = 1;
}

static void *yielder(void *arg) {
    (void)arg;
    yield_tid = gettid();
    long start = now_ms();
    while (!usr1_other && !stop_yielding && now_ms() - start < 3000) {
        sched_yield();
    }
    return 0;
}

/* musl's pthread_kill sends tkill to the tid clone wrote for the parent. */
int thread_signal(void) {
    struct sigaction sa;
    memset(&sa, 0, sizeof sa);
    sa.sa_handler = on_usr1_other;
    sigaction(SIGUSR1, &sa, 0);
    pthread_t t;
    pthread_create(&t, 0, yielder, 0);
    int rc = pthread_kill(t, SIGUSR1);
    stop_yielding = rc != 0; /* a kill that never went need not wait 3 s */
    pthread_join(t, 0);
    if (rc != 0 || !usr1_other) {
        return fail("pthread_kill to another thread", rc, usr1_other);
    }
    /* Handled on the caller's own thread would also end the yield. */
    if (usr1_tid != yield_tid) {
        return fail("pthread_kill handled on the wrong thread", usr1_tid, yield_tid);
    }
    ok("pthread-kill", "pthread_kill reached another thread by its pthread tid; handled",
       usr1_other);
    return 0;
}
