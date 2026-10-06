/* qwenchat: forgetting, a turn's text, the conversation, or everything. */
#include "qwenchat.h"
#include "qwenpool.h"

#include <cstring>

void wipe(std::string &s) {
    if (!s.empty()) memset(&s[0], 0, s.size());
    s.clear();
}

/* The KV cache is cleared with its data, not only marked free. */
void chat_reset(Chat &c) {
    llama_memory_clear(llama_get_memory(c.ctx), true);
    llama_sampler_reset(c.smpl);
    c.used = 0;
}

void chat_close(Chat &c) {
    if (c.ctx && c.smpl) chat_reset(c);
    if (c.smpl) llama_sampler_free(c.smpl);
    if (c.ctx) llama_free(c.ctx);
    /* The workers go after the context, the last thing that could wake them. */
    pool_close(c.pool);
    if (c.model) llama_model_free(c.model);
    llama_backend_free();
    /* The log keeps no pointer into a chat that is gone. */
    llama_log_set([](enum ggml_log_level, const char *, void *) {}, nullptr);
    c = Chat();
}
