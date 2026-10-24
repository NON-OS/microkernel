# Features: what the default image carries, and what it does not

This page lists every kernel feature in `Cargo.toml` that the default image leaves out, and says why for each one. The default image is the `standard` profile that `nonos.toml` names. A feature that isn't real, or that only a test needs, stays out. A real feature that cannot ship yet is listed with the reason and with the step that would let it ship.

## How the default image is decided

`nonos.toml` names `profile = "standard"`. The flake turns the profile into kernel features in one place (`resolve`, `tools/nix/config.nix`). It starts from the profile's `kernel` list, `microkernel-full-gui`, then adds `nonos-smp` when `smp = true` and `nonos-release` because the `standard` loader is not `dev-qemu`. The image carries the closure of those three over `Cargo.toml`.

To list what it leaves out yourself, run this from the repository root:

```sh
python3 - <<'EOF'
import tomllib
f = tomllib.load(open("Cargo.toml", "rb"))["features"]
on, todo = set(), ["microkernel-full-gui", "nonos-smp", "nonos-release"]
while todo:
    x = todo.pop()
    if x in f and x not in on:
        on.add(x); todo += f[x]
print("\n".join(sorted(k for k in f if k not in on)))
EOF
```

Host-tested on 2026-10-24: the closure has every production capsule below and leaves out 140 features. Of those, 91 are `microkernel-*` lane sets and 49 are the features in the tables.

## What the default image carries

`microkernel-full-gui` (`Cargo.toml`) is the desktop and every real program and driver:

- the desktop: the compositor, window manager, shell, login, settings, the apps, the terminal, the Linux personality, the browser, the wallet and the market;
- the network stack and both anonymity networks: `net.core`, `net.sockets`, Nym (`net.nym`, `net.socks5`) and Anyone (`net.anon`);
- the NOX settle and proof fetch services: `nonos-capsule-nox-settle`, `nonos-capsule-prove-fetch`;
- first-boot setup and the installer;
- every production hardware driver:
  - NICs: virtio, e1000, e1000e, igb, ixgbe, rtl8139, rtl8169, pcnet, vmxnet3, tulip, iwlwifi and rtl8821ce, plus USB networking through CDC-ECM, CDC-NCM and RNDIS;
  - storage: virtio-blk, virtio-scsi, NVMe, AHCI, PIIX IDE, pvscsi, BusLogic, MPT, SDHCI, UFS and USB mass storage;
  - USB host controllers: xHCI, EHCI, OHCI and UHCI, and USB hubs;
  - input: PS/2, virtio-input, USB HID, I2C HID and SPI HID;
  - display: virtio-gpu, VMware SVGA, Bochs (BGA) and Intel;
  - audio: HDA, AC'97, USB audio and USB microphones;
  - USB cameras (UVC), USB serial (CDC-ACM), smart cards (CCID) and FIDO security keys;
  - the SPI, SMBus and I2C buses;
  - VirtualBox and Hyper-V guest support.

The `hardened` and `airgapped` profiles start from the same set. `hardened` takes out `capsule-serial-debug`, and `airgapped` also takes out every network feature (`networkFeatures`, `tools/nix/config.nix`).

## A capsule that is in the image but not yet sealed

The kernel embeds each capsule's certificate, manifest and STARK trailer from `nonos-data/trust/capsules/`. The release seal writes those files with the release keys, and the developers commit them. `nix build` refuses a kernel that ships a capsule with no trust set (`unsealed`, `tools/nix/image.nix`). That gate applies before and after this change:

- On 2026-10-24, before this change, 15 capsules that the default image already carried had no trust set: pcnet, vmxnet3, tulip, virtio-input, vmware-svga, ac97, vmmdev, vmmouse, vmbus, piix-ide, virtio-scsi, pvscsi, buslogic, mpt and sdhci. Host-tested: the gate's own condition, run over `tools/nix/capsules.json`.
- The 24 capsules this change adds have no trust set either: the 22 drivers, plus nox-settle and prove-fetch.

So `nix build` of the default image waits for the next release seal, which writes all 39 trust sets in one step. A development image (`make dev-image`, `tools/nonos-dev-image`) is sealed with throwaway keys and builds and boots now.

## Off, and why

### Test and development only: never in a release

