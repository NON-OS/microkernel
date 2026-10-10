# NONOS 0.9.2 (pre-release)

This tag replaces the 0.9.2 pre-release of September 3. That tag was cut before most of what 0.9.2
is: the Linux personality, the Anyone transport, the shield, the installer and the Marketplace
all landed after it. This one is cut from the tree that holds them, built the same way on Linux
and macOS, with every capsule proved under one STARK policy root.

It is a pre-release. What it does not do yet is listed under [Known limitations](#known-limitations),
and it should be read before you install it on a machine you care about.

## What you download

| File | Made by | What it is |
|---|---|---|
| `nonos.img`, `nonos.iso` | the release seal, on the signing machine | The bootable image, its kernel and loader signed (Ed25519 and ML-DSA-65). |
| `nonos-release-bundle.tar.gz` | CI, from a reproducible build | The kernel ELF, the UEFI loader, the capsule catalogue, the SBOM, every trust root and the STARK trailer of every enrolled capsule. |
| `verifier.wasm` | CI | The STARK verifier the kernel's gate links (`nox_verify`), for checking proofs in a browser. |
| `nonos.cdx.json`, `nonos-build.json`, `provenance.json` | CI | Bill of materials, build record with every input hash, and the bundle's provenance. |
| `SHA256SUMS`, `BLAKE3SUMS` | CI | Sums over exactly the CI files above. |
| `IMAGE-SHA256SUMS`, `IMAGE-BLAKE3SUMS` | the release seal | Sums over `nonos.img` and `nonos.iso`. |

CI holds no signing key and signs nothing. The image is the same build CI makes, plus signatures.

## Verify it

```
sha256sum -c SHA256SUMS && sha256sum -c IMAGE-SHA256SUMS
b3sum -c BLAKE3SUMS && b3sum -c IMAGE-BLAKE3SUMS
gh attestation verify nonos-release-bundle.tar.gz --repo NON-OS/microkernel
```

Rebuild it yourself and compare: `nix build .#standard` at this tag gives the kernel, loader and
capsules in the bundle, byte for byte, on x86_64 Linux and on Apple silicon macOS. The
`reproducible` CI lane builds on both and compares every file.

## What is new since the September tag

### Running Linux programs

The Linux personality (`capsule_linux`) runs unmodified x86_64 Linux binaries, static and
dynamic, with threads, a filesystem view, sockets and signals, each confined and attested before
it runs. It installs Alpine, Debian/Kali and pacman packages. Every image carries CPython 3.12,
Lua 5.4, zstd, John the Ripper and `qwenchat` (llama.cpp), all built from pinned sources; a
dozen more (sqlite3, perl, tclsh, mruby, qjs, jq, gojq, rg, fd, nano, make, openssl) install
from the Marketplace.

### Private networking

- **Anyone transport** (`capsule_net_anon`): an onion routing client for the Anyone network, a
  Tor 0.4.8.11 fork. Model downloads go over it.
- **Nym**: the whole active set is held and drawn without bias, the replay window is checked on
  the live receive path, and the S-box is constant time.
- **TLS**: a streaming client with alert reasons, and DER lengths past the buffer refused.

### Wallet, proofs and attestation

- **Shield** (`capsule_shield`): the NOX Shield wallet service, on the same Rust core as the
  phone apps.
- **Anonymous device proof** (`capsule_prove`), with no network capability.
- **One STARK policy root** over every capsule, checked at spawn by `nox_verify`. The kernel and
  the loader carry their own attestation roots, and the kernel checks the loader at boot.
- **TPM machine key and vault**: records sealed to the machine; the wallet vault is sealed by the
  keyring.

### Installing and the Marketplace

- **Installer** (`capsule_install`, and `install-cli`): writes a whole NONOS disk and reads every
  sector back. It keeps setup answers, Wi-Fi, the wallet vault, models and installed packages.
- **Marketplace** with a signed catalogue, and **Qwen tiers** installed from the Models tab or
  `qwen get`, each file checked against its SHA-256.
- Image profiles come from `nonos.toml`: standard, hardened, airgapped, qemu, dev, core.

### Kernel

- **SMP on every core**: AP bring-up, IPIs, TLB shootdown, and deferred invalidation with one
  rendezvous per unmap.
- **Hardening that was off is on**: stack guards, the W^X check on both halves, uncached MMIO,
  the IOMMU brought up after paging, and every device mapping confined to the broker allowlist.
- **Smaller syscall surface**: `MkSpawn` and the five signing syscalls are gone, wallet curve
  crypto moved to ring 3, and the halo2 verifier removed from kernel and loader.
- **Fixes**: the SYSRET STAR RPL bits, IDT error codes for vectors 29 and 30, the RSDP checks,
  PCI masking, IOMMU register bounds, close-on-exec across fork, and fatal exceptions that now
  explain themselves on serial.

### Desktop

git in the Terminal (clone and push over HTTPS), an xterm-compatible terminal emulator, the
rebuilt Files app with search, tags and favourites, contrast-checked themes, text scaling and
reduced motion, and a startup chime.

### Verification

979 kernel functions are extracted from the Rust and proven in Lean (492 with a behavioural
property), next to 1,499 core Lean theorems with no `sorry`, 109 Kani harnesses and 107 runnable
proof crates (`verification/evidence/EVIDENCE.json`). The proofs found real bugs, among them
`align_up` defined four ways with four answers and a constant-time compare that computed its
borrow wrongly.

### Build

One Nix build behind `make`. The capsules, tools, kernel and image come out byte-identical on
Linux and macOS: fat LTO, host-neutral crate names in the rustc wrapper, one codegen unit for
upstream tools, portable BLAKE3, and pinned host tools. `make seal` signs and enrolls, and keeps
the committed STARK enrollment when it still proves the capsules.

## Breaking changes

- The `MkSpawn` and signing syscalls are removed. Capsules built out of tree must use
  `nonos_ed25519` and `nonos_secp256k1` and the current spawn path.
- New capability bits: `AttestRead`, `LocalSign`, `ForeignExec`.
- The submodules are folded into this one repository. Re-clone.
- The trust set is re-signed. Enrollments from the September tag do not verify against it.
- No in-place upgrade. Reinstall; nothing carries over from an older install.

## Known limitations

- **A USB stick boot does not bind the stick.** The kernel picks a disk at its first block I/O,
  and a boot from a stick makes none, so its state is not kept on the stick. The CI boot test
  checks that the stick boots and the USB storage driver starts, not that it binds.
- **Not in any image yet**: the e1000e, igc, USB Ethernet and rtsx drivers, the attack suite,
  the assembly installer and the SMP stress test.
- **Desktop**: it cannot power off, Files and the Editor do not keep files across a reboot,
  there is no login password, no USB hubs, and memory above 64 GiB is unused.
- **Hardening**: no KASLR, no compiler stack canaries, the capability ceiling is logged rather
  than enforced, and the package store is not encrypted.
- **aarch64** is a preview with no loader or image; **riscv64** does not build a kernel.
- **Not yet tested on a booted image**: live Nym and Anyone traffic, Secure Boot enabled, and an
  update from one release to the next.
