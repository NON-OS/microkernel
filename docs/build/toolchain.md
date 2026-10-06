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
