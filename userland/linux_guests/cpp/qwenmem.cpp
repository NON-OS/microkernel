/* The Qwen guests' memory check. See qwenmem.h. */
#include "qwenmem.h"
#include "qwenmem_file.h"

#include <cerrno>
#include <cstdio>
#include <cstring>
#include <sys/sysinfo.h>

bool mem_plan(const char *model, int n_ctx, int n_ubatch, MemPlan &p) {
    p = MemPlan();
    p.weights = model_bytes(model);
    struct sysinfo si;
    if (sysinfo(&si) == 0) p.free = (long long)(si.freeram + si.bufferram) * (si.mem_unit ? si.mem_unit : 1);
    else p.err = errno;
    Shape s;
    if (!model_shape(model, s)) return p.need = p.weights, true;
    /* A key and a value for every layer, KV head and head element: f16. */
    const long long per_pos = s.layers * s.kv_heads * (s.k_dim + s.v_dim) * 2;
    /*
     * The graph's buffer is some tens of MiB on the small models, with one
     * row of logits out; it grows with the width and the tokens a graph
     * runs. The rest is the vocabulary, the threads and the C++ heap.
     */
    const long long margin = (64LL << 20) + (long long)n_ubatch * s.embd * 64;
    int ctx = n_ctx ? n_ctx : (s.trained > 0 && s.trained < 4096 ? (int)s.trained : 4096);
    for (;;) {
        p.n_ctx = ctx;
        p.need = p.weights + per_pos * ctx + margin;
        if (p.free < 0 || p.need <= p.free) return true;
        if (n_ctx || ctx <= 2048) return false;
        ctx = 2048;
    }
}

static const char *base(const char *path) {
    const char *slash = strrchr(path, '/');
    return slash ? slash + 1 : path;
}

std::string mem_short(const char *model, const MemPlan &p) {
    char line[256];
    snprintf(line, sizeof line, "Not enough memory for %s: it needs %lld bytes and %lld are free.",
             base(model), p.need, p.free);
    return line;
}

std::string mem_unknown(const char *model, const MemPlan &p) {
    char line[256];
    snprintf(line, sizeof line, "Free memory is unknown (sysinfo failed, errno %d); loading %s anyway.",
             p.err, base(model));
    return line;
}
