# Target kernel: the object model

Today a capability is a bit in a 44-bit word per process (`src/capabilities/types/defs.rs:20-82`), the word is checked at the system-call gate through a per-token allowlist (`src/syscall/contract/resolver/check_syscall.rs:28`), and services ask the kernel whether a pid holds a bit (`mk_cap_check`, used for example at `userland/capsule_vfs/src/server/fs_gate.rs:46`). Nothing names an object and nothing can be passed to another process. Every isolation property that is finer than a bit (this guest's files, this destination, this device) therefore has to be enforced by a filter in some service, and the audit's capsule-to-capsule findings are exactly such filters failing.

The target replaces ambient bits with **object capabilities**: an unforgeable reference to one kernel object, with rights, held in a per-process table, passed over IPC. It keeps the 44-bit catalogue as **capability classes** so the image ceiling remains a single monotone bound the kernel enforces.

## Principles

1. Every kernel operation names the object it acts on through a capability the caller holds. There are no global names in the kernel: no service registry, no pid-addressed sends.
2. A capability can be copied with equal or fewer rights, moved, deleted, and revoked by the holder of the capability it was derived from.
3. Every object type and right belongs to a capability class. A process can never hold a capability whose class is outside its class mask, and its class mask is never outside the image ceiling.
4. The kernel allocates its own memory for objects from pools sized at boot and charges each allocation to the creating process's quota; page-table frames come from their own pool.

## Capabilities

```rust
/// A slot in a process's capability table. 32 bytes.
pub struct Cap {
    pub object: ObjRef,     // kernel pointer to the object, never visible to userland
    pub kind: ObjKind,      // u8
    pub rights: Rights,     // u16 bitmask, meaning per kind
    pub badge: u64,         // set once by Mint on endpoint and notification caps; 0 otherwise
    pub parent: SlotRef,    // derivation link for Revoke
}

pub type CapHandle = u32;   // index into the caller's table; 0 is never valid
```

- **Table.** Each process has one flat capability table of up to 4,096 slots, allocated with the process. Handles are table indices. A flat table is chosen over seL4-style guarded CNodes because NØNOS processes hold tens of capabilities, not millions, and a flat table is easier to specify and prove.
- **Derivation and revocation.** `Copy`, `Mint` and `Move` record the source slot as `parent`. `Revoke(h)` deletes every capability derived from `h`, transitively, in every process. The derivation links form a tree threaded through the slots, as in seL4's mapping database; the proof obligation is that the links stay a forest and that `Revoke` terminates.
- **Rights only shrink.** `Copy(h, rights)` and `Mint(h, rights, badge)` require `rights ⊆ h.rights`. `Mint` sets a badge only on a capability whose badge is 0.

## Capability classes and the ceiling

The 44 bits of today's word keep their names and numbers in `abi/caps.toml` and become **classes**. Each object kind and right maps to exactly one class:

| Class (today's bit) | Object kinds and rights it covers |
|---|---|
| IPC (3) | Endpoint send and receive, Notification signal and wait |
| Memory (4) | MemoryObject create, Frame map |
| Driver (16), Mmio (17), Irq (18), Dma (19), Pio (20) | Device, device frames, IRQHandler, IommuDomain, PortRange |
| SpawnBroker (23) | AddressSpace and Thread create for a child |
| ForeignExec (32) | Thread with a syscall-redirect endpoint, cross-space copy on a child's AddressSpace |
| Network (2), FileSystem (6), and other service bits | No kernel object. They become classes of **endpoint badges** that the services named in [04-classification.md](04-classification.md) define; the kernel checks only that a process whose class mask lacks Network never receives a capability to an endpoint registered as a Network-class endpoint |

Every process has a **class mask** fixed at creation:

```
class_mask(child) = verified_set(child) ∩ class_mask(creator) ∩ CEILING
```

where `verified_set` is what the verifier approved from the manifest ([migration/spawn-gate.md](migration/spawn-gate.md)) and `CEILING` is baked into the kernel image. The kernel refuses, at transfer time, any capability whose class is not in the receiver's class mask: in `Copy` to another table, in a transfer over IPC, and in creation. This one check is the kernel's whole enforcement of the ceiling, and it is monotone: no operation increases a class mask.

## Object kinds

| Kind | What it is | Key rights | Replaces today |
|---|---|---|---|
| **Process** | A capability table, one address space, a class mask, a quota, the measurement record, an exit endpoint | `Kill`, `ReadInfo` | The PCB's capability token and caps word (`src/process/caps/mint.rs:34`) |
| **AddressSpace** | A page-table tree, an ASID and PCID; frames come from the page-table pool | `Map`, `Unmap`, `Protect`, `Copy` (into it), `Start` | `memory/paging` address spaces, `process/address_space/lifecycle` |
| **Thread** | Registers, FPU state, priority, a fault endpoint, an optional syscall-redirect endpoint | `ReadRegs`, `WriteRegs`, `Resume`, `Suspend`, `SetPriority` up to the caller's own maximum | TIDs and thread groups, the foreign trap and context calls (`src/process/foreign/trap.rs:28`, `src/process/foreign/signal_call.rs`) |
| **Endpoint** | A synchronous rendezvous: a sender blocks until a receiver takes its message, a call creates a one-shot reply capability | `Send`, `Receive`, `Grant` (may carry capabilities) | Named inboxes and the correlation table (`src/ipc/nonos_channel/message.rs:25`, `src/syscall/microkernel/ipc/call/sys_ipc_call.rs:29`) |
| **Notification** | A 64-bit word of pending bits; signal ORs bits in, wait takes and clears them | `Signal`, `Wait` | IRQ wait slots (`src/hardware/broker/irq/slots.rs:34-38`), futex-style wakeups between processes |
| **MemoryObject** | A list of frames that can be mapped into several address spaces and given to IOMMU domains; may be sealed read-only | `Map`, `MapWrite`, `Seal`, `Read`, `DmaMap` | The surface registry (`kernel_core/surface_registry`), DMA buffers (`hardware/broker/dma`), the clock page's frame, boot package frames |
| **IrqHandler** | One interrupt source bound to a Notification bit, with ack | `Bind`, `Ack` | `MkIrqBind`, `MkIrqWait`, `MkIrqAck` |
| **IommuDomain** | One translation context; devices attach, MemoryObjects map at IOVAs | `Attach`, `Map`, `Unmap` | `hardware/broker/confine` per-holder domains (`src/hardware/broker/confine/capsule.rs:46`) |
| **Device** | One PCI function or platform device: a config-space window with an allow-list, its BARs as device MemoryObjects, its MSI vectors, its reset | `ConfigRead`, `ConfigWrite`, `MapBar`, `BindMsi`, `Reset`, `BusMaster` | The broker's claim and grant records (`src/hardware/broker/claim/claim.rs:22`) |
| **PortRange** | A range of I/O ports, enabled in the holder's TSS I/O bitmap when its threads run | `Use` | One system call per port access (`src/hardware/broker/pio/access/raw/in16.rs:22`) |
| **Quota** | A count of kernel objects and frames a process may cause the kernel to allocate | `Split` | Per-call limits scattered through handlers |
| **Verdict** | A single-use record of a verifier's approval: `D_file`, `D_load`, the approved class set, the namespace hash and the authority. Created only through a `VerdictAuthority` capability | `Consume` (by `process_create`) | The spawn gate's `Preflighted` record (`src/kernel_core/process_spawn/capsule_spawn/runner/preflighted.rs:24-39`) |

Two read-only kernel pages complete the model:

- **The measurement table**: for each live process, the digest the kernel computed over its mapped pages at first start, its class mask, and the digest of its creator. Readable through a MemoryObject capability given to the attest service.
- **The process-information page**: per process, state, CPU time, resident pages and parent, for procmgr. It replaces the `MkProcStat` family.

## Memory

- **Page-table pool.** Page-table frames come from a pool reserved at boot, separate from the pool that backs user frames. No user-controlled data ever sits in a frame adjacent to a page-table frame in the same pool. This answers the frame-adjacency observation behind NKA-2026-020 (`src/elf/loader/core/load_segment/populate_page.rs:38` allocates a data frame and the next table frame from the same ascending allocator) independently of any one bug.
- **One allocator chain.** `phys` (bitmap) under `frame_alloc` under `buddy_alloc`, with the dead `boot_memory`, `region`, `page_allocator`, `page_info` and DMA allocator deleted ([04-classification.md](04-classification.md)).
- **Cross-space writes.** The kernel writes into another address space only in three places: IPC delivery into a receiver's buffer, `copy_to_user` and `copy_from_user` into the current space, and `AddressSpace::Copy` on a child's space. All three go through the one page-safe primitive of the in-place fix for NKA-2026-020 (translate per page, never cross a frame boundary in one store, check the leaf's write permission).

## IPC

- **Synchronous endpoints** carry up to 64 bytes in registers and a buffer of up to 4 KiB, plus up to four capabilities when the sender's capability has `Grant`. Larger data goes through a shared MemoryObject. Today every message is copied twice through a kernel heap `Vec` (`src/syscall/microkernel/ipc/send.rs` into the inbox, then out); the target copies once, sender buffer to receiver buffer, during the rendezvous.
- **Badges** identify senders. A server mints one badged endpoint capability per client; the badge arrives with every message. This replaces both the kernel-stamped sender pid and the service-side `mk_cap_check(pid, bit)`, which can misidentify a sender after pid reuse (`userland/capsule_vfs/src/server/fs_gate.rs:23-25` documents the concern).
- **Notifications** carry asynchronous events: interrupts, timer expiry, child exit.
- **No names.** Service names live in the names service in userland, which maps a name to a capability it was given by init.

## Scheduling

The five fixed priority bands with round robin inside a band stay (`src/process/scheduler/selection/band_scan.rs:52`). Priority becomes a property of a Thread capability, settable only up to the setter's own maximum. The dead Linux-style policy registry (`src/process/scheduler/policy.rs`) is deleted.

## Faults, exits and the foreign mechanism

- A thread's fault (page fault, general protection, divide error) is delivered as a message to its fault endpoint, which its creator chooses. With no fault endpoint, the process is terminated with an exit code, as today's default signal action does (`src/process/signal/delivery/default.rs:43-50`).
- A thread with a syscall-redirect endpoint has every system call it makes delivered there, with its registers, and resumes when the handler replies. This is today's foreign mechanism (`src/process/foreign/trap.rs:28`) made generic: the Linux personality is one user of it.
- A process's exit is signalled on its exit endpoint, which replaces in-kernel liveness cells and lets init supervise from userland.

## What the kernel keeps of today's code

The model reuses the live mechanism: `memory/paging` for address spaces, the scheduler, the IOMMU and interrupt code, the broker's confinement, DMA mapping and claim epochs, the foreign mechanism's parking and resumption, `usercopy`. It deletes the ambient checks it replaces: the per-token allowlist built from 44 bits (`src/syscall/contract/allowlist/`), the capability table of per-syscall predicates (`src/syscall/contract/cap_table/`), and the in-kernel service registry (`src/services/registry.rs:58`). The token and its MAC (`src/capabilities/token/material.rs:41`) guard a kernel-internal structure against kernel-internal corruption; a capability table has no such token, so the MAC goes with it.
