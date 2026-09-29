#include "cproc.h"

static int proc_pids(void) {
    int n = 0;
    struct stat st;
    char path[32];
    for (int pid = 1; pid <= 4096; pid++) {
        snprintf(path, sizeof path, "/proc/%d", pid);
        n += stat(path, &st) == 0;
    }
    return n;
}

void part_isolation(void) {
    const char *p = "isolation";
    CHECK(p, proc_pids() == 1, proc_pids(), 1);
    int go[2];
    pipe(go);
    pid_t me = getpid();
    pid_t kid = fork();
    if (kid == 0) {
        char c;
        read(go[0], &c, 1);
        _exit(getppid() == me ? 0 : 1);
    }
    int with_kid = proc_pids();
    char path[64];
    snprintf(path, sizeof path, "/proc/%d/stat", kid);
    long n;
    char *kst = slurp(path, &n);
    int ppid = 0;
    if (kst) {
        sscanf(strrchr(kst, ')') + 4, "%d", &ppid);
    }
    write(go[1], "g", 1);
    int status = -1;
    waitpid(kid, &status, 0);
    CHECK(p, with_kid == 2 && ppid == getpid(), with_kid, ppid);
    CHECK(p, WIFEXITED(status) && WEXITSTATUS(status) == 0, status, 0);
    errno = 0;
    CHECK(p, open("/proc/99999/stat", O_RDONLY) == -1 && errno == ENOENT, errno, 0);
    struct stat root, st;
    stat("/", &root);
    CHECK(p, stat("/proc/self/root/..", &st) == 0 && st.st_ino == root.st_ino, st.st_ino, root.st_ino);
    symlink("/../../../..", "/tmp/cproc-esc");
    CHECK(p, stat("/tmp/cproc-esc", &st) == 0 && st.st_ino == root.st_ino, st.st_ino, root.st_ino);
    CHECK(p, chdir("/../../../..") == 0, errno, 0);
    char cwd[64];
    CHECK(p, getcwd(cwd, sizeof cwd) && strcmp(cwd, "/") == 0, 0, 0);
    /* An absolute name is taken from the root, wherever the process is. */
    CHECK(p, chdir("/proc") == 0 && chdir("/tmp") == 0 && getcwd(cwd, sizeof cwd), errno, 0);
    CHECK(p, strcmp(cwd, "/tmp") == 0, cwd[1], 0);
    int d = open("/tmp", O_RDONLY | O_DIRECTORY);
    chdir("/proc");
    int fd = openat(d, "cproc-at", O_WRONLY | O_CREAT, 0600);
    CHECK(p, fd >= 0 && stat("/tmp/cproc-at", &st) == 0, fd, errno);
    close(fd);
    close(d);
    chdir("/");
    done(p, "only the family's pids, a child's parent as getppid says, no way above the root");
}
