#include "cfiles.h"

void part_pwrite(void) {
    const char *p = "pwrite";
    int fd = mk("pw", "0123456789");
    CHECK(p, pwrite(fd, "AB", 2, 4) == 2, 0, 0);
    CHECK(p, lseek(fd, 0, SEEK_CUR) == 10, lseek(fd, 0, SEEK_CUR), 10);
    char b[16] = {0};
    CHECK(p, pread(fd, b, 10, 0) == 10 && memcmp(b, "0123AB6789", 10) == 0, b[4], b[5]);
    CHECK(p, pwrite(fd, "Z", 1, 12) == 1, 0, 0);
    struct stat st;
    fstat(fd, &st);
    CHECK(p, st.st_size == 13, st.st_size, 13);
    CHECK(p, pread(fd, b, 3, 10) == 3 && b[0] == 0 && b[1] == 0 && b[2] == 'Z', b[0], b[2]);
    ERR(p, pwrite(fd, "x", 1, -1), EINVAL);
    int pp[2];
    pipe(pp);
    ERR(p, pwrite(pp[1], "x", 1, 0), ESPIPE);
    close(pp[0]);
    close(pp[1]);
    int ro = open(DIR "/pw", O_RDONLY);
    ERR(p, pwrite(ro, "x", 1, 0), EBADF);
    close(ro);
    close(fd);
    /* A file takes a whole write in one call, however large. */
    static char big[3 << 20];
    int w = mk("pwbig", 0);
    CHECK(p, write(w, big, sizeof big) == sizeof big, errno, 0);
    CHECK(p, pwrite(w, big, 2 << 20, 1 << 20) == 2 << 20, errno, 0);
    CHECK(p, lseek(w, 0, SEEK_CUR) == sizeof big, lseek(w, 0, SEEK_CUR), 0);
    close(w);
    unlink(DIR "/pwbig");
    done(p, "writes at the offset, leaves the file offset, fills a gap, takes 3 MiB whole");
}
