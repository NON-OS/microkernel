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

Host-tested on 2026-10-24: the closure has every production capsule below and leaves out 155 features. Of those, 92 are `microkernel-*` lane sets and 63 are the features in the tables. `nonos-fault-proof`, added since, is a 156th and is not in those counts.

## What the default image carries

`microkernel-full-gui` (`Cargo.toml`) is the desktop and every real program and driver:

- the desktop: the compositor, window manager, shell, login, settings, the apps, the terminal, the Linux personality, the browser, the wallet and the market;
- the network stack and both anonymity networks: `net.core`, `net.sockets`, Nym (`net.nym`, `net.socks5`) and Anyone (`net.anon`);
- the NOX settle and proof fetch services: `nonos-capsule-nox-settle`, `nonos-capsule-prove-fetch`;
- first-boot setup and the installer;
- every production hardware driver:
  - NICs: virtio, e1000, e1000e, igb, rtl8139, rtl8169, pcnet, vmxnet3, tulip, iwlwifi, rtl8821ce, rtl8822ce, rtl8852be and mt7921;
    and the ASIX AX88179 and Realtek RTL8153 USB adapters, host-proved, not booted (QEMU has neither), both outside the Standard profile;
  - storage: virtio-blk, virtio-scsi, NVMe, AHCI, PIIX IDE, pvscsi, MPT, SDHCI, UFS and USB mass
    storage, and BusLogic, host-proved, not booted (QEMU has no BusLogic device);
  - USB host controllers: xHCI, EHCI, OHCI and UHCI, and USB hubs;
  - input: PS/2, virtio-input, USB HID and I2C HID, and Bluetooth LE keyboards and mice over a USB
    controller (`driver-bt-usb`, with the RTL8821C patch it loads), host-proved, not yet booted on hardware;
  - display: virtio-gpu and VMware SVGA;
  - audio: HDA, AC'97 and USB audio;
  - USB cameras (UVC), smart cards (CCID) and FIDO security keys;
  - the I2C bus;
  - VirtualBox and Hyper-V guest support.

`standard` takes `capsule-serial-debug` out of that set, so no capsule writes to the serial console (`drop`, `tools/nix/config.nix`). The `hardened` and `airgapped` profiles start from the same set and take out the same feature, and `airgapped` also takes out every network feature (`networkFeatures`, `tools/nix/config.nix`). Both take out Bluetooth, a radio (`radioFeatures`).

## The drivers this change adds, and what booted

the maintainers' rule: a driver QEMU can exercise goes in once a boot shows it working, and a driver QEMU cannot exercise goes in only if it stays idle when it finds no device. Each lane below booted the throwaway-key `standard-dev` image (lanes A to C: sha256 `e1b3cff8…`, commit 308615832; lane E: `2f423b3b…`, commit e90c51abf, with the OHCI fix) under QEMU q35 with VT-d (`iommu=intel`), TCG, for 25 minutes, with the egress capture on (`scripts/no_telemetry_qemu_capture.sh`, `NONOS_QEMU_LANES`). A `standard` image prints nothing from its capsules, so "up" means spawned, its device confined to its own IOMMU domain where it has one, and no exit for the whole run. It does not mean a transfer was seen, except where the capture shows one.

| driver | lane | what the boot showed | level |
|---|---|---|---|
| e1000e | A: `nic=e1000e ehci ufs usb-audio` | carried every frame of the boot: 2313 frames, egress PASS, MAC `02:75:03:de:83:9a` | booted, working |
| igb | B: `nic=igb uhci ohci` | carried every frame: 2307 frames, egress PASS, MAC `32:c8:c4:83:14:a0` | booted, working |
| ehci | A | up, with the lane's USB stick behind it | booted, up |
| uhci | C: `uhci` | up beside AHCI, with no refusal in the run (egress PASS, 2278 frames), on a tree before H-02; under H-02 its Command write was refused until 5ef54ad2e9, then booted on 667ef7b947: `8086:7020 started, serving driver.uhci0` | booted, up |
| | A | as the EHCI's companion, refused: `00:1d.7 reaches it through 00:1d.0 without ACS; driver.ehci0 holds it`, then status 6. The group is not split across capsules (the maintainers' H-02 ruling), so this is the rule working | booted, refused as designed |
| ohci | E: `nic=usb-net bochs-display usb-ccid ohci` | up, with AHCI up beside it and no refusal in the run | booted, up |
| | B, before the fix | status 6: it asked for its 256-byte BAR rounded up to a page, which the broker refuses as past the BAR (`[MMIO] err` EINVAL). Its retries took AHCI down with it. Fixed in the driver (`register_grant`, `facts/pci.rs`); EHCI had the same rounding, which QEMU's 4 KiB BAR hid and an Intel 1 KiB BAR would not. `ohci_proofs` and `ehci_proofs` check the grant against the broker's own window rule, and the old rounding is caught | host-tested |
| ufs | A | up, its UFS device present | booted, up |
| usb-hub, usb-audio | A, B | up | booted, up |
| ccid | E | up with a card reader present (`usb-ccid`, no card) | booted, up |
| uvc, fido | every lane | up and idle with no device; QEMU has no UVC camera, and a FIDO key needs the host's | booted, idle |
| prove-fetch | every lane | up; it sent no frame the capture failed | booted, up |
| nox-settle | none | not reached: every lane's boot waits at first-boot setup (`app.setup_wizard`), and nox-settle comes up with the desktop services after it (`desktop_services.rs`) | host-tested |

