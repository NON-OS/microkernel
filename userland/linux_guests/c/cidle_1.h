/* cidle, part 1 of 1: included once, by cidle.c. */

static struct sockaddr_in addr;

static int idle_s = 10;

static long now_ms(void) {
    struct timespec ts;
    clock_gettime(CLOCK_MONOTONIC, &ts);
    return ts.tv_sec * 1000 + ts.tv_nsec / 1000000;
}

static void *late(void *arg) {
    (void)arg;
    struct timespec ts = {idle_s, 0};
    nanosleep(&ts, 0);
    int c = socket(AF_INET, SOCK_STREAM, 0);
    connect(c, (void *)&addr, sizeof addr);
    return (void *)(long)c;
}
