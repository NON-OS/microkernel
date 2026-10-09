/* qwenchat: the next smaller tier, chosen and said. See qwenchat_smaller.h. */
#include "qwenchat_smaller.h"

#include <cstdio>
#include <sstream>

std::vector<Tier> tiers_from(const std::string &text) {
    std::vector<Tier> all;
    std::istringstream lines(text);
    std::string word, file, sha;
    long long bytes = 0;
    while (lines >> word >> file >> bytes >> sha) {
        /* A split model's parts follow one another under one word. */
        if (all.empty() || all.back().word != word) all.push_back({word, file, 0});
        all.back().bytes += bytes;
    }
    return all;
}

const Tier *smaller_than(const std::vector<Tier> &all, long long bytes, long long short_by) {
    const Tier *best = nullptr;
    for (const Tier &t : all) {
        if (t.bytes * 20 > bytes * 19 || (short_by > 0 && t.bytes > bytes - short_by)) continue;
        if (!best || t.bytes > best->bytes) best = &t;
    }
    return best;
}

std::string smaller_line(const Tier *t, bool window) {
    if (!t) return " No smaller tier would fit; Qwen needs more free memory than this machine has now.";
    std::string label, word;
    if (!model_tier(t->file, label, word)) label = t->word;
    char size[32];
    snprintf(size, sizeof size, "%lld.%02lld GB", t->bytes / 1000000000, t->bytes / 10000000 % 100);
    if (window)
        return " A smaller tier may fit: " + label + ", " + size + "; open it from the Store, or type qwen window " +
               t->word + " in the Terminal.";
    return " A smaller tier may fit: " + label + ", " + size + "; type qwen " + t->word +
           " to run it, or qwen tiers to list them all.";
}
