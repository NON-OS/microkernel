/* qwencheck: greedy generation with llama.cpp. See qwencheck.h. */
#include "qwencheck.h"

#include <chrono>
#include "llama.h"

static void quiet(enum ggml_log_level, const char *, void *) {}

static double now_s() {
    using namespace std::chrono;
    return duration<double>(steady_clock::now().time_since_epoch()).count();
}

/* llama.cpp's own log may quote a prompt: it is dropped, never printed. */
bool generate(const Args &a, const std::string &prompt, Run &r) {
    llama_log_set(quiet, nullptr);
    const double t0 = now_s();
    llama_backend_init();
    llama_model_params mp = llama_model_default_params();
    mp.n_gpu_layers = 0;
    mp.load_mode = LLAMA_LOAD_MODE_NONE; /* read(), never a file mapping */
    mp.lazy_mode = LLAMA_LAZY_MODE_OFF;
    llama_model *model = llama_model_load_from_file(a.model.c_str(), mp);
    if (!model) return false;
    const llama_vocab *vocab = llama_model_get_vocab(model);
    const int n = -llama_tokenize(vocab, prompt.c_str(), prompt.size(), nullptr, 0, true, true);
    if (n <= 0 || n > 4096) return false;
    std::vector<llama_token> toks(n);
    if (llama_tokenize(vocab, prompt.c_str(), prompt.size(), toks.data(), n, true, true) < 0) return false;
    r.prompt_tokens = n;
    llama_context_params cp = llama_context_default_params();
    cp.n_ctx = n + a.n_predict; /* the KV cache holds this prompt and reply, no more */
    cp.n_batch = n;
    cp.n_threads = cp.n_threads_batch = a.threads;
    cp.no_perf = true;
    llama_context *ctx = llama_init_from_model(model, cp);
    if (!ctx) return false;
    llama_sampler *smpl = llama_sampler_chain_init(llama_sampler_chain_default_params());
    llama_sampler_chain_add(smpl, llama_sampler_init_greedy());
    const double t_loaded = now_s();
    double t_first = t_loaded;
    llama_batch batch = llama_batch_get_one(toks.data(), toks.size());
    llama_token id;
    for (int i = 0; i < a.n_predict; i++) {
        if (llama_decode(ctx, batch)) return false;
        if (i > 0) r.decodes++;
        id = llama_sampler_sample(smpl, ctx, -1);
        if (i == 0) t_first = now_s();
        if (llama_vocab_is_eog(vocab, id)) break;
        r.ids.push_back(id);
        batch = llama_batch_get_one(&id, 1);
    }
    const double t_end = now_s();
    r.load_s = t_loaded - t0;
    r.ttft_s = t_first - t_loaded;
    r.decode_s = t_end - t_first;
    llama_sampler_free(smpl);
    llama_free(ctx); /* the KV cache and its prompt go with the context */
    llama_model_free(model);
    llama_backend_free();
    return true;
}
