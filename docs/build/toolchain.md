# Toolchain

Set up a machine to build NONOS, and look up the one pinned version of each compiler and tool the build uses.

## On your machine

The build needs Nix with flakes turned on. Beyond Nix, you need `git` to fetch the source and GNU `make` to type the short commands of the [Makefile](../../Makefile). Every compiler and tool the build runs comes from the flake, and nothing in the development shell comes from the host (`tools`, `tools/nix/shell.nix:1-14`).

| host | how |
|---|---|
| Linux on x86_64 or aarch64 | install Nix, turn on flakes |
| macOS on Apple silicon | install Nix, turn on flakes |
| Windows | WSL2 with a Linux distribution in it, then the Linux steps; or the devcontainer |

The flake evaluates for the hosts its `systems` list names: `x86_64-linux`, `aarch64-linux` and `aarch64-darwin` (`flake.nix:38-39`). An Intel Mac is not a host, because nixpkgs no longer builds for it. CI builds on Windows inside WSL2 with Ubuntu 24.04, installing only `curl`, `xz-utils`, `git`, `make` and `ca-certificates` before Nix (`additional-packages`, `.github/workflows/ci-build.yml:92-95`).

The devcontainer in `.devcontainer/` is a pinned Debian bookworm image with `git`, `make`, `curl` and Nix; every build tool still comes from the flake (`apt-get`, `.devcontainer/Dockerfile:6-10`). It runs `privileged`, because Nix's sandbox needs namespaces a default container does not grant (`.devcontainer/devcontainer.json:5`).

## Nix

Install Nix from [nixos.org](https://nixos.org/download), then turn on flakes. The `doctor` target prints the line to add when they are off (`Makefile:127-129`):

```
mkdir -p ~/.config/nix
echo 'experimental-features = nix-command flakes' >> ~/.config/nix/nix.conf
```

Not tested in this release.

The repository names no minimum Nix version. The devcontainer installs Nix 2.34.6, with the installer held to its sha256 (`NIX_INSTALLER_SHA256`, `.devcontainer/Dockerfile:22-27`). The commands on these pages not marked as untested were run with Nix 2.34.6. CI installs Nix with flakes and `sandbox = true` (`extra_nix_config`, `.github/actions/nix-setup/action.yml:27-33`).

Check the machine:

```
make doctor
```

It says `ok    nix, with flakes` when Nix answers with flakes on, then reports hardware virtualization, and ends with `This machine can build NONOS: make` (`doctor`, `Makefile:124-136`).

## Rust

`rust-toolchain.toml` pins the `channel` to `nightly-2026-01-16` with the `rust-src`, `llvm-tools-preview`, `clippy` and `rustfmt` components and the `minimal` profile (`rust-toolchain.toml:1-4`). The flake reads that same file through rust-overlay (`fromRustupToolchainFile`, `tools/nix/pins.nix:10`), so rustup and the flake cannot drift apart.

The flake derives four toolchains from the one pin:

| toolchain | targets added | used for |
|---|---|---|
| `rust`, `tools/nix/pins.nix:10` | none | the kernel, the [capsules](../overview/glossary.md#capsule) and the host tools |
| `rustUefi`, `tools/nix/pins.nix:13` | `x86_64-unknown-uefi` | the bootloader |
| `rustShell`, `tools/nix/pins.nix:16` | `x86_64-unknown-uefi`, `wasm32-unknown-unknown` | the development shell and the STARK verifier for the web |
| `rustMusl`, `tools/nix/pins.nix:19` | `x86_64-unknown-linux-musl` | the Rust programs of the Linux userland, such as ripgrep and fd |

The bootloader has its own `rust-toolchain.toml` with the same `channel` and the UEFI target (`nonos-bootloader/rust-toolchain.toml:1-4`).

Capsules that use `std` build against a copy of the pinned standard library with the NONOS platform layer of `toolchain/nonos-std` applied; the pinned toolchain itself is never patched (`mkRustStd`, `tools/nix/capsules.nix:16-20`).
