/* qwenchat: text to tokens, and tokens into the KV cache. See qwenchat.h. */
#include "qwenchat.h"

std::vector<llama_token> chat_tokens(const Chat &c, const std::string &s, size_t from) {
    const char *p = s.data() + from;
    const int32_t len = (int32_t)(s.size() - from);
    int n = -llama_tokenize(c.vocab, p, len, nullptr, 0, false, true);
    std::vector<llama_token> t(n > 0 ? n : 0);
    if (n > 0) llama_tokenize(c.vocab, p, len, t.data(), n, false, true);
    return t;
}

/* All of t in one decode: a turn goes in as one batch, however long. */
bool chat_feed(Chat &c, const llama_token *t, size_t n) {
    if (!n) return true;
    if (llama_decode(c.ctx, llama_batch_get_one(const_cast<llama_token *>(t), (int32_t)n))) return false;
    c.used += (int)n;
    return true;
}

bool chat_rewind(Chat &c, int at, const std::vector<llama_token> &t) {
    if (!llama_memory_seq_rm(llama_get_memory(c.ctx), 0, at, -1)) return false;
    c.used = at;
    return chat_feed(c, t.data(), t.size());
}
