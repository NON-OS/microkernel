# Code style: Errors, panics and admissions

Part of [Code style](code-style.md).

## Errors, panics and admissions

Shipping code does not panic. The same scan fails on every pattern in
`CODE_PATTERNS`: `.unwrap(`, `.expect(`, `panic!`, `todo!(`, `unimplemented!(`,
`unreachable!(` and `#[allow(dead_code)]`
(`nonos-verify/src/hygiene/patterns.rs:18-26`). Shipping means the trees in
`root_dirs`, the kernel, `userland` and four boot and attestation crates, less
what `skip` leaves out: build output, [proof
crates](../overview/glossary-peer-list.md#proof-crate), tests, vendored and
upstream sources, and `build.rs` files
(`nonos-verify/src/hygiene/roots.rs:5-31`). Return an error the caller can act
on, and when bring-up cannot go on, say on the console why. The scan fails at
this commit; [Tests and
proofs](tests-and-proofs-state-of-the.md#state-of-the-checks-at-this-commit) says where.

Three gates hold sites against a
[baseline](../overview/glossary-amnesic-boot.md#baseline) that may only shrink:

- Lint switches: `MARK` finds every `#[allow(` and `#![allow(` in `src` and
  `userland`, and each one must be listed in `scripts/baselines/allows.txt`
  (`scripts/check_allows.py:31-34`).
- Admissions: `MARK` finds words such as stub, unsupported and not implemented
  in shipping Rust, and each site must be listed in
  `scripts/baselines/stubs.txt` (`scripts/check_stubs.py:30-32`). The gate
  matches words, not intent: a refusal that names the chip it will not drive
  passes, and one that says "not implemented" is listed.
- Unreachable exports: `unreachable` finds every kernel `pub fn` that no other
  file mentions outside an import, and each one must be listed in
  `scripts/baselines/unreachable.txt` (`scripts/check_unreachable.py:26-35`).
  Its docstring gives the reason: a mechanism with no caller can be reviewed,
  merged and shipped without one line of it running.

Every proof crate must pass `cargo clippy` with `-D warnings` over all its
targets, except the crates named in `lintLib` and `lintNone`, two lists that
only shrink and that a new crate never joins (`tools/nix/checks.nix:44-62`).
Three proof crates fail that at this commit; [Tests and
proofs](tests-and-proofs-state-of-the.md#state-of-the-checks-at-this-commit) names them.

## Architecture boundaries

Shared kernel code reaches the CPU through the `ArchOps` trait and the `Arch`
alias, not through `crate::arch::x86_64` paths (`src/arch/abi.rs:17-36`). Two
counts are meant only to shrink:

- `cfg(target_arch` sites outside `src/arch`, counted into `cfg_count` against
  116 in `nonos-ci/baselines/cfg-target-arch-count.txt`
  (`nonos-ci/run-static-checks.sh:46-47`).
- `crate::arch::x86_64::` paths outside `src/arch`, counted into
  `arch_leak_count` against 100 in `nonos-ci/baselines/arch-x86_64-uses.txt`
  (`nonos-ci/run-static-checks.sh:55-60`).

Both have grown past their baselines. The same `grep` the script runs counts 234
and 135 at this commit, so `static-tree` fails on both. Do not add to either.

[Architectures](../architectures/README.md) describes the boundary.
