/* qwenchat: what a key does in the chat window. */
#include "qwenchat.h"
#include "qwenui.h"

#include <chrono>
#include <cstdio>

struct Live { Wl *w; View *v; };

/* Each piece is shown at once, thinking aloud never; pings are answered. */
static void to_window(void *to, const char *piece, size_t n, bool thought) {
    Live *l = (Live *)to;
    if (!thought) l->v->said.back().text.append(piece, n);
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
    v.said.push_back({true, v.input}), v.said.push_back({false, ""});
    v.input.clear();
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

/* The words that end the chat, the same as on a terminal. */
static bool closing(const std::string &s) { return s == "/bye" || s == "/exit"; }

bool window_key(const ChatArgs &a, Chat &c, Wl &w, View &v, Key k) {
    if (k.code == 0x0D && closing(v.input)) return false;
    if (k.code == 0x0D && v.input == "/reset") chat_reset(c), ui_wipe(w, v), v.status = "forgotten";
    else if (k.code == 0x0D && !v.input.empty() && !answer(a, c, w, v)) v.status = "fault";
    else if (k.code == 0x08 && !v.input.empty()) v.input.pop_back();
    else if (k.code == 0x1B) v.input.clear();
    else if (k.code >= 0x20 && k.code < 0x7F && v.input.size() < 2000) v.input += (char)k.code;
    return true;
}
