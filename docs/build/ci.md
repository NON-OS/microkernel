# CI

See what GitHub runs for a pull request, a push, a schedule and a tag, what `make check` covers on your machine, what `[skip ci]` does, and how the checks stood at this commit.

## On a pull request

Four workflows start on a pull request: `ci`, `verify`, `ci-boot-smoke` and `lean` (`pull_request`, `.github/workflows/ci.yml:7-10`). A newer push to the same pull request cancels the older run; a run on `main` always finishes (`concurrency`, `.github/workflows/ci.yml:19-23`).

### ci

```mermaid
flowchart LR
    build --> attestation
    ledger[trust-ledger] --> attestation
    reproducible --> attestation
    supply[supply-chain] --> attestation
    chain[trust-chain] --> attestation
    adversarial --> attestation
    evidence --> attestation
    benchmark --> attestation
```

`ci.yml` composes reusable workflows, and its `attestation` job fuses their reports and fails when a blocking one failed (`attestation`, `.github/workflows/ci.yml:86-91`):

| job | what it does |
|---|---|
| `build` | `nix build .#default` on Linux x86_64, Linux aarch64 and macOS; `make build` and `make check` inside WSL2 on Windows; `nix build` inside the devcontainer; and the `nonos-verify build` checks (`platforms`, `.github/workflows/ci-build.yml:39-49`) |
| `build-aarch64`, `boot-aarch64` | the aarch64 kernel through `make nonos-mk-arm`, checked as a loadable ELF, then booted under QEMU with each serial log held to its expected lines (`.github/workflows/ci-build-aarch64.yml`) |
| `trust-ledger` | `sha256sum -c MANIFEST.sha256` over the committed [trust set](../overview/glossary.md#trust-set) (`run`, `.github/workflows/ci.yml:41-50`) |
| `reproducible` | the three-machine comparison of [reproducible-builds.md](reproducible-builds.md) (`reproducible`, `.github/workflows/ci.yml:55-59`) |
| `production-build` | the build a release tag runs, against the committed trust set; skipped for Dependabot (`production`, `.github/workflows/ci.yml:64-69`) |
| `supply-chain` | the bill of materials and the policy checks of [sbom.md](sbom.md) |
| `trust-chain` | `nonos-verify trust-chain`: every embedded [capsule](../overview/glossary.md#capsule)'s certificate and manifest checked against the trust anchor policy, through `nonos_capsule_sign`, as the kernel checks it at spawn (`nonos_capsule_sign`, `.github/workflows/ci-trust-chain.yml:3-6`) |
| `adversarial` | `nonos-verify adversarial`: a valid signed capsule tampered with in many ways, each of which must be refused (`adversarial`, `.github/workflows/ci-adversarial.yml:3-7`) |
| `evidence`, `benchmark` | the evidence and benchmark reports |

### verify

`verify.yml` runs `nix flake check -L --keep-going` on Linux and on macOS, one `matrix` entry each, so every [proof crate](../overview/glossary.md#proof-crate) and static check of [nix-flake.md](nix-flake.md) runs on both (`.github/workflows/verify.yml:39-60`). Beside it run the checkers that pin their own toolchains:

| job | what it checks |
|---|---|
| `kani` | Kani 0.67.0 over `userland/fs_proofs` (`kani`, `.github/workflows/verify.yml:62-72`) |
| `verus` | the capability theorems in `verification/verus` (`verus`, `.github/workflows/verify.yml:74-92`) |
| `proof-crates-kani` | Kani over fifteen proof crates, the trailer parser and the loader's rollback floor (`crate`, `.github/workflows/verify.yml:94-122`) |
| `lean` | the Lean specification theorems (`lean`, `.github/workflows/verify.yml:124-149`) |
| `extraction` | the Charon and Aeneas extraction, checked for drift, and its refinement theorems (`extraction`, `.github/workflows/verify.yml:151-194`) |

### ci-boot-smoke

The boot test of every pull request; its header calls it the blocking boot check. It makes scratch keys that exist only on the runner, [seals](../overview/glossary.md#seal) the qemu profile, and boots it headless with a software TPM and no network (`Seal`, `.github/workflows/ci-boot-smoke.yml:82-96`). The boot passes when the serial console shows `Handoff OK`, `Capsules spawned`, the [package store](../overview/glossary.md#package-store) being served, and `[BOOT-ATTEST] bootloader measured and enrolled`. A second boot plugs the same image into the xHCI controller as a USB stick and expects the first three lines again, then the USB mass storage driver binding and the NONOS disk found on it (`expect`, `.github/workflows/ci-boot-smoke.yml:100-106`). With KVM the deadline is 300 seconds; on a runner without KVM the boot runs under TCG with 900 seconds (`BOOT_TIMEOUT`, `.github/workflows/ci-boot-smoke.yml:74-81`).

### lean

`lean.yml` builds the Lean 4 specification in `verification/lean` and fails when a profiled theorem depends on `sorryAx` (`sorryAx`, `.github/workflows/lean.yml:49-60`).

## On a push, a schedule or a tag

| workflow | when | what it does |
|---|---|---|
| `ci`, `verify`, `lean` | push to `main` or `develop` | the same as on a pull request |
| `ci-boot-smoke` | push to `main` | the same as on a pull request |
| `nightly` | daily, 06:00 UTC | build, trust chain, adversarial, supply chain, evidence, the QEMU runtime checks and the reproducibility comparison (`cron`, `.github/workflows/nightly.yml:7-10`) |
| `ci-boot-matrix` | daily, 06:00 UTC | boots every cell of the machine matrix; its header says it is not a merge check and that the SMP cells are known to fail (`schedule`, `.github/workflows/ci-boot-matrix.yml:3-14`) |
| `fuzz` | daily, 01:00 UTC | 30 minutes per fuzz target over the parsers of untrusted input; a crash fails the run and keeps its input (`cron`, `.github/workflows/fuzz.yml:3-12`) |
| `benchmark` | daily, 06:30 UTC | the benchmark suite (`cron`, `.github/workflows/benchmark.yml:27`) |
| `starks-bump` | daily, 05:17 UTC | moves the `starks` input to STARKs main, syncs every lock, runs `nix flake check` and opens a pull request; it never merges (`cron`, `.github/workflows/starks-bump.yml:3-14`) |
| `release` | a tag `v*` | every blocking module in production mode, the release bundle, one attestation, then a pre-release with signed build provenance (`tags`, `.github/workflows/release.yml:9-14`) |
| `cut-release` | by hand | points a version tag at a commit and starts `release` on it (`version`, `.github/workflows/cut-release.yml:9-20`) |
| `build-for-enrollment` | by hand | builds the capsule set with `nix build .#capsules` and publishes the ELFs with a `BLAKE3` manifest; it signs nothing (`BLAKE3`, `.github/workflows/build-for-enrollment.yml:3-9`) |
| `ci-windows-virtualbox` | by hand | boots a released ISO under VirtualBox on a Windows runner and keeps its serial log (`VirtualBox`, `.github/workflows/ci-windows-virtualbox.yml:1-7`) |

Dependabot looks for cargo and GitHub Actions updates once a week, for the root directory only, and keeps at most five of its pull requests open for each (`updates`, `.github/dependabot.yml:1-13`).

Only a push to the repository may write the binary cache. A pull request reads it and never writes it, so its code cannot place a store path others will fetch (`CACHIX_AUTH_TOKEN`, `.github/workflows/verify.yml:53-58`).

## On your machine

`make check` builds every flake check for your host, the same set `verify.yml` builds, and prints what each one proved; [nix-flake.md](nix-flake.md) lists them and shows how to run one. It does not run Kani, Verus, Lean, the extraction, the fuzzers or the boot smoke test; [make-targets.md](make-targets.md) lists the boot targets you can run yourself.

## `[skip ci]`

GitHub documents that it starts no `push` or `pull_request` workflow for a commit whose message contains `[skip ci]`; scheduled and manual workflows still run. Count how many recent commits on your branch carry it:

```
git log -300 --format=%s | grep -c '\[skip ci\]'
```

On this release's history the command prints 300: every one of the latest 300 commits carries it, so by GitHub's rule no push workflow ran on them. For those commits the record of the checks is a local run of the flake checks, and the run below is that record for this commit.

`cut-release` refuses to tag a commit with a completed check run that did not succeed, and counts nothing else, so a commit on which no workflow ran passes that test (`verdict`, `.github/workflows/cut-release.yml:40-54`).

## State of the checks at this commit

The flake checks of this commit were built on one x86_64-linux machine for this release, the same set `make check` builds. Of the 143 checks, 142 evaluated, 133 passed, 9 failed, and 1 did not evaluate. The 110 proof crates that passed ran 6,932 tests.

| check | state | why |
|---|---|---|
| `proofs-rtl8169_proofs` | failed | its tests passed; clippy then refused a hand-written `div_ceil` near `leading_zeros` in `userland/capsule_driver_rtl8169/src/log/line.rs:41` |
| `proofs-usb_msc_proofs` | failed | clippy asks for a `Default` beside the `new` in `userland/capsule_driver_usb_msc/src/state/types.rs:38` |
| `proofs-xhci_proofs` | failed | clippy refuses two assertions in its tests, in `userland/xhci_proofs/src/conformance/silicon_tests.rs` and `userland/xhci_proofs/src/event_ring/address_tests.rs` |
| `static-hygiene` | failed | `scripts/check_stubs.py` counts 15 new admissions of unsupported work against its baseline |
| `static-abi` | failed | `scripts/check_prebuilt.py` finds two binaries it cannot classify: the market index and the model catalogue under `nonos-data/` |
| `static-tree` | failed | `nonos-ci/run-static-checks.sh` refuses the `read` and `write` imports in `userland/capsule_driver_ahci/src/server/handlers/emmc/dispatch.rs:29` |
| `inputs` | failed | `tools/nix/inputs.json` did not match what the checked tree reads for `userland/capsule_market`, `userland/capsule_model_fetch` and `userland/model_fetch_proofs` |
| `nonos-verify` | failed | `nonos-verify hygiene` failed, and the end of its log does not say why |
| `busybox-source` | failed | it did not run: the machine running the checks could not download the pinned Zig compiler |
| `proofs-model_fetch_proofs` | did not evaluate | its derivation did not evaluate; `crate` computes its source from its entry in `tools/nix/inputs.json` (`crate`, `tools/nix/src.nix:22-34`) |

The `inputs` failure and the missing proof check concern the same three crates, and the committed entry of each names `.keys/marketplace_operator_ed25519.pub`, the market operator's public key. In a tree without that file, `crate` cannot build the source of these crates and the regenerated table no longer matches, which gives exactly these two results. They may therefore come from the tree the checks ran on rather than from the commit; a run on a full checkout is not tested in this release.

Every other check passed, among them all six `kernel-profile-*` and all six `kernel-features-*` checks, both live TPM suites, and the `starks-pin`, `git-pins`, `catalogues` and `rust-src-lock` drift checks. No workflow run of this commit on GitHub is recorded here.

## See also

- [The Nix flake](nix-flake.md)
- [Reproducible builds](reproducible-builds.md)
- [Tests and proofs](../contributing/tests-and-proofs.md)
- [Commits](../contributing/commits.md)
- [Review](../contributing/review.md)
