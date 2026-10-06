/* cunix, part 1 of 4: included once, by cunix.c. */

static int parts;

static int fail(const char *what, long a, long b) {
    printf("[C] cunix FAIL: %s (%ld, %ld)\n", what, a, b);
    fflush(stdout);
    return 1;
}

static void ok(const char *part, const char *detail, long n) {
    parts++;
    printf("[C] cunix %s ok: %s %ld\n", part, detail, n);
    fflush(stdout);
}

/* A sockaddr_un for a path, or for an abstract name when `abs` is set. */
static socklen_t name(struct sockaddr_un *a, const char *p, int abs) {
    memset(a, 0, sizeof *a);
    a->sun_family = AF_UNIX;
    if (abs) {
        memcpy(a->sun_path + 1, p, strlen(p));
        return offsetof(struct sockaddr_un, sun_path) + 1 + strlen(p);
    }
    strcpy(a->sun_path, p);
    return offsetof(struct sockaddr_un, sun_path) + strlen(p) + 1;
}
