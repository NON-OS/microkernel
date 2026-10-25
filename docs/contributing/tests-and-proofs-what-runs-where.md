# Tests and proofs: What runs where

Part of [Tests and proofs](tests-and-proofs.md).

## What runs where

| Kind | Where it lives | What runs it |
|---|---|---|
| [proof crates](../overview/glossary-peer-list.md#proof-crate) | `userland/*_proofs`, and a few host crates elsewhere | `nix flake check` |
| kernel feature and profile checks | the kernel crate | `nix flake check` |
| static checks | `scripts/`, `nonos-ci/`, `tools/` | `nix flake check` |
| drift checks | `tools/nix/` | `nix flake check` |
| Kani harnesses | `kani_proofs.rs` and similar files in proof crates | `.github/workflows/verify.yml` |
| Lean specification | `verification/lean/` | `.github/workflows/lean.yml`, `.github/workflows/verify.yml` |
| Charon and Aeneas extraction | `verification/extraction/` | `.github/workflows/verify.yml` |
| Verus | `verification/verus/` | `.github/workflows/verify.yml` |
| fuzz targets | the `fuzz/` directory of a few crates | `.github/workflows/fuzz.yml` |
| QEMU boot | the `qemu` profile | `.github/workflows/ci-boot-smoke.yml` |

The flake's checks are `proofChecks`, `cargoChecks`, `profileChecks`,
`staticChecks` and `driftChecks`, each check its own derivation, so a failure
names itself and a pass is cached until its inputs change
(`tools/nix/checks.nix:303`). The comment at the top of `tools/nix/checks.nix`
says why Kani, Verus, the extraction and fuzzing stay outside the flake: each
needs a network or a toolchain no lock pins yet.

## Run the checks

```
make check
```

Not tested in this release.

The `check` target runs `nix run .#check-report`, which builds every check for
this machine and prints what each one proved (`Makefile:58-61`). One check can
be built and its log streamed on its own:

```
nix build .#checks.x86_64-linux.proofs-ps2_input_proofs -L
```

Not tested in this release.

On a macOS host the two [TPM](../overview/glossary-stark-proof.md#tpm) proof
crates are left out, because the software TPM tools they drive build only for
Linux (`needsTpm`, `tools/nix/checks.nix:99-102`).

## What to run for a change

- Kernel or capsule code: the proof crate that mounts it, and `nix flake check`.
  [Proof crates](tests-and-proofs-proof-crates.md#proof-crates) shows how to run one crate by hand.
- A driver: its proof crate, extended to cover the change.
- A function a refinement theorem names: the extraction job and the Lean build,
  as in [Extraction and Verus](tests-and-proofs-lean.md#extraction-and-verus) and [Lean](tests-and-proofs-lean.md#lean).
- A changed ABI number or capability bit: `static-abi`, one of the [static
  checks](tests-and-proofs-static-checks.md#static-checks).
- A parser of untrusted input that has a fuzz target: that target, for a few
  minutes, as in [Fuzzing](tests-and-proofs-lean.md#fuzzing).
