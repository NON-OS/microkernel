/*
 * qwenwl: the few Wayland requests qwenchat's window needs, spoken on the
 * wire with no library: a toplevel, one shared-memory buffer it paints
 * into, and the keyboard. The personality's display server answers them;
 * it copies the buffer on every commit, so one buffer is enough.
 */
#pragma once
#include <cstdint>
#include <string>
#include <vector>

struct Wl {
    int fd = -1;
    std::vector<uint8_t> rx;
    uint32_t *px = nullptr;
    int w = 0, h = 0;
    bool closed = false;
    bool configured = false;
};

/* One request under construction: object, opcode, then arguments. */
struct Req {
    std::vector<uint8_t> b;
    uint16_t op;
    Req(uint32_t object, uint16_t opcode);
    Req &u32(uint32_t v);
    Req &str(const std::string &s);
};

/* A key the user pressed: a NONOS key code, which for text is the character. */
struct Key {
    uint32_t code;
};

bool wl_send(Wl &w, Req &r);
bool wl_send_fd(Wl &w, Req &r, int fd);
/* The next whole event, or false when none is queued (or the socket died). */
bool wl_event(Wl &w, uint32_t &object, uint16_t &op, std::vector<uint8_t> &body);
uint32_t wl_word(const std::vector<uint8_t> &body, size_t at);

bool wl_open(Wl &w, int width, int height, const char *title);
/* Answer pings and configures; append keys pressed. False once closed. */
bool wl_poll(Wl &w, std::vector<Key> &keys);
void wl_present(Wl &w);
