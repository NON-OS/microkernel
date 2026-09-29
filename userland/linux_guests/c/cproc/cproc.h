/* What every part of cproc shares: the harness and each part. */

#define _GNU_SOURCE
#include <cpuid.h>
#include <dirent.h>
#include <errno.h>
#include <fcntl.h>
#include <pthread.h>
#include <sched.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/mman.h>
#include <sys/resource.h>
#include <sys/stat.h>
#include <sys/syscall.h>
#include <sys/sysinfo.h>
#include <sys/sysmacros.h>
#include <sys/utsname.h>
#include <sys/wait.h>
#include <time.h>
#include <unistd.h>

#ifndef CPROC_H
#define CPROC_H

#define CHECK(p, cond, a, b) if (!check(p, __LINE__, cond, #cond, (long)(a), (long)(b))) return

extern int parts, failed;
extern char bad[512];
extern char **args;

int check(const char *part, int line, int good, const char *what, long a, long b);
void done(const char *part, const char *detail);
char *slurp(const char *path, long *n);
long field_of(const char *text, const char *key);
void spin_ms(long ms);
int is_nonos(void);
void part_dev(void);
void part_self(void);
void part_maps(void);
void part_system(void);
void part_stat(void);
void part_load(void);
void part_memory(void);
void part_isolation(void);
void part_mem(void);
void part_facts(void);

#endif
