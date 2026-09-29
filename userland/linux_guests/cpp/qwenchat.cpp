/* qwenchat: arguments and the conversation loop. See qwenchat.h. */
#include "qwenchat.h"

#include <cstdio>
#include <cstdlib>
#include <cstring>

/* -m model -t threads -c context -n reply_limit -temp t -seed s */
bool chat_args(int argc, char **argv, ChatArgs &a) {
    for (int i = 1; i + 1 < argc; i += 2) {
        const char *k = argv[i], *v = argv[i + 1];
        if (!strcmp(k, "-m")) a.model = v;
        else if (!strcmp(k, "-t")) a.threads = atoi(v);
        else if (!strcmp(k, "-c")) a.n_ctx = atoi(v);
        else if (!strcmp(k, "-n")) a.n_reply = atoi(v);
        else if (!strcmp(k, "-temp")) a.temp = (float)atof(v);
        else if (!strcmp(k, "-seed")) a.seed = (unsigned)strtoul(v, nullptr, 10);
        else if (!strcmp(k, "-ui")) a.window = !strcmp(v, "window");
        else return false;
    }
    return a.threads > 0 && a.threads <= 64 && a.n_ctx >= 256 && a.n_ctx <= 32768
        && a.n_reply > 0 && a.n_reply < a.n_ctx && a.temp >= 0.0f && a.temp <= 2.0f;
}

static void to_stdout(void *, const char *piece, size_t n) {
    fwrite(piece, 1, n, stdout);
    fflush(stdout);
}

static void say(const char *s) {
    fputs(s, stdout);
    fflush(stdout);
}

int main(int argc, char **argv) {
    ChatArgs a;
    if (!chat_args(argc, argv, a)) {
        say("usage: qwenchat [-m model] [-t n] [-c ctx] [-n reply] [-temp t] [-seed s] [-ui window]\n");
        return 2;
    }
    if (a.window) return chat_window(a);
    say("Loading the model from the local volume...\n");
    Chat c;
    if (!chat_open(a, c)) {
        say("The model could not be opened. Nothing left this machine.\n");
        return 1;
    }
    say("Qwen is running on this machine, offline. /reset clears the conversation, "
        "/quit ends it.\n");
    std::string line;
    char buf[1024];
    for (;;) {
        say("\nyou> ");
        line.clear();
        while (fgets(buf, sizeof buf, stdin)) {
            line += buf;
            if (!line.empty() && line.back() == '\n') break;
        }
        memset(buf, 0, sizeof buf);
        if (line.empty() || line == "/quit\n") break;
        if (line == "/reset\n") {
            chat_reset(c);
            say("(conversation cleared)\n");
            continue;
        }
        say("qwen> ");
        int made;
        if (!chat_turn(a, c, line, to_stdout, nullptr, made)) break;
        say("\n");
    }
    wipe(line);
    chat_close(c);
    say("\nThe conversation was wiped from memory.\n");
    return 0;
}
