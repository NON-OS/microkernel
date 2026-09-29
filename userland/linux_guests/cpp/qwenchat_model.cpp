/* qwenchat: the model, its context and its sampler. See qwenchat.h. */
#include "qwenchat.h"

#include <cstring>
#include <thread>

static void quiet(enum ggml_log_level, const char *, void *) {}

void wipe(std::string &s) {
    if (!s.empty()) memset(&s[0], 0, s.size());
    s.clear();
}

bool chat_open(const ChatArgs &a, Chat &c) {
    /* llama.cpp's own log may quote what was said: it is dropped. */
    llama_log_set(quiet, nullptr);
    llama_backend_init();
    llama_model_params mp = llama_model_default_params();
    mp.n_gpu_layers = 0;
    mp.load_mode = LLAMA_LOAD_MODE_NONE; /* read(), never a file mapping */
    mp.lazy_mode = LLAMA_LAZY_MODE_OFF;
    c.model = llama_model_load_from_file(a.model.c_str(), mp);
    if (!c.model) return false;
    c.vocab = llama_model_get_vocab(c.model);
    llama_context_params cp = llama_context_default_params();
    cp.n_ctx = a.n_ctx;
    cp.n_batch = a.n_ctx;
    unsigned cpus = std::thread::hardware_concurrency();
    int threads = a.threads ? a.threads : (int)(cpus ? (cpus < 8 ? cpus : 8) : 1);
    cp.n_threads = cp.n_threads_batch = threads;
    cp.no_perf = true;
    c.ctx = llama_init_from_model(c.model, cp);
    if (!c.ctx) return false;
    c.smpl = llama_sampler_chain_init(llama_sampler_chain_default_params());
    if (a.temp == 0.0f) {
        llama_sampler_chain_add(c.smpl, llama_sampler_init_greedy());
    } else {
        llama_sampler_chain_add(c.smpl, llama_sampler_init_top_k(40));
        llama_sampler_chain_add(c.smpl, llama_sampler_init_top_p(0.9f, 1));
        llama_sampler_chain_add(c.smpl, llama_sampler_init_temp(a.temp));
        llama_sampler_chain_add(c.smpl, llama_sampler_init_dist(a.seed));
    }
    return true;
}

/* The KV cache is cleared with its data, not only marked free. */
void chat_reset(Chat &c) {
    llama_memory_clear(llama_get_memory(c.ctx), true);
    llama_sampler_reset(c.smpl);
    c.used = 0;
}

void chat_close(Chat &c) {
    if (c.ctx) chat_reset(c);
    if (c.smpl) llama_sampler_free(c.smpl);
    if (c.ctx) llama_free(c.ctx);
    if (c.model) llama_model_free(c.model);
    llama_backend_free();
    c = Chat();
}
