/*
 * qwenwhy_check: a host check of qwenchat's refusal sentences (qwenchat_why.cpp).
 * Each of ENOENT, ENODEV, EIO, ENOSPC, ENOMEM, EBADMSG, EEXIST and EACCES has a
 * sentence of its own that names the model and ends a sentence, no two alike,
 * and any other errno none. ENOENT speaks of a live boot's memory, ENODEV of no
 * disk, and ENOSPC and ENOMEM give the model's files beside what a live session
 * holds, which is all memory less the larger of 1 GiB and a quarter.
 * Built and run on the host by `make qwenchat-why-check` (QwenChat.mk); exits 0
 * when every check holds, else names the first that does not.
 */
#include "qwenchat.h"

#include <cerrno>
#include <cstdio>
#include <set>

static int failed(const char *what, int err) {
    printf("qwenwhy_check: %s (errno %d)\n", what, err);
    return 1;
}

int main() {
    const long long GIB = 1LL << 30;
    /* The live boot's keep: 1 GiB up to 4 GiB of memory, a quarter above. */
    if (session_hold(2 * GIB) != GIB) return failed("2 GiB holds 1 GiB", 0);
    if (session_hold(4 * GIB) != 3 * GIB) return failed("4 GiB holds 3 GiB", 0);
    if (session_hold(8 * GIB) != 6 * GIB) return failed("8 GiB holds 6 GiB", 0);
    if (session_hold(GIB / 2) != 0 || session_hold(0) != -1) return failed("little or unknown memory", 0);
    const int named[] = {ENOENT, ENODEV, EIO, ENOSPC, ENOMEM, EBADMSG, EEXIST, EACCES};
    std::set<std::string> seen;
    /* The 0.5B, 0.6B and 4B files, on a 4 GiB live boot. */
    const struct { const char *name; long long bytes; } models[] = {
        {"qwen2.5-0.5b-instruct-q4_k_m.gguf", 491400032},
        {"Qwen3-0.6B-Q8_0.gguf", 639446688},
        {"Qwen3-4B-Q4_K_M.gguf", 2497280256},
    };
    for (const auto &m : models) {
        Room r;
        r.files = m.bytes, r.hold = session_hold(4 * GIB);
        for (int err : named) {
            const std::string s = why_refused(err, m.name, r);
            if (s.empty()) return failed("no sentence", err);
            if (s.find(m.name) == std::string::npos) return failed("the model is not named", err);
            if (s.back() != '.') return failed("not a whole sentence", err);
            if (s.size() > 400) return failed("longer than the window says at once", err);
            if (!seen.insert(s).second) return failed("the same sentence as another errno", err);
        }
        for (int err : {ENOSPC, ENOMEM}) {
            const std::string s = why_refused(err, m.name, r);
            if (s.find("3.22 GB in memory") == std::string::npos) return failed("what the session holds", err);
        }
    }
    const std::string four = why_refused(ENOMEM, "Qwen3-4B-Q4_K_M.gguf", Room{2497280256, 0});
    if (four.find("its files are 2.49 GB") == std::string::npos) return failed("the 4B's files", ENOMEM);
    if (why_refused(ENOENT, "m", Room()).find("in memory for this session") == std::string::npos)
        return failed("a live boot's memory", ENOENT);
    if (why_refused(ENODEV, "m", Room()).find("no NONOS disk") == std::string::npos)
        return failed("no disk", ENODEV);
    if (why_refused(ENOSPC, "m", Room()).find("(") != std::string::npos) return failed("sizes unknown", ENOSPC);
    for (int err : {0, ECANCELED, EINVAL, EPERM})
        if (!why_refused(err, "m", Room()).empty()) return failed("a sentence for an errno it does not name", err);
    /* Every pinned file is its tier by label and word; the empty state is one short line. */
    const struct { const char *file, *label, *word; } tiers[] = {
        {"/models/qwen2.5-coder-1.5b-instruct-q4_k_m.gguf", "Qwen2.5 Coder 1.5B", "coder-1.5b"},
        {"/models/qwen2.5-1.5b-instruct-q4_k_m.gguf", "Qwen2.5 1.5B", "medium"},
        {"/models/qwen2.5-32b-instruct-q4_k_m-00001-of-00005.gguf", "Qwen2.5 32B", "max"},
        {"/models/qwen2.5-3b-instruct-q4_k_m.gguf", "Qwen2.5 3B", "large"},
        {"/models/Qwen3-0.6B-Q8_0.gguf", "Qwen3 0.6B", "qwen3-0.6b"},
        {"/models/Qwen3-1.7B-Q8_0.gguf", "Qwen3 1.7B", "qwen3-1.7b"},
        {"/models/Qwen3-14B-Q4_K_M.gguf", "Qwen3 14B", "qwen3-14b"},
        {"/models/Qwen3-30B-A3B-Q4_K_M.gguf", "Qwen3 30B-A3B", "qwen3-30b-a3b"},
    };
    for (const auto &t : tiers) {
        std::string label, word;
        if (!model_tier(t.file, label, word) || label != t.label || word != t.word)
            return failed(t.file, 0);
        const std::string empty = no_model_yet(label, word);
        if (empty.size() > 2 * 84 || empty.find("qwen get " + word) == std::string::npos)
            return failed("the empty state is one short line with its action", 0);
        if (label.size() + 12 + 30 > 84) return failed("the header fits 84 cells", 0);
    }
    std::string label, word;
    if (model_tier("/models/other.gguf", label, word)) return failed("a file no tier names", 0);
    printf("qwenwhy_check: %zu sentences, each its own\n", seen.size());
    return 0;
}
