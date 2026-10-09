# admission_proofs

Host tests for the decoders the spawn gate runs first on attacker-supplied
bytes: the capsule manifest decoder from `src/security/capsule_manifest` and the
NONOS-ID certificate decoder from `src/security/nonos_id_cert`, mounted by
`#[path]` unchanged, with the kernel's `alg_id` table. The signature checks stay
in the kernel and are not mounted. `hostile_tests.rs` feeds both decoders
malformed input. 7 tests.

Run with `cargo test --release` from this directory. `nix flake check` runs
the same as `proofs-admission_proofs`, with overflow checks on and clippy. See [the proofs page](../../docs/handbook/verification/proofs.md).
