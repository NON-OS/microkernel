/* qwenchat: why the personality would not open a model, in words. See qwenchat.h. */
#include "qwenchat.h"

#include <cerrno>
#include <cstdio>
#include <cstring>

/*
 * What the kernel keeps for itself on a live boot: a live stick's data
 * volume is held in memory and grows only while more than the larger of
 * 1 GiB and a quarter of memory is free (src/fs/cryptoblock/ram.rs), so a
 * session can hold at most the rest. -1 when the total is unknown.
 */
long long session_hold(long long total) {
    if (total <= 0) return -1;
    const long long keep = total / 4 > (1LL << 30) ? total / 4 : (1LL << 30);
    return total > keep ? total - keep : 0;
}

/* "1.83 GB", as the Store and qwen tiers write sizes. */
static std::string gb(long long b) {
    char s[32];
    snprintf(s, sizeof s, "%lld.%02lld GB", b / 1000000000, b / 10000000 % 100);
    return s;
}

/* " (its files are X; this session can hold at most Y in memory)", or what is known of it. */
static std::string sizes(const Room &r) {
    if (r.files <= 0 && r.hold < 0) return "";
    std::string s = " (";
    if (r.files > 0) s += "its files are " + gb(r.files) + ", and running it takes about as much again";
    if (r.files > 0 && r.hold >= 0) s += "; ";
    if (r.hold >= 0) s += "this session can hold at most " + gb(r.hold) + " in memory";
    return s + ")";
}

/*
 * One sentence for each way bringing a pinned model onto the data volume,
 * or opening it there, is refused: the errno the personality passes on
 * from the kernel's import (src/syscall/microkernel/data/errno.rs) or its
 * own refusal. Each says what happened and what to do about it, and no two
 * read alike. On an installed NONOS the volume is on its disk; on a live
 * boot it is in memory for this session, gone at power off, and the import
 * of a model nobody fetched is ENOENT, as on a disk. ENODEV is a boot with
 * no NONOS disk at all. The caller adds that nothing left the machine.
 */
std::string why_refused(int err, const char *name, const Room &r) {
    const char *f = nullptr;
    std::string tail;
    switch (err) {
    case ENOENT:
        f = "The model %s is not on this machine yet. Get it from the Store, or with qwen get in the "
            "Terminal: it is downloaded and checked against its signed SHA-256 pin. An installed NONOS "
            "keeps it on its data volume; a live boot holds it in memory for this session, gone at power off.";
        break;
    case ENODEV:
        f = "This boot has no NONOS disk, so %s has nowhere to be kept, not even in memory. Start from a "
            "NONOS stick or an installed NONOS.";
        break;
    case EIO:
        f = "The data volume, or the disk under it, failed while %s was read or brought in, so it was "
            "not opened. Check the disk, then try again.";
        break;
    case ENOSPC:
        f = "The data volume has no room left for %s";
        tail = sizes(r) + ". On a live boot the volume is in memory; choose a smaller tier, or free room.";
        break;
    case ENOMEM:
        f = "This session has too little free memory to hold %s";
        tail = sizes(r) + ". Close other programs, or choose a smaller tier.";
        break;
    case EBADMSG:
        f = "%s is not the file its signed SHA-256 pin names, so none of it was kept. Get it again from "
            "the Store or with qwen get.";
        break;
    case EEXIST:
        f = "Another file holds the name %s on the data volume, and no record of the signed pin vouches "
            "for it, so it was not opened.";
        break;
    case EACCES:
        f = "%s was not opened: the data volume is locked until its passphrase is given, or this "
            "program held a network connection, and a model is never opened by one.";
        break;
    default:
        return "";
    }
    char line[512];
    snprintf(line, sizeof line, f, name);
    return line + tail;
}

/*
 * Each tier by the start of its files' names, as the Linux personality pins
 * them (userland/capsule_linux/src/linux/file/models/pinned_*.rs), with the
 * label setup and Settings show and the word qwen takes. A prefix ends where
 * no other tier's name could go on, so none is taken for another.
 */
static const struct { const char *prefix, *label, *word; } TIERS[] = {
    {"qwen2.5-0.5b-", "Qwen2.5 0.5B", "small"},
    {"qwen2.5-1.5b-", "Qwen2.5 1.5B", "medium"},
    {"qwen2.5-3b-", "Qwen2.5 3B", "large"},
    {"qwen2.5-7b-", "Qwen2.5 7B", "xlarge"},
    {"qwen2.5-14b-", "Qwen2.5 14B", "xxl"},
    {"qwen2.5-32b-", "Qwen2.5 32B", "max"},
    {"Qwen3-0.6B-", "Qwen3 0.6B", "qwen3-0.6b"},
    {"Qwen3-1.7B-", "Qwen3 1.7B", "qwen3-1.7b"},
    {"Qwen3-4B-", "Qwen3 4B", "qwen3-4b"},
    {"Qwen3-8B-", "Qwen3 8B", "qwen3-8b"},
    {"Qwen3-14B-", "Qwen3 14B", "qwen3-14b"},
    {"Qwen3-30B-A3B-", "Qwen3 30B-A3B", "qwen3-30b-a3b"},
    {"Qwen3-32B-", "Qwen3 32B", "qwen3-32b"},
    {"qwen2.5-coder-1.5b-", "Qwen2.5 Coder 1.5B", "coder-1.5b"},
    {"qwen2.5-coder-7b-", "Qwen2.5 Coder 7B", "coder-7b"},
    {"qwen2.5-coder-14b-", "Qwen2.5 Coder 14B", "coder-14b"},
    {"qwen2.5-coder-32b-", "Qwen2.5 Coder 32B", "coder-32b"},
};

bool model_tier(const std::string &file, std::string &label, std::string &word) {
    const std::string name = file.substr(file.rfind('/') + 1);
    for (const auto &t : TIERS)
        if (name.compare(0, strlen(t.prefix), t.prefix) == 0) return label = t.label, word = t.word, true;
    return false;
}

std::string no_model_yet(const std::string &label, const std::string &word) {
    return "No model yet. Install " + label + " from the Store, or type in the Terminal: qwen get " + word +
           ". Nothing leaves this machine.";
}
