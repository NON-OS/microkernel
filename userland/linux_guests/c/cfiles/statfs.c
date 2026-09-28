#include "cfiles.h"

/* f_flags bits for a mount's options, as /proc/self/mounts lists them. */
#define ST_KNOWN (0x20 | 1 | 2 | 4 | 8 | 0x1000)

static long mount_flags(const char *point) {
    static const struct { const char *name; long bit; } opt[] = {
        {"ro", 1}, {"nosuid", 2}, {"nodev", 4}, {"noexec", 8}, {"relatime", 0x1000}};
    FILE *m = fopen("/proc/self/mounts", "r");
    char line[512], at[128], opts[256];
    long f = -1;
    while (m && fgets(line, sizeof line, m)) {
        if (sscanf(line, "%*s %127s %*s %255s", at, opts) != 2 || strcmp(at, point) != 0) {
            continue;
        }
        f = 0x20;
        for (char *o = strtok(opts, ","); o; o = strtok(0, ",")) {
            for (unsigned i = 0; i < sizeof opt / sizeof opt[0]; i++) {
                f |= strcmp(o, opt[i].name) == 0 ? opt[i].bit : 0;
            }
        }
    }
    if (m) {
        fclose(m);
    }
    return f;
}

void part_statfs(void) {
    const char *p = "statfs";
    struct statfs t, pr;
    CHECK(p, statfs("/tmp", &t) == 0 && statfs("/proc", &pr) == 0, errno, 0);
    CHECK(p, t.f_type == 0x01021994 && pr.f_type == 0x9fa0, t.f_type, pr.f_type);
    CHECK(p, t.f_namelen == 255 && t.f_frsize == t.f_bsize, t.f_namelen, t.f_frsize);
    CHECK(p, (t.f_flags & ST_KNOWN) == mount_flags("/tmp"), t.f_flags, mount_flags("/tmp"));
    CHECK(p, (pr.f_flags & ST_KNOWN) == mount_flags("/proc"), pr.f_flags, mount_flags("/proc"));
    int fd = mk("sf", "x");
    struct statfs f;
    CHECK(p, fstatfs(fd, &f) == 0 && f.f_type == t.f_type, f.f_type, t.f_type);
    close(fd);
    done(p, "statfs and fstatfs: type, name length, fragment size and mount flags");
}
