/*
 * qwenwl: the window at a new size. The display server names one in a
 * configure when the window is made full screen (F11) and when it comes
 * back. The pixels go to a new pool and buffer of that size, under the
 * other pair of ids, and the old pair is destroyed: the next present
 * attaches the new buffer, drawn whole at the new size by the caller.
 */
#include "qwenwl.h"
#include "qwenwl_ids.h"

#include <sys/mman.h>
#include <unistd.h>

bool wl_resize(Wl &w, int width, int height) {
    if (width == w.w && height == w.h) return true;
    const uint32_t pool = w.pool == POOL ? POOL2 : POOL, buf = w.buf == BUF ? BUF2 : BUF;
    uint32_t *px = nullptr;
    int memfd = -1;
    if (!wl_buffer(w, width, height, pool, buf, px, memfd)) {
        if (px) munmap(px, (size_t)width * height * 4);
        if (memfd >= 0) close(memfd);
        return false;
    }
    Req old_buf(w.buf, 0), old_pool(w.pool, 1);
    wl_send(w, old_buf), wl_send(w, old_pool);
    munmap(w.px, (size_t)w.w * w.h * 4);
    if (w.memfd >= 0) close(w.memfd);
    w.px = px, w.memfd = memfd, w.w = width, w.h = height, w.pool = pool, w.buf = buf;
    w.resized = true;
    return true;
}
