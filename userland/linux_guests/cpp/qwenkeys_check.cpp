/*
 * qwenkeys_check: a host check that qwenchat's window types what the
 * display server's keys mean (qwenwl_keymap.cpp, qwenwl_keys.cpp). The
 * server sends Linux key codes and xkb modifiers, and tells a held key
 * once with repeat_info; qwenchat read the code as a character and never
 * repeated. Built and run on the host by `make nonos-mk-qwenkeys-check
 * NONOS_LINUX_GUESTS=1 NONOS_QWEN=1` (QwenChat.mk); exits 0 when every
 * check holds, else names the first that does not.
 */
#include "qwenwl_ids.h"
#include "qwenwl_keys.h"

#include <cstdio>
#include <fcntl.h>
#include <sys/socket.h>
#include <unistd.h>

static std::vector<uint8_t> words(std::vector<uint32_t> w) {
    std::vector<uint8_t> b(w.size() * 4);
    for (size_t i = 0; i < w.size(); i++)
        for (int j = 0; j < 4; j++) b[i * 4 + j] = (uint8_t)(w[i] >> (8 * j));
    return b;
}

static std::string typed(const std::vector<Key> &keys) {
    std::string s;
    for (Key k : keys) s += (char)k.code;
    return s;
}

#define CHECK(what, cond) if (!(cond)) return printf("qwenkeys_check: %s\n", what), 1

int main() {
    CHECK("a", wl_char(30, 0, 0) == 'a');
    CHECK("Shift a", wl_char(30, 1, 0) == 'A');
    CHECK("Caps a", wl_char(30, 0, 2) == 'A' && wl_char(30, 1, 2) == 'a');
    CHECK("Caps leaves digits", wl_char(2, 0, 2) == '1' && wl_char(2, 1, 0) == '!');
    CHECK("punctuation", wl_char(43, 0, 0) == '\\' && wl_char(40, 1, 0) == '"' && wl_char(53, 1, 0) == '?');
    CHECK("controls", wl_char(28, 0, 0) == 0x0D && wl_char(14, 0, 0) == 0x08 && wl_char(1, 0, 0) == 0x1B);
    CHECK("Ctrl and Alt type nothing", wl_char(30, 4, 0) == 0 && wl_char(30, 8, 0) == 0);
    CHECK("no character", wl_char(42, 0, 0) == 0 && wl_char(15, 0, 0) == 0 && wl_char(59, 0, 0) == 0);
    CHECK("space", wl_char(57, 0, 0) == ' ');
    Wl w;
    std::vector<Key> keys;
    wl_repeat_info(w, words({25, 600}));
    wl_key(w, words({1, 0, 30, 1}), keys, 0);
    wl_repeat(w, keys, 599);
    CHECK("once before the delay", typed(keys) == "a");
    wl_repeat(w, keys, 600), wl_repeat(w, keys, 680);
    CHECK("then at the rate", typed(keys) == "aaaa");
    wl_repeat(w, keys, 5000);
    CHECK("a stall types one, not its worth", typed(keys) == "aaaaa");
    wl_key(w, words({2, 0, 30, 0}), keys, 5010), wl_repeat(w, keys, 9000);
    CHECK("nothing after release", typed(keys) == "aaaaa");
    keys.clear(), wl_key(w, words({3, 0, 1, 1}), keys, 0), wl_repeat(w, keys, 2000);
    CHECK("Escape acts once", typed(keys) == "\x1b");
    keys.clear(), wl_key(w, words({4, 0, 14, 1}), keys, 0), wl_repeat(w, keys, 600);
    CHECK("Backspace repeats", typed(keys) == "\x08\x08");
    int sv[2];
    if (socketpair(AF_UNIX, SOCK_STREAM, 0, sv) != 0) return perror("socketpair"), 1;
    Wl d;
    d.fd = sv[0], fcntl(d.fd, F_SETFL, O_NONBLOCK);
    auto tell = [&](uint16_t op, std::vector<uint32_t> body) {
        std::vector<uint32_t> m = {KBD, (uint32_t)((8 + body.size() * 4) << 16) | op};
        m.insert(m.end(), body.begin(), body.end());
        return write(sv[1], m.data(), m.size() * 4) == (ssize_t)(m.size() * 4);
    };
    CHECK("write", tell(5, {25, 600}) && tell(4, {9, 1, 0, 0, 0}) && tell(3, {9, 0, 30, 1}));
    keys.clear(), wl_poll(d, keys);
    CHECK("wl_poll types Shift a", typed(keys) == "A" && d.rate == 25 && d.held == 30);
    CHECK("write", tell(2, {10, 7}) && (wl_poll(d, keys), true));
    CHECK("a leave lets go of the held key", d.held == 0);
    puts("qwenkeys_check: ok");
    return 0;
}
