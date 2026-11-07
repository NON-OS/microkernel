/*
 * qwenshow_check: a host check that qwenchat's window presents only the
 * rows that changed (qwenui_show.cpp, wl_present_rows in qwenwl_loop.cpp).
 * Over a socketpair standing in for the display: the first frame is
 * presented whole; a key that changes only the line names the input box's
 * rows; a status tick names the header band's; a new line said, a status
 * changed by a key, another present between, or a buffer at a new size is
 * presented whole. Each frame is drawn whole first, and the pixels that
 * changed between two frames lie inside the rows named. Built and run on
 * the host by `make nonos-mk-qwenshow-check NONOS_LINUX_GUESTS=1
 * NONOS_QWEN=1` (QwenWindow.mk); exits 0 when every check holds, else names
 * the first that does not.
 */
#include "qwenui.h"
#include "qwenwl_ids.h"

#include <cstdio>
#include <cstring>
#include <fcntl.h>
#include <sys/socket.h>
#include <unistd.h>

static int failed(const char *what) {
    printf("qwenshow_check: %s\n", what);
    return 1;
}

/* The rect of every wl_surface.damage the client sent since the last call. */
static std::vector<std::vector<uint32_t>> damages(int fd) {
    std::vector<uint8_t> bytes;
    uint8_t buf[4096];
    for (ssize_t n; (n = read(fd, buf, sizeof buf)) > 0;) bytes.insert(bytes.end(), buf, buf + n);
    std::vector<std::vector<uint32_t>> out;
    for (size_t at = 0; at + 8 <= bytes.size();) {
        uint32_t object, word;
        memcpy(&object, &bytes[at], 4), memcpy(&word, &bytes[at + 4], 4);
        const size_t size = word >> 16;
        if (size < 8 || at + size > bytes.size()) break;
        if (object == SURF && (word & 0xFFFF) == 2 && size == 24) {
            std::vector<uint32_t> r(4);
            memcpy(r.data(), &bytes[at + 8], 16);
            out.push_back(r);
        }
        at += size;
    }
    return out;
}

static bool one(const std::vector<std::vector<uint32_t>> &d, uint32_t y, uint32_t w, uint32_t h) {
    return d.size() == 1 && d[0][0] == 0 && d[0][1] == y && d[0][2] == w && d[0][3] == h;
}

static bool opened(Wl &w, int &display) {
    int pair[2];
    if (socketpair(AF_UNIX, SOCK_STREAM, 0, pair) != 0) return false;
    fcntl(pair[0], F_SETFL, O_NONBLOCK), fcntl(pair[1], F_SETFL, O_NONBLOCK);
    w.fd = pair[0], display = pair[1];
    w.w = w.own_w = 880, w.h = w.own_h = 560;
    w.pool = POOL, w.buf = BUF, w.configured = true;
    return wl_buffer(w, w.w, w.h, w.pool, w.buf, w.px, w.memfd);
}

/* Whether every row where frames `a` and `b` differ lies in [y, y + h). */
static bool changed_within(const std::vector<uint32_t> &a, const std::vector<uint32_t> &b, int width, int y, int h) {
    bool any = false;
    for (size_t i = 0; i < a.size(); i++) {
        if (a[i] == b[i]) continue;
        const int row = (int)(i / width);
        if (row < y || row >= y + h) return false;
        any = true;
    }
    return any;
}

static std::vector<uint32_t> pixels(const Wl &w) { return std::vector<uint32_t>(w.px, w.px + (size_t)w.w * w.h); }

int main() {
    Wl w;
    int display = -1;
    if (!opened(w, display)) return failed("the window did not open over a socketpair");
    View v;
    v.title = "Qwen3 0.6B  |  offline", v.status = "ready";
    ui_show(w, v);
    if (!one(damages(display), 0, 880, 560)) return failed("the first frame is not presented whole");

    std::vector<uint32_t> before = pixels(w);
    v.input = "hello";
    ui_show(w, v);
    const int by = ui_box_y(w), bh = ui_box_h();
    if (!one(damages(display), by, 880, bh)) return failed("a key that changes the line names more than the box");
    if (!changed_within(before, pixels(w), w.w, by, bh)) return failed("the line drew outside the box's rows");

    before = pixels(w);
    v.status = "loading the model 40%, Esc stops";
    ui_show_status(w, v);
    if (!one(damages(display), 0, 880, ui_head_h())) return failed("a status tick names more than the header");
    if (!changed_within(before, pixels(w), w.w, 0, ui_head_h())) return failed("the status drew outside the header");

    v.status = "no thinking", v.input.clear();
    ui_show(w, v);
    if (!one(damages(display), 0, 880, 560)) return failed("a key that changed the status is not presented whole");

    v.said.push_back({true, "hello"});
    ui_show_status(w, v);
    if (!one(damages(display), 0, 880, 560)) return failed("a new line said is not presented whole");

    ui_draw(w, v), wl_present(w);
    damages(display);
    v.input = "x";
    ui_show(w, v);
    if (!one(damages(display), 0, 880, 560)) return failed("a present between leaves the next one partial");

    if (!wl_resize(w, 1920, 1022)) return failed("no buffer at 1920x1022");
    damages(display);
    wl_present_rows(w, 0, ui_head_h());
    if (!one(damages(display), 0, 1920, 1022)) return failed("the first present at a new size is not whole");
    wl_present_rows(w, 1000, 100);
    if (!one(damages(display), 1000, 1920, 22)) return failed("rows past the buffer are not clipped");

    puts("qwenshow_check: ok");
    return 0;
}
