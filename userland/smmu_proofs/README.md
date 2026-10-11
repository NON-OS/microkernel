# smmu_proofs

Host tests for the pure parts of the aarch64 SMMUv3 driver, mounted by
`#[path]` from `src/arch/aarch64/iommu/smmu/encode/`:

- `idr.rs`: which units the driver programs. QEMU's SMMUv3 decodes, and each
  missing feature (stage 1, AArch64 tables, a two-level stream table,
  little-endian walks, coherency, the 4 KiB granule) is refused by name.
- `queue.rs`: PROD and CONS with their wrap bit, swept over every position of
  small queues: empty, full, advancing, and whether the unit has consumed up
  to a given command.
- `cmd.rs`, `ste.rs`, `cd.rs`, `pte.rs`: the command, stream table entry,
  context descriptor and stage 1 page table encodings, field by field
  against the architecture's layouts, including that a write-only grant
  maps read-write and that an address past 39 bits is refused.

Run with `RUST_TEST_THREADS=1 cargo test --release --config
profile.release.overflow-checks=true` from this directory. `nix flake check`
runs the same as `proofs-smmu_proofs`, with clippy.
