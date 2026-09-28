/* What every part of cfiles shares: the harness and each part. */

#define _GNU_SOURCE
#include <errno.h>
#include <fcntl.h>
#include <sched.h>
#include <signal.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/file.h>
#include <sys/resource.h>
#include <sys/sendfile.h>
#include <sys/stat.h>
#include <sys/syscall.h>
#include <sys/sysinfo.h>
#include <sys/times.h>
#include <sys/uio.h>
#include <sys/wait.h>
#include <sys/xattr.h>
#include <time.h>
#include <unistd.h>

#ifndef CFILES_H
#define CFILES_H

#define DIR "/tmp/cfiles"

#define CHECK(p, cond, a, b) if (!check(p, __LINE__, cond, #cond, (long)(a), (long)(b))) return

#define ERR(p, call, e) do { errno = 0; long r_ = (long)(call); \
    if (!check(p, __LINE__, r_ == -1 && errno == (e), #call " -> " #e, r_, errno)) return; } while (0)

extern int parts, failed;
extern char bad[512];

void nap_ms(long ms);
int check(const char *part, int line, int good, const char *what, long a, long b);
void done(const char *part, const char *detail);
int mk(const char *name, const char *text);
void part_pwrite(void);
void part_vec(void);
void part_sendfile(void);
void part_cfr(void);
void part_trunc(void);
void part_falloc(void);
void part_flock(void);
void part_fcntl(void);
void part_close_range(void);
void part_openat2(void);
void part_sync(void);
void part_xattr(void);
void part_usage(void);
void part_ids(void);

#endif
