#include "cproc.h"

void part_self(void) {
    const char *p = "self";
    char to[256] = {0}, path[128];
    long n;
    CHECK(p, readlink("/proc/self", to, sizeof to) > 0 && atoi(to) == getpid(), atoi(to), getpid());
    char *stat = slurp("/proc/self/stat", &n);
    CHECK(p, stat && atoi(stat) == getpid() && strstr(stat, "(cproc) R ") != 0, n, 0);
    int ppid = 0, threads = 0;
    char *after = strrchr(stat, ')');
    sscanf(after + 4, "%d", &ppid);
    { int i = 0; char *q = after + 2; while (i < 17 && q) { q = strchr(q + 1, ' '); i++; } if (q) threads = atoi(q + 1); }
    CHECK(p, ppid == getppid() && threads == 1, ppid, threads);
    char *status = slurp("/proc/self/status", &n);
    CHECK(p, strstr(status, "Name:\tcproc\n") && field_of(status, "\nPid:\t") == getpid(), 0, 0);
    CHECK(p, field_of(status, "\nPPid:\t") == getppid() && field_of(status, "\nThreads:\t") == 1, 0, 0);
    CHECK(p, strstr(status, "\nUid:\t0\t0\t0\t0\n") != 0, 0, 0);
    char *cmd = slurp("/proc/self/cmdline", &n);
    CHECK(p, n > 0 && strcmp(cmd + strlen(cmd) + 1, "one") == 0, n, 0);
    char *env = slurp("/proc/self/environ", &n);
    int home = 0;
    for (long i = 0; i < n; i += strlen(env + i) + 1) {
        home |= strcmp(env + i, "HOME=/root") == 0;
    }
    CHECK(p, home, n, 0);
    CHECK(p, strcmp(slurp("/proc/self/comm", &n), "cproc\n") == 0, n, 0);
    memset(to, 0, sizeof to);
    CHECK(p, readlink("/proc/self/exe", to, sizeof to) > 0 && strcmp(strrchr(to, '/'), "/cproc") == 0, 0, 0);
    char cwd[256];
    getcwd(cwd, sizeof cwd);
    memset(to, 0, sizeof to);
    CHECK(p, readlink("/proc/self/cwd", to, sizeof to) > 0 && strcmp(to, cwd) == 0, 0, 0);
    int fd = open("/tmp/cproc-file", O_RDWR | O_CREAT | O_TRUNC, 0600);
    write(fd, "abcdef", 6);
    lseek(fd, 4, SEEK_SET);
    snprintf(path, sizeof path, "/proc/self/fd/%d", fd);
    memset(to, 0, sizeof to);
    CHECK(p, readlink(path, to, sizeof to) > 0 && strcmp(to, "/tmp/cproc-file") == 0, fd, 0);
    snprintf(path, sizeof path, "/proc/self/fdinfo/%d", fd);
    char *info = slurp(path, &n);
    CHECK(p, info && field_of(info, "pos:\t") == 4 && strstr(info, "flags:\t0") != 0, n, 0);
    int seen = 0;
    DIR *d = opendir("/proc/self/fd");
    for (struct dirent *e; d && (e = readdir(d));) {
        seen += atoi(e->d_name) == fd || strcmp(e->d_name, "0") == 0;
    }
    closedir(d);
    CHECK(p, seen == 2, seen, fd);
    snprintf(path, sizeof path, "/proc/self/fd/%d", fd);
    int again = open(path, O_RDONLY);
    char b[4] = {0};
    CHECK(p, again >= 0 && read(again, b, 3) == 3 && memcmp(b, "abc", 3) == 0, again, errno);
    close(again);
    close(fd);
    done(p, "stat, status, cmdline, environ, comm, exe, cwd, fd and fdinfo");
}
