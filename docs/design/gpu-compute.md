# GPU compute for local AI: the design

Status: a design for after 0.9.3. No code. The milestones, their cost and the
owner's decisions are in [the plan](gpu-compute-plan.md). Citations are to Intel's
Tiger Lake PRM: "Vol 2a" is IHD-OS-TGL-Vol 2a-12.21, and so on for the others.

## The goal
Qwen chats today run on the CPU, in `qwenchat` under the Linux personality
([Local model](../using/local-ai.md)). The aim is to let one chat run its heavy
arithmetic on the machine's GPU instead. The model, the chat and the network rules
stay where they are. The GPU must be confined like every other device: it reaches
only memory it was granted, through the IOMMU.

## The first target: Intel Xe-LP (Tiger Lake), then Arc A

- **Why Xe-LP first.** Its PRM is public and complete enough to drive it:
  - Vol 8 describes submitting work through execlists (pp. 2 to 3);
  - Vol 6 describes the 48-bit per-process GTT (pp. 3 to 36);
  - Vol 2a describes the compute walker, GPGPU_WALKER and MEDIA_VFE_STATE
    (pp. 465, 695);
  - Vol 9 describes the EU instruction set (from p. 353).
  It shares its device with gpu-2's display work, and owners have Tiger Lake laptops.
- **What Xe-LP will not give.** It shares system memory with the CPU. Generating text
  with a quantised model is bound by memory bandwidth, so the iGPU's gain over the
  CPU is limited for that phase. It is larger for reading the prompt, which is bound
  by arithmetic. That is an expectation, not a measurement; the owner's bench will
  give the number.
- **Arc A (DG2)** has its own public PRM (Intel documents 774452 to 774522) and its
  own fast memory. It is where a real speed-up lies. It comes second, on the same
  capsule design, because it adds local memory management and a discrete card's
  bring-up.
- **AMD and NVIDIA, not first.** AMD publishes instruction sets (GPUOpen) but no
  register reference for current parts. NVIDIA publishes compute classes and QMD
  layouts but no bring-up sequences ([NVIDIA](../hardware/gpu-nvidia.md)).

## The shape
```
qwenchat (Linux personality) --compute wire--> capsule_gpu_xe (driver capsule)
                                                 | broker: claim, MMIO, DMA grants, IRQ
                                                 v
                                              the GPU, behind its IOMMU domain
```

- **`capsule_gpu_xe`**, a signed driver capsule, claims the GPU through the broker as
  every driver does. It owns the GGTT entries for its contexts, one context per client
  with its own PPGTT (Vol 6), and submits through execlists on the render engine (Vol
  8). Vol 8 has the host submit through execlists itself, so no GuC firmware is
  needed for that.
- **A fixed kernel set.** The capsule carries the EU kernels a model needs, assembled
  at build time from Vol 9's instruction set, and signed with it. Clients choose a
  kernel by number and pass buffers; they never upload code. So no shader compiler
  sits in the trusted base, and a client cannot make the GPU run what was not
  reviewed.
- **The compute wire.** Allocate and free a buffer, write and read it, run kernel N
  over buffers with these sizes, and wait on a fence. Every size and index is checked
  in the capsule before anything reaches the GPU, like the display wire's checks.
- **In qwenchat.** A backend that sends the model's operations over the wire, behind
  a switch, so that the CPU path stays the default. See the plan's decision 1.

## Confinement
- **DMA.** The GPU sits in the capsule's own IOMMU domain, which maps only the
  capsule's grants (see [IOMMU](../kernel/iommu-what-the-kernel.md)). Its PPGTT and
  GGTT can only reach pages that domain maps. A wrong page table entry faults at the
  IOMMU; it never reaches another program's memory.
- **Clients.** Each client's PPGTT maps only that client's buffers, so one chat cannot
  read another's weights or activations. Buffers are zeroed before they are mapped,
  and again when they are freed.
- **Death.** When the capsule dies, the broker tears down its domain, so the GPU can
  reach nothing. The next capsule resets the engines before its first submission.
  When a client dies, its context is retired and its buffers are scrubbed.
- **The isolation proof.** A qwen tier's measured set (`WAYLAND | MODELS_RO | TTY`)
  does not include talking to a compute capsule. Adding it changes what the vendor
  rows measure. That is an owner decision (the plan's decision 3), not a quiet change.
- **Unchanged.** The network refusal, the console rule and model loading. Weights go
  from the data volume to the capsule's buffers through the client, never a network.
