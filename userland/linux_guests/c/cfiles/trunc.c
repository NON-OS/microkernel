#include "cfiles.h"

void part_trunc(void) {
    const char *p = "truncate";
    int fd = mk("tr", "abcdef");
    CHECK(p, ftruncate(fd, 3) == 0, 0, 0);
    struct stat st;
    fstat(fd, &st);
    CHECK(p, st.st_size == 3, st.st_size, 3);
    CHECK(p, ftruncate(fd, 8) == 0, 0, 0);
    char b[8];
    CHECK(p, pread(fd, b, 8, 0) == 8 && b[2] == 'c' && b[3] == 0 && b[7] == 0, b[2], b[7]);
    close(fd);
    CHECK(p, truncate(DIR "/tr", 2) == 0, 0, 0);
    stat(DIR "/tr", &st);
    CHECK(p, st.st_size == 2, st.st_size, 2);
    ERR(p, truncate(DIR "/nothere", 1), ENOENT);
    ERR(p, truncate(DIR, 1), EISDIR);
    ERR(p, truncate(DIR "/tr", -1), EINVAL);
    int ro = open(DIR "/tr", O_RDONLY);
    ERR(p, ftruncate(ro, 1), EINVAL);
    close(ro);
    done(p, "shrink and grow, by descriptor and by name");
}
