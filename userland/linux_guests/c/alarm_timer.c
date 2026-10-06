/*
 * A POSIX timer, checked against Linux: timer_create on the monotonic clock
 * with SIGEV_SIGNAL, armed for 100 ms with a 100 ms period, gives three
 * SI_TIMER signals carrying its value through sigtimedwait; timer_gettime
 * reads its period back, timer_getoverrun answers, timer_delete ends it.
 */
#define _GNU_SOURCE
#include <string.h>

#include "alarm.h"

void posix_timer(void) {
    sigset_t rt;
    sigemptyset(&rt);
    sigaddset(&rt, SIGRTMIN);
    sigprocmask(SIG_BLOCK, &rt, 0);
    struct sigevent ev;
    memset(&ev, 0, sizeof ev);
    ev.sigev_notify = SIGEV_SIGNAL;
    ev.sigev_signo = SIGRTMIN;
    ev.sigev_value.sival_int = 77;
    timer_t id;
    int rc = timer_create(CLOCK_MONOTONIC, &ev, &id);
    struct itimerspec ts = {{0, 100000000}, {0, 100000000}}, got;
    timer_settime(id, 0, &ts, 0);
    siginfo_t si;
    struct timespec wait = {1, 0};
    int fired = 0;
    for (int i = 0; i < 3; i++) {
        if (sigtimedwait(&rt, &si, &wait) == SIGRTMIN && si.si_code == SI_TIMER &&
            si.si_value.sival_int == 77) {
            fired++;
        }
    }
    timer_gettime(id, &got);
    int over = timer_getoverrun(id);
    part(rc == 0 && fired == 3 && got.it_interval.tv_nsec == 100000000 && over >= 0,
         "timer_create 100 ms: three SI_TIMER signals, value 77", fired);
    part(timer_delete(id) == 0, "timer_delete", 0);
}
