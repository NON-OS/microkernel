// Waiting, as Linux waits: a timed futex, a condition variable broadcast, an
// eventfd read blocking until another thread writes, epoll_wait's timeout and
// its wake from another thread, a non-blocking pipe and its end of file, a
// write to a full pipe waiting for room, edge-triggered epoll, two readers
// blocked on one pipe, poll, ppoll and select with their timeouts, a closed
// descriptor leaving epoll, timerfd one-shot, periodic and absolute, and the
// descriptor ioctls and an epoll list carried through fork, and the scheduler
// calls with epoll_create and epoll_pwait2, and tgkill with the numbers
// getpid and gettid give. Each part prints as it passes and every part runs,
// so one run names each part that fails, and a hang the part it hung in.
#define _GNU_SOURCE
#include <errno.h>
#include <fcntl.h>
#include <poll.h>
#include <sched.h>
#include <pthread.h>
#include <signal.h>
#include <stdint.h>
#include <stdio.h>
#include <string.h>
#include <sys/select.h>
#include <sys/epoll.h>
#include <sys/eventfd.h>
#include <sys/ioctl.h>
#include <sys/timerfd.h>
#include <sys/syscall.h>
#include <sys/wait.h>
#include <time.h>
#include <ucontext.h>
#include <unistd.h>

static int parts;

static long now_ms(void) {
    struct timespec ts;
    clock_gettime(CLOCK_MONOTONIC, &ts);
    return ts.tv_sec * 1000 + ts.tv_nsec / 1000000;
}

static void nap_ms(long ms) {
    struct timespec ts = {ms / 1000, (ms % 1000) * 1000000};
    nanosleep(&ts, 0);
}

static int fail(const char *what, long a, long b) {
    printf("[C] cwait FAIL: %s (%ld, %ld)\n", what, a, b);
    fflush(stdout);
    return 1;
}

static void ok(const char *part, const char *detail, long n) {
    parts++;
    printf("[C] cwait %s ok: %s %ld\n", part, detail, n);
    fflush(stdout);
}

static pthread_mutex_t lock = PTHREAD_MUTEX_INITIALIZER;
static pthread_cond_t cond = PTHREAD_COND_INITIALIZER;
static int go_flag, woke;

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

static int efd_late;
static void *late_write(void *arg) {
    nap_ms(100);
    uint64_t v = (uint64_t)(uintptr_t)arg;
    write(efd_late, &v, 8);
    return 0;
}

static int pipe_drain;
static void *late_read(void *arg) {
    (void)arg;
    char buf[4096];
    nap_ms(100);
    read(pipe_drain, buf, sizeof buf);
    return 0;
}

static int two_in;
static char two_got[2];
static void *pipe_reader(void *arg) {
    read(two_in, &two_got[(uintptr_t)arg], 1);
    return 0;
}

static int late_fd;
static void *late_byte(void *arg) {
    (void)arg;
    nap_ms(100);
    write(late_fd, "z", 1);
    return 0;
}

