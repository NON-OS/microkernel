/*
 * The leader ends with a plain exit, not exit_group. On Linux that ends only
 * the leader: the process lives on in its other threads, and its status is
 * the exit code of the last thread to end. The worker prints its line a second
 * after the leader has gone and ends with 42, so a log with the PASS line and
 * a status of 42 is Linux's behaviour; a process that ended at the leader's
 * exit never prints the line and reports 0.
 */
#include <pthread.h>
#include <stdio.h>
#include <sys/syscall.h>
#include <time.h>
#include <unistd.h>

static void *work(void *arg) {
    (void)arg;
    struct timespec t = {1, 0};
    nanosleep(&t, 0);
    printf("[C] leaderexit PASS: worker ran 1 s after the leader's exit\n");
    fflush(stdout);
    syscall(SYS_exit, 42);
    return 0;
}

int main(void) {
    pthread_t t;
    if (pthread_create(&t, 0, work, 0) != 0) {
        printf("[C] leaderexit FAIL: create\n");
        fflush(stdout);
        return 1;
    }
    printf("[C] leaderexit: leader exits, worker runs on\n");
    fflush(stdout);
    syscall(SYS_exit, 0);
    return 1;
}
