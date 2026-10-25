
**Not booted. Not hardware-verified.** Host-tested only:

- kernel_proofs whole crate: 1673 passed, clippy clean (kernel half).
- xhci_proofs: 291 passed, clippy clean, including the 7 new switch-over
  tests (`conformance/intel_ports_*`).
- `tools/nix/inputs.py --check` clean.
- Capsule `cargo check` for x86_64-nonos-user clean. Kernel x86 lib had 0
  warnings before the driver half was added.

Left before it can be gated: the 17 mutants not run (`MUTANTS.md`), the 39
static-abi commands, the x86 smp and aarch64 checks, and a QEMU boot showing
the xhci path unchanged (qemu-xhci is 1b36:000d, so the switch is never
reached there).
