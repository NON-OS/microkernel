/*
 * qwensmaller-check: the tier offered when one does not fit, on the host.
 * The list is /models/tiers as the personality writes it from its pins
 * (file/models/pinned_*.rs), digests left out, a 7B in two parts.
 */
#include <cstdio>
#include <cstring>

#include "qwenchat_smaller.h"

static const char *TIERS =
    "small qwen2.5-0.5b-instruct-q4_k_m.gguf 491400032 -\n"
    "medium qwen2.5-1.5b-instruct-q4_k_m.gguf 1117320736 -\n"
    "large qwen2.5-3b-instruct-q4_k_m.gguf 2104932768 -\n"
    "xlarge qwen2.5-7b-instruct-q4_k_m-00001-of-00002.gguf 3993201344 -\n"
    "xlarge qwen2.5-7b-instruct-q4_k_m-00002-of-00002.gguf 689872288 -\n"
    "qwen3-0.6b Qwen3-0.6B-Q8_0.gguf 639446688 -\n"
    "qwen3-4b Qwen3-4B-Q4_K_M.gguf 2497280256 -\n"
    "qwen3-8b Qwen3-8B-Q4_K_M.gguf 5027783488 -\n"
    "coder-1.5b qwen2.5-coder-1.5b-instruct-q4_k_m.gguf 1117320768 -\n";

static int failed = 0;

static void expect(bool ok, const char *what) {
    if (!ok) failed++, fprintf(stderr, "qwensmaller_check: %s\n", what);
}

/* The word offered under the tier `word`, short by `short_by`; "" for none. */
static std::string under(const std::vector<Tier> &all, const char *word, long long short_by) {
    for (const Tier &t : all)
        if (t.word == word) {
            const Tier *s = smaller_than(all, t.bytes, short_by);
            return s ? s->word : "";
        }
    return "?";
}

int main() {
    const std::vector<Tier> all = tiers_from(TIERS);
    expect(all.size() == 8, "a split model is one tier");
    expect(all[3].word == "xlarge" && all[3].bytes == 4683073632LL, "a split model's parts are summed");
    expect(under(all, "qwen3-0.6b", 0) == "small", "the stick tier offers Qwen2.5 0.5B");
    expect(under(all, "small", 0).empty(), "the smallest offers none");
    expect(under(all, "medium", 0) == "qwen3-0.6b", "1.5B is not offered its Coder twin");
    expect(under(all, "coder-1.5b", 0) == "qwen3-0.6b", "Coder 1.5B is not offered 1.5B");
    expect(under(all, "qwen3-4b", 0) == "large", "4B offers 3B");
    expect(under(all, "qwen3-4b", 1500000000LL) == "qwen3-0.6b", "1.5 GB short skips the tiers it cannot spare");
    expect(under(all, "qwen3-4b", 3000000000LL).empty(), "short by more than the file offers none");
    expect(under(all, "qwen3-8b", 0) == "xlarge", "8B offers the split 7B");
    expect(tiers_from("").empty() && tiers_from("small x.gguf\n").empty(), "a short line is no tier");
    const std::string term = smaller_line(&all[0], false), win = smaller_line(&all[0], true);
    expect(term.find("Qwen2.5 0.5B, 0.49 GB; type qwen small to run it") != std::string::npos, "terminal words");
    expect(win.find("type qwen window small in the Terminal") != std::string::npos, "window words");
    expect(smaller_line(nullptr, false).find("No smaller tier would fit") != std::string::npos, "none said");
    if (failed) return 1;
    puts("qwensmaller_check: ok");
    return 0;
}
