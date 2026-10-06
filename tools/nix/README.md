# Building NONOS

One way to build, one way to check, and a clear line between what anyone can
reproduce and what only the release keys can add. This page is the map of the
machinery; [docs/build](../../docs/build/README.md) is the step by step guide.

## Build

Install Nix (https://nixos.org/download) with flakes turned on, then:

```
nix build # the reproducible artifacts for nonos.toml, in./result
nix flake check # every proof crate and every static check
```

That is the whole story on every platform:

| platform | how |
|---|---|
| Linux, any distribution | install Nix, the two commands |
| macOS on Apple silicon | install Nix, the two commands |
| Windows | WSL2 with any Linux distribution, then the same two commands; or open the repository in the devcontainer (`.devcontainer/`, VS Code or Codespaces), which needs neither |

Nothing comes from the host but the source. The toolchain is pinned in
`flake.lock` and `rust-toolchain.toml`, every crate by the sha256 in its own
`Cargo.lock`, every C source by the sha256 in `tools/nix/sources.txt`, and the
build runs in Nix's sandbox with no network.

STARKs, the prover and verifier the trust path is built on, is pinned once:
the `starks` input in `flake.lock`, which follows STARKs main. The manifests
that use it name only the branch; `tools/nonos-starks-sync` writes the
input's commit into every `Cargo.lock`, and `nix flake check` fails if one
disagrees. `starks-bump.yml` moves the input when main moves and opens a pull
request for ek, since the trust path changes only by ek's merge. `nix develop` drops you into the
same tools for work by hand; `make` is a short way to type all of this
(`make help`).

The first build compiles everything; the binary cache (below) means later ones
fetch what CI already built.

## Configure

`nonos.toml` is the configuration of the one build. `nix build` builds what it
says; `nix build.#<profile>` builds a profile with the rest of the file
unchanged.

| profile | the image | loader policy |
|---|---|---|
| `standard` | the full system for real hardware: every driver, the desktop, first-boot setup, the installer | standard |
| `hardened` | standard, with no serial console for capsules | production: Secure Boot and a TPM required |
| `airgapped` | hardened, with no network driver, network stack or online program compiled in; installs from offline media only | production |
| `qemu` | the desktop for virtual machines, without the drivers only real hardware has | standard-qemu |
| `dev` | qemu with the loader's development policy; never sealed for release | dev-qemu |
| `core` | the microkernel and its base capsules, no desktop | standard-qemu |

A profile sets floors. Asking it for less (a weaker loader, a feature it takes
out) fails before anything builds. The kernel's features are derived from
`Cargo.toml`, so what a profile takes out is not in the binary at all.

Every boot of every image is amnesic: nothing is kept unless the person
chooses to install in first-boot setup. `install = false` takes first-boot
setup and the installer out, so no boot of that image can ever keep anything
or write a disk. `smp`, `rollback_index`, extra `features` and what the
package `store` carries are the other keys.

Every profile but core carries first-boot setup and the installer. The
installer sits in `microkernel-desktop-offline`, the desktop with no network
and no serial console for capsules, and first-boot setup is the
`microkernel-setup-wizard` feature, which brings in neither, so hardened and
airgapped keep both.
`make profiles` prints what each profile is for and what it promises, and
`nix flake check` type-checks every profile's kernel with exactly its features.

## What is reproducible, and what is not

**Reproducible**, byte for byte, from the same commit and `nonos.toml` on any
machine: the kernel ELF, every capsule ELF, the Linux userland, the bootloader
EFI binary. `result/nonos-build.json` names every one of them by sha256. Rust
paths are rewritten to fixed names (`/cargo`, `/rust`, `/nonos`), clocks
are the commit's, and nothing reads the host. `ci-reproducible` builds on
three machines with no shared cache and fails if any hash differs.

**Not reproducible, by design:** the sealed image. Enrolling a binary under a
STARK root draws fresh randomness for every proof, and the signatures need
keys no build ever sees. Two seals of one commit differ in their trailers,
signatures and roots, never in the artifacts inside them: the seal ends by
rebuilding the kernel and loader from the tree and failing unless they are the
ones it sealed.

The kernel and loader embed what the tree holds: the capsule trust set under
`nonos-data/trust`, the kernel root, and the kernel's public keys. The seal
writes those files and ek commits them, so the next `nix build` gives exactly
the sealed artifacts. Until the kernel's public keys are committed under
`nonos-data/trust/keys/`, `nix build` leaves the loader out and writes
`bootloader/README` saying why (`tools/nix/artifacts.nix`).

## Seal (ek)

```
nix run.#seal -- [--profile P] [--release]
```

Runs on the machine that holds the keys (`tools/nonos-key-ceremony`), outside
the sandbox, in the order the trust chain needs:

1. the market index and the model catalogue, signed by the market operator
 key, which two capsules embed (`nonos-data/market`, `nonos-data/models`);
2. each capsule's certificate and signed manifest, then one STARK enrollment
 of the whole set;
3. the kernel, built against that set, enrolled under its own root;
4. the loader, built against the kernel root and the kernel's public keys,
 enrolled under its own root;
5. the kernel signed (Ed25519 and ML-DSA-65) with its trailer embedded,
 `boot_root.approval` and `kernel.approval` from the device policy key,
 Secure Boot from the db key, then the ESP, the store, the USB image and the
 ISO in `target/release/<profile>/`;
6. every check the boot will make (the trust ledger, every signature, every
 declared capability, every STARK proof by the gates' own verifier), and the
 rebuild above.

It stages only public files, never prints or copies a key, and refuses to
stage anything git would track as a private one. `--release` also refuses a
dirty tree, the dev profile, and a production loader without Secure Boot.
Then commit the staged files under `nonos-data/`.

## Boot

```
nix run.#qemu # the sealed image, in a window
nix run.#qemu -- --tpm # with a software TPM (measured boot)
nix run.#qemu -- --install-target # beside a blank NVMe disk to install to
nix run.#qemu -- --installed # the disk the installer wrote, alone
nix run.#qemu -- --fresh --model auto # a new data disk with the Qwen tier setup picks
nix run.#qemu -- --stick # the sealed stick alone, as hardware boots it
```

A boot runs from the ESP, as its own FAT drive, and a virtio data disk with
the store and the disk plan, as mk/10-qemu.mk laid them out. The data disk,
and the encrypted volume on it, is kept from one boot to the next; a new seal
or `--fresh` starts it again (`tools/nonos_qemu/disk.py`).

`--headless --timeout N --expect PATTERN` is the boot smoke CI runs.

## From the build to the installed system

The image is `target/release/<profile>/nonos.img`: the package store at
LBA 256 and the EFI system partition from 128 MiB, the layout an installed disk
uses (`userland/nonos_disk_map`). The ESP holds:

```
EFI/BOOT/BOOTX64.EFI the enrolled, Secure Boot signed loader
EFI/nonos/kernel.bin the signed kernel with its STARK trailer
EFI/nonos/bootloader.trailer the loader's STARK trailer
EFI/nonos/boot_root.approval the record the kernel holds the loader to
EFI/nonos/kernel.approval under which the TPM releases the device secret
EFI/nonos/boot.cfg
startup.nsh
```

The installer copies these from memory, not from the stick: the loader hands
the verified bytes to the kernel, and the installer reads them back with
`MkInstallSource`. So the disk gets the exact files that booted and were
checked. It writes the same `boot.cfg` and `startup.nsh` the seal writes, and
carries the signed programs and setup answers from the running store. What
was sealed is what is installed, with no second path.

`target/release/<profile>/nonos-release.json` records it all: the build
manifest, every sealed file by sha256, and what to commit.

## What a build sees

A derivation's source is only what it reads, so an edit reaches a build only
through a file that build reads, and everything else keeps its store path and
is never rebuilt. `tools/nix/src.nix` makes each source from
`tools/nix/inputs.json`, which `tools/nix/inputs.py` writes:

| build | what it sees |
|---|---|
| a capsule, `nonos-rt`, a crates.io tool | its crate, every crate its path dependencies reach (libc among them), the files a `#[path]`, `include_bytes!`, `include_str!` or other path literal in that Rust names, the `.cargo` directories above them, and the target JSON |
| the kernel | its manifest, `build.rs`, `src/`, the crates it depends on by path, the files they name (the trust set, the linker scripts, the PQClean sources), its target JSON and the scripts its build runs |
| the loader, each host tool | its crate, the crates it reaches and the files they name; the loader also gets the kernel keys the seal commits |
| a proof check | its crate, the crates it reaches and the sources it mounts by `#[path]`, followed through the modules those declare |
| the Linux userland | one program's recipe and the files it names (`userlandScripts`) |
| the static and drift checks, `nonos-verify` | the whole tree but media, as a reviewer reads it |

No build sees a `*.md` file unless its Rust names that file. A file the build
script is handed for a marker in the source (`MARKERS` in `inputs.py`: the
market index, the model catalogue, the trust set's `COMMIT`) reaches only the
capsules that carry the marker, so a seal rebuilds only those.

So a docs edit rebuilds no capsule, no Linux program, no host tool and no
proof; an edit to one capsule rebuilds that capsule, the proofs that mount
its files, and the kernel and images that embed it. The kernel and the loader
carry the commit's time (`SOURCE_DATE_EPOCH`) and the kernel its short rev,
so they and the images are rebuilt for every commit whatever it changes.

After adding a path dependency, a `#[path]`, an `include_*!` or a new crate
root, run `python3 tools/nix/inputs.py` and commit `inputs.json`; the
`inputs` check fails until it matches the tree. A build that misses a file it
reads fails to compile, naming the file.

## Cache and bill of materials

CI pushes every build to the Cachix cache the repository variable
`NONOS_CACHIX` names (with the `CACHIX_AUTH_TOKEN` secret); `cachix use
<name>` reads it on any machine. Reproducibility runs never read it.

`nix build.#sbom` is the CycloneDX bill of materials: every crate from every
lock, every C source, every toolchain and the flake inputs, each by version
and hash. It is the same file on every machine, and `ci-supply-chain` keeps
it with its reports.

## Where things are

| | |
|---|---|
| `flake.nix`, `flake.lock` | the inputs and the outputs |
| `nonos.toml` | the configuration |
| `tools/nix/` | how each artifact is built, one file per job |
| `tools/nix/capsules.json`, `store.json` | what make's Capsule.mk files declare, printed by `mk/60-nix.mk`; `python3 tools/nix/catalogues.py` regenerates them and `nix flake check` fails when they are stale |
| `tools/nix/inputs.json` | what each cargo build reads, written by `python3 tools/nix/inputs.py` (What a build sees) |
| `tools/nix/sources.txt`, `git-sources.json` | the C sources and git dependencies, by hash |
| `tools/nonos_seal/`, `tools/nonos_qemu/` | the seal and the QEMU runner |
| `.devcontainer/` | the container path |
| | what a machine that boots must still confirm |

## When something fails

- **A lock names another STARKs commit.** `tools/nonos-starks-sync` in
 `nix develop`, then commit the locks. A crate whose code does not build
 against STARKs main stays on its old commit, fails to build and says so, and
 `starks-pin` names it until it is ported.
- **Another git dependency changed.** `tools/nonos-nix-pin` fills its tree
 hash into `tools/nix/git-sources.json`.
- **A capsule declaration changed.** `python3 tools/nix/catalogues.py`, then
 commit the two JSON files.
- **A Cargo.lock changed.** Nothing to do: the lock is the pin.
- **A boot says store status 9, or a file is missing from the store.**
 `tools/nonos-store-check nonos.img` (or the stick's device) reads the
 store as vfs does and names each entry it would refuse, and why.
- **The `inputs` check fails, or a build cannot find a file the tree has.**
 A path dependency, `#[path]` or `include_*!` was added: `python3
 tools/nix/inputs.py`, then commit `tools/nix/inputs.json`.
