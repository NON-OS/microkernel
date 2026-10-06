/* qwenui: what qwenchat's window shows, drawn into its buffer. */
#pragma once
#include <string>
#include <vector>

#include "qwenwl.h"

struct Said {
    bool user;
    std::string text;
};

struct View {
    std::string title;  /* the model, in the header */
    std::vector<Said> said;
    std::string input;
    std::string status; /* right of the header: loading, thinking, speed */
    bool thinks = false; /* the model can think aloud: /think and /nothink */
};

void ui_draw(Wl &w, const View &v);
/* Drawn again and shown when the window took a new size (wl_resize). */
void ui_redraw_resized(Wl &w, const View &v);
/* Text in the atlas's cells at (x, y), clipped to the buffer. */
void ui_text(Wl &w, int x, int y, const std::string &s, uint32_t color);
void ui_fill(Wl &w, int x, int y, int width, int height, uint32_t color);
/* `s` broken into lines of at most `cols` cells, at spaces where it can. */
std::vector<std::string> ui_wrap(const std::string &s, size_t cols);
/* Wipe what was said, the draft and the pixels that showed them. */
void ui_wipe(Wl &w, View &v);

struct ChatArgs;
struct Chat;
/* One key in the chat window; false once the person asks to close it. */
bool window_key(const ChatArgs &a, Chat &c, Wl &w, View &v, Key k);

/*
 * The window while the model loads or a turn computes: the compositor is
 * answered, the status shows how far along and for how long, and Esc or
 * the window's close stops the work.
 */
struct Busy {
    Wl *w;
    View *v;
    Chat *c;
    const char *doing = "loading the model";
    double since = 0, shown = 0;
    bool quit = false; /* the window was closed, not only the work stopped */
};
/* Chat::stop for the window: true once Esc is pressed or the window closed. */
bool busy_stop(void *to);
double busy_now();

/*
 * While the model opens (qwenchat_window_open.cpp): Chat::step for the
 * window, the note shown meanwhile, and the status when it did not open.
 */
void window_importing(void *to, int part, int parts, long long bytes);
extern const char *const WINDOW_FIRST_USE;
const char *window_not_opened(int err);
