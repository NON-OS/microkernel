/* qwenchat: what the window says while the model opens, and when it did not. */
#include "qwenchat.h"
#include "qwenui.h"

#include <cerrno>
#include <cstdio>

/*
 * Before each part is opened the first time: which part, and how large.
 * Opening it may bring it onto the data volume, which the window cannot
 * draw through, so this is on screen for as long as that takes. It says
 * opening, not checking: a part already on the volume is checked no more.
 */
void window_importing(void *to, int part, int parts, long long bytes) {
    Busy &b = *(Busy *)to;
    char line[64];
    if (parts > 1)
        snprintf(line, sizeof line, "opening part %d of %d, %lld.%02lld GB", part, parts, bytes / 1000000000,
                 bytes / 10000000 % 100);
    else
        snprintf(line, sizeof line, "opening the model, %lld.%02lld GB", bytes / 1000000000, bytes / 10000000 % 100);
    b.v->status = line;
    b.shown = busy_now();
    ui_draw(*b.w, *b.v), wl_present(*b.w);
}

/*
 * Said while the parts are opened, and taken away once they are. Only a
 * part brought in from a disk is sealed and checked here; one downloaded
 * with qwen get or from the Store was checked as it came.
 */
const char *const WINDOW_FIRST_USE =
    "Opening the model. A file brought in from a disk is checked against its signed SHA-256 pin "
    "once; a large one takes minutes.";

/* The status line under a model that did not open: what stopped it, in a few words. */
const char *window_not_opened(int err) {
    switch (err) {
    case ENOENT: return "no model yet, Esc closes";
    case ENOMEM: return "not enough memory, Esc closes";
    case ENOSPC: return "no room for the model, Esc closes";
    default: return "the model did not open, Esc closes";
    }
}
