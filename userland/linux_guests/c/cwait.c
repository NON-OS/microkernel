// Waiting, as Linux waits: a timed futex, a condition variable broadcast, an
// eventfd read blocking until another thread writes, epoll_wait's timeout and
// its wake from another thread, a non-blocking pipe and its end of file, a
// write to a full pipe waiting for room, edge-triggered epoll, two readers
// blocked on one pipe, poll, ppoll and select with their timeouts, and a
// closed descriptor leaving epoll. Each part prints as it passes, so a hang
// names the part it hung in.
#define _GNU_SOURCE
#include <errno.h>
#include <fcntl.h>
#include <poll.h>
#include <pthread.h>
#include <stdint.h>
#include <stdio.h>
#include <string.h>
#include <sys/select.h>
#include <sys/epoll.h>
#include <sys/eventfd.h>
#include <time.h>
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

int main(void) {
    long t0 = now_ms();
    if (timed_futex() || broadcast() || eventfd_semaphore() || eventfd_blocking() ||
        epoll_timeout() || pipe_nonblock() || pipe_full() || edge() || two_readers() ||
        poll_select() || close_forgets()) {
        return 1;
    }
    printf("[C] cwait PASS: %d parts in %ld ms\n", parts, now_ms() - t0);
    fflush(stdout);
    return 0;
}
