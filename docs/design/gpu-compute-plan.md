# GPU compute for local AI: milestones, cost and decisions

Part of [GPU compute for local AI](gpu-compute.md). The durations are estimates for
one session working full time, with an owner's Tiger Lake laptop able to boot test
builds from the second milestone on. No milestone is "working" until the owner has
booted it; QEMU has no Intel Xe device.

## Milestones

- **C0, a read-only probe** (2 weeks): GT power wake, the engine list and fuses,
  through the broker. Done when the owner's boot prints the engines that its Vol 2c
  registers name.
- **C1, one context** (6 to 8 weeks): the render engine, via execlists, runs a batch
  that stores a value. Done when the value lands in a granted page on the owner's
  machine.
- **C2, an EU assembler** (8 to 10 weeks), for Vol 9's instructions; GPGPU_WALKER
  runs a vector add. Done when the output equals the CPU's, bit for bit.
- **C3, the kernels a Qwen model needs** (3 to 4 months): quantised matrix-vector and
  matrix products, norms, rotary embedding, softmax, activation, row gather. Done when
  each equals a CPU reference within a stated error, on every tier's shapes.
- **C4, the compute wire and the qwenchat backend** (1 to 2 months), with client
  contexts, scrubbing, death and reset. Done when a chat answers the same as on the
  CPU, and an adversarial review is done.
- **C5, speed** (open): batch sizes and memory placement, measured against the CPU.
  Done when the owner's bench gives a number, either way.
- **C6, Arc A (DG2)** (3 to 4 months): local memory, the discrete card's bring-up,
  and the same kernels. Done when C1 to C4 pass again on an Arc card.

Altogether, about 9 to 12 months to C4 on Xe-LP, and more for Arc. Each milestone
lands on its own with proofs against a register model, as the display work does, and
none of it is in 0.9.x.

## What each milestone needs from the kernel

- **DMA grants larger than any class has today.** The display class's ceiling is
  8,192 pages, 32 MiB (`src/hardware/broker/dma/limits.rs`). A 4B model's weights are
  gigabytes. That needs a compute class with its own ceiling, for track A's review.
- **A broker scope row** for the GPU's compute function. On an iGPU, display and
  compute share one PCI function, so the two capsules cannot both claim it. Either
  the display capsule serves compute too, or the kernel learns to split a function
  between two claims. Both are decisions for G and A.
- **Interrupts** for completion, or a polled fence to start with (C1 polls).

## Decisions for the owner

1. **qwenchat's backend.** qwenchat is built from llama.cpp at a pinned commit. A GPU
   path means adding a backend module to that build. It would be written by us,
   against llama.cpp's own backend interface, and it changes a third-party build. Say
   yes or no before C4.
2. **The publisher key** for `capsule_gpu_xe`, as for every new driver capsule.
3. **The isolation proof's set.** Whether a qwen tier may talk to the compute capsule,
   and so re-seal the qwen listings with that bit in their measured set.
4. **Arc A after Xe-LP, or Arc A first** for the speed, if an Arc machine is
   available.

## Risks, stated plainly

- Vol 9 describes the instruction set, but an assembler written from it, with no
  vendor compiler to compare against, may hit encodings the PRM states loosely. Every
  kernel is checked against a CPU reference on the owner's machine.
- Tiger Lake's memory bandwidth may leave text generation little faster than the CPU;
  C5 exists to find out before Arc work begins.
- Power management (render standby, frequency) is in the PRM but is unproven here, and
  can stall a context. C1 runs with the GT held awake.
