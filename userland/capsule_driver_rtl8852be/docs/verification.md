# Verification

- Capsule: `(cd userland/capsule_driver_rtl8852be && cargo build --release
  --target ../x86_64-nonos-user.json -Zbuild-std=core,alloc
  -Zbuild-std-features=compiler-builtins-mem)`, no warnings.
- Proofs: `(cd userland/rtl8852_proofs && cargo test --release)`, 55 tests
  on the modelled chip (`userland/rtl8852_proofs/README.md`).
- Host mutants: `tools/nonos-host-mutants
  verification/host_mutants/rtl8852be.json`. Each of the 46 takes one rule
  or bound out of the driver (a timeout, a retry count, a size edge, a
  ready check, a field of a command, a step of the order, a power-off on
  failure, the drawn address's bits, a stage mapping) and names the test
  that then fails; all 46 are killed.
- Kernel side: `userland/kernel_proofs` holds the inventory's ids and the
  broker's scope row to `src/ids.rs`, the RTL8852CE to "no driver", the
  card's name in the NICS table and its spawn.
- The firmware's sha256 pin: `scripts/baselines/upstream.sha256`.

What the proofs cannot show is in [hardware.md](hardware.md).
