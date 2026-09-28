/*
 * /dev, /proc and /sys as a Linux program reads them: the devices and their
 * numbers, the program's own /proc directory against what the calls say,
 * its threads under task/, the system files against sysinfo and uname, the
 * one /sys file Go reads, and isolation: only the family's own pids exist
 * under /proc, no path climbs out of the root, a directory descriptor keeps
 * its place across a chdir, /proc/self/mem is refused, and no file under
 * /proc or /sys names the machine's CPU, or the build host's CPU, boot id
 * or name (/etc/cproc-host). Run with the arguments "one two". Every part
 * runs and prints; the two whose answers differ from Linux by design (mem,
 * facts) say which answer they expected.
 */

#include "cproc.h"

int main(int argc, char **argv) {
    args = argv;
    if (argc != 3 || strcmp(argv[1], "one") || strcmp(argv[2], "two")) {
        printf("[C] cproc FAIL: run as cproc one two\n");
        return 1;
    }
    part_dev();
    part_self();
    part_maps();
    part_system();
    part_isolation();
    part_mem();
    part_facts();
    if (failed) {
        printf("[C] cproc FAIL: %d of %d parts:%s\n", failed, parts + failed, bad);
        return 1;
    }
    printf("[C] cproc PASS: %d parts\n", parts);
    return 0;
}
