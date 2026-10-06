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

## Run the checks

```
make check
```

Not tested in this release.

The `check` target runs `nix run .#check-report`, which builds every check for this machine and prints what each one proved (`Makefile:58-61`). One check can be built and its log streamed on its own:

```
nix build .#checks.x86_64-linux.proofs-ps2_input_proofs -L
```

Not tested in this release.

On a macOS host the two TPM proof crates are left out, because the software TPM tools they drive build only for Linux (`needsTpm`, `tools/nix/checks.nix:99-102`).

## State of the checks at this commit

The flake defines 143 checks for `x86_64-linux` at this commit. In a run of all of them on x86_64 Linux, `proofs-model_fetch_proofs` did not evaluate, so it was not built; the `inputs` failure below names the same crate. The other 142 were built and 133 passed, among them every `kernel-features-*` and `kernel-profile-*` check and 110 of the 113 `proofs-*` checks. Nine failed:

| Check | What failed |
|---|---|
| `inputs` | `tools/nix/inputs.json` is stale for `userland/capsule_market`, `userland/capsule_model_fetch` and `userland/model_fetch_proofs` |
| `static-abi` | `scripts/check_prebuilt.py` finds two binaries it cannot classify: `nonos-data/market/index.bin` and `nonos-data/models/catalogue.bin`. The scripts after it in the list did not run |
| `static-hygiene` | `scripts/check_stubs.py` finds 15 admissions that are not in its baseline. The scripts after it did not run |
| `static-tree` | several gates: the `cfg(target_arch` count is 234 against a baseline of 116, the `crate::arch::x86_64::` count 135 against 100, and the end of its log shows the forbidden `read` and `write` import at `userland/capsule_driver_ahci/src/server/handlers/emmc/dispatch.rs:29` |
| `nonos-verify` | clippy passes, then the `hygiene` scan fails. It writes its findings to a file, so the log ends as the scan starts. Its comment patterns match shipping comments such as the "for now" in `src/hardware/inventory/family.rs` |
| `proofs-rtl8169_proofs` | its 67 tests pass; clippy's `manual_div_ceil` lint fails on `userland/capsule_driver_rtl8169/src/log/line.rs` |
| `proofs-usb_msc_proofs` | clippy's `new_without_default` lint fails on `new` at `userland/capsule_driver_usb_msc/src/state/types.rs:38` |
| `proofs-xhci_proofs` | clippy's `int_plus_one` and `assertions_on_constants` lints fail in two test files of the crate |
| `busybox-source` | failed; its log kept no lines |

`scripts/check_allows.py` runs after `check_stubs.py` inside `static-hygiene`, so the failure above stops it. Run on its own against this tree, it passes its self-test and then reports 31 lint switches that are not in its baseline.
