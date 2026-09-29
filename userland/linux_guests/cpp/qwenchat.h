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

struct ChatArgs {
    std::string model = "/models/qwen2.5-0.5b-instruct-q4_k_m.gguf";
    int threads = 1;
    int n_ctx = 4096;
    int n_reply = 512;
    float temp = 0.7f;
    unsigned seed = LLAMA_DEFAULT_SEED;
    bool window = false;
};

struct Chat {
    llama_model *model = nullptr;
    llama_context *ctx = nullptr;
    const llama_vocab *vocab = nullptr;
    llama_sampler *smpl = nullptr;
    int used = 0; /* positions of the KV cache in use */
};

bool chat_args(int argc, char **argv, ChatArgs &a);
bool chat_open(const ChatArgs &a, Chat &c);
/* Where a reply goes, a piece at a time as it is generated. */
typedef void (*Put)(void *to, const char *piece, size_t n);
/* One user turn in, the reply streamed to `put`. False on a fault. */
bool chat_turn(const ChatArgs &a, Chat &c, std::string &said, Put put, void *to, int &made);
/* The window: the same conversation, drawn and typed into. */
int chat_window(const ChatArgs &a);
void chat_reset(Chat &c);
void chat_close(Chat &c);
void wipe(std::string &s);
