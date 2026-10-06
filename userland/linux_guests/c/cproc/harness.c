#include "cproc.h"

int parts, failed;

char bad[512];

char **args;

int check(const char *part, int line, int good, const char *what, long a, long b) {
    if (good) {
        return 1;
    }
    printf("[C] cproc %s FAIL at line %d: %s (%ld, %ld)\n", part, line, what, a, b);
    fflush(stdout);
    failed++;
    strncat(bad, " ", sizeof bad - strlen(bad) - 1);
    strncat(bad, part, sizeof bad - strlen(bad) - 1);
    return 0;
}

void done(const char *part, const char *detail) {
    parts++;
    printf("[C] cproc %s ok: %s\n", part, detail);
    fflush(stdout);
}
