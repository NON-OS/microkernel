/* qwenchat: the conversation in a window on the NONOS desktop. */
#include "qwenchat.h"
#include "qwenui.h"

#include <cerrno>
#include <unistd.h>

int chat_window(const ChatArgs &a) {
    Wl w;
    if (!wl_open(w, 1024, 680, "Qwen - local AI on NONOS")) return 1;
    View v;
    v.title = a.model.substr(a.model.rfind('/') + 1) + "  |  on this machine, offline";
    v.status = "loading the model";
    std::vector<Key> keys;
    for (int i = 0; i < 100 && !w.configured; i++) wl_poll(w, keys), usleep(20000);
    ui_draw(w, v), wl_present(w);
    Chat c;
    if (!chat_open(a, c)) {
        v.status = chat_failure(a, c);
        ui_draw(w, v), wl_present(w);
        for (; wl_poll(w, keys); usleep(50000)) {}
        return c.err == ENOMEM ? 4 : 1;
    }
    v.status = c.mem.free < 0 ? "ready, free memory unknown" : "ready";
    ui_draw(w, v), wl_present(w);
    bool open = true;
    for (bool shown = w.configured; open && wl_poll(w, keys);) {
        if (!shown && w.configured) shown = true, ui_draw(w, v), wl_present(w);
        if (keys.empty()) { usleep(15000); continue; }
        for (Key k : keys) open = open && window_key(a, c, w, v, k);
        keys.clear();
        ui_draw(w, v), wl_present(w);
    }
    ui_wipe(w, v), chat_close(c);
    return 0;
}