| feature | why it is off | evidence |
|---|---|---|
| `nonos-dev-attest` | admits capsules on their Merkle path with no STARK proof | `compile_error!` beside `nonos-release`, `src/lib.rs`; `resolve` asserts it needs loader `dev-qemu` |
| `nonos-dev-unconfined-dma`, `nonos-dev-unremapped-interrupts` | DMA with no IOMMU domain, or interrupts with no remapping | each refuses `nonos-production` and `nonos-release` (`src/memory/iommu/dev_unconfined.rs`, `dev_unremapped.rs`) |
| `nonos-attest-refusal-smoketest` | embeds four deliberately broken capsules | `compile_error!` beside `nonos-production`, `src/lib.rs` |
| `nonos-capsule-shield-vectors` | Test 5, the pinned wallet vectors | `compile_error!` under `nonos-release`, `src/userspace/capsule_shield_vectors/mod.rs`; `resolve` refuses it without loader `dev-qemu` |
| `nonos-terminal-smoketest`, `nonos-image-viewer-smoketest`, `nonos-video-player-smoketest`, `nonos-ntp-smoketest`, `nonos-tool-selftest` | boot lanes that read serial markers | `Cargo.toml` comments; each pulls in `nonos-user-entry-proof`, and the NTP lane also pulls in `capsule-serial-debug` |
| `nonos-user-entry-proof` | a page-table audit before every return to user mode, which halts on failure | `Cargo.toml` comment; a cost on every user entry, for the test lanes |
| `nonos-trap-proof-sp0`, `nonos-trap-proof-kernel-abort` | aarch64 lanes that compile a deliberate exception into the boot | `Cargo.toml` comment: "a kernel built with one stops at that exception by design" |
| `nonos-bench-micro` | microbenchmarks at boot | `Cargo.toml` comment; the benchmark lane turns it on |
| `nonos-arch-preview` | brings up aarch64 and riscv64 | `src/lib.rs`: not a release target |
| `dbg-ring`, `heap-track` | a debug ring, and a spinlock and hash on every heap operation | `Cargo.toml` comments |
| `input-probe-inject`, `nonos-capsule-input-probe` | inject synthetic keystrokes, and the capsule that tests the input stack | `Cargo.toml` comments; `microkernel-input-probe` is a test marker (`markers`, `tools/nix/config.nix`) |
| `nonos-capsule-input-proof` | a test capsule that writes the first key code and pointer motion it sees to the serial console | `userland/capsule_input_proof/src/proof/markers.rs`. **Taken out of the default image by this change.** It was spawned on every boot (`spawn_plan/apps.rs`) |
| `nonos-capsule-hello` | the docs' example window | `userland/capsule_hello/README.md`. **Taken out of the default image by this change** |
| `nonos-capsule-tokio-smoke` | an async runtime test that never finishes and held about a sixth of a core | `Cargo.toml` comment on `microkernel-desktop-gui-async-gate` |
| `nonos-capsule-std-proof` | a `std` port proof, reached from the store disk | `userland/capsule_std_proof/README.md` |
| `std` | host tests | never in the kernel |

### Superseded: the work is done elsewhere in the image

| feature | why it is off | evidence |
|---|---|---|
| `nonos-capsule-net-l2`, `-net-ip`, `-net-udp`, `-net-tcp`, `-net-dhcp`, `-net-dns` | `net.core` registers each of these handles itself, and the kernel never spawns the separate capsules when `net.core` is on | `spawn_legacy_stack`, `src/userspace/init/spawn_plan/network/spawn_legacy_stack.rs`; each capsule's README |
| `nonos-capsule-nonos-install` | the installers that ship are `capsule_install` and `install-cli` | `userland/capsule_nonos_install/README.md` |
| `crypto-ed25519-dalek`, `crypto-curve25519` | alternative backends; the in-tree `crypto-ed25519-int` is on | `Cargo.toml` |
| `mlkem512`/`768`/`1024`, `mldsa2`/`3`/`5` | ML-DSA-65 verification, which capsule attestation uses, is always compiled. These gate only `crypto/pqc/kyber` and its re-exports | `src/crypto/pqc/mod.rs` |
| `default`, `microkernel` | aliases for the core sets | `Cargo.toml` |

### Real, but held back

| feature | why it cannot ship yet | what would let it ship |
|---|---|---|
| `nonos-capsule-net-ntp` | Not spawned beside `net.core` (`spawn_legacy_stack`). It is direct UDP to a time server, allowed only on the Direct route (`capsule_net_ntp/src/main.rs`), and its request carries the raw local clock (`sync.rs`), which fingerprints the machine. The desktop therefore has no network time sync, by design for now. | Send a random transmit timestamp in place of the clock, add a time source that crosses the anonymity networks, and spawn it beside `net.core` |
| `nonos-capsule-acpi` | Running AML needs Admin for `MkAcpi*`, and its letter recommends a `Firmware` capability bit before any profile carries it (`the design notes`). Not in `capsules.json`. | A Firmware bit, a catalogue entry, a seal, and the FADT PM1 control and reset ports added to the broker's reserved list (see [privacy guarantees](../security/privacy-guarantees.md)) |
| `nonos-capsule-driver-gpio` | Not in `capsules.json` or the mk include block. Its wire was never checked against the client (`the design notes-*`). | A catalogue entry, a check of the client against the capsule, and a seal |
| `nonos-capsule-clock` | Built and sealed, but nothing in the desktop opens it (`docs/using/apps.md`: no tile anywhere). It holds TimeSet. | A launcher tile, then add it to `microkernel-desktop-offline` |
| `nonos-capsule-image-viewer` | The desktop's open path has no `app.image_viewer` (`PendingApp::named`, `src/userspace/init/instance_spawn/named.rs`) | A `PendingApp` variant, then add it to `microkernel-desktop-offline` |
| `nonos-capsule-ripgrep`, `nonos-capsule-sd` | With the feature on, init runs each once at boot (`src/userspace/init/entry.rs`). They are not in the terminal's tool registry. | An entry in `src/userspace/tool_capsules/registry.rs` in place of the boot spawn |

### Removed

| feature | why |
|---|---|
| `nonos-capsule-flacprobe` | Its kernel module and capsule were not in the tree, so the feature could not compile. Removed, with its cfg arms in `src/userspace/init/entry.rs` and `src/userspace/mod.rs` and its guarded include in `mk/20-build.mk`. |
| `sha1-legacy` | Nothing referred to it. |

## On in the default image, with a privacy cost

`capsule-serial-debug` is on in `standard` and `qemu` and off in `hardened` and `airgapped`. It lets capsules write to the serial console, which `Cargo.toml` calls "an information-leak surface". Whether `standard` should drop it is open, because bare-metal diagnosis reads those lines. [Privacy guarantees](../security/privacy-guarantees.md) says what it exposes.

## The `microkernel-*` sets

Ninety-one sets are named `microkernel-<x>`. Each is the core kernel plus one capsule or one lane's capsules: the per-capsule production profiles (`microkernel-keyring`, `microkernel-driver-igb`, and the rest), the smoketests, and the desktop's own building blocks. They are build compositions, not features of their own. Everything they contain is accounted for above.
