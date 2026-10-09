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
    std::string why; /* llama's own reason, when it refused the model */
    MemPlan mem;     /* what the model was judged to need, and what was free */
    /*
     * Asked now and then while the model loads and while a turn computes,
     * on this thread; true stops the load or the turn. The window answers
     * its compositor from here, so it stays alive however long they take.
     */
    bool (*stop)(void *to) = nullptr;
    void *stop_to = nullptr;
    bool stopped = false; /* the person stopped the load or the last turn */
    /*
     * Told before each part of the model is opened the first time, which
     * may bring it onto the data volume: part, of how many, and its size.
     */
    void (*step)(void *to, int part, int parts, long long bytes) = nullptr;
    void *step_to = nullptr;
    float loaded = 0;     /* how much of the model is in, 0 to 1 */
    unsigned asks = 0;    /* stop is asked once in this many chances */
};

/* Whether the person has stopped it; stop is asked at most every few calls. */
bool chat_stop(Chat &c);
bool chat_args(int argc, char **argv, ChatArgs &a);
bool chat_open(const ChatArgs &a, Chat &c);
/*
 * Every part of the model opened once, in order, before anything reads it:
 * the personality brings a pinned part onto the data volume then, checked
 * against its signed pin. False with c.err at the first refusal, which is
 * final: nothing opens the model again in this run.
 */
bool chat_import(Chat &c, const std::string &model);
/* What a refusal for want of room says: the model's files, and what a live
 * session can hold in memory; either is unknown at or below zero, -1. */
struct Room {
    long long files = 0;
    long long hold = -1;
};
/* All of `total` bytes of memory less what a live boot keeps for the system. */
long long session_hold(long long total);
/* The sentence for a model the personality refused with `err`; "" if none. */
std::string why_refused(int err, const char *name, const Room &r);
/*
 * The tier a pinned model file is, by its name: the label a person reads
 * ("Qwen3 0.6B") and the word qwen and the Store take ("qwen3-0.6b").
 * False for a file no tier names, which is then shown by its name.
 */
bool model_tier(const std::string &file, std::string &label, std::string &word);
/* What the window says when the tier's model is not on this machine yet: one line, the action. */
std::string no_model_yet(const std::string &label, const std::string &word);
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
/*
 * One user turn in, the reply streamed to `put`. False on a fault. When
 * the person stops it, c.stopped is set and the turn is taken back out of
 * the cache, as if it had never been said.
 */
bool chat_turn(const ChatArgs &a, Chat &c, std::string &said, Put put, void *to, int &made);
/* The terminal: a prompt, the reply as it comes, a line of numbers. */
int chat_terminal(const ChatArgs &a);
/* The window: the same conversation, drawn and typed into. */
int chat_window(const ChatArgs &a);
void chat_reset(Chat &c);
void chat_close(Chat &c);
void wipe(std::string &s);
