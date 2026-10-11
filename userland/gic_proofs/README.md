# gic_proofs

Host tests for the pure parts of the aarch64 GIC driver, mounted by `#[path]`
from `src/arch/aarch64/gic/`:

- `select.rs`: which GIC is driven, from the device tree's answer and
  whether the CPU implements the GICv3 system registers. Every combination
  is pinned, including the refusal of a GICv3 tree on a CPU without them.
- `v2/encode.rs`: the GICv2 `GICC_IAR` decode (spurious and special INTIDs
  are never acknowledged back) and the `GICD_SGIR` encodings, checked
  exhaustively over every SGI and every CPU target list.

Run with `RUST_TEST_THREADS=1 cargo test --release --config
profile.release.overflow-checks=true` from this directory. `nix flake check`
runs the same as `proofs-gic_proofs`, with clippy.
