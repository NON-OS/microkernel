/* qwenwl: requests out, events in, on the display socket. */
#include "qwenwl.h"

#include <cstring>
#include <sys/socket.h>
#include <unistd.h>

Req::Req(uint32_t object, uint16_t opcode) : op(opcode) {
    b.resize(8);
    memcpy(&b[0], &object, 4);
}

Req &Req::u32(uint32_t v) {
    const uint8_t *p = (const uint8_t *)&v;
    b.insert(b.end(), p, p + 4);
    return *this;
}

/* Length with the terminator, the bytes, the NUL, padded to a word. */
Req &Req::str(const std::string &s) {
    u32((uint32_t)s.size() + 1);
    b.insert(b.end(), s.begin(), s.end());
    b.push_back(0);
    while (b.size() % 4) b.push_back(0);
    return *this;
}

static void seal(Req &r) {
    uint32_t word = ((uint32_t)r.b.size() << 16) | r.op;
    memcpy(&r.b[4], &word, 4);
}

bool wl_send(Wl &w, Req &r) {
    seal(r);
    return write(w.fd, r.b.data(), r.b.size()) == (ssize_t)r.b.size();
}

bool wl_send_fd(Wl &w, Req &r, int fd) {
    seal(r);
    iovec iov = {r.b.data(), r.b.size()};
    alignas(cmsghdr) char ctl[CMSG_SPACE(sizeof(int))] = {};
    msghdr m = {};
    m.msg_iov = &iov;
    m.msg_iovlen = 1;
    m.msg_control = ctl;
    m.msg_controllen = sizeof ctl;
    cmsghdr *c = CMSG_FIRSTHDR(&m);
    c->cmsg_level = SOL_SOCKET;
    c->cmsg_type = SCM_RIGHTS;
    c->cmsg_len = CMSG_LEN(sizeof(int));
    memcpy(CMSG_DATA(c), &fd, sizeof(int));
    return sendmsg(w.fd, &m, 0) == (ssize_t)r.b.size();
}
