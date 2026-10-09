/*
 * A tier too large for this machine's memory, or for a live boot's
 * volume, offers the next one down: the largest pinned tier whose files
 * are clearly smaller, read from /models/tiers, which the personality
 * writes from its signed pins (capsule_linux file/models/catalog.rs).
 */
#pragma once
#include <string>
#include <vector>

#include "qwenchat.h"

struct Tier {
    std::string word; /* what qwen and the Store take */
    std::string file; /* its first file, which names its label */
    long long bytes = 0; /* all its files */
};
/* /models/tiers, "word file bytes sha256" a line, summed by tier in order. */
std::vector<Tier> tiers_from(const std::string &text);
/*
 * The largest tier at least a twentieth under `bytes`, and under it by
 * `short_by` too when that is known (above zero); nullptr if none. The
 * twentieth keeps a tier of the same size, Coder beside Instruct, from
 * being offered for the other.
 */
const Tier *smaller_than(const std::vector<Tier> &all, long long bytes, long long short_by);
/* The sentence offering `t`, or saying there is none, for the terminal or the window. */
std::string smaller_line(const Tier *t, bool window);
/* After chat_open refused for want of memory or room: the offer; "" for any other errno. */
std::string offer_smaller(const ChatArgs &a, const Chat &c, bool window);
