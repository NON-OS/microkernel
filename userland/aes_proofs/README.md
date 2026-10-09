# aes_proofs

Host tests for the software AES. `nonos_aes` is checked against FIPS 197 and its
CTR mode against SP 800-38A, with its S-box and `xtime` mounted by `#[path]` and
compared with a table. The AES inside `capsule_net_nym` (`src/crypto/aes`) is
mounted the same way and held to the same vectors. 12 tests.

Run with `cargo test --release` from this directory. `nix flake check` runs
the same as `proofs-aes_proofs`, with overflow checks on and clippy. See [the proofs page](../../docs/handbook/verification/proofs.md).
