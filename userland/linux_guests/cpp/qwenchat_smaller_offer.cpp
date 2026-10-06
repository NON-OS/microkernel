/* qwenchat: the next smaller tier offered after a refusal. See qwenchat_smaller.h. */
#include "qwenchat_smaller.h"
#include "qwenmem_file.h"

#include <cerrno>
#include <fstream>
#include <sstream>

std::string offer_smaller(const ChatArgs &a, const Chat &c, bool window) {
    if (c.err != ENOMEM && c.err != ENOSPC) return "";
    std::ifstream in("/models/tiers");
    std::stringstream text;
    text << in.rdbuf();
    const std::vector<Tier> all = tiers_from(text.str());
    /* A list it could not read offers nothing rather than saying none fits. */
    if (all.empty()) return "";
    /* This tier's files as the pins size them; the files here when no pin names it. */
    std::string label, word;
    long long bytes = model_bytes(a.model.c_str());
    if (model_tier(a.model, label, word))
        for (const Tier &t : all)
            if (t.word == word) bytes = t.bytes;
    /* The plan's shortfall is known; a refusal from the volume says none. */
    const long long short_by = c.err == ENOMEM && c.mem.free >= 0 ? c.mem.need - c.mem.free : 0;
    return smaller_line(smaller_than(all, bytes, short_by), window);
}