static int timed_futex(void) {
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

static int broadcast(void) {
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

static int eventfd_semaphore(void) {
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

static int eventfd_blocking(void) {
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

static int epoll_timeout(void) {
    int ep = epoll_create1(EPOLL_CLOEXEC);
    int fd = eventfd(0, EFD_NONBLOCK);
    struct epoll_event ev = {.events = EPOLLIN, .data.u64 = 0xfeed};
    epoll_ctl(ep, EPOLL_CTL_ADD, fd, &ev);
    long t0 = now_ms();
    int n = epoll_wait(ep, &ev, 1, 100);
    long took = now_ms() - t0;
    if (n != 0 || took < 95 || took > 2000) {
        return fail("epoll_wait 100ms", n, took);
    }
    ok("epoll-timeout", "epoll_wait 100ms with nothing ready answered 0 after ms", took);
    efd_late = fd;
    pthread_t t;
    t0 = now_ms();
    pthread_create(&t, 0, late_write, (void *)(uintptr_t)1);
    n = epoll_wait(ep, &ev, 1, -1);
    took = now_ms() - t0;
    pthread_join(t, 0);
    if (n != 1 || ev.events != EPOLLIN || ev.data.u64 != 0xfeed || took < 90) {
        return fail("epoll_wait woken by a write", n, took);
    }
    ok("epoll-wake", "epoll_wait with no timeout woken by another thread's write, ms", took);
    close(fd);
    close(ep);
    return 0;
}

static int pipe_nonblock(void) {
    int p[2];
    pipe2(p, O_NONBLOCK);
    char c;
    if (read(p[0], &c, 1) != -1 || errno != EAGAIN) {
        return fail("empty non-blocking pipe read", errno, EAGAIN);
    }
    if (!(fcntl(p[0], F_GETFL) & O_NONBLOCK)) {
        return fail("F_GETFL after pipe2(O_NONBLOCK)", fcntl(p[0], F_GETFL), O_NONBLOCK);
    }
    fcntl(p[0], F_SETFL, 0);
    if (fcntl(p[0], F_GETFL) & O_NONBLOCK) {
        return fail("F_SETFL 0 kept O_NONBLOCK", fcntl(p[0], F_GETFL), 0);
    }
    close(p[1]);
    struct pollfd pf = {.fd = p[0], .events = POLLIN};
    int ready = poll(&pf, 1, 0);
    if (read(p[0], &c, 1) != 0 || ready != 1 || !(pf.revents & POLLHUP)) {
        return fail("end of file after the write end closed", ready, pf.revents);
    }
    close(p[0]);
    ok("pipe", "EAGAIN empty, O_NONBLOCK set and cleared, end of file and POLLHUP; revents",
       pf.revents);
    return 0;
}

static int pipe_full(void) {
    int p[2];
    pipe(p);
    fcntl(p[1], F_SETFL, O_NONBLOCK);
    static char block[4096];
    long held = 0;
    while (write(p[1], block, sizeof block) == sizeof block) {
        held += sizeof block;
    }
    fcntl(p[1], F_SETFL, 0);
    pipe_drain = p[0];
    pthread_t t;
    long t0 = now_ms();
    pthread_create(&t, 0, late_read, 0);
    ssize_t n = write(p[1], block, sizeof block);
    long took = now_ms() - t0;
    pthread_join(t, 0);
    close(p[0]);
    close(p[1]);
    if (held != 65536 || n != sizeof block || took < 90) {
        return fail("write to a full pipe", held, took);
    }
    ok("pipe-full", "a 4096 byte write to a full 65536 byte pipe waited for a reader, ms", took);
    return 0;
}

static int edge(void) {
    int p[2];
    pipe2(p, O_NONBLOCK);
    int ep = epoll_create1(0);
    struct epoll_event ev = {.events = EPOLLIN | EPOLLET, .data.u64 = 1};
    epoll_ctl(ep, EPOLL_CTL_ADD, p[0], &ev);
    struct epoll_event lt = {.events = EPOLLOUT, .data.u64 = 2};
    epoll_ctl(ep, EPOLL_CTL_ADD, p[1], &lt);
    struct epoll_event got[2];
    char c;
    write(p[1], "x", 1);
    int first = epoll_wait(ep, got, 2, 0);
    int again = epoll_wait(ep, got, 2, 0);
    read(p[0], &c, 1);
    int drained = read(p[0], &c, 1) == -1 && errno == EAGAIN;
    write(p[1], "y", 1);
    int rearmed = epoll_wait(ep, got, 2, 0);
    // The write end is level-triggered and always writable: counted each time.
    if (first != 2 || again != 1 || !drained || rearmed != 2) {
        return fail("edge-triggered counts", first * 100 + again * 10 + rearmed, drained);
    }
    if (epoll_ctl(ep, EPOLL_CTL_ADD, p[0], &ev) != -1 || errno != EEXIST) {
        return fail("adding twice", errno, EEXIST);
    }
    close(p[0]);
    close(p[1]);
    close(ep);
    ok("edge", "EPOLLET reported once per rise, again after EAGAIN; events", first + again + rearmed);
    return 0;
}

static int two_readers(void) {
    int p[2];
    pipe(p);
    two_in = p[0];
    pthread_t t[2];
    pthread_create(&t[0], 0, pipe_reader, (void *)0);
    pthread_create(&t[1], 0, pipe_reader, (void *)1);
    nap_ms(100);
    write(p[1], "ab", 2);
    pthread_join(t[0], 0);
    pthread_join(t[1], 0);
    close(p[0]);
    close(p[1]);
    int both = (two_got[0] == 'a' && two_got[1] == 'b') || (two_got[0] == 'b' && two_got[1] == 'a');
    if (!both) {
        return fail("two blocked pipe readers", two_got[0], two_got[1]);
    }
    ok("pipe-readers", "two threads blocked reading one pipe, both answered; bytes", 2);
    return 0;
}

static int poll_select(void) {
    int p[2];
    pipe(p);
    struct pollfd pf = {.fd = p[0], .events = POLLIN};
    long t0 = now_ms();
    int n = poll(&pf, 1, 100);
    long polled = now_ms() - t0;
    t0 = now_ms();
    poll(0, 0, 50);
    long slept = now_ms() - t0;
    struct timespec ts = {0, 100 * 1000000};
    t0 = now_ms();
    int m = ppoll(&pf, 1, &ts, 0);
    long ppolled = now_ms() - t0;
    if (n != 0 || polled < 95 || slept < 45 || m != 0 || ppolled < 95 || polled > 2000) {
        return fail("poll and ppoll timeouts", polled, ppolled);
    }
    fd_set rd;
    FD_ZERO(&rd);
    FD_SET(p[0], &rd);
    struct timeval tv = {0, 100 * 1000};
    t0 = now_ms();
    int s = select(p[0] + 1, &rd, 0, 0, &tv);
    long selected = now_ms() - t0;
    if (s != 0 || FD_ISSET(p[0], &rd) || selected < 95) {
        return fail("select timeout", s, selected);
    }
    FD_SET(p[0], &rd);
    late_fd = p[1];
    pthread_t t;
    t0 = now_ms();
    pthread_create(&t, 0, late_byte, 0);
    s = select(p[0] + 1, &rd, 0, 0, 0);
    long woken = now_ms() - t0;
    pthread_join(t, 0);
    close(p[0]);
    close(p[1]);
    if (s != 1 || !FD_ISSET(p[0], &rd) || woken < 90) {
        return fail("select woken by a write", s, woken);
    }
    ok("poll-select", "poll, ppoll and select waited their 100ms timeouts, select woken after ms",
       woken);
    return 0;
}

static int close_forgets(void) {
    int ep = epoll_create1(0);
    int p[2];
    pipe(p);
    write(p[1], "x", 1);
    struct epoll_event ev = {.events = EPOLLIN, .data.u64 = 7};
    epoll_ctl(ep, EPOLL_CTL_ADD, p[0], &ev);
    int number = p[0];
    close(p[0]);
    close(p[1]);
    int q[2];
    pipe(q);
    int reused = q[0] == number;
    int added = epoll_ctl(ep, EPOLL_CTL_ADD, q[0], &ev);
    struct epoll_event got;
    int stale = epoll_wait(ep, &got, 1, 0);
    int far = dup2(0, 100000) == -1 && errno == EBADF;
    close(q[0]);
    close(q[1]);
    close(ep);
    if (!reused || added != 0 || stale != 0 || !far) {
        return fail("close leaves epoll", added * 10 + stale, reused * 10 + far);
    }
    ok("close-forget", "a closed descriptor left epoll and its number was added again; stale events",
       stale);
    return 0;
}

static int timers(void) {
    int tf = timerfd_create(CLOCK_MONOTONIC, TFD_NONBLOCK | TFD_CLOEXEC);
    uint64_t n = 0;
    if (read(tf, &n, 8) != -1 || errno != EAGAIN) {
        return fail("unarmed timer read", (long)n, errno);
    }
    struct itimerspec every = {{0, 50 * 1000000}, {0, 50 * 1000000}};
    timerfd_settime(tf, 0, &every, 0);
    nap_ms(180);
    struct itimerspec now;
    timerfd_gettime(tf, &now);
    if (read(tf, &n, 8) != 8 || n < 3 || n > 4 || now.it_interval.tv_nsec != 50 * 1000000) {
        return fail("periodic timer count", (long)n, now.it_interval.tv_nsec);
    }
    long periodic = (long)n;
    close(tf);
    tf = timerfd_create(CLOCK_MONOTONIC, 0);
    struct itimerspec once = {{0, 0}, {0, 100 * 1000000}};
    timerfd_settime(tf, 0, &once, 0);
    long t0 = now_ms();
    ssize_t got = read(tf, &n, 8);
    long waited = now_ms() - t0;
    if (got != 8 || n != 1 || waited < 90) {
        return fail("blocking timer read", (long)n, waited);
    }
    struct timespec at;
    clock_gettime(CLOCK_MONOTONIC, &at);
    at.tv_nsec += 100 * 1000000;
    if (at.tv_nsec >= 1000000000) {
        at.tv_sec++;
        at.tv_nsec -= 1000000000;
    }
    struct itimerspec abs = {{0, 0}, at};
    timerfd_settime(tf, TFD_TIMER_ABSTIME, &abs, 0);
    int ep = epoll_create1(0);
    struct epoll_event ev = {.events = EPOLLIN, .data.u64 = 3};
    epoll_ctl(ep, EPOLL_CTL_ADD, tf, &ev);
    t0 = now_ms();
    int ready = epoll_wait(ep, &ev, 1, -1);
    long absolute = now_ms() - t0;
    close(ep);
    close(tf);
    if (ready != 1 || absolute < 90 || absolute > 2000) {
        return fail("absolute timer through epoll", ready, absolute);
    }
    printf("[C] cwait timerfd detail: periodic 50ms fired %ld times in 180ms, one-shot read waited %ld "
           "ms, absolute +100ms woke epoll after %ld ms\n",
           periodic, waited, absolute);
    ok("timerfd", "periodic, one-shot blocking and absolute timers; periodic count", periodic);
    return 0;
}

static int ioctls_fork(void) {
    int p[2];
    pipe(p);
    write(p[1], "abc", 3);
    int held = -1;
    ioctl(p[0], FIONREAD, &held);
    int one = 1;
    ioctl(p[0], FIONBIO, &one);
    char buf[4];
    ssize_t got = read(p[0], buf, sizeof buf);
    int drained = read(p[0], buf, 1) == -1 && errno == EAGAIN;
    ioctl(p[1], FIOCLEX);
    int cloexec = fcntl(p[1], F_GETFD) == FD_CLOEXEC;
    if (held != 3 || got != 3 || !drained || !cloexec) {
        return fail("descriptor ioctls", held, got * 10 + drained * 2 + cloexec);
    }
    int ep = epoll_create1(0);
    struct epoll_event ev = {.events = EPOLLIN, .data.u64 = 9};
    epoll_ctl(ep, EPOLL_CTL_ADD, p[0], &ev);
    write(p[1], "d", 1);
    pid_t child = fork();
    if (child == 0) {
        _exit(epoll_wait(ep, &ev, 1, 0) == 1 && ev.data.u64 == 9 ? 0 : 1);
    }
    int status = -1;
    waitpid(child, &status, 0);
    close(ep);
    close(p[0]);
    close(p[1]);
    if (child < 0 || !WIFEXITED(status) || WEXITSTATUS(status) != 0) {
        return fail("epoll list through fork", child, status);
    }
    ok("ioctl-fork", "FIONREAD, FIONBIO, FIOCLEX, and a forked child saw the epoll list; bytes held",
       held);
    return 0;
}

/* musl answers ENOSYS for the policy calls by design, so they are made raw.
 * SCHED_FIFO at priority 0 is EINVAL everywhere, the range being checked
 * before the privilege; at priority 1 it depends on privilege, and a CPU-1
 * mask on the CPU count, so those two are reported rather than checked: on
 * NONOS they are EPERM and EINVAL. */
static int scheduler(void) {
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

static volatile sig_atomic_t usr1;
static void on_usr1(int sig) {
    (void)sig;
    usr1 = 1;
}

static int thread_kill(void) {
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

/* The raw call, so the answers are the kernel's and not musl's own checks. */
static long altstack(const stack_t *set, stack_t *old) {
    return syscall(SYS_sigaltstack, set, old) == -1 ? -errno : 0;
}

static char alt[16384] __attribute__((aligned(16)));
static volatile uintptr_t alt_here, alt_saved_rsp, alt_uc_sp;
static volatile long alt_inside_flags = -1, alt_inside_set, alt_uc_flags = -1;

static int on_alt(uintptr_t p) {
    return p > (uintptr_t)alt && p <= (uintptr_t)alt + sizeof alt;
}

static void on_usr2(int sig, siginfo_t *info, void *ctx) {
    (void)sig;
    (void)info;
    char here;
    alt_here = (uintptr_t)&here;
    stack_t now, other = {.ss_sp = alt, .ss_size = sizeof alt, .ss_flags = 0};
    alt_inside_flags = altstack(0, &now) == 0 ? now.ss_flags : -1;
    alt_inside_set = altstack(&other, 0);
    ucontext_t *uc = ctx;
    alt_uc_flags = uc->uc_stack.ss_flags;
    alt_uc_sp = (uintptr_t)uc->uc_stack.ss_sp;
    alt_saved_rsp = (uintptr_t)uc->uc_mcontext.gregs[REG_RSP];
}

static int alternate_stack(void) {
    stack_t got, small = {.ss_sp = alt, .ss_size = 1024, .ss_flags = 0};
    stack_t bad = {.ss_sp = alt, .ss_size = sizeof alt, .ss_flags = 4};
    stack_t set = {.ss_sp = alt, .ss_size = sizeof alt, .ss_flags = 0};
    stack_t off = {.ss_flags = SS_DISABLE};
    long none = altstack(0, &got) == 0 ? got.ss_flags : -1;
    long nomem = altstack(&small, 0), inval = altstack(&bad, 0);
    if (none != SS_DISABLE || nomem != -ENOMEM || inval != -EINVAL) {
        return fail("sigaltstack before one is set", none * 100 - nomem, inval);
    }
    long rc = altstack(&set, 0);
    long flags = altstack(0, &got) == 0 ? got.ss_flags : -1;
    if (rc != 0 || flags != 0 || got.ss_sp != alt || got.ss_size != sizeof alt) {
        return fail("sigaltstack set and read back", rc, flags);
    }
    struct sigaction sa;
    memset(&sa, 0, sizeof sa);
    sa.sa_sigaction = on_usr2;
    sa.sa_flags = SA_SIGINFO | SA_ONSTACK;
    sigaction(SIGUSR2, &sa, 0);
    syscall(SYS_tgkill, getpid(), gettid(), SIGUSR2);
    getpid();
    long off_rc = altstack(&off, 0);
    long after = altstack(0, &got) == 0 ? got.ss_flags : -1;
    if (!on_alt(alt_here) || on_alt(alt_saved_rsp) || alt_uc_sp != (uintptr_t)alt) {
        return fail("SA_ONSTACK handler on the alternate stack", on_alt(alt_here),
                    on_alt(alt_saved_rsp));
    }
    if (alt_inside_flags != SS_ONSTACK || alt_inside_set != -EPERM || alt_uc_flags != 0) {
        return fail("sigaltstack inside the handler", alt_inside_flags, alt_inside_set);
    }
    if (off_rc != 0 || after != SS_DISABLE) {
        return fail("sigaltstack disabled", off_rc, after);
    }
    ok("altstack",
       "ENOMEM, EINVAL, set, SA_ONSTACK handler on it with SS_ONSTACK and EPERM inside, "
       "disabled; flags inside",
       alt_inside_flags);
    return 0;
}

int main(void) {
    int (*const part[])(void) = {
        timed_futex,  broadcast, eventfd_semaphore, eventfd_blocking, epoll_timeout,
        pipe_nonblock, pipe_full, edge,             two_readers,      poll_select,
        close_forgets, timers,   ioctls_fork,       scheduler,        thread_kill,
        alternate_stack,
    };
    const int count = sizeof part / sizeof part[0];
    long t0 = now_ms();
    int failed = 0;
    // Every part runs, so one run names every part that fails; a hang still
    // stops it, at the part it hangs in.
    for (int i = 0; i < count; i++) {
        failed += part[i]();
    }
    if (failed) {
        printf("[C] cwait FAIL: %d parts failed, %d passed\n", failed, parts);
        fflush(stdout);
        return 1;
    }
    printf("[C] cwait PASS: %d parts in %ld ms\n", parts, now_ms() - t0);
    fflush(stdout);
    return 0;
}
