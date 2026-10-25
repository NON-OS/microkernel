# Tests and proofs: Proof crates

Part of [Tests and proofs](tests-and-proofs.md).

## Proof crates

A [proof crate](../overview/glossary-peer-list.md#proof-crate) is a host crate
that compiles shipping source with `#[path]` and runs it under `cargo test`,
with the system calls that source makes answered by a shim.
`userland/ps2_input_proofs` is a small one. It mounts the PS/2 driver's
`constants`, `discover`, `init` and `setup` modules from the
[capsule](../overview/glossary-boot-root-record.md#capsule) source
(`userland/ps2_input_proofs/src/lib.rs:32-40`) and replaces `nonos_libc` with a
shim that models the controller (`userland/ps2_input_proofs/Cargo.toml:19-22`).
Its 38 tests pass at this commit.

There are 107 crates under `userland/*_proofs`, counted as `proof_crates`
(`verification/evidence/EVIDENCE.json:2184`). The flake runs each one the same
way: one test thread through `RUST_TEST_THREADS`, a release build with overflow
checks on, then clippy (`tools/nix/checks.nix:85-94`). Overflow checks stay on
because capsules ship without them: on a device an overflow wraps quietly, and a
proof built the same way would agree with the wrapped value and pass. The
comment above the script names the case that taught this, a 1968 timestamp in
`net.ntp` that wrapped the clock while its proof stayed green
(`tools/nix/checks.nix:81-84`).

To run one crate the same way by hand:

```
nix develop
cargo test --release --manifest-path userland/ps2_input_proofs/Cargo.toml \
    --config profile.release.overflow-checks=true
```

Not tested in this release.

`tpm_enroll_proofs` and `tpm_key_proofs` run against a software TPM, and a live
test that skips fails the check instead of passing quietly (`needsTpm`,
`tools/nix/checks.nix:32-34`).

### Adding a proof crate

1. Put it at `userland/<name>_proofs` and commit its `Cargo.lock`. `proofDirs`
   picks up every directory under `userland` whose name ends in `_proofs` and
   that has a lock (`tools/nix/checks.nix:18-21`).
2. Mount the shipping source with `#[path]`; a copy would test code that does
   not ship.
3. Regenerate `tools/nix/inputs.json` with `python3 tools/nix/inputs.py`. The
   `inputs` drift check runs it with `--check` and fails when a path dependency
   or a `#[path]` was added without it (`tools/nix/checks.nix:265-269`).
4. Make it pass clippy with `-D warnings` over all targets. `lintLib` and
   `lintNone` take no new members (`tools/nix/checks.nix:44-54`).
5. A new [driver capsule](../overview/glossary-dma-pool.md#driver-capsule) ships
   with its proof crate;
   [Contributing](contributing-where-code-goes.md#where-code-goes) gives the
   rule.
