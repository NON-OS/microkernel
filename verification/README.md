# NONOS verification

What NONOS checks about its own code, in three layers here and more in the
tree. The full inventory, with counts and what each piece does not cover, is
[the proofs page](../docs/handbook/verification/proofs.md).

## 1. Runnable proofs

A host crate that includes the **real capsule source** via `#[path]` (only the
syscall clock is shimmed) and executes it with `cargo test`:

- **Store operations**: mkdir-p, chmod enforcement, truncate zero-fill,
  recursive copy/rmdir, fd reindex, child counts, usage, mtime.
- **Path security**: canonicalization and the `/capsules` read-only guard,
  including slash-smuggling.
- **Protocol codec**: hostile/malformed input handling.
- **Caller attestation**: userspace impersonation is rejected.
- **File-manager logic**: listing parse, dedup, type classification.
- **Wire parser proofs**: Ethernet, IPv4, and UDP parser bounds and round trips.
- **Fuzz proofs**: millions of structured and random inputs asserting the
  parsers never panic and never violate their invariants (no-impersonation,
  path canonicalization).

Writing these already found and fixed real bugs (directory dedup, dotfile
classification). Run:

```sh
cd userland/fs_proofs
cargo test --release
```

Run it inside `nix develop`. `nix flake check` runs this crate and every other
`userland/*_proofs` crate, as `proofs-<crate>`.

## 2. Kani

`userland/fs_proofs/src/kani_proofs.rs` proves, over every input (bounded), that
the untrusted-input surfaces are **panic-free and UB-free**, plus the
**authority** theorem (no userspace impersonation) and the **canonicalization**
theorem (every normalized path is rooted and slash-clean). It also proves
bounded Ethernet, IPv4, and UDP parser payload bounds. Run:

```sh
cd userland/fs_proofs
cargo kani --output-format terse
```

## 3. Verus

`verification/verus` proves theorems about the capability bit operations,
page-table permission encoding and IPC message length guards, as restated in
its spec functions. It includes no kernel file, so a kernel change does not
reach it. Run:

```sh
cd verification/verus
verus --crate-type=lib src/lib.rs
```

## What is and is not tied to the code

The runnable proofs and Kani compile the shipped source through `#[path]`, so
a change to that source reaches them. Verus restates the kernel's bit
operations and rules in its own spec functions, so it proves the restatement,
and a drift between the two is not caught there. The Lean specification in
`lean/` is tied to the code by extraction for some modules, by proof crates for
others, and not at all for the rest; `lean/REFINEMENT.md` names which.

## CI

`.github/workflows/verify.yml` runs `nix flake check`, which holds every proof
crate's tests, and separate jobs for Kani (`kani` on `fs_proofs`,
`proof-crates-kani` on fifteen more crates, `nonos-attest-path` and the
loader's `boot_proofs`), Verus, the Lean specification and the extraction.
`.github/workflows/lean.yml` builds the Lean specification again and fails on
any `sorryAx` in its axiom profile.
