# Qwen architectures on NONOS, booted at 1d2f92e34, 2026-10-25

models/qwen36-2 at 1d2f92e34: the qwen3.5-0.8b and qwen3.6-27b tiers, the
`proven` default rule, and current main.

## The image and the machine

- **Image.** `make dev-image` (qemu-dev) of 1d2f92e34, sealed with the
  throwaway keys tools/nonos-dev-image makes in its own copy of the tree.
  The checkout's .keys/ was never read.
- **Machine.** QEMU without KVM (TCG, "QEMU software CPU, running the plain
  x86-64 build"), 4 threads, `--mem=12G` (the guest reports 12.43 GB),
  IOMMU, fresh disk with the tiers small, qwen3-0.6b and qwen3.5-0.8b laid
  on it. Each file was checked against its pin on the host first. The steps
  are in drive.steps.

## What booted (booted)

- **The allowlist.** tiers.png: `qwen tiers` lists qwen3.5-0.8b (833 MB,
  1.03 GB to run) and qwen3.6-27b (19.09 GB, 19.86 GB, "too large to run
  here (12.43 GB of memory)"). refused-word.png: `qwen get qwen9-1b` gives
  "qwen9-1b is not a tier".
- **The memory refusal.** install-27b.png and info-27b.png:
  `market install linux.qwen-qwen3.6-27b` queued it, then `market info`
  shows "This machine has too little memory to run it; choose a smaller
  tier (reason 19)". Serial line 1755:
  "[LINUX] qwen-qwen3.6-27b: its model is not installed, reason 19". Nothing
  was downloaded.
- **qwen2 answered.** small-answer.png and end.png: `qwen small` loaded
  qwen2.5-0.5b-instruct-q4_k_m.gguf, "verified" against its pin, and
  answered "Paris": 1 token, 170.1 s (ttft_ms=153688). Throughput is not
  measured: this is emulation.
- **qwen35 loaded and verified.** end.png: `qwen qwen3.5-0.8b` loaded
  Qwen3.5-0.8B-Q8_0.gguf, and "qwen: verified" its pin. The run was not
  ended by the boot.

## Not shown

- **qwen35 did not answer** within the step's 20 minutes. It is loaded,
  not proven.
- **qwen3 was not run.** Ctrl-D did not end small's chat before the next
  line was typed, so "qwen qwen3-0.6b What is ..." was read by small as its
  question. The "Paris" above is small's answer to that line.
- So only qwen2 is proven by this boot. The qwen3 marks in a36cfbff1's
  PROVEN, which the stick-tier assertion needs, have no booted evidence yet.
  The branch needs one more boot, with `/exit` between the chats and longer
  waits, before it can be gated.
