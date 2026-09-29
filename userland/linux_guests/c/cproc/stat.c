#include "cproc.h"

static int inited = 7;

/* The n-th field of /proc/self/stat, numbered as proc(5) numbers them. */
static unsigned long long stat_field(int n) {
    long len;
    char *q = strrchr(slurp("/proc/self/stat", &len), ')') + 2;
    for (int i = 3; i < n && q; i++) {
        q = strchr(q, ' ');
        q = q ? q + 1 : 0;
    }
    return q ? strtoull(q, 0, 10) : 0;
}

void part_stat(void) {
    const char *p = "stat";
    unsigned long long code0 = stat_field(26), code1 = stat_field(27);
    unsigned long here = (unsigned long)&part_stat, data = (unsigned long)&inited;
    CHECK(p, code0 <= here && here < code1, code0, code1);
    CHECK(p, (unsigned long)args == stat_field(28) + 8, (long)args, stat_field(28));
    CHECK(p, stat_field(45) <= data && data < stat_field(46), stat_field(45), stat_field(46));
    int fds[2];
    CHECK(p, pipe(fds) == 0, errno, 0);
    pid_t c = fork();
    if (c == 0) {
        /* A child that waits for a grandchild: Linux counts both to us. */
        pid_t g = fork();
        if (g == 0) {
            struct rusage r;
            spin_ms(500);
            getrusage(RUSAGE_SELF, &r);
            unsigned long long mine[2] = {stat_field(10), r.ru_nvcsw + r.ru_nivcsw};
            _exit(write(fds[1], mine, sizeof mine) != sizeof mine);
        }
        _exit(g < 0 || waitpid(g, 0, 0) != g);
    }
    int st = -1;
    unsigned long long mine[2] = {0, 0};
    CHECK(p, c > 0 && waitpid(c, &st, 0) == c && st == 0, c, st);
    CHECK(p, read(fds[0], mine, sizeof mine) == sizeof mine, errno, 0);
    close(fds[0]);
    close(fds[1]);
    struct rusage kids;
    getrusage(RUSAGE_CHILDREN, &kids);
    unsigned long long cut = stat_field(16) + stat_field(17), cmin = stat_field(11);
    CHECK(p, cut >= 20 && cmin >= mine[0], cut, cmin);
    unsigned long long switched = kids.ru_nvcsw + kids.ru_nivcsw;
    CHECK(p, switched >= mine[1], switched, mine[1]);
    done(p, "startcode, endcode, startstack, start_data, end_data, and the times, faults and switches of a waited child and of the grandchild it waited for");
}
