# tpm_key_proofs

Host test crate for the kernel's TPM machine-key derivation and the TPM FIFO
transport under it. It compiles the kernel's own `src/security/tpm/` wire code
through `#[path]`, so the bytes under test are the bytes that ship. The
machine key and the TPM are described in
[docs/handbook/trust/tpm.md](../../docs/handbook/trust/tpm.md); the wallet's
vault seals under that key ([docs/handbook/apps/wallet.md](../../docs/handbook/apps/wallet.md)).

## What the tests check

42 `#[test]` functions:

- `machine_key/`: each of the derivation's six commands framed byte for byte
  against the TPM 2.0 specification (`command_tests.rs`, `wire_tests.rs`), the
  responses parsed and refused as they should be (`parse_tests.rs`), and, where
  `swtpm` is installed, the whole sequence run against a software TPM over its
  socket (`live_tests.rs`).
- `fifo/`: the TIS FIFO transport against a model of the register interface,
  including every send and receive failure (`tests.rs`, `send_fail_tests.rs`,
  `recv_fail_tests.rs`, `qemu_tests.rs`).

Without `swtpm` the live tests pass and print that they were skipped.

## Running

```sh
cd userland/tpm_key_proofs
cargo test --release
```

CI runs it through `nix flake check` (`tools/nix/checks.nix`) on Linux only,
with `swtpm` and `tpm2-tools` present; a log line saying a live test was
skipped fails the check.
