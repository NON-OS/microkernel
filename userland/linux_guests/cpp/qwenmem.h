/*
 * Whether a model fits before it is loaded. The weights are read whole
 * into memory, so they take what their files take, every part of a split
 * model; the KV cache takes its positions at two bytes a value; and the
 * graph a turn runs on takes a margin besides. All of it is compared with
 * the free memory sysinfo reports, and a model that would not fit is not
 * started, instead of failing half way through its weights.
 */
#pragma once
#include <string>

struct MemPlan {
    long long weights = 0; /* the model's files, all their parts */
    long long need = 0;    /* the weights, the KV cache and the margin */
    long long free = -1;   /* sysinfo's free memory; -1 when it would not say */
    int n_ctx = 0;         /* KV positions planned; 0 when the file said nothing */
    int err = 0;           /* errno from sysinfo when free is -1 */
};

/*
 * Plans a load of `model` with a KV cache of n_ctx positions, or when
 * n_ctx is 0 the model's own up to 4096, then 2048 if 4096 would not fit.
 * n_ubatch is the most tokens one graph runs. False only when the plan is
 * known not to fit; an unreadable file is left for the load to refuse.
 */
bool mem_plan(const char *model, int n_ctx, int n_ubatch, MemPlan &p);
/* One line for a person: the model, the bytes it needs, the bytes free. */
std::string mem_short(const char *model, const MemPlan &p);
/* One line for when sysinfo would not say, and the load goes ahead. */
std::string mem_unknown(const char *model, const MemPlan &p);
