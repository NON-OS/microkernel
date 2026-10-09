#include "cfiles.h"

static long spin(void) {
    volatile long n = 0;
    struct timespec a, z;
    clock_gettime(CLOCK_MONOTONIC, &a);
    do {
        for (int i = 0; i < 100000; i++) {
            n += i;
        }
        clock_gettime(CLOCK_MONOTONIC, &z);
    } while ((z.tv_sec - a.tv_sec) * 1000 + (z.tv_nsec - a.tv_nsec) / 1000000 < 1500);
    return n;
}

static long ms(struct timeval t) {
    return t.tv_sec * 1000 + t.tv_usec / 1000;
}

void part_usage(void) {
    const char *p = "usage";
    struct sysinfo si;
    memset(&si, 0, sizeof si);
    int rc = sysinfo(&si);
    CHECK(p, rc == 0 && si.mem_unit == 1 && si.totalram > 0, rc, si.mem_unit);
    CHECK(p, si.freeram <= si.totalram && si.procs >= 1, si.freeram, si.procs);
    struct rusage before, after, kids;
    getrusage(RUSAGE_SELF, &before);
    spin();
    getrusage(RUSAGE_SELF, &after);
    long used = ms(after.ru_utime) + ms(after.ru_stime) - ms(before.ru_utime) - ms(before.ru_stime);
    CHECK(p, used >= 100, used, 100);
    struct rusage th;
    CHECK(p, getrusage(RUSAGE_THREAD, &th) == 0 && ms(th.ru_utime) + ms(th.ru_stime) >= 100, ms(th.ru_utime), 0);
    getrusage(RUSAGE_CHILDREN, &kids);
    long before_kid = ms(kids.ru_utime) + ms(kids.ru_stime);
    int spun[2];
    pipe(spun);
    pid_t kid = fork();
    if (kid == 0) {
        spin();
        write(spun[1], "s", 1);
        _exit(0);
    }
    char c;
    read(spun[0], &c, 1);
    nap_ms(300);
    /* Exited but not yet waited for: Linux does not count it yet. */
    getrusage(RUSAGE_CHILDREN, &kids);
    long unwaited = ms(kids.ru_utime) + ms(kids.ru_stime) - before_kid;
    waitpid(kid, 0, 0);
    getrusage(RUSAGE_CHILDREN, &kids);
    long child = ms(kids.ru_utime) + ms(kids.ru_stime) - before_kid;
    CHECK(p, unwaited == 0 && child >= 100, unwaited, child);
    ERR(p, getrusage(7, &kids), EINVAL);
    struct tms t;
    clock_t now = times(&t);
    CHECK(p, now > 0 && t.tms_utime + t.tms_stime >= 10 && t.tms_cutime + t.tms_cstime >= 10, t.tms_utime, t.tms_cutime);
    done(p, "sysinfo, and getrusage and times measure a busy loop and a waited child");
}
