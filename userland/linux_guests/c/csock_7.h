/* csock, part 7 of 11: included once, by csock.c. */

static int blocking_accept(void) {
    int l = listener(&late_port, 0);
    pthread_t t;
    long t0 = now_ms();
    pthread_create(&t, 0, late_dial, 0);
    int s = accept(l, 0, 0);
    long waited = now_ms() - t0;
    void *c;
    pthread_join(t, &c);
    close((int)(long)c);
    close(s);
    close(l);
    if (s < 0 || waited < 80) {
        return fail("blocking_accept: waited for the connect", s, waited);
    }
    ok("blocking_accept", "waited ms", waited >= 80);
    return 0;
}

static int late_fd;

static void *late_write(void *arg) {
    (void)arg;
    nap_ms(100);
    if (write(late_fd, "late", 4) != 4) {
        printf("[C] csock late_write: errno %d\n", errno);
    }
    return 0;
}

static int blocking_recv(void) {
    int a, b;
    char buf[8];
    if (tcp_pair(&a, &b)) {
        return fail("blocking_recv: pair", 0, errno);
    }
    late_fd = a;
    pthread_t t;
    long t0 = now_ms();
    pthread_create(&t, 0, late_write, 0);
    long got = recv(b, buf, 8, 0);
    long waited = now_ms() - t0;
    pthread_join(t, 0);
    close(a);
    close(b);
    if (got != 4 || waited < 80) {
        return fail("blocking_recv: waited for the bytes", got, waited);
    }
    ok("blocking_recv", "4 bytes after waiting", waited >= 80);
    return 0;
}

/* csock: the options a server sets, and a socket a forked child shares. */

static int get_int(int s, int level, int opt) {
    int v = -1;
    socklen_t len = sizeof v;
    return getsockopt(s, level, opt, &v, &len) ? -errno : v;
}
