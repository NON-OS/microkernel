#include "cproc.h"

void part_mem(void) {
    const char *p = "mem";
    errno = 0;
    int fd = open("/proc/self/mem", O_RDONLY);
    if (is_nonos()) {
        CHECK(p, fd == -1 && errno == EACCES, fd, errno);
        done(p, "refused, as NONOS refuses a second way into memory");
        return;
    }
    CHECK(p, fd >= 0, fd, errno);
    close(fd);
    done(p, "opened, as Linux opens it for the process itself");
}
