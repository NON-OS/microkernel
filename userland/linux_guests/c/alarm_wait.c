/*
 * The signal waits, checked against Linux: a blocked SIGUSR1 shows in
 * sigpending, sigsuspend runs its handler and answers EINTR with the mask put
 * back, sigqueue hands a value to sigtimedwait, and a sigtimedwait with
 * nothing pending times out. The SIGUSR1 handler reads its own ucontext: the
 * interrupted rip must lie in this program's text and the saved mask must be
 * the one sigsuspend replaced, which holds only at Linux's offsets.
 */
#define _GNU_SOURCE
#include <errno.h>
#include <string.h>
#include <ucontext.h>
#include <unistd.h>

#include "alarm.h"

extern char __executable_start[], etext[];
volatile sig_atomic_t usr1, usr1_rip_in_text, usr1_saved_blocked;

static void on_usr1(int sig, siginfo_t *info, void *ctx) {
    (void)sig;
    (void)info;
    ucontext_t *uc = ctx;
    unsigned long rip = (unsigned long)uc->uc_mcontext.gregs[REG_RIP];
    usr1_rip_in_text = rip >= (unsigned long)__executable_start && rip < (unsigned long)etext;
    usr1_saved_blocked = sigismember(&uc->uc_sigmask, SIGUSR1);
    usr1++;
}

void signal_waits(void) {
    struct sigaction sa;
    memset(&sa, 0, sizeof sa);
    sa.sa_sigaction = on_usr1;
    sa.sa_flags = SA_SIGINFO;
    sigaction(SIGUSR1, &sa, 0);
    sigset_t block, none, pending, after, two;
    sigemptyset(&block);
    sigaddset(&block, SIGUSR1);
    sigprocmask(SIG_BLOCK, &block, 0);
    raise(SIGUSR1);
    sigpending(&pending);
    part(usr1 == 0 && sigismember(&pending, SIGUSR1), "a blocked SIGUSR1 waits, pending", 0);
    sigemptyset(&none);
    errno = 0;
    int rc = sigsuspend(&none), e = errno;
    sigprocmask(SIG_BLOCK, 0, &after);
    part(rc == -1 && e == EINTR && usr1 == 1 && sigismember(&after, SIGUSR1),
         "sigsuspend ran the handler, EINTR, mask restored", usr1);
    part(usr1_rip_in_text && usr1_saved_blocked,
         "the handler's ucontext: rip in the program's text, uc_sigmask the saved mask", 0);

    sigemptyset(&two);
    sigaddset(&two, SIGUSR2);
    sigprocmask(SIG_BLOCK, &two, 0);
    union sigval v = {.sival_int = 1234};
    sigqueue(getpid(), SIGUSR2, v);
    siginfo_t si;
    struct timespec wait = {1, 0}, brief = {0, 100000000}, t0;
    rc = sigtimedwait(&two, &si, &wait);
    part(rc == SIGUSR2 && si.si_code == SI_QUEUE && si.si_value.sival_int == 1234 &&
             si.si_pid == getpid(),
         "sigqueue into sigtimedwait: SI_QUEUE, value", si.si_value.sival_int);
    clock_gettime(CLOCK_MONOTONIC, &t0);
    errno = 0;
    rc = sigtimedwait(&two, &si, &brief);
    e = errno;
    long ms = ms_since(&t0);
    part(rc == -1 && e == EAGAIN && ms >= 90 && ms < 1000, "sigtimedwait timed out: EAGAIN, ms",
         ms);
}
