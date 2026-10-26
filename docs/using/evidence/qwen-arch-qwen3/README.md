# qwen3 on NONOS: qwen3-0.6b answered, booted at 1d2f92e34, 2026-10-25

The same image as ../qwen-arch (models/qwen36-2 at 1d2f92e34, a qemu-dev
dev image sealed with tools/nonos-dev-image's throwaway keys; the
checkout's .keys/ was never read). This boot runs qwen3-0.6b alone, with
`/exit` after it, because the first boot's Ctrl-D did not end the chat
before the next line was typed.

- **Machine.** QEMU without KVM (TCG, "QEMU software CPU, running the plain
  x86-64 build"), 4 threads, `--mem=12G`, IOMMU, a fresh disk with
  qwen3-0.6b laid on it (checked against its pin on the host first). The
  steps are in drive.steps.
- **The answer (booted).** qwen3-30.png: `qwen qwen3-0.6b What is the
  capital of France? Answer in one word.` loaded Qwen3-0.6B-Q8_0.gguf,
  "qwen: verified" its signed SHA-256 pin, and answered "Paris": 1 token,
  113.9 s for the turn ("[LINUX] metrics: load_ms=1379378 ... ttft_ms=92336").
  Throughput is not measured: this is emulation.
- qwen3-exit.png: `/exit` ended the chat.

This proves the dense qwen3 architecture on NONOS, the evidence the qwen3
tiers' PROVEN marks in default_tier.rs name.
