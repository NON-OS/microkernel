#include "cproc.h"

/*
 * The strings no file may hold: the machine's own CPU brand, as CPUID gives
 * it, and each line of /etc/cproc-host, the build host's facts.
 */
static int forbidden(char out[][128]) {
    unsigned r[12];
    int n = 0;
    if (__get_cpuid(0x80000002, &r[0], &r[1], &r[2], &r[3]) &&
        __get_cpuid(0x80000003, &r[4], &r[5], &r[6], &r[7]) &&
        __get_cpuid(0x80000004, &r[8], &r[9], &r[10], &r[11])) {
        char brand[49] = {0};
        memcpy(brand, r, 48);
        char *s = brand;
        while (*s == ' ') s++;
        for (char *e = s + strlen(s); e > s && e[-1] == ' '; *--e = 0) {}
        if (strlen(s) > 4) {
            strcpy(out[n++], s);
        }
    }
    long len;
    char *host = slurp("/etc/cproc-host", &len);
    for (char *line = host; line && *line && n < 8;) {
        char *end = strchr(line, '\n');
        int l = end ? end - line : (int)strlen(line);
        if (l > 4 && l < 127) {
            memcpy(out[n], line, l);
            out[n++][l] = 0;
        }
        line = end ? end + 1 : 0;
    }
    return n;
}

void part_facts(void) {
    const char *p = "facts";
    const char *files[] = {
        "/proc/cpuinfo", "/proc/meminfo", "/proc/stat", "/proc/uptime", "/proc/loadavg",
        "/proc/version", "/proc/filesystems", "/proc/self/status", "/proc/self/stat",
        "/proc/self/maps", "/proc/self/mounts", "/proc/self/mountinfo", "/proc/self/cgroup",
        "/proc/self/limits", "/proc/sys/kernel/hostname", "/proc/sys/kernel/osrelease",
        "/proc/sys/kernel/random/boot_id", "/proc/sys/kernel/ostype",
        "/sys/kernel/mm/transparent_hugepage/hpage_pmd_size",
    };
    char bad_strings[8][128];
    int nb = forbidden(bad_strings), found = 0, read_ok = 0;
    for (unsigned i = 0; i < sizeof files / sizeof files[0]; i++) {
        long n;
        char *text = slurp(files[i], &n);
        read_ok += n >= 0;
        for (int j = 0; text && j < nb; j++) {
            if (strstr(text, bad_strings[j])) {
                printf("[C] cproc facts: %s names \"%s\"\n", files[i], bad_strings[j]);
                found++;
            }
        }
    }
    CHECK(p, nb >= 1 && read_ok == (int)(sizeof files / sizeof files[0]), nb, read_ok);
    if (is_nonos()) {
        CHECK(p, found == 0, found, nb);
        done(p, "no file names the machine's CPU or the build host");
        return;
    }
    CHECK(p, found > 0, found, nb);
    done(p, "Linux's own files name its CPU and host, as they should");
}
