# Design principles: Proofs live next to the code

Part of [Design principles](design-principles.md).

## Proofs live next to the code

A [proof crate](glossary-peer-list.md#proof-crate) mounts the shipped source by `#[path]`
and tests it on the host with `cargo test`. For example, `kernel_proofs` mounts
the kernel's service registry policy (`service_policy` in
`userland/kernel_proofs/src/lib.rs:136-138`). `nix flake check` runs every
`*_proofs` crate under `userland/` that has a lock file, the bootloader's
`boot_proofs` and six more host crates (`proofDirs` in
`tools/nix/checks.nix:18-29`). Each runs its tests with overflow checks on in
the release build, then clippy with warnings as errors (`proof` in
`tools/nix/checks.nix:70-97`). Clippy skips three crates that are not clean yet
and lints only the library of eight more (`lintNone` and `lintLib` in
`tools/nix/checks.nix:44-62`).

The flake defines 114 proof-crate checks. In the check run on this commit, 113
were built and 110 passed; `proofs-model_fetch_proofs` did not evaluate, as
[CI](../build/ci.md) explains. `rtl8169_proofs`, `usb_msc_proofs` and
`xhci_proofs` failed at their clippy step; `rtl8169_proofs` had passed its 67
tests first.

The Lean 4 models in `verification/lean/` are not built by `nix flake check`.
Its `static-evidence` check requires the `sorry_count` that
`verification/evidence/collect-evidence.sh` finds in their sources to be zero
(`tools/nix/checks.nix:240-246`), and it passes on this commit. Kani, Verus, the
Charon and Aeneas extraction and the fuzzers run outside the flake, as the
header of `tools/nix/checks.nix` says. Everything NONOS trusts without proof is
listed in [verification/ASSUMPTIONS.md](../../verification/ASSUMPTIONS.md).

The limit: proof crates test code on the host; they do not boot it. What has
been seen on a machine is in the [support matrix](../hardware/MATRIX.md).

## See also

- [Overview](README.md)
- [Architecture](architecture.md)
- [Threat model](threat-model.md)
- [Code style](../contributing/code-style.md)
- [Tests and proofs](../contributing/tests-and-proofs.md)
- [Capsule isolation](../security/capsule-isolation.md)
