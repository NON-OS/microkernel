#include "cwait.h"

static int pipe_drain;
static void *late_read(void *arg) {
    (void)arg;
    char buf[4096];
    nap_ms(100);
    read(pipe_drain, buf, sizeof buf);
    return 0;
}


int pipe_nonblock(void) {
    int p[2];
    pipe2(p, O_NONBLOCK);
    char c;
    if (read(p[0], &c, 1) != -1 || errno != EAGAIN) {
        return fail("empty non-blocking pipe read", errno, EAGAIN);
    }
    if (!(fcntl(p[0], F_GETFL) & O_NONBLOCK)) {
        return fail("F_GETFL after pipe2(O_NONBLOCK)", fcntl(p[0], F_GETFL), O_NONBLOCK);
    }
    fcntl(p[0], F_SETFL, 0);
    if (fcntl(p[0], F_GETFL) & O_NONBLOCK) {
        return fail("F_SETFL 0 kept O_NONBLOCK", fcntl(p[0], F_GETFL), 0);
    }
    close(p[1]);
    struct pollfd pf = {.fd = p[0], .events = POLLIN};
    int ready = poll(&pf, 1, 0);
    if (read(p[0], &c, 1) != 0 || ready != 1 || !(pf.revents & POLLHUP)) {
        return fail("end of file after the write end closed", ready, pf.revents);
    }
    close(p[0]);
    ok("pipe", "EAGAIN empty, O_NONBLOCK set and cleared, end of file and POLLHUP; revents",
       pf.revents);
    return 0;
}

int pipe_full(void) {
    int p[2];
    pipe(p);
    fcntl(p[1], F_SETFL, O_NONBLOCK);
    static char block[4096];
    long held = 0;
    while (write(p[1], block, sizeof block) == sizeof block) {
        held += sizeof block;
    }
    fcntl(p[1], F_SETFL, 0);
    pipe_drain = p[0];
    pthread_t t;
    long t0 = now_ms();
    pthread_create(&t, 0, late_read, 0);
    ssize_t n = write(p[1], block, sizeof block);
    long took = now_ms() - t0;
    pthread_join(t, 0);
    close(p[0]);
    close(p[1]);
    if (held != 65536 || n != sizeof block || took < 90) {
        return fail("write to a full pipe", held, took);
    }
    ok("pipe-full", "a 4096 byte write to a full 65536 byte pipe waited for a reader, ms", took);
    return 0;
}
