/*
 * The file and system-information calls, each against what Linux answers:
 * pwrite64, preadv and pwritev and their v2 forms, sendfile,
 * copy_file_range, truncate and ftruncate, fallocate as tmpfs serves it,
 * fadvise64, flock and fcntl record locks between processes (with a lock
 * that waits and locks that go at close and at exit), close_range, openat2
 * with RESOLVE_ flags, sync, syncfs and fdatasync, the xattr calls, sysinfo,
 * getrusage and times against a measured busy loop, the id and group calls,
 * the priority calls and personality. Every part runs and prints; a failing
 * part is named with the numbers it saw. Nothing printed depends on the
 * machine, so the host's run prints the same lines.
 */

#include "cfiles.h"

int main(void) {
    mkdir(DIR, 0755);
    part_pwrite();
    part_vec();
    part_sendfile();
    part_cfr();
    part_trunc();
    part_falloc();
    part_flock();
    part_fcntl();
    part_close_range();
    part_openat2();
    part_sync();
    part_statfs();
    part_xattr();
    part_usage();
    part_ids();
    if (failed) {
        printf("[C] cfiles FAIL: %d of %d parts:%s\n", failed, parts + failed, bad);
        return 1;
    }
    printf("[C] cfiles PASS: %d parts\n", parts);
    return 0;
}
