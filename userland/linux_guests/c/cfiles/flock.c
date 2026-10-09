#include "cfiles.h"

void part_flock(void) {
    const char *p = "flock";
    int fd = mk("fl", "x");
    CHECK(p, flock(fd, LOCK_EX) == 0, 0, 0);
    int dupd = dup(fd);
    CHECK(p, flock(dupd, LOCK_EX | LOCK_NB) == 0, errno, 0);
    int other = open(DIR "/fl", O_RDONLY);
    ERR(p, flock(other, LOCK_SH | LOCK_NB), EWOULDBLOCK);
    int ready[2];
    pipe(ready);
    pid_t kid = fork();
    if (kid == 0) {
        int mine = open(DIR "/fl", O_RDONLY);
        int rc = flock(mine, LOCK_SH | LOCK_NB);
        int nb = rc == -1 && errno == EWOULDBLOCK;
        write(ready[1], "r", 1);
        rc = flock(mine, LOCK_SH);
        _exit(nb && rc == 0 ? 0 : 1);
    }
    char c;
    read(ready[0], &c, 1);
    nap_ms(200);
    int st = 0;
    CHECK(p, waitpid(kid, &st, WNOHANG) == 0, st, 0);
    close(dupd);
    CHECK(p, waitpid(kid, &st, WNOHANG) == 0, st, 0);
    flock(fd, LOCK_UN);
    waitpid(kid, &st, 0);
    CHECK(p, WIFEXITED(st) && WEXITSTATUS(st) == 0, st, 0);
    CHECK(p, flock(fd, LOCK_EX) == 0, 0, 0);
    close(fd);
    CHECK(p, flock(other, LOCK_EX | LOCK_NB) == 0, errno, 0);
    close(other);
    done(p, "shared by a dup, refused to another open, waited for, gone at close");
}
