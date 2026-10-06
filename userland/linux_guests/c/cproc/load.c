#include "cproc.h"

void part_load(void) {
    const char *p = "load";
    spin_ms(11000);
    long n;
    double l1 = -1;
    sscanf(slurp("/proc/loadavg", &n), "%lf", &l1);
    struct sysinfo si;
    sysinfo(&si);
    long a = (long)(l1 * 100 + 0.5), b = (long)(si.loads[0] * 100 / 65536);
    CHECK(p, a > 0 && b > 0 && a - b <= 5 && b - a <= 5, a, b);
    done(p, "eleven seconds on the CPU show in loadavg and in sysinfo's loads alike");
}
