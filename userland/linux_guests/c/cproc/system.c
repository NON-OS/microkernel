#include "cproc.h"

void part_system(void) {
    const char *p = "system";
    long n;
    struct sysinfo si;
    sysinfo(&si);
    long total = field_of(slurp("/proc/meminfo", &n), "MemTotal:");
    CHECK(p, total == (long)(si.totalram * si.mem_unit / 1024), total, si.totalram);
    cpu_set_t set;
    sched_getaffinity(0, sizeof set, &set);
    int cpus = 0;
    char *info = slurp("/proc/cpuinfo", &n);
    for (char *at = info; (at = strstr(at, "processor\t:")); at++) {
        cpus++;
    }
    CHECK(p, cpus == CPU_COUNT(&set) && cpus == get_nprocs(), cpus, CPU_COUNT(&set));
    double up = -1, idle = -1;
    sscanf(slurp("/proc/uptime", &n), "%lf %lf", &up, &idle);
    CHECK(p, up > 0 && idle >= 0, (long)up, (long)idle);
    double l1, l5, l15;
    int run, all, last;
    CHECK(p, sscanf(slurp("/proc/loadavg", &n), "%lf %lf %lf %d/%d %d", &l1, &l5, &l15, &run, &all, &last) == 6 && run >= 1 && all >= run, run, all);
    struct utsname u;
    uname(&u);
    char want[160];
    snprintf(want, sizeof want, "Linux version %s ", u.release);
    CHECK(p, strncmp(slurp("/proc/version", &n), want, strlen(want)) == 0, n, 0);
    CHECK(p, strstr(slurp("/proc/filesystems", &n), "\tproc\n") != 0, n, 0);
    snprintf(want, sizeof want, "%s\n", u.sysname);
    CHECK(p, strcmp(slurp("/proc/sys/kernel/ostype", &n), want) == 0, n, 0);
    snprintf(want, sizeof want, "%s\n", u.release);
    CHECK(p, strcmp(slurp("/proc/sys/kernel/osrelease", &n), want) == 0, n, 0);
    snprintf(want, sizeof want, "%s\n", u.nodename);
    CHECK(p, strcmp(slurp("/proc/sys/kernel/hostname", &n), want) == 0, n, 0);
    CHECK(p, atol(slurp("/proc/sys/kernel/pid_max", &n)) >= getpid(), n, 0);
    char boot[64], uuid[64];
    strcpy(boot, slurp("/proc/sys/kernel/random/boot_id", &n));
    CHECK(p, n == 37 && strcmp(boot, slurp("/proc/sys/kernel/random/boot_id", &n)) == 0, n, 0);
    strcpy(uuid, slurp("/proc/sys/kernel/random/uuid", &n));
    CHECK(p, n == 37 && strcmp(uuid, slurp("/proc/sys/kernel/random/uuid", &n)) != 0, n, 0);
    long over = atol(slurp("/proc/sys/vm/overcommit_memory", &n));
    CHECK(p, over >= 0 && over <= 2 && atol(slurp("/proc/sys/fs/pipe-max-size", &n)) > 0, over, 0);
    CHECK(p, strcmp(slurp("/sys/kernel/mm/transparent_hugepage/hpage_pmd_size", &n), "2097152\n") == 0, n, 0);
    struct stat st;
    CHECK(p, stat("/proc", &st) == 0 && S_ISDIR(st.st_mode), errno, 0);
    CHECK(p, stat("/proc/meminfo", &st) == 0 && S_ISREG(st.st_mode) && st.st_size == 0, st.st_size, 0);
    errno = 0;
    CHECK(p, open("/sys/kernel/nothere", O_RDONLY) == -1 && errno == ENOENT, errno, 0);
    done(p, "meminfo, cpuinfo, uptime, loadavg, version, sys/kernel, sys/vm, hugepage size");
}
