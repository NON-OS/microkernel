# The Nix flake

Use the flake directly: what it takes in, the packages and apps it gives out, and how to run its checks one at a time or all together.

## Inputs

[flake.nix](../../flake.nix) has three inputs, and [flake.lock](../../flake.lock) pins each to one commit and one NAR hash (`inputs`, `flake.nix:18-33`):

| input | follows | locked commit |
|---|---|---|
| `nixpkgs` | `github:NixOS/nixpkgs/nixos-unstable` | `c59305bab2065cfecc4944690d9eedbb56f3a9fa` at `flake.lock:9` |
| `rust-overlay` | `github:oxalica/rust-overlay`, with its own nixpkgs replaced by ours | `dcee1adabb61484343af863501d2e3d91ef51f72` at `flake.lock:37` |
| `starks` | `github:NON-OS/STARKs/main`, read as source and not as a flake | `a41bb8bb0dacfea614acc6fb500d0b96abd78d2b` at `flake.lock:53` |

`starks` is the STARK prover and verifier the kernel, the loader and the enroll tool are built with, and the flake is the only place it is pinned (`starks`, `flake.nix:24-32`). Every `Cargo.lock` that uses it must name the same commit, and the `starks-pin` check fails when one does not (`starks-pin`, `tools/nix/checks.nix:274-276`). The `starks-bump` workflow moves the input when STARKs main moves and opens a pull request; it never merges (`cron`, `.github/workflows/starks-bump.yml:3-14`).
