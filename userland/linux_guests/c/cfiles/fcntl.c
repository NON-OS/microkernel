#include "cfiles.h"

static struct flock span(short type, off_t start, off_t len) {
    struct flock f;
    memset(&f, 0, sizeof f);
    f.l_type = type;
    f.l_whence = SEEK_SET;
    f.l_start = start;
    f.l_len = len;
    return f;
}

void part_fcntl(void) {
    const char *p = "fcntl_lock";
    int fd = mk("rl", "0123456789abcdefghij");
    struct flock f = span(F_WRLCK, 0, 10);
    CHECK(p, fcntl(fd, F_SETLK, &f) == 0, errno, 0);
    int ready[2], go[2];
    pipe(ready);
    pipe(go);
    pid_t me = getpid();
    pid_t kid = fork();
    if (kid == 0) {
        int mine = open(DIR "/rl", O_RDWR);
        int res = 0;
        struct flock q = span(F_WRLCK, 5, 10);
        fcntl(mine, F_GETLK, &q);
        res |= !(q.l_type == F_WRLCK && q.l_pid == me && q.l_start == 0 && q.l_len == 10) << 0;
        q = span(F_WRLCK, 5, 10);
        res |= !(fcntl(mine, F_SETLK, &q) == -1 && (errno == EAGAIN || errno == EACCES)) << 1;
        q = span(F_WRLCK, 10, 10);
        res |= !(fcntl(mine, F_SETLK, &q) == 0) << 2;
        write(ready[1], "r", 1);
        q = span(F_RDLCK, 0, 5);
        res |= !(fcntl(mine, F_SETLKW, &q) == 0) << 3;
        char c;
        read(go[0], &c, 1);
        q = span(F_WRLCK, 0, 0);
        res |= !(fcntl(mine, F_SETLK, &q) == 0) << 4;
        _exit(res);
    }
    char c;
    read(ready[0], &c, 1);
    nap_ms(200);
    int st = 0;
    CHECK(p, waitpid(kid, &st, WNOHANG) == 0, st, 0);
    struct flock q = span(F_RDLCK, 12, 1);
    fcntl(fd, F_GETLK, &q);
    CHECK(p, q.l_type == F_WRLCK && q.l_pid == kid, q.l_type, q.l_pid);
    int again = open(DIR "/rl", O_RDONLY);
    close(again);
    write(go[1], "g", 1);
    waitpid(kid, &st, 0);
    CHECK(p, WIFEXITED(st) && WEXITSTATUS(st) == 0, st, 0);
    q = span(F_WRLCK, 0, 0);
    CHECK(p, fcntl(fd, F_SETLK, &q) == 0, errno, 0);
    q = span(F_UNLCK, 0, 0);
    CHECK(p, fcntl(fd, F_SETLK, &q) == 0, errno, 0);
    int a = open(DIR "/rl", O_RDWR), b = open(DIR "/rl", O_RDWR);
    struct flock o = span(F_WRLCK, 30, 5);
    CHECK(p, fcntl(a, F_OFD_SETLK, &o) == 0, errno, 0);
    o = span(F_WRLCK, 32, 1);
    ERR(p, fcntl(b, F_OFD_SETLK, &o), EAGAIN);
    close(a);
    o = span(F_WRLCK, 32, 1);
    CHECK(p, fcntl(b, F_OFD_SETLK, &o) == 0, errno, 0);
    close(b);
    close(fd);
    done(p, "record locks conflict by range, wait, and go at close and at exit");
}
