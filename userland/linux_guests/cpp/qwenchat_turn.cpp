/* qwenchat: one turn, the user's words in and the reply out. */
#include "qwenchat.h"

#include <algorithm>
#include <cstring>

/* The tokens the template's turn end takes, with room to spare. */
static const int CLOSE_ROOM = 8;

static bool fits(const ChatArgs &a, const Chat &c, size_t n) {
    return c.used + (int)n + a.n_reply + CLOSE_ROOM <= c.n_ctx;
}

static bool blank(const char *s, int n) {
    return n <= 0 || std::all_of(s, s + n, [](char ch) { return ch == ' ' || ch == '\n' || ch == '\r' || ch == '\t'; });
}

static void zero(std::vector<llama_token> &t) { std::fill(t.begin(), t.end(), 0); }

bool chat_turn(const ChatArgs &a, Chat &c, std::string &said, Put put, void *to, int &made) {
    made = 0;
    c.stopped = false;
    /* The line end the user typed is not part of what was said. */
    while (!said.empty() && (said.back() == '\n' || said.back() == '\r')) said.back() = 0, said.pop_back();
    std::string turn;
    size_t skip = 0;
    const bool built = prompt_turn(c, said, turn, skip);
    wipe(said);
    if (!built) return false;
    /* Qwen3 is asked not to think by an empty thought ahead of the answer. */
    const std::vector<llama_token> none, &pre = c.think ? none : c.no_thought;
    std::vector<llama_token> t = chat_tokens(c, turn, c.used ? skip : 0);
    if (!fits(a, c, t.size() + pre.size())) {
        /* Older turns are forgotten, not truncated mid-thought. */
        chat_reset(c);
        put(to, "(earlier turns forgotten to make room) ", 39, false);
        zero(t), t = chat_tokens(c, turn);
    }
    wipe(turn);
    if (!fits(a, c, t.size() + pre.size())) return zero(t), put(to, "(too long)", 10, false), true;
    const int start = c.used;
    bool ok = chat_feed(c, t.data(), t.size());
    zero(t);
    const int reply_at = c.used;
    ok = ok && chat_feed(c, pre.data(), pre.size());
    /* Thinking aloud is marked; the blank lines around it are not shown. */
    bool thought = false, lead = true, rethink = !pre.empty();
    std::vector<llama_token> kept; /* the answer without its thinking */
    kept.reserve((size_t)a.n_reply);
    char piece[256];
    for (int i = 0; ok && !chat_stop(c) && i < a.n_reply; i++) {
        llama_token id = llama_sampler_sample(c.smpl, c.ctx, -1);
        if (llama_vocab_is_eog(c.vocab, id)) break;
        const int n = llama_token_to_piece(c.vocab, id, piece, sizeof piece, 0, false);
        if (id == c.think_on || id == c.think_off) {
            thought = id == c.think_on, lead = true, rethink = true;
        } else if (!(lead && blank(piece, n))) {
            lead = false;
            if (n > 0) put(to, piece, (size_t)n, thought);
            if (!thought) kept.push_back(id);
        }
        made++;
        ok = chat_feed(c, &id, 1);
    }
    memset(piece, 0, sizeof piece);
    /* Stopped: what this turn put in the cache comes back out, the turns before stay. */
    if (c.stopped) return zero(kept), chat_rewind(c, start, {});
    /*
     * Qwen3's template keeps no thinking in the turns before the last, and
     * a model shown its old thoughts loses its way in the next: the reply
     * is taken back out of the cache and the answer alone put in.
     */
    if (ok && rethink) ok = chat_rewind(c, reply_at, kept);
    zero(kept);
    /* The turn is closed in the cache as the template closes it. */
    std::vector<llama_token> end = chat_tokens(c, c.close);
    return ok && chat_feed(c, end.data(), end.size());
}
