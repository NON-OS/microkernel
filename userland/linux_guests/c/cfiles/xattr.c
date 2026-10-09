#include "cfiles.h"

void part_xattr(void) {
    const char *p = "xattr";
    close(mk("xa", "x"));
    const char *f = DIR "/xa";
    char b[32];
    ERR(p, getxattr(f, "user.k", b, sizeof b), ENODATA);
    CHECK(p, listxattr(f, b, sizeof b) == 0, 0, 0);
    CHECK(p, setxattr(f, "user.k", "val", 3, 0) == 0, errno, 0);
    CHECK(p, getxattr(f, "user.k", 0, 0) == 3, 0, 0);
    CHECK(p, getxattr(f, "user.k", b, sizeof b) == 3 && memcmp(b, "val", 3) == 0, b[0], 0);
    ERR(p, getxattr(f, "user.k", b, 1), ERANGE);
    ERR(p, setxattr(f, "user.k", "v", 1, XATTR_CREATE), EEXIST);
    ERR(p, setxattr(f, "user.none", "v", 1, XATTR_REPLACE), ENODATA);
    CHECK(p, listxattr(f, b, sizeof b) == 7 && strcmp(b, "user.k") == 0, 0, 0);
    ERR(p, setxattr(f, "nonamespace", "v", 1, 0), EOPNOTSUPP);
    CHECK(p, removexattr(f, "user.k") == 0, errno, 0);
    ERR(p, removexattr(f, "user.k"), ENODATA);
    ERR(p, getxattr(DIR "/nothere", "user.k", b, sizeof b), ENOENT);
    done(p, "set, get, list and remove, with Linux's flags and errors");
}
