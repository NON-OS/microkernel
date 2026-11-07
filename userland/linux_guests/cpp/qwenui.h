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
    /* What the last ui_show presented (qwenui_show.cpp): the counts and the
     * header's words, never a copy of what was said or typed, which
     * ui_wipe must be able to wipe. */
    struct Drawn {
        uint32_t at = 0; /* Wl::presents after it: any present since makes it stale */
        size_t said_n = 0, said_bytes = 0;
        std::string title, status;
        bool thinks = false;
    } drawn;
};

void ui_draw(Wl &w, const View &v);
/*
 * Drawn, then presented as little as covers what changed since the last
 * ui_show: ui_show the input box alone when only the line being typed can
 * have changed, ui_show_status the header band alone when only the status
 * did (for a caller that changes nothing else); otherwise the whole frame.
 */
void ui_show(Wl &w, View &v);
void ui_show_status(Wl &w, View &v);
/* The header band's height, and the input box's top and height. */
int ui_head_h();
int ui_box_y(const Wl &w);
int ui_box_h();
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
