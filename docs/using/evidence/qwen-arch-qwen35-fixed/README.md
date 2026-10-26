# qwen35 on NONOS, fixed: qwen3.5-0.8b answers two turns, booted at 781f45ac1, 2026-10-26

models/qwen36-2 at 781f45ac1, which carries the fix for the fault in
../qwen-arch-qwen35: a model with recurrent layers never rewinds its cache.
A qemu-dev image made by tools/nonos-dev-image with its throwaway keys
(.keys/ never read). QEMU without KVM, 4 threads, `--mem=12G`, IOMMU, a
fresh disk with qwen3.5-0.8b laid on it. The steps are in drive.steps.

## What booted

qwen35-exit.png:

- `qwen qwen3.5-0.8b What is the capital of France? Answer in one word.`
  loaded Qwen3.5-0.8B-Q8_0.gguf and "qwen: verified" its pin
  (load_ms=1270402). It answered "Paris": 1 token, 95.1 s.
- The second turn, "And of Italy? One word.", answered "Milan": 2 tokens,
  51.8 s. No fault. The fix lets a second turn run on a hybrid model. The
  answer itself is wrong (Rome); a 0.8B model's answer is not a claim of
  quality.
- `/exit`: "The conversation was wiped from memory.",
  "[LINUX] process 2 exited, status 0".

Throughput is not measured: this is emulation. This is the qwen35
architecture booted to answers without a fault, the evidence the qwen35
tiers' PROVEN marks name.
