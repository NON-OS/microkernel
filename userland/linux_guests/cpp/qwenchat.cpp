/* qwenchat: arguments, and which of the two faces to show. See qwenchat.h. */
#include "qwenchat.h"

#include <cstdio>
#include <cstdlib>
#include <cstring>

/* -m model -t threads -c context -n reply_limit -temp t -rp p -seed s -ui window */
bool chat_args(int argc, char **argv, ChatArgs &a) {
    for (int i = 1; i + 1 < argc; i += 2) {
        const char *k = argv[i], *v = argv[i + 1];
        if (!strcmp(k, "-m")) a.model = v;
        else if (!strcmp(k, "-t")) a.threads = atoi(v);
        else if (!strcmp(k, "-c")) a.n_ctx = atoi(v);
        else if (!strcmp(k, "-n")) a.n_reply = atoi(v);
        else if (!strcmp(k, "-temp")) a.temp = (float)atof(v);
        else if (!strcmp(k, "-rp")) a.repeat = (float)atof(v);
        else if (!strcmp(k, "-seed")) a.seed = (unsigned)strtoul(v, nullptr, 10);
        else if (!strcmp(k, "-ui")) a.window = !strcmp(v, "window");
        else return false;
    }
    const bool ctx_ok = a.n_ctx == 0 || (a.n_ctx >= 256 && a.n_ctx <= 32768 && a.n_reply <= a.n_ctx / 2);
    return a.threads >= 0 && a.threads <= 64 && ctx_ok && a.n_reply > 0 && a.n_reply <= 1024
        && a.temp >= 0.0f && a.temp <= 2.0f && a.repeat >= 1.0f && a.repeat <= 2.0f;
}

int main(int argc, char **argv) {
    ChatArgs a;
    if (!chat_args(argc, argv, a)) {
        fputs("usage: qwenchat [-m model] [-t threads] [-c ctx] [-n reply] [-temp t] [-rp penalty]"
              " [-seed s] [-ui window]\n", stdout);
        fflush(stdout);
        return 2;
    }
    return a.window ? chat_window(a) : chat_terminal(a);
}
