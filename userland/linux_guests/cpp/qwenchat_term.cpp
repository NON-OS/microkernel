/* qwenchat: the conversation on the terminal. See qwenchat.h. */
#include "qwenchat.h"

#include <cerrno>
#include <chrono>
#include <cstdio>
#include <cstring>

static const char *DIM = "\x1b[2m", *PLAIN = "\x1b[0m";

static void say(const char *s) {
    fputs(s, stdout);
    fflush(stdout);
}

/* Each piece is out as soon as it is made; thinking aloud is dim. */
static void to_tty(void *to, const char *piece, size_t n, bool thought) {
    bool &dim = *(bool *)to;
    if (thought != dim) fputs(thought ? DIM : PLAIN, stdout), dim = thought;
    fwrite(piece, 1, n, stdout);
    fflush(stdout);
}

static double now_s() {
    using namespace std::chrono;
    return duration<double>(steady_clock::now().time_since_epoch()).count();
}

/* One line, without its end or trailing blanks; false at the end of input. */
static bool read_line(std::string &line) {
    char buf[512];
    line.clear();
    while (fgets(buf, sizeof buf, stdin) && (line += buf, line.back() != '\n')) {}
    memset(buf, 0, sizeof buf);
    if (line.empty()) return false;
    while (!line.empty() && strchr(" \t\r\n", line.back())) line.back() = 0, line.pop_back();
    return true;
}

/* The terminal echoes what is typed, so the line is never printed back. */
int chat_terminal(const ChatArgs &a) {
    const char *slash = strrchr(a.model.c_str(), '/'), *name = slash ? slash + 1 : a.model.c_str();
    printf("Loading %s from the local volume...\n", name), fflush(stdout);
    Chat c;
    if (!chat_open(a, c)) return say((chat_failure(a, c) + "\n").c_str()), c.err == ENOMEM ? 4 : 1;
    if (c.mem.free < 0) say((mem_unknown(a.model.c_str(), c.mem) + "\n").c_str());
    printf("Qwen %s on this machine: offline, private, %d threads.\n/reset forgets the conversation, %s"
           "/exit or /bye ends it.\n", name, c.threads, c.can_think ? "/think and /nothink switch thinking, " : "");
    std::string line;
    line.reserve(4096);
    int status = 0;
    for (bool dim = false; say("\nyou> "), read_line(line);) {
        if (line == "/exit" || line == "/bye" || line == "\x04") break;
        if (line.empty()) continue;
        if (line == "/reset") { chat_reset(c), say("(the conversation is forgotten)\n"); continue; }
        if (line == "/think" || line == "/nothink") {
            c.think = c.can_think && line == "/think";
            say(!c.can_think ? "(this model does not think)\n" : c.think ? "(thinking aloud)\n" : "(no thinking)\n");
            continue;
        }
        say("qwen> ");
        int made = 0;
        const double t0 = now_s();
        const bool ok = chat_turn(a, c, line, to_tty, &dim, made);
        const double s = now_s() - t0;
        printf("%s\n%s(%d tokens, %.1f s, %.1f tok/s)%s\n", dim ? PLAIN : "", DIM, made, s, made / s, PLAIN);
        dim = false, fflush(stdout);
        if (!ok) { say("The model stopped with a fault.\n"), status = 1; break; }
    }
    wipe(line);
    chat_close(c);
    say("\nThe conversation was wiped from memory.\n");
    return status;
}
