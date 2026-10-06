/* qwenchat: the model, its context and its sampler. See qwenchat.h. */
#include "qwenchat.h"
#include "qwenmem_file.h"
#include "qwenpool.h"

#include <cerrno>
#include <cstdio>
#include <cstring>
#include <sys/sysinfo.h>

/*
 * llama says nothing on the console, but its first error is kept: a model
 * it refuses often leaves errno as some earlier call set it, and only its
 * own line says why. The first, since the lines after it only say that
 * the load failed.
 */
static void quiet(enum ggml_log_level level, const char *text, void *to) {
    if (level != GGML_LOG_LEVEL_ERROR || !to || !text || !((Chat *)to)->why.empty()) return;
    std::string &why = ((Chat *)to)->why;
    why.assign(text, strnlen(text, 200));
    while (!why.empty() && (why.back() == '\n' || why.back() == ' ')) why.pop_back();
}

/*
 * Qwen's own top-k, top-p and temperature, and a light penalty on what
 * it said in the last 64 tokens; the penalty sees only the 20 kept. At
 * temperature 0 the most likely of them is taken, nothing is drawn.
 */
static llama_sampler *sampler(const ChatArgs &a, const Chat &c) {
    llama_sampler *s = llama_sampler_chain_init(llama_sampler_chain_default_params());
    llama_sampler_chain_add(s, llama_sampler_init_top_k(20));
    const int32_t n_vocab = llama_vocab_n_tokens(c.vocab);
    if (a.repeat != 1.0f) llama_sampler_chain_add(s, llama_sampler_init_penalties(n_vocab, 64, a.repeat, 0.0f, 0.0f));
    if (a.temp == 0.0f) return llama_sampler_chain_add(s, llama_sampler_init_greedy()), s;
    llama_sampler_chain_add(s, llama_sampler_init_top_p(0.8f, 1));
    llama_sampler_chain_add(s, llama_sampler_init_temp(a.temp));
    llama_sampler_chain_add(s, llama_sampler_init_dist(a.seed));
    return s;
}

bool chat_stop(Chat &c) {
    /* The compute asks between every two nodes of the graph; a few hundred a token. */
    if (c.stopped) return true;
    if (!c.stop || c.asks++ % 32) return false;
    return c.stopped = c.stop(c.stop_to);
}

static bool loading(float done, void *to) {
    Chat &c = *(Chat *)to;
    c.loaded = done;
    /* Once a tensor: asked every time. */
    if (c.stop && !c.stopped) c.stopped = c.stop(c.stop_to);
    return !c.stopped;
}

static bool aborting(void *to) { return chat_stop(*(Chat *)to); }

/* A turn goes in with one decode, and runs as one graph up to this. */
static const int UBATCH = 512;

bool chat_open(const ChatArgs &a, Chat &c) {
    llama_log_set(quiet, &c);
    c.why.clear();
    /* A refused part stops here, before the plan or the loader open it again. */
    if (!chat_import(c, a.model)) return false;
    if (!mem_plan(a.model.c_str(), a.n_ctx, UBATCH, c.mem)) return c.err = ENOMEM, false;
    llama_backend_init();
    llama_model_params mp = llama_model_default_params();
    mp.n_gpu_layers = 0;
    /* read() straight into the weights' buffer: one copy, never mapped */
    mp.load_mode = LLAMA_LOAD_MODE_NONE;
    mp.lazy_mode = LLAMA_LAZY_MODE_OFF;
    mp.progress_callback = loading, mp.progress_callback_user_data = &c;
    c.stopped = false;
    errno = 0;
    c.model = llama_model_load_from_file(a.model.c_str(), mp);
    if (!c.model) return c.err = c.stopped ? ECANCELED : errno ? errno : EINVAL, false;
    c.vocab = llama_model_get_vocab(c.model);
    const int trained = llama_model_n_ctx_train(c.model);
    c.n_ctx = a.n_ctx ? a.n_ctx : c.mem.n_ctx ? c.mem.n_ctx : trained > 0 && trained < 4096 ? trained : 4096;
    if (a.n_reply > c.n_ctx / 2) return c.err = EINVAL, false;
    const int cpus = cpus_online();
    c.threads = a.threads ? a.threads : (cpus < 8 ? cpus : 8);
    llama_context_params cp = llama_context_default_params();
    cp.n_ctx = c.n_ctx;
    /*
     * Only the last position's logits are read, so the output buffer holds
     * one row of them, not one per token of the turn.
     */
    cp.n_batch = c.n_ctx, cp.n_ubatch = UBATCH, cp.n_outputs_max = 1;
    cp.n_threads = cp.n_threads_batch = c.threads;
    cp.no_perf = true;
    cp.abort_callback = aborting, cp.abort_callback_data = &c;
    errno = 0;
    c.ctx = llama_init_from_model(c.model, cp);
    if (!c.ctx) return c.err = errno ? errno : ENOMEM, false;
    c.pool = pool_open(c.ctx, c.threads);
    c.smpl = sampler(a, c);
    prompt_setup(c);
    return true;
}

std::string chat_failure(const ChatArgs &a, const Chat &c) {
    if (c.err == ENOMEM && c.mem.free >= 0 && c.mem.need > c.mem.free) return mem_short(a.model.c_str(), c.mem);
    const char *slash = strrchr(a.model.c_str(), '/');
    const char *name = slash ? slash + 1 : a.model.c_str();
    char line[512];
    /* The model's files as the personality sizes them (a pinned part not
     * brought in yet is its pinned size), and what a live session holds. */
    Room room;
    room.files = model_bytes(a.model.c_str());
    struct sysinfo si;
    if (sysinfo(&si) == 0) room.hold = session_hold((long long)si.totalram * (si.mem_unit ? si.mem_unit : 1));
    const std::string refused = why_refused(c.err, name, room);
    /* The personality's refusal in words, then llama's own reason if it gave one. */
    if (!refused.empty())
        return refused + (c.why.empty() ? "" : " (" + c.why + ")") + " Nothing left this machine.";
    else if (!c.why.empty())
        snprintf(line, sizeof line, "The model %s could not be opened: %s (errno %d). Nothing left this machine.",
                 name, c.why.c_str(), c.err);
    else
        snprintf(line, sizeof line, "The model %s could not be opened (errno %d). Nothing left this machine.",
                 name, c.err);
    return line;
}