## Taken back out, after the lanes

These were in this change's first draft and are out of the default image again. Each is listed under "Real, but held back" with what would let it ship.

- ixgbe, igc, cdc-ncm, cdc-acm and usb-mic: QEMU has no device for them (no 82599 or I225, no NCM function, its `usb-serial` is FTDI, its `usb-audio` plays only), so no lane can show them working.
- cdc-ecm and rndis: lane E put QEMU's `usb-net` in place of the NIC, and the capture held no frame from the guest (`EMPTY`), so neither carried traffic. net.core's calls to both went unanswered (`[DIAG] ipc.call unanswered driver.rndis0`, `driver.cdc_ecm0`, ETIMEDOUT).
- bga: lane E, status 6. Its framebuffer map asks for write-combining (`MK_MMIO_MAP_WC`, L17), and the broker knows no MMIO flag (`FLAGS_KNOWN = 0`, `src/hardware/broker/mmio/map.rs`), so it answers EOPNOTSUPP.

## The ACPI capsule, power only

`nonos-capsule-acpi` is in `microkernel-full-gui`, so Standard, Hardened and Air-Gapped carry it. It is built with `release-power-only` (`userland/capsule_acpi/Capsule.mk`), which keeps what the battery, AC, lid and buttons need and leaves out every part that acts on the machine beyond them (`userland/capsule_acpi/src/release/rule.rs`, proved in `capsule_acpi_proofs`; the capsule's README, "Release build", gives each reason). Hardened and Air-Gapped keep it: it reaches no network, claims no PCI function and does no DMA, and its tables are measured into the attestation registry.

- Runs: the tables loaded, the EC found, the regions connected with their _REG, `_INI`, ACPI mode with the runtime GPEs and the SCI, the EC's GPE and _Qxx, batteries and AC adapters (`_STA`, `_BIX`/`_BIF`, `_BST`, `_PSR`) reported through `MkAcpiPower`, lids and control-method buttons, and what only reads (inventory, `_PRT`, SMBIOS, serial and legacy devices, proximity, the PCI `_DSM` facts, hotkey names).
- Left out: `\_PIC(1)`, every `_OSC`, `\_S5` to the kernel and PREPARE_SLEEP's `\_TTS`/`\_PTS`, the reserved ranges, hidden functions' `_PS0`, hot plug, platform device records, `_BTP`, the capsule's own wiped power-off, the fixed power button's hand-over, cooling, processor records, backlight, hardware-error acknowledgement, port facts, Type-C, device power and the ops that act (SET_BRIGHTNESS, PREPARE_SLEEP, DEVICE_POWER, GPIO_EVENT, SET_POWER_CAP answer EPERM).
- A power-button press reaching the capsule (a PNP0C0C Notify) goes to the desktop as its Power key; the fixed button stays with the kernel's poll. A critical battery is reported, and the kernel's own watch (ask the desktop, then power off 30 s later) acts on it.
- MkBatteryStatus form 1 hands the bar and About the report's whole word, so they say "57% Charging". The word carries no rate or capacity, so neither shows a time left.

Still open from before: a `Firmware` capability bit in place of Admin, and the seal.

## A capsule that is in the image but not yet sealed

The kernel embeds each capsule's certificate, manifest and STARK trailer from `nonos-data/trust/capsules/`. The release seal writes those files with the release keys, and the maintainers commit them. `nix build` refuses a kernel that ships a capsule with no trust set (`unsealed`, `tools/nix/image.nix`). That gate applies before and after this change:

- On 2026-10-24, before this change, 15 capsules that the default image already carried had no trust set: pcnet, vmxnet3, tulip, virtio-input, vmware-svga, ac97, vmmdev, vmmouse, vmbus, piix-ide, virtio-scsi, pvscsi, buslogic, mpt and sdhci. Host-tested: the gate's own condition, run over `tools/nix/capsules.json`.
- The 13 capsules this change adds have no trust set either: the 11 drivers, plus nox-settle and prove-fetch.
- On 2026-10-27 S4 added `driver-ixgbe`, `driver-cdc-ecm`, `driver-cdc-ncm`, `driver-rndis`, `driver-smbus` and `driver-bga`, and took `driver-ccid`, `driver-fido` and `driver-uvc` out of the profile (no client in this release can use them; their reasons are in docs/hardware/MATRIX.md).
- On 2026-10-27 S4 added `driver-igc` (Intel I225 and I226, the 2.5 GbE of most mini PCs and many boards since 2020) to `microkernel-full-gui`; it has no trust set either.
- On 2026-10-27 S4 added `driver-rtl8822ce` (Realtek RTL8822CE, 10ec:c822 and 10ec:c82f, a 2x2 Wi-Fi 5 card in many laptops since 2019) to `microkernel-full-gui`; it has no trust set either. Its PHY tables are rtw88's, taken under that code's BSD-3-Clause option (userland/capsule_driver_rtl8822ce/README.md).
- On 2026-10-27 S4 added `driver-mt7921` (MediaTek MT7921 and MT7922 Wi-Fi: AMD RZ608 and RZ616, the ThinkPad T14 Gen 4 AMD among many) to `microkernel-full-gui`; it has no trust set either. Its card firmware is not in the image: the loader reads the pinned package from the boot medium (`scripts/firmware_package.py`).
- On 2026-10-27 S4 added `driver-rtl8852be` (Realtek RTL8852BE, 10ec:b852 and 10ec:b85b, a 2x2 Wi-Fi 6 card in many laptops since 2021, run as a 20 MHz legacy station) to `microkernel-full-gui`; it has no trust set either. Its register tables are rtw89's, taken under that code's BSD-3-Clause option (userland/capsule_driver_rtl8852be/docs/licence.md).
- On 2026-10-28 S4 added `acpi` (capsule_acpi, built with its `release-power-only` feature) to `microkernel-full-gui`, so the bar reads the battery; it has no trust set either. What that build runs and leaves out is below, under "The ACPI capsule, power only".
- On 2026-10-28 S4 added `driver-bt-usb` (the Bluetooth LE host over USB, for keyboards and mice; on the RTL8821CE combo card it first loads Realtek's RTL8821C patch, linux-firmware's, embedded under its redistribution licence: userland/capsule_driver_bt_usb/firmware/README.md) to `microkernel-full-gui`; it has no trust set either. `hardened` and `airgapped` take it out.
- `driver-ax88179` (ASIX AX88179, AX88178A) is built by its own `microkernel-driver-ax88179` profile and is not in `microkernel-full-gui`: its register programming was read from Linux, which Decision 10 leaves to the owner.
- `driver-rtl8153` (Realtek RTL8153, RTL8153B) likewise: its own `microkernel-driver-rtl8153` profile, not in `microkernel-full-gui`, until Decision 10 is ruled.
- A trust set that exists can still be stale. `nix build` also refuses a kernel when a shipped capsule's sealed manifest grants other caps than its `Capsule.mk` now says (`scripts/check_sealed_caps.py`, called from `tools/nix/image.nix`). On 2026-10-24, nine sealed manifests were stale this way. Five follow from this change's narrowing: keyring, login, payment, wallet-nonos and virtio-gpu. Four come from changes on main since the last seal: input-router, linux, boot-splash and i2c-pci. The next seal rewrites all nine. Host-tested: the script's `--self-test`, run by the flake's `static-abi` check.

So `nix build` of the default image waits for the next release seal, which writes all 37 trust sets in one step. A development image (`make dev-image`, `tools/nonos-dev-image`) is sealed with throwaway keys and builds and boots now.

The 37, as `scripts/baselines/unsealed.txt` holds them. The flake's `static-release-trust` check fails when a release profile ships any other capsule without its trust set, when one of these is sealed and still listed, or when this list and the baseline differ:

<!-- unsealed: begin -->
`acpi`, `driver-ac97`, `driver-bga`, `driver-bt-usb`, `driver-buslogic`, `driver-cdc-ecm`, `driver-cdc-ncm`, `driver-e1000e`, `driver-ehci`, `driver-igb`, `driver-igc`, `driver-ixgbe`, `driver-mpt`, `driver-mt7921`, `driver-ohci`, `driver-pcnet`, `driver-piix-ide`, `driver-pvscsi`, `driver-rndis`, `driver-rtl8822ce`, `driver-rtl8852be`, `driver-sdhci`, `driver-smbus`, `driver-tulip`, `driver-ufs`, `driver-uhci`, `driver-usb-audio`, `driver-usb-hub`, `driver-virtio-input`, `driver-virtio-scsi`, `driver-vmbus`, `driver-vmmdev`, `driver-vmmouse`, `driver-vmware-svga`, `driver-vmxnet3`, `nox-settle`, `prove-fetch`.
<!-- unsealed: end -->

## Off, and why

### Test and development only: never in a release

| feature | why it is off | evidence |
|---|---|---|
| `nonos-dev-attest` | admits capsules on their Merkle path with no STARK proof | `compile_error!` beside `nonos-release`, `src/lib.rs`; `resolve` asserts it needs loader `dev-qemu` |
| `nonos-dev-unconfined-dma`, `nonos-dev-unremapped-interrupts` | DMA with no IOMMU domain, or interrupts with no remapping | each refuses `nonos-production` and `nonos-release` (`src/memory/iommu/dev_unconfined.rs`, `dev_unremapped.rs`) |
| `nonos-attest-refusal-smoketest` | embeds four deliberately broken capsules | `compile_error!` beside `nonos-production`, `src/lib.rs` |
| `nonos-panic-proof` | panics init once the spawn plan has run, to show the panic path's halt, wipe and power-off booted | `compile_error!` beside `nonos-production` or `nonos-release`, `src/userspace/init/injected.rs`; named in no profile, and `scripts/check_proof_features.py` holds it to the compile-only `make check-dark-features` |
| `nonos-fault-proof` | raises a kernel #UD in init once the spawn plan has run, to show a CPU exception's kernel branch wiping and powering off booted | the same guard and the same check as `nonos-panic-proof` (`src/userspace/init/injected.rs`) |
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
| `pqc-unvetted` | SPHINCS+, NTRU and Classic McEliece, whose in-kernel implementations each fail their first full run against published vectors | `Cargo.toml` comment; `nonos-ci/check-pqc-unvetted.sh` refuses it in any default or release profile |
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
| `nonos-capsule-payment` | The meter. Every development image (loader `dev-qemu`) carries it with settle, through `marketDevFeatures` in `tools/nix/config.nix`; a release profile that asks for it fails to evaluate. The owner chooses its release profile after re-signing. It also keeps `/data/meter.key` on a persistent install (`userland/capsule_payment/README.md`). | The owner's re-sign and choice |
| `nonos-capsule-driver-gpio` | Not in `capsules.json` or the mk include block. Its wire was never checked against the client. | A catalogue entry, a check of the client against the capsule, and a seal |
| `nonos-capsule-driver-intel-display` | Held by the maintainers' ruling until it boots on real Intel hardware. Shipped unproven, it takes over the panel on every Intel laptop. | A real-hardware boot from the hardware-breadth session, then add it to `microkernel-full-gui` |
| `nonos-capsule-driver-spi`, `nonos-capsule-driver-spi-hid` | the maintainers' rule for the default image is that a driver whose device is absent must stay idle. These two leave at once on a machine with no SPI function (`src/userspace/init/spawn_plan/drivers_spi.rs`), and nothing in the hardware inventory classifies a device as `SerialSpi` (`src/hardware/inventory/`), so their spawn cannot be gated on presence either. A booted standard-dev image hung after another driver that exits for want of its device left (`[EXIT] code=2`, then a lock wait in `src/kernel_core/surface_registry/pin`), the case the black-desktop fix addresses. | The inventory classifying LPSS SPI functions as `SerialSpi` and the spawn gated on it, or the drivers idling without a device |
| `nonos-capsule-driver-smbus` | Booted (standard-dev, QEMU q35 with VT-d, 2026-10-24): `driver.smbus0` claimed the ICH9 SMBus function 00:1f.3. That shares an IOMMU group with the SATA controller 00:1f.2, with no ACS between them, so the IOMMU then refused AHCI (`[VT-D] refused: 00:1f.3 reaches it through 00:1f.2 without ACS; driver.smbus0 holds it`), and `driver.ahci0` gave up (status 6). On a chipset grouped the same way, carrying SMBus takes the SATA boot disk away. | Claim order or group handling that keeps the disk controller's claim ahead of the SMBus one, shown on such a machine |
| `nonos-capsule-driver-ixgbe`, `nonos-capsule-driver-igc` | No QEMU device (82599, I225), so no boot has shown them working. igc came from main wired and host-tested, not signed or booted | A boot on the hardware, from the hardware-breadth session |
| `nonos-capsule-driver-cdc-ecm`, `nonos-capsule-driver-rndis` | The `nic=usb-net` lane (QEMU's usb-net, RNDIS and CDC-ECM configurations) captured no frame from the guest: neither driver carried traffic | A lane that carries the boot's traffic over usb-net, with a debug image to find where it stops |
| `nonos-capsule-driver-cdc-ncm`, `nonos-capsule-driver-cdc-acm`, `nonos-capsule-driver-usb-mic` | QEMU has no NCM function, its `usb-serial` is FTDI rather than ACM, and its `usb-audio` has no input, so no lane can show them working | A boot with the device, on hardware or another emulator |
| `nonos-capsule-driver-bga` | Its framebuffer map asks for `MK_MMIO_MAP_WC`, which the broker does not implement (`FLAGS_KNOWN = 0`, `src/hardware/broker/mmio/map.rs`): EOPNOTSUPP, status 6 on the `bochs-display` lane. It also matches QEMU's default `-vga std` | Write-combining MMIO in the broker (a kernel change, the maintainers'), or the driver mapping its framebuffer uncached, then the lane again |
| `nonos-capsule-clock` | Built and sealed, but nothing in the desktop opens it (`docs/using/apps.md`: no tile anywhere). It holds TimeSet. | A launcher tile, then add it to `microkernel-desktop-offline` |
| `nonos-capsule-image-viewer` | The desktop's open path has no `app.image_viewer` (`PendingApp::named`, `src/userspace/init/instance_spawn/named.rs`) | A `PendingApp` variant, then add it to `microkernel-desktop-offline` |
| `nonos-capsule-ripgrep`, `nonos-capsule-sd` | With the feature on, init runs each once at boot (`src/userspace/init/entry.rs`). They are not in the terminal's tool registry. | An entry in `src/userspace/tool_capsules/registry.rs` in place of the boot spawn |

### Removed

| feature | why |
|---|---|
| `nonos-capsule-flacprobe` | Its kernel module and capsule were not in the tree, so the feature could not compile. Removed, with its cfg arms in `src/userspace/init/entry.rs` and `src/userspace/mod.rs` and its guarded include in `mk/20-build.mk`. |
| `sha1-legacy` | Nothing referred to it. |

## Taken out of the default image

`capsule-serial-debug` lets capsules write to the serial console. `Cargo.toml` calls it "an information-leak surface". `standard` now takes it out, as `hardened` and `airgapped` already did. The `qemu` and `dev` profiles, and a profile's development twin, keep it for diagnosis. What changes for a person: the Terminal's `log` keeps nothing on a `standard` image (`keep`, `src/sys/serial/tail.rs`).

`proof_io` goes with it. Its proof is a serial line, and its third step needs Debug, so in an image without `capsule-serial-debug` it exits 3, a false proof failure on every boot. It is now spawned only beside that feature (`run_user_entry_proof` and `launch_final_payload`, `src/userspace/init/entry.rs`), and `microkernel-proof-io`, the lane its line is for, turns the feature on. Host-tested: `kernel_proofs` `proof_io_spawn_tests` reads the cfg around every spawn in `src/`. A mutant that drops it from the final spawn is caught.

## The `microkernel-*` sets

Ninety-one sets are named `microkernel-<x>`. Each is the core kernel plus one capsule or one lane's capsules: the per-capsule production profiles (`microkernel-keyring`, `microkernel-driver-igb`, and the rest), the smoketests, and the desktop's own building blocks. They are build compositions, not features of their own. Everything they contain is accounted for above.
