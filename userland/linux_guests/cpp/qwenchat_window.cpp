/* qwenchat: the conversation in a window on the NONOS desktop. */
#include "qwenchat.h"
#include "qwenui.h"

#include <chrono>
#include <cstdio>
#include <unistd.h>

struct Live { Wl *w; View *v; };

/* Each piece lands in the reply and is shown at once; pings are answered. */
static void to_window(void *to, const char *piece, size_t n) {
    Live *l = (Live *)to;
    l->v->said.back().text.append(piece, n);
    std::vector<Key> ignored;
    wl_poll(*l->w, ignored);
    ui_draw(*l->w, *l->v), wl_present(*l->w);
}

static double now_s() {
    using namespace std::chrono;
    return duration<double>(steady_clock::now().time_since_epoch()).count();
}

static bool answer(const ChatArgs &a, Chat &c, Wl &w, View &v) {
    std::string said = v.input + "\n";
    v.said.push_back({true, v.input});
    v.input.clear();
    v.said.push_back({false, ""});
    v.status = "thinking";
    ui_draw(w, v), wl_present(w);
    Live live = {&w, &v};
    int made = 0;
    const double t0 = now_s();
    bool ok = chat_turn(a, c, said, to_window, &live, made);
    char line[64];
    snprintf(line, sizeof line, "%d tokens, %.1f tok/s", made, made / (now_s() - t0 + 1e-9));
    v.status = line;
    return ok;
}

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
        v.status = "the model could not be opened, errno " + std::to_string(c.err);
        ui_draw(w, v), wl_present(w);
        for (; wl_poll(w, keys); usleep(50000)) {}
        return 1;
    }
    v.status = "ready";
    ui_draw(w, v), wl_present(w);
    for (bool shown = w.configured; wl_poll(w, keys);) {
        if (!shown && w.configured) shown = true, ui_draw(w, v), wl_present(w);
        if (keys.empty()) { usleep(15000); continue; }
        for (Key k : keys) {
            if (k.code == 0x0D && v.input == "/reset") chat_reset(c), ui_wipe(w, v), v.status = "forgotten";
            else if (k.code == 0x0D && !v.input.empty() && !answer(a, c, w, v)) v.status = "fault";
            else if (k.code == 0x08 && !v.input.empty()) v.input.pop_back();
            else if (k.code == 0x1B) v.input.clear();
            else if (k.code >= 0x20 && k.code < 0x7F && v.input.size() < 2000) v.input += (char)k.code;
        }
        keys.clear();
        ui_draw(w, v), wl_present(w);
    }
    ui_wipe(w, v);
    chat_close(c);
    return 0;
}
