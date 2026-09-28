#include "cfiles.h"

void part_sync(void) {
    const char *p = "sync";
    int fd = mk("sy", "data");
    sync();
    CHECK(p, syncfs(fd) == 0, errno, 0);
    CHECK(p, fdatasync(fd) == 0 && fsync(fd) == 0, errno, 0);
    int pp[2];
    pipe(pp);
    ERR(p, fdatasync(pp[0]), EINVAL);
    ERR(p, syncfs(999), EBADF);
    close(pp[0]);
    close(pp[1]);
    close(fd);
    done(p, "sync, syncfs, fsync and fdatasync; a pipe cannot be synced");
}
