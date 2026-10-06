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

## Outputs

The flake gives `packages`, `checks`, `apps`, `devShells` and `formatter` for each host in `systems` (`flake.nix:38-50`). [tools/nix/default.nix](../../tools/nix/default.nix) wires one file per job together for each host.

### Packages

List them:

```
nix eval --json .#packages.x86_64-linux --apply builtins.attrNames
```

On x86_64-linux this lists 175 packages:

| package | what it is |
|---|---|
| `default` | the build `nonos.toml` describes: every artifact and `nonos-build.json` (`default`, `tools/nix/default.nix:57-59`) |
| `standard`, `hardened`, `airgapped`, `qemu`, `dev`, `core` | the same for each [build profile](../overview/glossary.md#build-profile), the rest of `nonos.toml` unchanged (`profiles`, `tools/nix/default.nix:60`) |
| `standard-dev`, `hardened-dev`, `airgapped-dev`, `qemu-dev`, `core-dev` | each profile's development twin, with path-only attestation (`nameValuePair`, `tools/nix/default.nix:61-63`) |
| `kernel`, `bootloader` | the kernel and the loader of the configured build alone (`kernel`, `tools/nix/default.nix:66-67`) |
| `capsules` and `capsule-<slug>` | every [capsule](../overview/glossary.md#capsule) together, and each alone: 116 of them (`capsules`, `tools/nix/default.nix:68`) |
| `linux-userland` and `linux-<program>` | the Linux userland together, and its 17 programs alone (`userland`, `tools/nix/default.nix:69`) |
| `busybox` | BusyBox built from source (`busybox`, `tools/nix/default.nix:70`) |
| `upstream-<tool>` | the 10 crates.io tools packaged as capsules (`upstream`, `tools/nix/default.nix:100`) |
| `host-tools` and `host-<tool>` | the 7 host programs the [seal](../overview/glossary.md#seal) and the checks run, such as `capsule-sign` and `nonos-stark-enroll` (`hostTools`, `tools/nix/default.nix:91`) |
| `sbom` | the CycloneDX bill of materials (`sbom`, `tools/nix/default.nix:92`) |
| `profiles` | the text `make profiles` prints (`profiles`, `tools/nix/default.nix:90`) |
| `toolchain`, `rust-nonos-std`, `nonos-rt` | the pinned Rust, the standard library with the NONOS layer, and the startup object of `std` capsules (`toolchain`, `tools/nix/default.nix:94-96`) |
| `verifier-wasm` | the STARK verifier from the `starks` input, built for the web (`verifier`, `tools/nix/default.nix:71-86`) |
| `periodic-cache` | the periodic cache `nonos.shield` embeds (`periodic`, `tools/nix/default.nix:87-89`) |

### Apps

| app | runs | used by |
|---|---|---|
| `seal` | `tools/nonos-seal` with the flake's tools on `PATH` (`seal`, `tools/nix/apps.nix:40`) | `make seal` |
| `qemu` | `tools/nonos-qemu`, with QEMU, swtpm and the firmware (`qemu`, `tools/nix/apps.nix:43-53`) | `make boot` |
| `receipt` | `tools/nonos-receipt` (`receipt`, `tools/nix/apps.nix:56`) | `make build` |
| `check-report` | `tools/nonos-check-report` (`report`, `tools/nix/apps.nix:57`) | `make check` |

Every app first moves to the root of the git checkout it was started in, and refuses to run outside one (`root`, `tools/nix/apps.nix:15-16`). [seal.md](seal.md) and [make-targets.md](make-targets.md) describe the seal and the QEMU runner.

### Development shell and formatter

`devShells.default` is the shell [toolchain.md](toolchain.md) describes, and `formatter` is `nixfmt` (`formatter`, `tools/nix/default.nix:104-105`).
