# attest_key_proofs

Host test crate for the attestation key's public half and the document that
carries it. The kernel's parser and encoder (`src/security/tpm/ak/` and
`src/security/attest_doc/`) are compiled through `#[path]`, so the bytes under
test are the bytes that ship. Attestation is described in
[docs/handbook/trust/tpm.md](../../docs/handbook/trust/tpm.md).

## What the tests check

12 `#[test]` functions: the public key parsed from a TPM's answer
(`public_tests.rs`), the binding of that key into the document
(`binding_tests.rs`), and the document's layout (`document_tests.rs`), with
fixtures in `src/fixtures.rs`.

## Running

```sh
cd userland/attest_key_proofs
cargo test --release
```

CI runs it through `nix flake check` (`tools/nix/checks.nix`), with overflow
checks on and clippy over all targets.
