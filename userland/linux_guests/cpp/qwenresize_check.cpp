/*
 * qwenresize_check: a host check that qwenchat's window takes the size the
 * display server names (qwenwl_loop.cpp, qwenwl_resize.cpp). It ignored the
 * toplevel's configure and kept its 880 by 560 buffer, so a window made full
 * screen showed the old frame in a corner of the screen. Over a socketpair
 * standing in for the display: a configure of 1920 by 1022 is answered by a
 * new pool and buffer of that size under the other ids, the old pair
 * destroyed, and only then the ack; the next present attaches the new
 * buffer; and a configure of 0 by 0 gives the window its own size back.
 * Built and run on the host by `make nonos-mk-qwenresize-check
 * NONOS_LINUX_GUESTS=1 NONOS_QWEN=1` (QwenChat.mk); exits 0 when every check holds, else names the first that
 * does not.
 */
#include "qwenwl.h"
#include "qwenwl_ids.h"

#include <cstdio>
#include <cstring>
#include <fcntl.h>
#include <sys/mman.h>
#include <sys/socket.h>
#include <unistd.h>

struct Msg {
    uint32_t object;
    uint16_t op;
    std::vector<uint32_t> words;
};

static int failed(const char *what) {
    printf("qwenresize_check: %s\n", what);
    return 1;
}

/* One event from the display's side: object, opcode, words. */
static void tell(int fd, uint32_t object, uint16_t op, std::vector<uint32_t> words) {
    std::vector<uint32_t> m = {object, (uint32_t)((8 + words.size() * 4) << 16) | op};
    m.insert(m.end(), words.begin(), words.end());
    if (write(fd, m.data(), m.size() * 4) != (ssize_t)(m.size() * 4)) perror("write");
}

/* Every request the client has sent so far. */
static std::vector<Msg> heard(int fd) {
    std::vector<uint8_t> bytes;
    uint8_t buf[4096];
    for (ssize_t n; (n = read(fd, buf, sizeof buf)) > 0;) bytes.insert(bytes.end(), buf, buf + n);
    std::vector<Msg> out;
    for (size_t at = 0; at + 8 <= bytes.size();) {
        uint32_t object, word;
        memcpy(&object, &bytes[at], 4), memcpy(&word, &bytes[at + 4], 4);
        size_t size = word >> 16;
        if (size < 8 || at + size > bytes.size()) break;
        Msg m = {object, (uint16_t)word, std::vector<uint32_t>((size - 8) / 4)};
        if (size > 8) memcpy(m.words.data(), &bytes[at + 8], size - 8);
        out.push_back(m), at += size;
    }
    return out;
}

static bool is(const Msg &m, uint32_t object, uint16_t op) { return m.object == object && m.op == op; }

/* The window as wl_open leaves it, configured, over one end of a socketpair. */
static bool opened(Wl &w, int &display) {
    int pair[2];
    if (socketpair(AF_UNIX, SOCK_STREAM, 0, pair) != 0) return false;
    fcntl(pair[0], F_SETFL, O_NONBLOCK), fcntl(pair[1], F_SETFL, O_NONBLOCK);
    w.fd = pair[0], display = pair[1];
    w.w = w.own_w = 880, w.h = w.own_h = 560;
    w.pool = POOL, w.buf = BUF, w.configured = true;
    return wl_buffer(w, w.w, w.h, w.pool, w.buf, w.px, w.memfd);
}

/* A configure of `width` by `height` under serial `serial`, polled. */
static std::vector<Msg> configure(Wl &w, int display, uint32_t width, uint32_t height, uint32_t serial) {
    heard(display);
    tell(display, TOP, 0, {width, height, 4, 2});
    tell(display, XSURF, 0, {serial});
    std::vector<Key> keys;
    wl_poll(w, keys);
    return heard(display);
}

int main() {
    Wl w;
    int display = -1;
    if (!opened(w, display)) return failed("the window did not open over a socketpair");
    std::vector<Msg> m = configure(w, display, 1920, 1022, 7);
    if (m.size() != 5) return failed("a configure to 1920x1022 is not five requests");
    if (!is(m[0], SHM, 0) || m[0].words[0] != POOL2 || m[0].words[1] != 1920u * 1022 * 4)
        return failed("no pool of 1920x1022 under the other id");
    if (!is(m[1], POOL2, 0) || m[1].words[0] != BUF2 || m[1].words[2] != 1920 || m[1].words[3] != 1022 ||
        m[1].words[4] != 1920 * 4)
        return failed("no buffer of 1920x1022 in the new pool");
    if (!is(m[2], BUF, 0) || !is(m[3], POOL, 1)) return failed("the old buffer and pool are kept");
    if (!is(m[4], XSURF, 4) || m[4].words[0] != 7) return failed("the configure is not acked after the buffer");
    if (w.w != 1920 || w.h != 1022 || !w.resized || w.buf != BUF2 || w.pool != POOL2)
        return failed("the window does not have its new size");
    w.px[1920 * 1022 - 1] = 0xFF00FF00; /* the last pixel of the new size is the window's */
    heard(display);
    wl_present(w);
    m = heard(display);
    if (m.empty() || !is(m[0], SURF, 1) || m[0].words[0] != BUF2) return failed("the present attaches the old buffer");
    m = configure(w, display, 0, 0, 8);
    if (m.size() != 5 || !is(m[0], SHM, 0) || m[0].words[0] != POOL || !is(m[1], POOL, 0) || m[1].words[2] != 880 ||
        m[1].words[3] != 560 || !is(m[4], XSURF, 4) || m[4].words[0] != 8)
        return failed("a configure of 0 by 0 does not give back 880x560");
    if (w.w != 880 || w.h != 560 || w.buf != BUF) return failed("the window keeps the full-screen size");
    m = configure(w, display, 880, 560, 9);
    if (m.size() != 1 || !is(m[0], XSURF, 4)) return failed("the same size makes a new buffer");
    puts("qwenresize_check: ok");
    return 0;
}
