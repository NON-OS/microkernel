# mechanism_proofs

Host-runnable proofs binding kernel mechanisms to the properties their Lean
models state. Each module pulls in the real Rust via `#[path]` and runs it, so
the property is proven of the code the kernel executes, not a copy of it.

Modules land here as their Lean model moves from a specification to a code-bound
proof. The binding level of every model is recorded in
`verification/lean/REFINEMENT.md`.

Bound so far:

- `buddy`: the order, size and buddy-address arithmetic in
  `src/memory/buddy_alloc/constants/helpers.rs`, which the allocator runs. A
  split conserves size (`order_to_size(k+1) == 2 * order_to_size(k)`) and the
  buddy address is an involution (`buddy_address(buddy_address(a, o), o) == a`).
  Lean: `Nonos/Buddy.lean`.
- `phys::bitmap`: the bit-index arithmetic in
  `src/memory/phys/bitmap/index.rs`, which `bit_ops.rs` calls. A mask selects
  exactly one bit, and the byte and bit position reconstruct the index
  losslessly. Lean: `Nonos/Bitmap.lean`.
- `region`: the half-open range algebra in `src/memory/region/overlap.rs`, which
  `MemRegion::overlaps`, `contains` and `contains_range` delegate to. Overlap is
  symmetric and is exactly the negation of disjointness. Lean:
  `Nonos/Interval.lean` and `Nonos/Vma.lean`.
- `quota`: the resource-token check in `src/capabilities/resource/limits.rs`,
  which `has_bytes` and `has_ops` delegate to. A request is covered exactly when
  it is within the remaining budget. Lean: `Nonos/Quota.lean`.
- `ring`: the input-ring index arithmetic in
  `src/kernel_core/surface_registry/ring_math.rs`, which `input_ring.rs` calls. A
  wrapped index stays within the capacity, and a full ring is detected when the
  head would reach the tail. Lean: `Nonos/Ring.lean`.
- `mmio`: the window-validity arithmetic in
  `src/drivers/security/mmio_range.rs`, which `validate_mmio_region` delegates
  to. A valid window is non-empty and does not wrap the address space. Lean:
  `Nonos/Mmio.lean`.
- `refcount`: the page reference-count decrement in
  `src/memory/page_info/manager/refcount.rs`, which `decrement_ref_count`
  delegates to. A decrement never underflows. Lean: `Nonos/Refcount.lean`.
- `nonce`: the nonce composition in
  `src/capabilities/resource/nonce_compose.rs`, which `next_nonce` delegates to.
  The monotonic counter is recoverable from the low 32 bits, so distinct
  counters never collide. Lean: `Nonos/Nonce.lean`.
- `bounds`: the relocation-target bounds test in
  `src/elf/reloc/apply/range.rs`, which `in_segment` delegates to. An in-range
  access sits wholly inside the segment with neither end overflowing. Lean:
  `Nonos/Bounds.lean`.
- `scheduler`: the priority order in `src/process/scheduler/policy_types.rs`,
  through `SchedAttr::effective_priority` directly. Deadline tops the order and
  idle bottoms it, and a real-time task preempts a timesharing one. Lean:
  `Nonos/Priority.lean`.

- `timer`: empty. The load-balancer interval it bound was removed from the
  kernel with the per-CPU scheduler that used it.

Also mounted, with tests beside them rather than a Lean model each:

- `spawn`: the spawn gate's capability arithmetic in
  `src/security/capsule_manifest/verify/caps_bits.rs` (`check_ceiling` and
  `check_grant` delegate to it) and the delegation expiry in
  `src/capabilities/delegation/lifetime.rs`.
- `heap`: the zero-on-free allocator in `userland/libc/src/heap/zero_on_free.rs`.
- `iommu`: the VT-d capability decode and the context and second-level entry
  encoding under `src/arch/x86_64/iommu`.
- `context`: the RFLAGS sanitizer in `src/arch/x86_64/context/rflags.rs`.
- `compositor`: the software row compositing in
  `userland/compositor/src/sw_blitter/row.rs`.
- `constants`: the kernel constants the Lean files quote as literals, held
  equal to them by `constants_tests`.

Run:

    cargo test --release
    cargo kani

`nix flake check` runs the tests as `proofs-mechanism_proofs`; the
`proof-crates-kani` job in `verify.yml` runs the Kani harnesses. See
[the proofs page](../../docs/handbook/verification/proofs.md).
