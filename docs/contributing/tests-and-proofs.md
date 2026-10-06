# Tests and proofs

How NONOS checks its own code, how to run each kind of check, and which checks fail at this commit.

## What runs where

| Kind | Where it lives | What runs it |
|---|---|---|
| proof crates | `userland/*_proofs`, and a few host crates elsewhere | `nix flake check` |
| kernel feature and profile checks | the kernel crate | `nix flake check` |
| static checks | `scripts/`, `nonos-ci/`, `tools/` | `nix flake check` |
| drift checks | `tools/nix/` | `nix flake check` |
| Kani harnesses | `kani_proofs.rs` and similar files in proof crates | `.github/workflows/verify.yml` |
| Lean specification | `verification/lean/` | `.github/workflows/lean.yml`, `.github/workflows/verify.yml` |
| Charon and Aeneas extraction | `verification/extraction/` | `.github/workflows/verify.yml` |
| Verus | `verification/verus/` | `.github/workflows/verify.yml` |
| fuzz targets | the `fuzz/` directory of a few crates | `.github/workflows/fuzz.yml` |
| QEMU boot | the `qemu` profile | `.github/workflows/ci-boot-smoke.yml` |

The flake's checks are `proofChecks`, `cargoChecks`, `profileChecks`, `staticChecks` and `driftChecks`, each check its own derivation, so a failure names itself and a pass is cached until its inputs change (`tools/nix/checks.nix:303`). The comment at the top of `tools/nix/checks.nix` says why Kani, Verus, the extraction and fuzzing stay outside the flake: each needs a network or a toolchain no lock pins yet.
