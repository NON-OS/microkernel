# boot_memory_proofs

Host tests for `src/arch/aarch64/boot/memory/dma_place.rs`, mounted by
`#[path]`: where boot puts the below-4 GiB DMA pool.

- QEMU virt's layout (RAM at 1 GiB, the image a few hundred KiB in, the tree
  at the base of RAM) carves the pool from the allocator's span past the
  image, never over it.
- A board whose RAM starts at zero, and one with a low region the allocator
  does not take.
- Against a brute-force search over small layouts: the run returned is the
  lowest one inside an available region, below 4 GiB, page aligned, and
  clear of every range to avoid; `None` exactly when no such run exists.

Run with `RUST_TEST_THREADS=1 cargo test --release --config
profile.release.overflow-checks=true` from this directory. `nix flake check`
runs the same as `proofs-boot_memory_proofs`, with clippy.
