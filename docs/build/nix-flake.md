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
| `standard-dev`, `hardened-dev`, `airgapped-dev`, `qemu-dev`, `core-dev` | each profile's development twin, with path-only [attestation](../overview/glossary.md#attestation) (`nameValuePair`, `tools/nix/default.nix:61-63`) |
| `kernel`, `bootloader` | the kernel and the loader of the configured build alone (`kernel`, `tools/nix/default.nix:66-67`) |
| `capsules` and `capsule-<slug>` | every entry of the build catalogue `tools/nix/capsules.json` together, and each alone: 116 of them, the 97 [capsules](../overview/glossary.md#capsule) the build includes and the 19 signed Linux userland programs (`capsules`, `tools/nix/default.nix:68`) |
| `linux-userland` and `linux-<program>` | the Linux userland together, and its 17 programs alone; `qwenchat` builds three of the 19 signed binaries, one per x86-64 level (`userland`, `tools/nix/default.nix:69`) |
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

Every app first moves to the root of the git checkout it was started in, and refuses to run outside one (`root`, `tools/nix/apps.nix:15-16`). [seal.md](seal.md) and [make-targets.md](make-targets.md) describe the seal and the QEMU runner, and [host-tools.md](host-tools.md) the other programs under `tools/` that the apps, the checks and the make targets run.

### Development shell and formatter

`devShells.default` is the shell [toolchain.md](toolchain.md) describes, and `formatter` is `nixfmt` (`formatter`, `tools/nix/default.nix:104-105`).

## Checks

`nix flake check` builds every check for the host. Each check is its own derivation, cached until its inputs change, so a failure names itself (`checks.nix`, `tools/nix/checks.nix:1-3`). The set is the union of five groups (`proofChecks`, `tools/nix/checks.nix:303`):

| group | names | what each one does |
|---|---|---|
| proof crates | `proofs-<crate>` | `cargo test` of one [proof crate](../overview/glossary.md#proof-crate) with overflow checks on and one test thread, then clippy with warnings as errors (`script`, `tools/nix/checks.nix:85-95`) |
| cargo checks | `nonos-verify`, `attest-poc`, `attest-battery`, `qjs-prelude`, and six `kernel-features-<set>` | the verification engine's lint and hygiene scan, the attestation tests and attacks, the browser prelude tests, and a kernel `cargo check` per optional feature set (`cargoChecks`, `tools/nix/checks.nix:119-167`) |
| profile checks | `kernel-profile-<profile>` | a kernel `cargo check` with exactly the features each profile resolves to (`profileChecks`, `tools/nix/checks.nix:174-193`) |
| static checks | `static-hygiene`, `static-abi`, `static-tree`, `static-evidence` | the Python and shell checks over the whole tree (`staticChecks`, `tools/nix/checks.nix:211-247`) |
| drift checks | `catalogues`, `inputs`, `git-pins`, `wallpaper-pins`, `starks-pin`, `shield-vectors-pin`, `busybox-source`, `rust-src-lock`, `config` | the flake's own inputs held to the tree they mirror (`driftChecks`, `tools/nix/checks.nix:250-301`) |

The proof crates are every `userland/*_proofs` directory with a `Cargo.lock`, and seven more named by hand (`proofDirs`, `tools/nix/checks.nix:20-30`). The two live [TPM](../overview/glossary.md#tpm) suites run on Linux only and fail when a live test was skipped (`needsTpm`, `tools/nix/checks.nix:32-34`). A few crates are not yet clippy clean in their tests, or at all, and are listed by name; the comment beside the lists says they only shrink (`lintLib`, `tools/nix/checks.nix:44-54`).

Count the checks for a host:

```
nix eval --json .#checks.x86_64-linux --apply builtins.attrNames
```

At this commit that gives 143 checks on x86_64-linux and on aarch64-linux, and 141 on aarch64-darwin, where the two TPM suites are left out. On x86_64-linux the 143 are 114 proof crates, 6 kernel feature sets, 6 kernel profiles, 4 static checks, 4 other cargo checks and 9 drift checks. [ci.md](ci.md) says how they stood at this commit.

### Run them

```
make check
```

Not tested in this release.

`make check` runs `nix run .#check-report` (`check`, `Makefile:60-61`). It builds every check for this host in one `nix build --keep-going`, then prints `PASS` or `FAIL` for each, the number of tests each proof crate ran, and the size of the bill of materials, writes the same record as a JSON file in the checkout, and exits non-zero when any check failed (`main`, `tools/nonos-check-report:58-69`). It asks Nix for every check's derivation in one evaluation, so a single check that does not evaluate stops the whole report. CI runs `nix flake check -L --keep-going` instead, on Linux and on macOS, one `matrix` entry each (`.github/workflows/verify.yml:39-60`).

One check by name, with its log streamed:

```
nix build .#checks.x86_64-linux.proofs-prove_proofs -L
```

Not tested in this release.

### What stays outside the flake

Kani, Verus, the Charon and Aeneas extraction, `cargo-fuzz` runs and the `cargo-audit` advisory feed need a network or a toolchain no lock pins yet, so they keep their own workflows (`checks.nix`, `tools/nix/checks.nix:1-8`). [ci.md](ci.md) lists them.

## What each build sees

A derivation sees only the files it reads. [tools/nix/src.nix](../../tools/nix/src.nix) builds each source from the entry for its crate in `tools/nix/inputs.json`, and leaves every `*.md` file out unless the Rust names it (`crate`, `tools/nix/src.nix:17-34`). An edit to the documentation rebuilds no artifact; an edit to one capsule rebuilds that capsule, the proofs that mount its files, and the kernel that embeds it. The static and drift checks are the exception: they read the whole tree, documentation included, so any edit runs them again (`root`, `tools/nix/checks.nix:10-12`).

A crate with no entry stops the flake from evaluating that build, with a message that names the crate (`crate`, `tools/nix/src.nix:25`). After you add a path dependency, a `#[path]`, an `include_bytes!` or a new crate root, regenerate the table and commit it:

```
python3 tools/nix/inputs.py
python3 tools/nix/inputs.py --check
```

Not tested in this release.

The first line rewrites `tools/nix/inputs.json`; it does so whatever other argument it is given. Only `--check` compares without writing, and the `inputs` check runs exactly that (`main`, `tools/nix/inputs.py:390-402`).

## The binary cache

CI reads and fills a Cachix cache that the repository variable `NONOS_CACHIX` names. Only a push to the repository passes the `CACHIX_AUTH_TOKEN` that writes it, never a pull request, so no pull request can put a store path into the cache others read (`cachix-token`, `.github/workflows/verify.yml:53-58`). The reproducibility workflow builds with the cache turned off; the setup action's `use-cache` input defaults to `true` and that workflow passes `false` (`.github/actions/nix-setup/action.yml:19-22`).

## See also

- [Toolchain](toolchain.md)
- [Make targets](make-targets.md)
- [Reproducible builds](reproducible-builds.md)
- [CI](ci.md)
- [Host tools](host-tools.md)
- [Tests and proofs](../contributing/tests-and-proofs.md)
