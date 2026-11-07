/*
 * qwenui: a frame presented as little as covers what changed. The window
 * presented its whole surface for every key and every status tick, so the
 * bridge copied and the compositor composed 880 by 560 pixels where the
 * line being typed or the status had changed. The buffer always holds the
 * whole frame (ui_draw), so naming fewer rows shows the same pixels.
 */
#include "qwenui.h"

static size_t said_bytes(const View &v) {
    size_t n = 0;
    for (const Said &s : v.said) n += s.text.size();
    return n;
}

/*
 * The conversation, title and hints as the last ui_show presented them,
 * with no other present since. What was said only grows or is wiped, so
 * its count and its length tell any change.
 */
static bool body_same(const Wl &w, const View &v) {
    const View::Drawn &d = v.drawn;
    return d.at == w.presents && d.said_n == v.said.size() && d.said_bytes == said_bytes(v) &&
           d.title == v.title && d.thinks == v.thinks;
}

static void keep(const Wl &w, View &v) {
    View::Drawn &d = v.drawn;
    d.at = w.presents;
    d.said_n = v.said.size(), d.said_bytes = said_bytes(v);
    d.title = v.title, d.status = v.status, d.thinks = v.thinks;
}

void ui_show(Wl &w, View &v) {
    const bool line_only = body_same(w, v) && v.drawn.status == v.status;
    ui_draw(w, v);
    if (line_only) wl_present_rows(w, ui_box_y(w), ui_box_h());
    else wl_present(w);
    keep(w, v);
}

void ui_show_status(Wl &w, View &v) {
    const bool status_only = body_same(w, v);
    ui_draw(w, v);
    if (status_only) wl_present_rows(w, 0, ui_head_h());
    else wl_present(w);
    keep(w, v);
}
