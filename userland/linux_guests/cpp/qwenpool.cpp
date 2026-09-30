/* The Qwen guests' compute threads. See qwenpool.h. */
#include "qwenpool.h"

#include <unistd.h>

#include "ggml-cpu.h"

/*
 * sysconf(_SC_NPROCESSORS_ONLN) counts the bits sched_getaffinity sets, so
 * this is what the system says at run time, never a number built in.
 */
int cpus_online() {
    long n = sysconf(_SC_NPROCESSORS_ONLN);
    return n > 0 ? (int)n : 1;
}

ggml_threadpool_t pool_open(llama_context *ctx, int threads) {
    ggml_threadpool_params p = ggml_threadpool_params_default(threads);
    /*
     * Poll 0: a worker with nothing to do waits on the pool's condition
     * variable at once instead of spinning first. Between two tokens the
     * main thread samples and prints; the workers should not burn the CPUs
     * it and the rest of the machine could use.
     */
    p.poll = 0;
    ggml_threadpool_t pool = ggml_threadpool_new(&p);
    /* One pool for both: a second would be paused and resumed each turn. */
    if (pool) llama_attach_threadpool(ctx, pool, nullptr);
    return pool;
}

void pool_close(ggml_threadpool_t pool) {
    if (pool) ggml_threadpool_free(pool);
}
