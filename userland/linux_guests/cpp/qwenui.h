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
};

void ui_draw(Wl &w, const View &v);
/* Text in the atlas's cells at (x, y), clipped to the buffer. */
void ui_text(Wl &w, int x, int y, const std::string &s, uint32_t color);
void ui_fill(Wl &w, int x, int y, int width, int height, uint32_t color);
/* `s` broken into lines of at most `cols` cells, at spaces where it can. */
std::vector<std::string> ui_wrap(const std::string &s, size_t cols);
/* Wipe what was said, the draft and the pixels that showed them. */
void ui_wipe(Wl &w, View &v);
