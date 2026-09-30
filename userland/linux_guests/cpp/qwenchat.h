/*
 * qwenchat: a conversation with a pinned Qwen model, on the terminal.
 * The model is read with read(), never mapped, from /models, which the
 * personality serves only to a family with no internet socket. Nothing
 * said either way is logged: the conversation lives in this process and
 * its KV cache, both wiped when a turn ends or the process exits.
 */
#pragma once
#include <string>
#include <vector>

#include "llama.h"
#include "qwenmem.h"

struct ChatArgs {
    std::string model = "/models/qwen2.5-0.5b-instruct-q4_k_m.gguf";
    int threads = 0; /* 0: one per online CPU, at most 8 */
    int n_ctx = 0;   /* 0: the model's own up to 4096, 2048 if memory is short */
    int n_reply = 512;
    float temp = 0.7f; /* 0: greedy, the most likely token and nothing else */
    float repeat = 1.1f; /* penalty on the last 64 tokens it said; 1: none */
    unsigned seed = LLAMA_DEFAULT_SEED;
    bool window = false;
};

struct Chat {
    llama_model *model = nullptr;
    llama_context *ctx = nullptr;
    const llama_vocab *vocab = nullptr;
    llama_sampler *smpl = nullptr;
    ggml_threadpool_t pool = nullptr; /* the context's threads, made once */
    const char *tmpl = nullptr; /* the model's chat template, or "chatml" */
    std::string close;          /* what ends the assistant's turn in it */
    llama_token think_on = LLAMA_TOKEN_NULL, think_off = LLAMA_TOKEN_NULL;
    std::vector<llama_token> no_thought; /* the empty thought that skips it */
    bool can_think = false; /* the template can ask for no thinking (Qwen3) */
    bool think = false;     /* think aloud before answering; off at first */
    int threads = 0; /* what the context runs with */
    int n_ctx = 0;   /* positions the KV cache has */
    int used = 0;    /* positions of the KV cache in use */
    int err = 0;     /* errno when the model or its context would not open */
    MemPlan mem;     /* what the model was judged to need, and what was free */
};

bool chat_args(int argc, char **argv, ChatArgs &a);
bool chat_open(const ChatArgs &a, Chat &c);
/* The template, its turn end and the thinking tags, once the model is open. */
void prompt_setup(Chat &c);
/* The user's turn as the template has it; from `skip` on, it follows on. */
bool prompt_turn(const Chat &c, const std::string &said, std::string &out, size_t &skip);
/* The tokens of s from byte `from` on, special tokens read as such. */
std::vector<llama_token> chat_tokens(const Chat &c, const std::string &s, size_t from = 0);
/* n tokens into the KV cache in one decode; false on a fault. */
bool chat_feed(Chat &c, const llama_token *t, size_t n);
/* The cache from position `at` on taken back, and t put in its place. */
bool chat_rewind(Chat &c, int at, const std::vector<llama_token> &t);
/* Why chat_open failed, in one line for a person. */
std::string chat_failure(const ChatArgs &a, const Chat &c);
/* Where a reply goes, a piece at a time; `thought` while thinking aloud. */
typedef void (*Put)(void *to, const char *piece, size_t n, bool thought);
/* One user turn in, the reply streamed to `put`. False on a fault. */
bool chat_turn(const ChatArgs &a, Chat &c, std::string &said, Put put, void *to, int &made);
/* The terminal: a prompt, the reply as it comes, a line of numbers. */
int chat_terminal(const ChatArgs &a);
/* The window: the same conversation, drawn and typed into. */
int chat_window(const ChatArgs &a);
void chat_reset(Chat &c);
void chat_close(Chat &c);
void wipe(std::string &s);
