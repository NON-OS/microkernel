/*
 * qwenwl: the keyboard's events, and the repeat of a held key. The server
 * tells each press and release once, as Wayland does, with repeat_info's
 * rate and delay; a client repeats a held key itself. qwenchat repeats the
 * keys that edit its line, a character and Backspace, never Enter or
 * Escape, which act once.
 */
#include "qwenwl_keys.h"

#include <time.h>

long long wl_now_ms() {
    timespec t;
    clock_gettime(CLOCK_MONOTONIC, &t);
    return (long long)t.tv_sec * 1000 + t.tv_nsec / 1000000;
}

static bool repeats(uint32_t ch) { return ch == 0x08 || (ch >= 0x20 && ch < 0x7F); }

/* wl_keyboard.key: serial, time, key, state. */
void wl_key(Wl &w, const std::vector<uint8_t> &body, std::vector<Key> &keys, long long now) {
    const uint32_t code = wl_word(body, 8), state = wl_word(body, 12);
    if (state != 1) {
        if (code == w.held) w.held = 0;
        return;
    }
    const uint32_t ch = wl_char(code, w.depressed, w.locked);
    if (ch) keys.push_back(Key{ch});
    w.held = ch && repeats(ch) ? code : 0;
    w.next_ms = now + w.delay_ms;
}

/* wl_keyboard.modifiers: serial, depressed, latched, locked, group. */
void wl_mods(Wl &w, const std::vector<uint8_t> &body) {
    w.depressed = wl_word(body, 4), w.locked = wl_word(body, 12);
}

/* wl_keyboard.repeat_info: rate a second, delay in ms; a rate of 0 is none. */
void wl_repeat_info(Wl &w, const std::vector<uint8_t> &body) {
    w.rate = (int)wl_word(body, 0), w.delay_ms = (int)wl_word(body, 4);
}

/*
 * The repeats due by `now`, as the held key types under the modifiers now.
 * After a stall (the window busy answering) the repeat picks up from now
 * rather than typing the whole stall's worth at once.
 */
void wl_repeat(Wl &w, std::vector<Key> &keys, long long now) {
    if (!w.held || w.rate <= 0 || now < w.next_ms) return;
    const long long period = 1000 / w.rate > 0 ? 1000 / w.rate : 1;
    if (now - w.next_ms > 1000) w.next_ms = now;
    const uint32_t ch = wl_char(w.held, w.depressed, w.locked);
    for (; w.next_ms <= now; w.next_ms += period)
        if (ch) keys.push_back(Key{ch});
}
