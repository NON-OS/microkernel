# gic_proofs

Host tests for the pure parts of the aarch64 GIC driver, mounted by `#[path]`
from `src/arch/aarch64/gic/`:

- `select.rs`: which GIC is driven, from the device tree's answer and
  whether the CPU implements the GICv3 system registers. Every combination
  is pinned, including the refusal of a GICv3 tree on a CPU without them.
- `v2/encode.rs`: the GICv2 `GICC_IAR` decode (spurious and special INTIDs
  are never acknowledged back) and the `GICD_SGIR` encodings, checked
  exhaustively over every SGI and every CPU target list.
- `intid.rs`: which acknowledge values are finished, and a handler slot for
  every line and every LPI the kernel hands out, none shared.
- `its/encode/`: every ITS command field, `GITS_TYPER`, `GITS_BASER<n>` and
  `GITS_CBASER` (64 KiB pages with PA[51:48] included), and the
  redistributor's LPI tables, against IHI 0069.

Run with `RUST_TEST_THREADS=1 cargo test --release --config
profile.release.overflow-checks=true` from this directory. `nix flake check`
runs the same as `proofs-gic_proofs`, with clippy.
