#include "cproc.h"

static volatile int stop;

static volatile pid_t helper_tid;

static void *helper(void *arg) {
    (void)arg;
    helper_tid = syscall(SYS_gettid);
    while (!stop) {
        struct timespec ts = {0, 20000000};
        nanosleep(&ts, 0);
    }
    return 0;
}

void part_maps(void) {
    const char *p = "maps";
    long n;
    char *maps = slurp("/proc/self/maps", &n);
    uintptr_t local = (uintptr_t)&n;
    int stack = 0, lines = 0;
    for (char *line = maps; line && *line; line = strchr(line, '\n') ? strchr(line, '\n') + 1 : 0) {
        unsigned long lo, hi;
        char perms[5];
        if (sscanf(line, "%lx-%lx %4s", &lo, &hi, perms) != 3) {
            break;
        }
        lines++;
        stack |= lo <= local && local < hi && perms[0] == 'r' && perms[1] == 'w';
    }
    CHECK(p, lines >= 3 && stack, lines, stack);
    char *statm = slurp("/proc/self/statm", &n);
    long f[7];
    CHECK(p, sscanf(statm, "%ld %ld %ld %ld %ld %ld %ld", &f[0], &f[1], &f[2], &f[3], &f[4], &f[5], &f[6]) == 7 && f[0] > 0, f[0], 0);
    char *limits = slurp("/proc/self/limits", &n);
    struct rlimit rl;
    getrlimit(RLIMIT_NOFILE, &rl);
    char want[128];
    snprintf(want, sizeof want, "Max open files            %-20lu %-20lu files", (unsigned long)rl.rlim_cur, (unsigned long)rl.rlim_max);
    CHECK(p, strstr(limits, want) != 0, rl.rlim_cur, 0);
    CHECK(p, strstr(slurp("/proc/self/mounts", &n), " /proc proc ") != 0, n, 0);
    CHECK(p, strstr(slurp("/proc/self/mountinfo", &n), " /proc ") != 0, n, 0);
    /* v1 or v2, each line is id:controllers:/path. */
    char *cg = slurp("/proc/self/cgroup", &n);
    char *c1 = strchr(cg, ':'), *c2 = c1 ? strchr(c1 + 1, ':') : 0;
    CHECK(p, n > 0 && c2 && c2[1] == '/' && cg[n - 1] == '\n', n, 0);
    pthread_t t;
    pthread_create(&t, 0, helper, 0);
    while (!helper_tid) {
        sched_yield();
    }
    int tasks = 0;
    DIR *d = opendir("/proc/self/task");
    for (struct dirent *e; d && (e = readdir(d));) {
        tasks += atoi(e->d_name) > 0;
    }
    closedir(d);
    char path[96];
    snprintf(path, sizeof path, "/proc/self/task/%d/stat", (int)helper_tid);
    char *tstat = slurp(path, &n);
    int tid_ok = tstat && atoi(tstat) == helper_tid;
    long threads = field_of(slurp("/proc/self/status", &n), "\nThreads:\t");
    stop = 1;
    pthread_join(t, 0);
    CHECK(p, tasks == 2 && tid_ok && threads == 2, tasks, threads);
    done(p, "maps holds the stack, statm, limits, mounts, cgroup, and task/ has each thread");
}
