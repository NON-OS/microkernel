# kernel_proofs

Host-runnable proofs for the kernel's memory isolation and authorization
boundary. The real page-permission, user-copy, syscall-decode, capability, and
ELF-loader source is included through `#[path]` and run directly, so the
invariants are proved about the code that enforces them.

## W^X

A page must never be both writable and executable. The proof works on the real
`to_pte_flags` encoding: a permission set that is not a write-execute violation
never encodes a page-table entry that is simultaneously writable and executable,
so a mapper that rejects `is_wx_violation` cannot install a W+X page. The flag
encoding (present, writable, user, no-execute) is also proved faithful to the
permission bits. Runnable and by Kani over all permission patterns.

## User-copy bounds

`check_range` guards every copy between the kernel and userspace. It is proved
total, and an accepted range is page aligned and lies inside user space without
wrapping. Null pointers, oversized lengths, addresses past user space, and
overflowing ranges are all rejected. Runnable and by Kani over all addresses and
lengths.

## Syscall decode

An untrusted `u64` syscall id crosses the kernel boundary. Decoding it is proved
total (no value panics), the id table stays consistent with the name table, and
known ids round trip through their numeric value. Kani proves totality over all
`u64`.

## Authorization

The real `is_allowed` capability table is proved to deny an empty token every
syscall, to permit a crypto syscall only for a token that grants the crypto
capability, and to never remove access when a capability is added. Capability
ids are shown to occupy distinct single bits.

## ELF loader

The capsule loader parses attacker-controlled ELF. A truncated header is
rejected, and an accepted program-header table fits inside the file with no
integer overflow in `phoff + phnum * phentsize`, over adversarial headers with
large offsets and counts.

## Clock scale

MkTimeMillis and MkTimeMonotonic scale the time-stamp counter to
milliseconds. The real scale is proved to agree with the old 64-bit product
wherever that fits, to keep counting exactly past the 71 days at 3 GHz where
the product outgrew 64 bits (a panic with overflow checks on), and to
saturate rather than wrap for a counter too slow to fit (`clock_scale`).

## Thread start

MkThreadSpawn and MkForeignThread ask the real start rule before anything is
built. It is proved to agree with the x86_64 user entry builder at every edge
(the bound is read from the builder's source) and with the user-copy bound,
and the spawn path is held to ending a thread it published and could not
start, instead of leaving it New in the process table for good
(`thread_refusal`).

## Narrowed arguments

A syscall register that names a pid, a port, an endpoint or a device field
is narrowed with the real `narrow` rule or refused, never cut down with a
cast. The rule is proved to round trip every value its field holds and to
refuse every wider one, and each call site is scanned for a truncating cast
left in it, with the few kept on purpose listed and explained
(`narrowed_args`).

## User-sized buffers

A copy whose length a caller chose takes its kernel buffer through the real
`take_buffer`, which is proved to give room for every length a heap can
hold and to refuse, not abort on, one it cannot; the kernel's allocation
failure handler halts the machine. MkLocalSign and MkLocalVerify are held to
checking the range before they allocate, and `read_user_bytes` to the
fallible buffer (`user_buffer_tests`).

## Handing over

A call that takes from a queue for good checks the caller's buffer before it
takes. MkInputEventDrain and MkProcOutput are held to that order, so a bad
buffer leaves the keystrokes or the line for the next call, and MkStdinRead
and the IPC receives are held to the order they already had
(`handover_tests`).

## Devices, DMA and services

The broker's IOMMU posture (`confine_posture`), its DMA grant records and the
shutdown wipe of live grants (`dma_wipe`), the IOVA space, PCI addresses and
the VT-d command and window rules are mounted from `src/hardware/broker` and
`src/arch/x86_64/iommu`. `service_policy_tests` holds the registry to its rule
that reaching a service which carries traffic off the machine takes Network.
The crate has many more modules, one per kernel rule; `src/lib.rs` lists them.

## Run

```sh
cd userland/kernel_proofs
cargo test --release
cargo kani                # all-input isolation and decode checks (requires Kani)
```

`nix flake check` runs the tests as `proofs-kernel_proofs`; the
`proof-crates-kani` job in `verify.yml` runs the Kani harnesses. See
[the proofs page](../../docs/handbook/verification/proofs.md).
