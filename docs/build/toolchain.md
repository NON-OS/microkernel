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

## Target files

NONOS targets are custom target files, built with `-Zbuild-std`:

| file | what it builds | settings that matter |
|---|---|---|
| [x86_64-nonos.json](../../x86_64-nonos.json) | the x86_64 kernel | `features` turns off MMX, SSE and AVX and turns on soft float (`x86_64-nonos.json:21-22`) |
| [aarch64-nonos.json](../../aarch64-nonos.json) | the aarch64 kernel | `features` asks for strict alignment, pointer authentication, MTE and RNDR (`aarch64-nonos.json:21`) |
| [userland/x86_64-nonos-user.json](../../userland/x86_64-nonos-user.json) | every capsule on x86_64 | `features` keeps SSE and SSE2 (`userland/x86_64-nonos-user.json:21`) |
| [userland/aarch64-nonos-user.json](../../userland/aarch64-nonos-user.json) | capsules for aarch64 | an aarch64 user target |
| [userland/riscv64-nonos-user.json](../../userland/riscv64-nonos-user.json) | capsules for riscv64 | a riscv64 user target |

The kernel target links with `rust-lld`, aborts on panic, disables the red zone and uses the `kernel` code model with static relocation (`linker`, `x86_64-nonos.json:15-32`). The x86_64 user target builds position independent executables and keeps the red zone (`relocation`, `userland/x86_64-nonos-user.json:16-33`).

The flake builds the kernel for `x86_64-nonos.json` with `-Zbuild-std=core,alloc` and `compiler-builtins-mem` (`cargo`, `tools/nix/image.nix:116-120`), every capsule for the `x86_64-nonos-user` target (`userTarget`, `tools/nix/capsules.nix:11`), and the loader for `x86_64-unknown-uefi` (`cargo`, `tools/nix/image.nix:161-162`). The tree has no riscv64 kernel target file; [the architectures pages](../architectures/README.md) say what each architecture runs.

## Other compilers and tools

CMake, Go and clang come from nixpkgs, and each version below is asserted when the flake evaluates, so a nixpkgs update that moves one fails at once (`assert`, `tools/nix/pins.nix:71-74`). Zig does not come from nixpkgs: the build uses the 0.16.0 release tarball from ziglang.org for the host, held to the sha256 in `tools/nix/sources.txt` (`zigPlatform`, `tools/nix/userland.nix:20-30`).

| tool | version | builds |
|---|---|---|
| Zig | 0.16.0, the ziglang.org release | the C programs of the Linux userland, and BusyBox |
| CMake | 4.4.3 | the CMake builds of the Linux userland |
| Go | 1.26.8 | the Go programs of the Linux userland, such as gojq |
| clang | major version 21 | every C file built into a NONOS binary, such as the kernel's PQClean code |

Python is 3.12, with the `cryptography` package for the scripts that need it (`pythonTools`, `tools/nix/pins.nix:77-81`). On Linux the software TPM is swtpm 0.9.0 on libtpms 0.9.6, the released pair the live TPM proofs run against; on macOS it is the swtpm nixpkgs builds (`swtpm`, `tools/nix/pins.nix:47-51`). QEMU and its UEFI firmware come from nixpkgs, and the firmware is QEMU's own build of edk2 (`firmware`, `tools/nix/shell.nix:9-10`).

## The development shell

```
make shell
```

Not tested in this release.

`make shell` runs `nix develop` (`shell`, `Makefile:104-105`). The shell carries the pinned Rust with its standard library layer, Zig, CMake, Python, clang and LLVM, QEMU, swtpm, the image tools (`xorriso`, `mtools`, `gptfdisk`, `dosfstools`), the signing and hashing tools (`osslsigncode`, `openssl`, `b3sum`), Node.js, `cargo-audit`, `cargo-deny`, `cargo-cyclonedx`, GNU make, git, jq and perl (`tools`, `tools/nix/shell.nix:14-49`). On Linux it adds `sbsigntool` and `tpm2-tools` (`linuxOnly`, `tools/nix/shell.nix:8`).

It sets `NONOS_IN_FLAKE`, and points `OVMF` and `OVMF_VARS` at the firmware, so the make targets in `mk/` never search the host (`NONOS_IN_FLAKE`, `tools/nix/shell.nix:58-64`).

## Hardware virtualization

A QEMU boot uses KVM when `/dev/kvm` opens read and write, and the TCG emulator otherwise (`accel`, `tools/nonos_qemu/machine.py:26-36`). The runner would also pick the macOS hypervisor on an Intel Mac, but the flake does not evaluate on one. Apple silicon runs only arm64 guests in its hypervisor, so an x86_64 NONOS guest is emulated there, and `make doctor` says so (`doctor`, `Makefile:124-135`). Under TCG the runner gives each guest CPU its own host thread.
