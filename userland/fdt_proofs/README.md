# fdt_proofs

Host tests for the kernel's flattened device tree parser (`src/arch/fdt`),
mounted by `#[path]` so the code under test is the code the kernel compiles.

- `fixtures/` holds QEMU `virt` trees (GICv3, GICv2, SMMUv3) dumped from QEMU
  11.1.1 and a Raspberry Pi 4 tree cut from the firmware's
  `bcm2711-rpi-4-b.dtb`. Each `.dtb` is compiled from the `.dts` beside it
  with dtc 1.8.1: `dtc -q -I dts -O dtb -o X.dtb X.dts`. The provenance of
  each is at the top of its `.dts`.
- The fixture tests pin what the boot path reads: memory, CPUs, console, GIC,
  timer, PCIe ECAM, reservations, platform devices and their SPIs, and on the
  Pi the `ranges` translation that moves every peripheral.
- The hostile tests build blobs no tool would emit (bad headers, misplaced
  blocks, unbalanced or over-deep trees, bad cell counts, broken strings,
  unmapped addresses) and check each is refused with the right error.
- The mutation tests truncate the real blobs at every length and damage every
  structure byte, running every entry point over each copy.

Run with `RUST_TEST_THREADS=1 cargo test --release --config
profile.release.overflow-checks=true` from this directory. `nix flake check`
runs the same as `proofs-fdt_proofs`, with clippy.
