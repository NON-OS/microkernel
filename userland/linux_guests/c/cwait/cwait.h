/*
 * Waiting, as Linux waits: a timed futex, a condition variable broadcast, an
 * eventfd read blocking until another thread writes, epoll_wait's timeout and
 * its wake from another thread, a non-blocking pipe and its end of file, a
 * write to a full pipe waiting for room, edge-triggered epoll, two readers
 * blocked on one pipe, poll, ppoll and select with their timeouts, a closed
 * descriptor leaving epoll, timerfd one-shot, periodic and absolute, and the
 * descriptor ioctls and an epoll list carried through fork, and the scheduler
 * calls with epoll_create and epoll_pwait2, and tgkill with the numbers
 * getpid and gettid give. Each part prints as it passes and every part runs,
 * so one run names each part that fails, and a hang the part it hung in.
 */
#ifndef CWAIT_H
#define CWAIT_H

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

extern int parts;
extern int woke;
extern int efd_late;

long now_ms(void);
void nap_ms(long ms);
int fail(const char *what, long a, long b);
void ok(const char *part, const char *detail, long n);
void *late_write(void *arg);

int timed_futex(void);
int broadcast(void);
int eventfd_semaphore(void);
int eventfd_blocking(void);
int epoll_timeout(void);
int pipe_nonblock(void);
int pipe_full(void);
int edge(void);
int two_readers(void);
int poll_select(void);
int close_forgets(void);
int timers(void);
int ioctls_fork(void);
int scheduler(void);
int thread_kill(void);
int alternate_stack(void);
int thread_signal(void);

#endif
