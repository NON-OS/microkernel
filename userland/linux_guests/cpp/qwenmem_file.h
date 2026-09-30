/* What a model file says about the memory it takes. See qwenmem.h. */
#pragma once
#include <string>

struct Shape {
    long long layers = 0, embd = 0, trained = 0;
    long long kv_heads = 0, k_dim = 0, v_dim = 0;
};

/* The bytes of the model's files, every part of a split model. */
long long model_bytes(const char *model);
/* The model's layers and heads from its GGUF keys; false if unreadable. */
bool model_shape(const char *model, Shape &s);
