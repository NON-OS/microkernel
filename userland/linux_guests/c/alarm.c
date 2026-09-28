/*
 * Signals that arrive while a thread waits, and the calls that wait for them.
 * alarm(1) then sleep(10): on Linux SIGALRM ends the sleep early with its
 * handler run, well under two seconds in. Then setitimer and getitimer with
 * pause; the signal waits and POSIX timers follow in alarm_wait.c. Every part
 * prints its line.
 */
#define _GNU_SOURCE
#include <stdio.h>
#include <string.h>
#include <sys/time.h>
#include <unistd.h>
#include "alarm.h"

volatile sig_atomic_t alarms;
static int failed, parts;

static void on_alrm(int sig) { (void)sig; alarms++; }

void part(int ok, const char *what, long n) {
    printf("[C] alarm %s: %s (%ld)\n", ok ? "ok" : "FAIL", what, n);
    fflush(stdout);
    failed += !ok;
    parts++;
}

long ms_since(struct timespec *t0) {
    struct timespec t;
    clock_gettime(CLOCK_MONOTONIC, &t);
    return (t.tv_sec - t0->tv_sec) * 1000 + (t.tv_nsec - t0->tv_nsec) / 1000000;
}

void catch(int sig, void (*fn)(int)) {
    struct sigaction sa;
    memset(&sa, 0, sizeof sa);
    sa.sa_handler = fn;
    sigaction(sig, &sa, 0);
}

int verdict(void) {
    printf("[C] alarm %s: %d of %d parts held\n", failed ? "FAIL" : "PASS", parts - failed, parts);
    fflush(stdout);
    return failed != 0;
}

int main(void) {
    struct timespec t0;
    catch(SIGALRM, on_alrm);
    clock_gettime(CLOCK_MONOTONIC, &t0);
    alarm(1);
    unsigned left = sleep(10);
    long ms = ms_since(&t0);
    part(alarms == 1 && ms < 2000 && left >= 8, "alarm(1) ended sleep(10) early, ms", ms);
    struct itimerval it = {{0, 200000}, {0, 200000}}, now;
    alarms = 0;
    clock_gettime(CLOCK_MONOTONIC, &t0);
    setitimer(ITIMER_REAL, &it, 0);
    /* Bounded: where pause fails at once instead of waiting, this still ends. */
    for (int i = 0; alarms < 3 && i < 50; i++) {
        pause();
    }
    ms = ms_since(&t0);
    getitimer(ITIMER_REAL, &now);
    part(ms >= 550 && ms < 1500 && now.it_interval.tv_usec == 200000 &&
             now.it_value.tv_usec <= 200000,
         "setitimer 200 ms interval: three SIGALRMs through pause, ms", ms);
    memset(&it, 0, sizeof it);
    setitimer(ITIMER_REAL, &it, &now);
    getitimer(ITIMER_REAL, &now);
    part(now.it_value.tv_sec == 0 && now.it_value.tv_usec == 0, "a disarmed timer reads zero", 0);
    part(alarm(0) == 0, "alarm(0) with nothing armed answers 0", 0);
    signal_waits();
    posix_timer();
    return verdict();
}
