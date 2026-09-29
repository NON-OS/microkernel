/* qwenchat: one turn, the user's words in and the reply out. */
#include "qwenchat.h"

#include <cstring>

static const char *SYSTEM = "<|im_start|>system\nYou are Qwen, a helpful assistant. You run "
                            "locally on NONOS, with no network.<|im_end|>\n";

static bool feed(Chat &c, std::vector<llama_token> &t) {
    if (t.empty()) return true;
    if (llama_decode(c.ctx, llama_batch_get_one(t.data(), (int32_t)t.size()))) return false;
    c.used += (int)t.size();
    return true;
}

static std::vector<llama_token> tokens(const Chat &c, const std::string &s) {
    int n = -llama_tokenize(c.vocab, s.data(), (int32_t)s.size(), nullptr, 0, false, true);
    std::vector<llama_token> t(n > 0 ? n : 0);
    if (n > 0) llama_tokenize(c.vocab, s.data(), (int32_t)s.size(), t.data(), n, false, true);
    return t;
}

bool chat_turn(const ChatArgs &a, Chat &c, std::string &said, Put put, void *to, int &made) {
    made = 0;
    std::string turn = "<|im_start|>user\n" + said + "<|im_end|>\n<|im_start|>assistant\n";
    wipe(said);
    std::vector<llama_token> t = tokens(c, (c.used ? "" : std::string(SYSTEM)) + turn);
    if (c.used + (int)t.size() + a.n_reply > a.n_ctx) {
        /* Older turns are forgotten, not truncated mid-thought. */
        chat_reset(c);
        put(to, "(earlier turns forgotten to make room) ", 39);
        t = tokens(c, std::string(SYSTEM) + turn);
    }
    wipe(turn);
    if (c.used + (int)t.size() + a.n_reply > a.n_ctx) return put(to, "(too long)", 10), true;
    bool ok = feed(c, t);
    char piece[256];
    for (int i = 0; ok && i < a.n_reply; i++) {
        llama_token id = llama_sampler_sample(c.smpl, c.ctx, -1);
        t.assign(1, id);
        if (llama_vocab_is_eog(c.vocab, id)) break;
        int n = llama_token_to_piece(c.vocab, id, piece, sizeof piece, 0, false);
        if (n > 0) put(to, piece, (size_t)n);
        made++;
        ok = feed(c, t);
    }
    memset(piece, 0, sizeof piece);
    /* The turn is closed in the cache as the template closes it. */
    std::vector<llama_token> end = tokens(c, "<|im_end|>\n");
    return ok && feed(c, end);
}
