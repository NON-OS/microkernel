# attest_doc_proofs

Host test crate that holds the kernel's attestation encoders to the readers
on the other side of the syscall, and the About window's proof board to its
rules. Every file is compiled through `#[path]`, unchanged. It depends on
`nonos_route_proof`. The attestation model is described in
[docs/handbook/trust/stark.md](../../docs/handbook/trust/stark.md) and the
About window in [docs/handbook/apps/system-apps.md](../../docs/handbook/apps/system-apps.md).

## What is under test

- The attestation document: the kernel's encoder
  (`src/security/attest_doc/document.rs`) against About's parser
  (`capsule_about/src/about/data/doc_parse/`), `parse_tests.rs` (16 tests).
- The attest policy record: the kernel's (`src/security/attest_policy/record.rs`)
  against libc's reader (`userland/libc/src/attest_policy.rs`),
  `policy_record_tests.rs` (8).
- The `MkBootAttest` record: the kernel's encoder
  (`src/security/boot/loader_check/record.rs`) against libc's reader,
  `boot_record_tests.rs` (5).
- About's proof board (`capsule_about/src/about/data/proofs/census.rs`,
  `session.rs`, `words.rs`), `proof_board_tests.rs` (16): every process that
  holds a capability is either admitted by the spawn gate or named, a Linux
  guest with an empty mask counts as sandboxed, each half of the headline is
  a pass only when everything under it holds, and no headline claims more than
  the verdict under it.

45 `#[test]` functions in all.

## Running

```sh
cd userland/attest_doc_proofs
cargo test --release
```

CI runs it through `nix flake check` (`tools/nix/checks.nix`), with overflow
checks on and clippy over the library only (the crate is in the `lintLib`
list).
