# Contributing to NONOS: Set up

Part of [Contributing to NONOS](README.md).

## Set up

NONOS builds through its Nix flake, so Nix with flakes turned on is the one tool
you install. The flake exports `packages`, `checks`, `apps`, `devShells` and a
`formatter` for three host `systems`: `x86_64-linux`, `aarch64-linux` and
`aarch64-darwin` (`flake.nix:38-49`). The development shell carries the pinned
Rust toolchain and the tools the checks and boots use. Its header names WSL2 as
a host it works on. On macOS it leaves out the two tools in `linuxOnly`,
`sbsigntool` for [Secure
Boot](../overview/glossary-rollback-index.md#secure-boot) signing and
`tpm2-tools` (`tools/nix/shell.nix:1-8`).

If you would rather not install Nix on the host, `.devcontainer/` describes a
container that installs Nix 2.34.6 and writes a `nix.conf` with flakes and the
build sandbox turned on (`.devcontainer/Dockerfile:25-30`). The container runs
`privileged`, because Nix's sandbox needs namespaces a default container does
not grant (`.devcontainer/devcontainer.json:5`).

```
nix develop
make check
```

Not tested in this release.

`make check` builds every flake check for your machine and prints what each one
proved; [Tests and proofs](tests-and-proofs.md) lists the checks and says which
fail at this commit. `make help` prints the short forms of the other build
commands, and [the build guide](../build/README.md) covers them one by one.

## Before you start

- Read the [Code of Conduct](../../CODE_OF_CONDUCT.md). It asks you to argue
  about the code and bring evidence, and it says where to report conduct
  problems.
- Report a security problem privately, never in a public issue.
  [SECURITY.md](../../SECURITY.md) and [Reporting a
  vulnerability](../security/reporting-a-vulnerability.md) say how.
- NONOS is licensed under the GNU Affero General Public License, version 3 or
  any later version. Most source files carry the notice in their header, and a
  new file should too; [Code style](code-style-formatting.md#file-header) shows
  the header and says where it is missing.
- Code for an architecture other than x86_64 is a preview or not supported; read
  [Architectures](../architectures/README.md) before you work on one.
