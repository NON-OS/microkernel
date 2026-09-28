#include "cfiles.h"

void part_cfr(void) {
    const char *p = "copy_file_range";
    int in = mk("cfr-in", "0123456789");
    int out = mk("cfr-out", "..........");
    loff_t oi = 2, oo = 5;
    CHECK(p, copy_file_range(in, &oi, out, &oo, 4, 0) == 4 && oi == 6 && oo == 9, oi, oo);
    char b[11] = {0};
    pread(out, b, 10, 0);
    CHECK(p, memcmp(b, ".....2345.", 10) == 0, b[5], b[8]);
    CHECK(p, lseek(in, 0, SEEK_CUR) == 10 && lseek(out, 0, SEEK_CUR) == 10, 0, 0);
    ERR(p, copy_file_range(in, &oi, out, &oo, 1, 1), EINVAL);
    int pp[2];
    pipe(pp);
    ERR(p, copy_file_range(in, 0, pp[1], 0, 1, 0), EINVAL);
    close(pp[0]);
    close(pp[1]);
    close(in);
    close(out);
    done(p, "between two files at offsets; flags and a pipe refused");
}
