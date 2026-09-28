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
    pid_t c = fork();
    if (c == 0) {
        /* Fresh memory faults on first touch; a fork's copy need not. */
        char *pages = mmap(0, 1 << 20, PROT_READ | PROT_WRITE, MAP_PRIVATE | MAP_ANONYMOUS, -1, 0);
        memset(pages, 1, 1 << 20);
        spin_ms(500);
        _exit(0);
    }
    CHECK(p, c > 0 && waitpid(c, 0, 0) == c, c, errno);
    unsigned long long cut = stat_field(16) + stat_field(17), cmin = stat_field(11);
    CHECK(p, cut >= 20 && cmin >= 64, cut, cmin);
    done(p, "startcode, endcode, startstack, start_data, end_data, and a waited child's times and faults");
}
