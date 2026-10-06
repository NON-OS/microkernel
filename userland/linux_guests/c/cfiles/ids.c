#include "cfiles.h"

void part_ids(void) {
    const char *p = "ids";
    uid_t r = 9, e = 9, s = 9;
    int rc = getresuid(&r, &e, &s);
    CHECK(p, rc == 0 && r == 0 && e == 0 && s == 0, r, e);
    gid_t gr = 9, ge = 9, gs = 9;
    rc = getresgid(&gr, &ge, &gs);
    CHECK(p, rc == 0 && gr == 0 && ge == 0 && gs == 0, gr, ge);
    CHECK(p, setresuid(-1, 0, -1) == 0 && setresgid(0, -1, -1) == 0, errno, 0);
    ERR(p, setresuid(1, 1, 1), EPERM);
    ERR(p, setresgid(-1, 5, -1), EPERM);
    gid_t g[4];
    CHECK(p, getgroups(4, g) == 0, 0, 0);
    ERR(p, setgroups(0, g), EPERM);
    errno = 0;
    CHECK(p, getpriority(PRIO_PROCESS, 0) == 0 && errno == 0, errno, 0);
    CHECK(p, setpriority(PRIO_PROCESS, 0, 5) == 0, errno, 0);
    CHECK(p, getpriority(PRIO_PROCESS, 0) == 5, getpriority(PRIO_PROCESS, 0), 5);
    ERR(p, setpriority(PRIO_PROCESS, 0, 2), EACCES);
    CHECK(p, getpriority(PRIO_PROCESS, getpid()) == 5, 0, 0);
    ERR(p, getpriority(9, 0), EINVAL);
    CHECK(p, syscall(SYS_personality, 0xffffffff) == 0, 0, 0);
    done(p, "root's ids with no capability to change them, groups, nice, personality");
}
