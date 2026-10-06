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
