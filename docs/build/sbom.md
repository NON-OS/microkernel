# SBOM

Get the NONOS bill of materials here, see what it lists and what it leaves out, and find the other records of what the build is made of.

## Get it

```
nix build .#sbom
jq '.components | length' result
```

Not tested in this release.

`nix build .#sbom` writes `nonos.cdx.json`, a CycloneDX 1.5 document (`bom`, `tools/nix/sbom.nix:53-67`). The same file sits in every build's `result/` (`sbom`, `tools/nix/artifacts.nix:66`), is published with every release (`assets`, `.github/workflows/ci-release-artifacts.yml:60-64`), and is kept with the reports of the supply chain workflow, which fails when it is not CycloneDX or lists no component (`jq`, `.github/workflows/ci-supply-chain.yml:37-44`). `make check` ends by printing how many components it lists and how many are pinned by hash (`sbom_out`, `tools/nonos-check-report:92-110`).

It is computed from the pins themselves, the `lockFiles` and the other pin files, not from what one run happened to fetch, so it is the same file on every machine (`tools/nix/sbom.nix:1-6`). No SPDX document is produced.

## What it lists

The document's own component is the operating system `nonos`, at the version in [VERSION](../../VERSION), 0.9.2 (`metadata`, `tools/nix/sbom.nix:57-60`). The components are:

| component | type | identified by |
|---|---|---|
| every crate of every lock file the build reads | `library` | name, version and `pkg:cargo` package URL, with the SHA-256 from the lock for a crates.io crate, or the git URL and commit for a git crate (`crate`, `tools/nix/sbom.nix:12-27`) |
| every entry of [sources.txt](../../tools/nix/sources.txt): the sources of the Linux userland, the Zig and CMake releases, llama.cpp | `library`, or `application` for Zig and CMake | name and download URL, with the SHA-256 when the pin is one (`source`, `tools/nix/sbom.nix:29-35`) |
| the flake inputs nixpkgs, rust-overlay and starks | `framework` | the locked commit, the repository URL and the NAR hash (`flakeInput`, `tools/nix/sbom.nix:37-44`) |
| the toolchain | `application` | the versions of Rust, CMake, clang and Python (`toolchain`, `tools/nix/sbom.nix:46-51`) |

Crates that appear in several locks are listed once for each name, version and source (`crates`, `tools/nix/sbom.nix:8-10`).

The lock files it reads are the kernel's, the loader's, the standard library's for `-Zbuild-std`, those of ripgrep and fd, of every capsule built from source, of the crates.io tools packaged as capsules, of `nonos-rt`, the startup object of `std` capsules, and of the host tools: the signing and enrollment tools, `embed-trailer`, `sign-kernel`, `nonos-mk`, `nonos-pack` and `nonos-verify` (`lockFiles`, `tools/nix/default.nix:26-34`).

## What it leaves out

- Test only crates: the locks of the [proof crates](../overview/glossary.md#proof-crate) and of the attestation test battery are not in `lockFiles`, so their dependencies are listed only when another lock names them too (`lockFiles`, `tools/nix/default.nix:26-34`).
- Nix packages other than the four toolchain entries, such as QEMU, swtpm, Go or the image tools. They come from nixpkgs at the commit the `nixpkgs` component names, and the document does not list them one by one.
- A hash for llama.cpp: its pin is a Nix tree hash rather than a SHA-256 hex string, so it is listed with its URL only (`source`, `tools/nix/sbom.nix:29-35`).

## The cargo SBOM and the policy checks

The supply chain workflow also runs `nonos-verify supply-chain` in the development shell (`.github/workflows/ci-supply-chain.yml`). From the repository root, it runs (`run`, `nonos-verify/src/supply_chain.rs:9-76`):

| step | what it does |
|---|---|
| `cargo audit --json` | the RustSec advisory scan |
| `cargo deny check` | the policy in [deny.toml](../../deny.toml) |
| `cargo tree --workspace --duplicates` | records crates present in more than one version |
| `git submodule status --recursive` | fails when a submodule is off its committed pin |
| `cargo cyclonedx --format json` | a second, cargo generated CycloneDX SBOM |

The tools come from the flake's shell (`tools/nix/shell.nix`).

The `deny.toml` policy, for the kernel crate:

- licences: only those on the allow list are accepted, among them `AGPL-3.0`, `Apache-2.0`, `MIT`, `BSD-3-Clause`, `ISC`, `MPL-2.0` and `Zlib` (`allow`, `deny.toml:29-50`);
- bans: `openssl`, `openssl-sys` and `time` below 0.3 are refused, and a wildcard version is refused (`deny`, `deny.toml:58-70`);
- sources: crates.io and the STARKs git repository only (`sources`, `deny.toml:74-78`);
- advisories: yanked crates are refused, and three entries are tolerated with a written reason each, two unmaintained crates and the yanked `spin` 0.9.8 (`ignore`, `deny.toml:14-27`).
