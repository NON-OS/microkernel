/* The Qwen guests' memory check: what a model file says. See qwenmem.h. */
#include "qwenmem_file.h"

#include <cstdlib>
#include <cstring>
#include <sys/stat.h>

#include "gguf.h"
#include "llama.h"

static long long bytes_of(const char *path) {
    struct stat st;
    return stat(path, &st) == 0 ? (long long)st.st_size : 0;
}

/* "<prefix>-0000N-of-0000M.gguf" is one of M parts, and all M are read. */
long long model_bytes(const char *model) {
    const char *of = strstr(model, "-of-");
    for (const char *p = of; p; p = strstr(p + 1, "-of-")) of = p;
    const int parts = of ? atoi(of + 4) : 0;
    char prefix[512], part[512];
    for (int n = 0; n < parts && n < 1000; n++) {
        if (llama_split_prefix(prefix, sizeof prefix, model, n, parts) <= 0) continue;
        long long sum = 0;
        for (int i = 0; i < parts; i++)
            if (llama_split_path(part, sizeof part, prefix, i, parts) > 0) sum += bytes_of(part);
        return sum;
    }
    return bytes_of(model);
}

/* A whole-number key of the model's architecture, or 0. */
static long long key(const gguf_context *g, const std::string &arch, const char *name) {
    const int64_t k = gguf_find_key(g, (arch + name).c_str());
    if (k < 0) return 0;
    switch (gguf_get_kv_type(g, k)) {
    case GGUF_TYPE_UINT32: return gguf_get_val_u32(g, k);
    case GGUF_TYPE_INT32: return gguf_get_val_i32(g, k);
    case GGUF_TYPE_UINT64: return (long long)gguf_get_val_u64(g, k);
    default: return 0; /* a value per layer, which no Qwen model has */
    }
}

/* Only the header and the keys are read, never a tensor. */
bool model_shape(const char *model, Shape &s) {
    gguf_init_params gp = {true, nullptr};
    gguf_context *g = gguf_init_from_file(model, gp);
    if (!g) return false;
    const int64_t a = gguf_find_key(g, "general.architecture");
    std::string arch = a >= 0 && gguf_get_kv_type(g, a) == GGUF_TYPE_STRING ? gguf_get_val_str(g, a) : "";
    arch += '.';
    s.layers = key(g, arch, "block_count");
    s.embd = key(g, arch, "embedding_length");
    s.trained = key(g, arch, "context_length");
    const long long heads = key(g, arch, "attention.head_count");
    s.kv_heads = key(g, arch, "attention.head_count_kv");
    if (!s.kv_heads) s.kv_heads = heads;
    /* Qwen3 says its head size; Qwen2.5's is the width over the heads. */
    s.k_dim = key(g, arch, "attention.key_length");
    if (!s.k_dim && heads) s.k_dim = s.embd / heads;
    s.v_dim = key(g, arch, "attention.value_length");
    if (!s.v_dim) s.v_dim = s.k_dim;
    gguf_free(g);
    return s.layers > 0 && s.kv_heads > 0 && s.k_dim > 0 && s.embd > 0;
}
