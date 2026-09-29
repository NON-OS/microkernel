/*
 * qwencheck: run the model the personality pins, greedy, and check the
 * tokens against the host's. The model is read with read(), never mapped.
 * Nothing the model is shown or says is printed: the verdict is the exit
 * status, and the numbers go to /dev/nonos-metrics, which takes numbers.
 */
#pragma once
#include <string>
#include <vector>

struct Args {
    std::string model = "/models/qwen2.5-0.5b-instruct-q4_k_m.gguf";
    std::string prompt_file;
    std::vector<int> expect;
    int n_predict = 32;
    int threads = 1;
};

struct Run {
    std::vector<int> ids;
    int prompt_tokens = 0;
    double load_s = 0, ttft_s = 0, decode_s = 0;
    int decodes = 0;
    /* Which step failed, 0 for none, and errno at that moment. */
    int stage = 0, err = 0;
};

enum Stage { LOAD = 1, TOKENIZE, CONTEXT, DECODE };

enum Exit { MATCH = 0, FAILED = 1, MISMATCH = 3 };

bool parse_args(int argc, char **argv, Args &a);
bool generate(const Args &a, const std::string &prompt, Run &r);
void report(const Args &a, const Run &r, bool match);
