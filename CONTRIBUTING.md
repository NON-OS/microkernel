# Contributing

## Build and check

Install Nix with flakes turned on (https://nixos.org/download), then from the checkout:

```
nix build         # the reproducible artifacts, in ./result
nix flake check   # every proof crate and every static check
```

The same two commands on every machine:

| platform | before the two commands |
|---|---|
| Linux, any distribution | install Nix |
| macOS on Apple silicon | install Nix |
| Windows | install WSL2 with any Linux distribution, and Nix inside it |
| anywhere with Docker | open the repository in the devcontainer (`.devcontainer/`, VS Code or Codespaces); Nix is already there |

Nothing else comes from your machine. The toolchain, every crate and every C source are pinned by
hash, and the build runs offline in Nix's sandbox, so your artifacts have the same hashes as
everyone else's. `nix develop` gives you the same tools for work by hand. `make help` lists the
short forms.

[`docs/build/`](docs/build/README.md) is the guide, one page per step: installing Nix on each
platform, the build and its receipt, the profiles in `nonos.toml`, the checks, the seal, booting,
installing to a disk, verifying a release, and every failure seen so far with its fix.
[`tools/nix/README.md`](tools/nix/README.md) is the map of how each artifact is built.

## The rest

The contributing guide lives in the documentation:
[community/contributing](https://github.com/NON-OS/nonos-docs/blob/main/community/contributing.md).

It covers where to work (capsules, the kernel, proofs, hardware bring-up), and the house rules
every change is held to. Contribution is rewarded; see
[community/rewards](https://github.com/NON-OS/nonos-docs/blob/main/community/rewards.md).

Security issues go through
[private reporting](https://github.com/NON-OS/nonos-docs/blob/main/security/reporting.md),
not a public issue.
