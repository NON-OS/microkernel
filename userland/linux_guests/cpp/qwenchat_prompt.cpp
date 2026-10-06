/* qwenchat: the model's own chat template, and thinking aloud. */
#include "qwenchat.h"

#include <cstring>

static const char *SYSTEM = "You are Qwen, a helpful assistant. You run locally on NONOS, with no network.";
/* What Qwen3's template adds when asked not to think: an empty thought. */
static const char *NO_THOUGHT = "<think>\n\n</think>\n\n";

/*
 * Formatted in one buffer big enough for the answer, so no copy of the
 * words is left behind by a reallocation. Empty on failure.
 */
static std::string apply(const char *tmpl, const llama_chat_message *m, size_t n, bool open) {
    size_t chars = 0;
    for (size_t i = 0; i < n; i++) chars += strlen(m[i].role) + strlen(m[i].content);
    std::string out(chars * 2 + 512, '\0');
    int32_t len = llama_chat_apply_template(tmpl, m, n, open, &out[0], (int32_t)out.size());
    if (len > (int32_t)out.size()) {
        wipe(out);
        out.assign((size_t)len + 64, '\0');
        len = llama_chat_apply_template(tmpl, m, n, open, &out[0], (int32_t)out.size());
    }
    if (len < 0 || len > (int32_t)out.size()) return wipe(out), out;
    out.resize((size_t)len);
    return out;
}

/* The one token `s` is in this vocabulary, or none. */
static llama_token single(const Chat &c, const char *s) {
    llama_token t[2];
    return llama_tokenize(c.vocab, s, (int32_t)strlen(s), t, 2, false, true) == 1 ? t[0] : LLAMA_TOKEN_NULL;
}

void prompt_setup(Chat &c) {
    const char *t = llama_model_chat_template(c.model, nullptr);
    llama_chat_message m[3] = {{"system", SYSTEM}, {"user", "u"}, {"assistant", "a"}};
    /*
     * A template llama.cpp does not know, or one whose system turn is not
     * where every conversation starts, is read as ChatML, Qwen's own.
     */
    const std::string sys = t ? apply(t, m, 1, false) : "", first = t ? apply(t, m, 2, true) : "";
    c.tmpl = !sys.empty() && first.size() > sys.size() && !first.compare(0, sys.size(), sys) ? t : "chatml";
    c.can_think = t && strstr(t, "enable_thinking") != nullptr;
    c.think_on = single(c, "<think>"), c.think_off = single(c, "</think>");
    if (c.can_think) c.no_thought = chat_tokens(c, NO_THOUGHT);
    /* The turn end is what follows the reply when a whole turn is shown. */
    const std::string open = apply(c.tmpl, m, 2, true), all = apply(c.tmpl, m, 3, false);
    const bool follows = !open.empty() && all.size() > open.size() && !all.compare(0, open.size(), open)
        && all[open.size()] == 'a';
    c.close = follows ? all.substr(open.size() + 1) : "<|im_end|>\n";
    if (c.close.empty()) c.close = "<|im_end|>\n";
}

bool prompt_turn(const Chat &c, const std::string &said, std::string &out, size_t &skip) {
    llama_chat_message m[2] = {{"system", SYSTEM}, {"user", said.c_str()}};
    /* The system turn alone, which the conversation already holds. */
    skip = apply(c.tmpl, m, 1, false).size();
    out = apply(c.tmpl, m, 2, true);
    if (out.size() <= skip) return wipe(out), false;
    return true;
}
