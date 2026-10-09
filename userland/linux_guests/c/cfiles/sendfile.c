#include "cfiles.h"

void part_sendfile(void) {
    const char *p = "sendfile";
    int in = mk("sf-in", "hello sendfile world");
    int out = mk("sf-out", 0);
    off_t off = 6;
    CHECK(p, sendfile(out, in, &off, 8) == 8 && off == 14, off, 14);
    CHECK(p, lseek(in, 0, SEEK_CUR) == 20, lseek(in, 0, SEEK_CUR), 20);
    lseek(in, 0, SEEK_SET);
    CHECK(p, sendfile(out, in, 0, 5) == 5 && lseek(in, 0, SEEK_CUR) == 5, lseek(in, 0, SEEK_CUR), 5);
    char b[16] = {0};
    pread(out, b, 13, 0);
    CHECK(p, memcmp(b, "sendfilehello", 13) == 0, b[0], b[8]);
    off = 20;
    CHECK(p, sendfile(out, in, &off, 5) == 0, 0, 0);
    int ro = open(DIR "/sf-in", O_RDONLY);
    ERR(p, sendfile(ro, in, 0, 1), EBADF);
    close(ro);
    close(in);
    close(out);
    done(p, "file to file, at an offset and at the file offset, and end of file");
}
