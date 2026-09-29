/* qwenwl: the window, from the registry to its first painted frame. */
#include "qwenwl.h"
#include "qwenwl_ids.h"

#include <cstdlib>
#include <cstring>
#include <sys/mman.h>
#include <sys/socket.h>
#include <sys/un.h>
#include <unistd.h>

static const char *WANT[] = {"wl_compositor", "wl_shm", "xdg_wm_base", "wl_seat"};

/* The four globals by interface name, collected until the sync fires. */
static bool globals(Wl &w, uint32_t found[4]) {
    uint32_t object;
    uint16_t op;
    std::vector<uint8_t> body;
    for (int idle = 0; idle < 400;) {
        if (!wl_event(w, object, op, body)) {
            usleep(10000), idle++;
            continue;
        }
        if (object == SYNC) break;
        if (object != REG || op != 0) continue;
        size_t len = wl_word(body, 4);
        std::string iface(body.begin() + 8, body.begin() + 8 + (len ? len - 1 : 0));
        for (int i = 0; i < 4; i++)
            if (iface == WANT[i]) found[i] = wl_word(body, 0);
    }
    return found[0] && found[1] && found[2] && found[3];
}

static bool connect_display(Wl &w) {
    const char *dir = getenv("XDG_RUNTIME_DIR");
    std::string path = std::string(dir ? dir : "/run/user/0") + "/wayland-0";
    sockaddr_un a = {};
    a.sun_family = AF_UNIX;
    strncpy(a.sun_path, path.c_str(), sizeof a.sun_path - 1);
    w.fd = socket(AF_UNIX, SOCK_STREAM, 0);
    return w.fd >= 0 && connect(w.fd, (sockaddr *)&a, sizeof a) == 0;
}

static bool buffer(Wl &w) {
    size_t size = (size_t)w.w * w.h * 4;
    int fd = memfd_create("qwenchat", 0);
    if (fd < 0 || ftruncate(fd, size) != 0) return false;
    void *p = mmap(nullptr, size, PROT_READ | PROT_WRITE, MAP_SHARED, fd, 0);
    if (p == MAP_FAILED) return false;
    w.px = (uint32_t *)p;
    Req pool(SHM, 0);
    if (!wl_send_fd(w, pool.u32(POOL).u32(size), fd)) return false;
    Req buf(POOL, 0);
    return wl_send(w, buf.u32(BUF).u32(0).u32(w.w).u32(w.h).u32(w.w * 4).u32(1));
}

bool wl_open(Wl &w, int width, int height, const char *title) {
    w.w = width, w.h = height;
    if (!connect_display(w)) return false;
    Req reg(1, 1), sync(1, 0);
    wl_send(w, reg.u32(REG)), wl_send(w, sync.u32(SYNC));
    uint32_t g[4] = {};
    if (!globals(w, g)) return false;
    const uint32_t ver[4] = {4, 1, 2, 5}, id[4] = {COMP, SHM, XDG, SEAT};
    for (int i = 0; i < 4; i++) {
        Req b(REG, 0);
        wl_send(w, b.u32(g[i]).str(WANT[i]).u32(ver[i]).u32(id[i]));
    }
    Req s(COMP, 0), xs(XDG, 2), top(XSURF, 1), t(TOP, 2), app(TOP, 3), kb(SEAT, 1), c(SURF, 6);
    wl_send(w, s.u32(SURF)), wl_send(w, xs.u32(XSURF).u32(SURF)), wl_send(w, top.u32(TOP));
    wl_send(w, t.str(title)), wl_send(w, app.str("nonos.qwenchat"));
    wl_send(w, kb.u32(KBD)), wl_send(w, c);
    return buffer(w);
}
