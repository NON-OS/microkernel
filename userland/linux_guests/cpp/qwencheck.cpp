/* qwencheck: arguments, the prompt, and the verdict. See qwencheck.h. */
#include "qwencheck.h"

#include <cstdio>
#include <cstdlib>
#include <cstring>

/* -m model -f prompt_file -n n_predict -t threads -expect "id id ..." */
bool parse_args(int argc, char **argv, Args &a) {
    for (int i = 1; i + 1 < argc; i += 2) {
        const char *k = argv[i], *v = argv[i + 1];
        if (!strcmp(k, "-m")) {
            a.model = v;
        } else if (!strcmp(k, "-f")) {
            a.prompt_file = v;
        } else if (!strcmp(k, "-n")) {
            a.n_predict = atoi(v);
        } else if (!strcmp(k, "-t")) {
            a.threads = atoi(v);
        } else if (!strcmp(k, "-expect")) {
            for (char *p = argv[i + 1]; *p;) {
                char *end;
                long id = strtol(p, &end, 10);
                if (end == p) break;
                a.expect.push_back((int)id);
                p = end;
            }
        } else {
            return false;
        }
    }
    return !a.prompt_file.empty() && !a.expect.empty() && a.n_predict > 0 && a.n_predict <= 512
        && a.threads > 0 && a.threads <= 16;
}

static bool read_prompt(const std::string &path, std::string &out) {
    FILE *f = fopen(path.c_str(), "rb");
    if (!f) return false;
    char buf[4096];
    size_t n;
    while ((n = fread(buf, 1, sizeof buf, f)) > 0 && out.size() < (1u << 16)) out.append(buf, n);
    fclose(f);
    return !out.empty();
}

int main(int argc, char **argv) {
    Args a;
    std::string msg;
    if (!parse_args(argc, argv, a) || !read_prompt(a.prompt_file, msg)) return FAILED;
    std::string prompt = "<|im_start|>system\nYou are a helpful assistant.<|im_end|>\n"
                         "<|im_start|>user\n" + msg + "<|im_end|>\n<|im_start|>assistant\n";
    Run r;
    bool ok = generate(a, prompt, r);
    /* The prompt and the reply are wiped before the verdict is said. */
    memset(&prompt[0], 0, prompt.size());
    memset(&msg[0], 0, msg.size());
    if (!ok) return FAILED;
    bool match = r.ids == a.expect;
    report(a, r, match);
    return match ? MATCH : MISMATCH;
}
