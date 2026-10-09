/* qwenwl: events off the display socket, whole ones only. */
#include "qwenwl.h"

#include <cstring>
#include <unistd.h>

uint32_t wl_word(const std::vector<uint8_t> &body, size_t at) {
    uint32_t v = 0;
    if (at + 4 <= body.size()) memcpy(&v, &body[at], 4);
    return v;
}

bool wl_event(Wl &w, uint32_t &object, uint16_t &op, std::vector<uint8_t> &body) {
    for (;;) {
        if (w.rx.size() >= 8) {
            uint32_t word;
            memcpy(&object, &w.rx[0], 4);
            memcpy(&word, &w.rx[4], 4);
            size_t size = word >> 16;
            if (size < 8) return w.closed = true, false;
            if (w.rx.size() >= size) {
                op = (uint16_t)word;
                body.assign(w.rx.begin() + 8, w.rx.begin() + size);
                w.rx.erase(w.rx.begin(), w.rx.begin() + size);
                return true;
            }
        }
        uint8_t buf[4096];
        ssize_t n = read(w.fd, buf, sizeof buf);
        if (n <= 0) return false;
        w.rx.insert(w.rx.end(), buf, buf + n);
    }
}
