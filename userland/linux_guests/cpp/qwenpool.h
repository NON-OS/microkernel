/*
 * One set of compute threads for the life of a llama.cpp context. Without
 * one, ggml starts and joins a fresh set of threads for every decode, which
 * for a reply is every token, and each of them spins for a while before it
 * sleeps. This pool is made once, its workers go straight to sleep on a
 * futex whenever there is no graph to run, and it is freed after the
 * context that ran on it.
 */
#pragma once
#include "llama.h"

/* The CPUs this process may run on, as sysconf says, at least 1. */
int cpus_online();
/* The pool, attached to ctx for the prompt and the reply; null if not made. */
ggml_threadpool_t pool_open(llama_context *ctx, int threads);
/* Stops and joins the workers. Only after llama_free of every user. */
void pool_close(ggml_threadpool_t pool);
