# prove_proofs

Host tests for `nonos.prove` (`userland/capsule_prove`). The capsule's pure half,
`src/assemble/`, is mounted by `#[path]` with the kernel's own boot-slot encoders
from `src/security/boot/slots` and the libc readers it uses (`boot_slots`,
`enroll`, `heap/span`). The records are built the enroll tool's way and the
registry by `nonos-device-attest`'s `Registry`, so a layout one side writes and
the other does not read fails here. 30 tests.

Run with `cargo test --release` from this directory. `nix flake check` runs
the same as `proofs-prove_proofs`, with overflow checks on and clippy. See [the device proof page](../../docs/handbook/trust/device-proof.md)
and [the proofs page](../../docs/handbook/verification/proofs.md).
