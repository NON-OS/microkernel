#include "cfiles.h"

void nap_ms(long ms) {
    struct timespec ts = {ms / 1000, (ms % 1000) * 1000000};
    nanosleep(&ts, 0);
}

int mk(const char *name, const char *text) {
    char path[128];
    snprintf(path, sizeof path, DIR "/%s", name);
    int fd = open(path, O_RDWR | O_CREAT | O_TRUNC, 0644);
    if (fd >= 0 && text) {
        write(fd, text, strlen(text));
    }
    return fd;
}
