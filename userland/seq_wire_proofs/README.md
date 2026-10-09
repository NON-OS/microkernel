# seq_wire_proofs

Host tests for the request decode of the services on the older `seq | op` wire:
`capsule_ramfs`, `capsule_keyring`, `capsule_payment` and `capsule_installer`.
Each service's `protocol` module is mounted by `#[path]` under the service's
name and fed well-formed and malformed requests. 8 tests.

Run with `cargo test --release` from this directory. `nix flake check` runs
the same as `proofs-seq_wire_proofs`, with overflow checks on and clippy. See [the proofs page](../../docs/handbook/verification/proofs.md).
