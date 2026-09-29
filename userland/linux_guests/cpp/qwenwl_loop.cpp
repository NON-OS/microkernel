/* qwenwl: staying up, taking keys, and showing each frame. */
#include "qwenwl.h"
#include "qwenwl_ids.h"

/* xdg_wm_base.ping, xdg_surface.configure, xdg_toplevel.close, key. */
bool wl_poll(Wl &w, std::vector<Key> &keys) {
    uint32_t object;
    uint16_t op;
    std::vector<uint8_t> body;
    while (!w.closed && wl_event(w, object, op, body)) {
        if (object == XDG && op == 0) {
            Req pong(XDG, 3);
            wl_send(w, pong.u32(wl_word(body, 0)));
        } else if (object == XSURF && op == 0) {
            Req ack(XSURF, 4);
            wl_send(w, ack.u32(wl_word(body, 0)));
            w.configured = true;
        } else if (object == TOP && op == 1) {
            w.closed = true;
        } else if (object == KBD && op == 3 && wl_word(body, 12) == 1) {
            keys.push_back(Key{wl_word(body, 8)});
        }
    }
    return !w.closed;
}

void wl_present(Wl &w) {
    if (!w.configured) return;
    Req attach(SURF, 1), damage(SURF, 2), commit(SURF, 6);
    wl_send(w, attach.u32(BUF).u32(0).u32(0));
    wl_send(w, damage.u32(0).u32(0).u32(w.w).u32(w.h));
    wl_send(w, commit);
}
