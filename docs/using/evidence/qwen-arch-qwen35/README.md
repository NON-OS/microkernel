# qwen35 on NONOS at 1d2f92e34: qwen3.5-0.8b answered, then its turn faulted

The same image as ../qwen-arch and ../qwen-arch-qwen3 (models/qwen36-2 at
1d2f92e34, qemu-dev, tools/nonos-dev-image's throwaway keys; .keys/ never
read). QEMU without KVM, 4 threads, `--mem=12G`, IOMMU, a fresh disk with
qwen3.5-0.8b laid on it. The steps are in drive.steps.

## What booted

qwen35-40.png: `qwen qwen3.5-0.8b What is the capital of France? Answer in
one word.` loaded Qwen3.5-0.8B-Q8_0.gguf, "qwen: verified" its pin, and
answered "PARIS": 2 tokens, 90.1 s for the turn
("load_ms=1109175 ... ttft_ms=80400"). Then "The model stopped with a
fault." and "[LINUX] process 2 exited, status 1".

## Why it faulted, and the fix

qwenchat takes a Qwen3 reply's empty thought back out of the cache after
each turn (chat_rewind, llama_memory_seq_rm from a position on). Qwen3.5 is
a hybrid with recurrent layers, whose cache cannot take back its end, so
that call failed and the turn ended as a fault. The same happens on the
host with the same file and llama.cpp 00af635.

The fix (the next commit on this branch) marks a model with recurrent
layers (llama_model_is_recurrent or llama_model_is_hybrid) as not
rewinding. Its turns stay in the cache as they were said, and a stopped
turn forgets the conversation rather than rewinding. Host-tested: two turns
of qwen3.5-0.8b, qwen3-0.6b and small each answer "Paris" then "Rome", with
no fault.

So this boot shows the qwen35 architecture answering on NONOS. Its tiers
stay unmarked (PROVEN) until a boot of the fixed qwenchat answers without a
fault: ../qwen-arch-qwen35-fixed.
