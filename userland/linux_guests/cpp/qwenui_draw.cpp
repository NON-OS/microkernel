/* qwenui: the header, the conversation, and the line being typed. */
#include "qwenglyphs.h"
#include "qwenui.h"

static const uint32_t BG = 0xFF0B0D10, BAND = 0xFF10262C, INK = 0xFFE6EAEE;
static const uint32_t CYAN = 0xFF66E0E8, DIM = 0xFF7A8690, BOX = 0xFF161B20;
static const int PAD = 16, HEAD = 44, FOOT = 26, BOXH = CELL_H + 18;

static size_t cells(const std::string &s) {
    size_t n = 0;
    for (unsigned char c : s) n += (c & 0xC0) != 0x80;
    return n;
}

std::vector<std::string> ui_wrap(const std::string &s, size_t cols) {
    std::vector<std::string> out(1);
    std::string word;
    auto place = [&](const std::string &wd) {
        if (!out.back().empty() && cells(out.back()) + 1 + cells(wd) > cols) out.emplace_back();
        if (!out.back().empty()) out.back() += ' ';
        out.back() += wd;
        while (cells(out.back()) > cols) { /* a word longer than a line */
            std::string rest = out.back().substr(cols);
            out.back().resize(cols);
            out.push_back(rest);
        }
    };
    for (char c : s) {
        if (c == '\n') place(word), word.clear(), out.emplace_back();
        else if (c == ' ') place(word), word.clear();
        else word += c;
    }
    place(word);
    return out;
}

void ui_draw(Wl &w, const View &v) {
    ui_fill(w, 0, 0, w.w, w.h, BG);
    ui_fill(w, 0, 0, w.w, HEAD, BAND);
    ui_text(w, PAD, (HEAD - CELL_H) / 2, v.title, INK);
    ui_text(w, w.w - PAD - (int)cells(v.status) * CELL_W, (HEAD - CELL_H) / 2, v.status, CYAN);
    const size_t cols = (w.w - 2 * PAD) / CELL_W - 6;
    std::vector<std::pair<std::string, uint32_t>> lines;
    for (const Said &s : v.said) {
        std::vector<std::string> body = ui_wrap(s.text, cols);
        for (size_t i = 0; i < body.size(); i++)
            lines.push_back({(i ? "      " : s.user ? "you   " : "qwen  ") + body[i], s.user ? CYAN : INK});
        lines.push_back({"", INK});
    }
    const int top = HEAD + PAD, bottom = w.h - FOOT - BOXH - PAD;
    const size_t fit = (bottom - top) / CELL_H;
    size_t from = lines.size() > fit ? lines.size() - fit : 0;
    for (size_t i = from; i < lines.size(); i++)
        ui_text(w, PAD, top + (int)(i - from) * CELL_H, lines[i].first, lines[i].second);
    const int by = w.h - FOOT - BOXH;
    ui_fill(w, PAD, by, w.w - 2 * PAD, BOXH, BOX);
    ui_fill(w, PAD, by, 3, BOXH, CYAN);
    std::string shown = "> " + v.input;
    if (cells(shown) + 1 > cols + 4) shown = "> ..." + shown.substr(shown.size() - (cols - 2));
    ui_text(w, PAD + 12, by + 9, shown + "_", INK);
    ui_text(w, PAD, w.h - FOOT + 4,
            "Enter sends   Esc clears   /reset forgets   /bye closes   nothing leaves this machine", DIM);
}
