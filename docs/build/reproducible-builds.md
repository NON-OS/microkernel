# Reproducible builds

Check a NONOS build yourself: what is pinned, where the reproducible boundary runs, how to compare two builds, and what is not reproducible yet.

## The boundary

The artifacts in `result/` are built to be reproducible: the same commit and the same `nonos.toml` should give the same bytes on any machine. Enrollment and signing are not, because the [seal](../overview/glossary.md#seal) draws fresh randomness for every STARK proof and signs with keys the build never sees (`BOUNDARY`, `tools/nix/manifest.py:25-29`). The boundary is the tree the flake's `artifacts` function writes: the kernel ELF, every [capsule](../overview/glossary.md#capsule) ELF, the Linux userland, the loader EFI binary and the bill of materials (`artifacts`, `tools/nix/artifacts.nix:1-15`).

The sealed image still traces back to the source. The seal's last check rebuilds the kernel and the loader from the tree with `nix build` and stops unless they are the bytes it sealed (`reproduced`, `tools/nonos_seal/verify.py:70-86`).

## What is pinned

| what | pinned by |
|---|---|
| nixpkgs, rust-overlay and STARKs | commit and NAR hash in [flake.lock](../../flake.lock) |
| Rust | the nightly in [rust-toolchain.toml](../../rust-toolchain.toml), read by the flake through rust-overlay |
| every crate | the sha256 in the crate's own `Cargo.lock`; each crate is a fixed output derivation keyed by it (`vendor.nix`, `tools/nix/vendor.nix:1-14`) |
| git dependencies | STARKs by the flake input, and every other one by tree hash in [git-sources.json](../../tools/nix/git-sources.json) |
| C sources and the Zig, CMake and llama.cpp releases | name, URL and hash in [sources.txt](../../tools/nix/sources.txt), 47 entries: a sha256 for each tarball, a commit and Nix tree hash for llama.cpp |
| CMake, Go and clang from nixpkgs | a version asserted when the flake evaluates (`assert`, `tools/nix/pins.nix:71-74`) |
| the standard library's lock for `-Zbuild-std` | a committed copy, held to the toolchain's by the `rust-src-lock` check (`rustSrcLock`, `tools/nix/checks.nix:293-298`) |
| the committed BusyBox binary | held to a BusyBox built from source by the `busybox-source` check (`busybox`, `tools/nix/checks.nix:287-292`) |
| GitHub Actions | each action by commit hash, such as `actions/checkout@3d3c42e5aac5ba805825da76410c181273ba90b1` |
| the devcontainer | the Debian base image by digest, and Nix 2.34.6 by the sha256 of its installer (`NIX_INSTALLER_SHA256`, `.devcontainer/Dockerfile:6-27`) |

Every `Cargo.lock` that uses STARKs must name the commit `flake.lock` pins, or the `starks-pin` check fails (`starks`, `tools/nix/checks.nix:274-276`).
