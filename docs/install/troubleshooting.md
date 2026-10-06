# Troubleshooting

The messages NONOS shows when a boot, setup or an install stops, what each one means, and what to do about it.

## Collect what NONOS says

Before you change anything, keep what the machine told you:

- The screen. A refusal from the loader stays up for 30 seconds or until you press a key, then the machine restarts (`nonos-bootloader/src/display/boot/refusal/hold.rs`). Write down its title and the text under `THE LOADER'S REASON` (`nonos-bootloader/src/display/boot/refusal/draw.rs`).
- The kernel log, from the Terminal, once the desktop is up. `log` shows the newest 200 lines, `log` with words shows only the lines that name one of them, and `log >` with a file name keeps them (`userland/capsule_terminal/src/command/builtin/log.rs`). The kernel keeps the first 64 KiB of the boot and the last 64 KiB after it (`src/sys/serial/tail.rs`).
- The serial console, if the machine has a serial port. Under QEMU it is the terminal `make boot` runs in, and with `--headless` the boot tool writes it to `target/qemu/serial.log` (`tools/nonos_qemu/__main__.py`).

```
log
log rtl wifi
log > boot-log.txt
```

Not tested in this release.

The kernel keeps that log only on images built with capsule serial output, such as `standard` and `qemu`. A `hardened` or `airgapped` image keeps nothing, and `log` prints `log: no line matches`. If `log` prints `log: the kernel would not hand over its log (this terminal needs AttestRead)`, this Terminal was not granted the log.

To report a machine, follow [Reporting a machine](../hardware/report.md).

## The boot menu says REFUSED HERE

Under each entry the menu says whether this machine meets what that entry needs (`nonos-bootloader/src/bootmenu/ready.rs`). After `REFUSED HERE: NO` it names what is missing:

| Name | What to do |
|---|---|
| `CRYPTO SELF-TEST` | the loader's own BLAKE3 or Ed25519 gave a wrong answer; write the stick again |
| `SIGNING KEYS` | this loader carries no key to check the kernel with; use a sealed image |
| `HARDWARE RNG` | turn on RDRAND or the TPM in the firmware; a virtual machine needs virtio-rng |
| `SECURE BOOT`, `PK`, `DB` | Hardened, or any entry on a `hardened` or `airgapped` image: turn Secure Boot on, with its keys enrolled, or pick Standard on a `standard` image |
| `TPM 2.0` | Hardened, or any entry on a `hardened` or `airgapped` image: turn the TPM on in the firmware, or pick Standard on a `standard` image |

Air-Gapped also needs a TPM, though the menu does not list it there ([Boot modes](boot-modes.md)).

## The loader stops

A loader refusal shows `BOOT REFUSED`, a title, then three numbered parts: `WHAT HAPPENED`, `WHAT TO DO` and `THE LOADER'S REASON` (`nonos-bootloader/src/display/boot/refusal/draw.rs`). The title comes from the start of the reason (`advice` in `nonos-bootloader/src/display/boot/refusal/advice.rs`). The titles this release shows:

| Title | Meaning | What to do |
|---|---|---|
| No kernel on this disk | no `EFI/nonos/kernel.bin` on the EFI partition | boot the stick you made, or write it again |
| The kernel is not signed | the kernel carries no release signature | boot a sealed image |
| The kernel signature does not match | Ed25519 and ML-DSA-65 did not both verify | write the stick again from one release: loader and kernel must match |
| The kernel's STARK attestation failed | the proof did not verify against the root this loader enrolled | use the loader and kernel of one release together |
| The kernel has no STARK attestation | this loader boots only an enrolled kernel | the same |
| The kernel could not be read | its program headers did not load | the media may be damaged; write it again |
| This kernel is older than allowed | its rollback index is below this machine's TPM floor; the reason reads `Rollback: tpm floor` and the two numbers | boot a release at least as new as the last one used here ([Update](update.md)) |
| This machine lacks what the entry requires | the entry's policy check failed; the reason reads `Security policy enforcement failed`, and the menu's `REFUSED HERE` line names what is missing | pick an entry the menu shows as ready, or turn on Secure Boot, the TPM and RDRAND |
| Not enough randomness | the hardware gave too little randomness for the boot's keys | turn on RDRAND or the TPM |
| Boot stopped | any other reason; the reason itself says what stopped | keep the reason and report it |

Three reasons, as the screen gives them:

- `This CPU has no NX bit` or `This CPU addresses under 36 bits of physical memory`, under `Boot stopped`. The processor lacks a feature NONOS needs. If the firmware can turn NX off, check that it is on (`nonos-bootloader/src/boot/security/hardware.rs`).
- `Air-Gapped needs a TPM: its rollback floor keeps an older signed kernel from booting`, under `Boot stopped`, and the same for Hardened. Turn the TPM on, or pick another entry (`nonos-bootloader/src/boot/crypto/rollback/floor.rs`).
- `Rollback floor could not be raised to index` and a number, on Hardened and Air-Gapped. The TPM did not take the new floor. Because the reason starts with `Rollback`, the screen's title reads `This kernel is older than allowed`, though the kernel was not refused for its age. On other entries the loader only warns `Rollback floor not raised: an older kernel may still boot` (`nonos-bootloader/src/boot/crypto/rollback/raise.rs`).

The loader also has titles for each missing Secure Boot key, the TPM and the random source (`POLICY` in `nonos-bootloader/src/display/boot/refusal/policy.rs`), but in this release every policy refusal reaches the screen as `Security policy enforcement failed` (`nonos-bootloader/src/boot/security/policy.rs`), so those titles are not shown.

If the firmware itself will not start the stick, and no NONOS screen appears at all, one cause is Secure Boot turned on without the NONOS db certificate enrolled: the seal signs the loader with that key alone (`tools/nonos_seal/chain.py`). Turn Secure Boot off and boot Standard ([Requirements](requirements.md#secure-boot-and-the-tpm)). Not tested in this release.

## The kernel stops

The kernel draws a notice band when it cannot go on, and halts (`src/boot/stop.rs`, `src/kernel_core/init/entry/`):

| On screen | Meaning | What to do |
|---|---|---|
| `NONOS BOOT STOPPED`, with a step and a detail | an early kernel step failed | keep the step and detail and report them |
| `The bootloader failed the kernel's check` | the loader's measurement, boot-root record or STARK proof did not verify | boot an image whose loader is enrolled, written from one release |
| `The bootloader could not be checked` | the boot carried no boot-root record or no loader trailer | boot a sealed image |
| `Install NONOS: this image has no installer` | the image was built without setup or the installer | restart and pick another entry; nothing was written |

[Panic and boot stop](../kernel/panic-and-boot-stop.md) explains the kernel side.

## Programs that do not start

On Safe Mode, Air-Gapped and Recovery boots the kernel refuses network drivers and services, and on Safe Mode also the audio driver and server, Snake and the hello demo. It logs each one as `[PROFILE] Safe Mode: not started: driver.hda0` and the like (`src/kernel_core/process_spawn/capsule_spawn/runner/profile_refuse.rs`). That is the mode doing its job: boot Standard to have them ([Boot modes](boot-modes.md)).

## Setup and Wi-Fi

| Message | Meaning and what to do |
|---|---|
| `No Wi-Fi driver is running: none for this chip yet, or it did not start.` | no driver answers for this card; Settings names the chip. The screen also suggests a USB Wi-Fi adapter, but NONOS 0.9.2 has no USB Wi-Fi driver: use a wired card |
| `card not supported yet; use Ethernet or USB Wi-Fi` | the iwlwifi driver took the card but has no air path for it (`userland/nonos_wifi_client/src/driver/stage.rs`); use a wired card |
| `firmware stopped:` and a step | the RTL8821CE firmware load stopped at that step; report it with the log |
| `This boot runs no network: the boot menu chose it.` | Safe Mode or Air-Gapped; boot Standard for a network |
| `[SETUP] not started:` and a reason | setup could not draw; the desktop starts without it |
| `Not applied to this session:` and names | the settings service refused those answers; set them again in Settings |
| `Nothing was applied to this session: there is no settings service` | no answer took effect this boot |

A Wi-Fi join ends with one of these lines (`userland/nonos_wifi_client/src/driver/join_text.rs`):

| Line | What to do |
|---|---|
| `The network was not heard on any channel` | move closer, press `s` to look again |
| `The access point refused the association` | the access point said no; try again |
| `The handshake did not finish; check the passphrase` | a wrong passphrase ends here; type it again |
| `A passphrase is 8 to 63 characters, or 64 hex digits` | the passphrase has the wrong length |
| `The network's security is not supported (open, TKIP or Enterprise)` | NONOS cannot join this network |
| `WPA3: the access point did not accept the password` | type the password again |
| `This driver cannot join networks yet` | the iwlwifi driver cannot join on this card |
| `The driver did not answer` | the driver stopped answering; collect the log |

Remembering a network can fail with `No NONOS store on this boot's disk`, `This boot keeps nothing across reboots` (Amnesic chosen), `No TPM to seal the passphrase with`, or, after a firmware or kernel change, `Sealed under a different boot state` (`userland/nonos_wifi_client/src/saved/error.rs`).

## The installer

| Where | Message | What to do |
|---|---|---|
| Welcome | `the bootloader did not record the running image` | boot from a stick written from a sealed image |
| Welcome | `the bootloader handed over no trailer or boot-root record, and a disk without them would not boot` | the same |
| Disks | `No driver is serving a disk yet, so there is nowhere to install for now.` | wait for the next look, press `r`, and check the disk is enabled in the firmware |
| Disks | `Intel RST/VMD is on: set the BIOS storage mode to AHCI (or turn VMD off), then boot this stick again.` | change the storage mode in the firmware |
| Disks | `holds the loader this boot ran, so it may be the boot disk: not offered` | the disk carries this same build and the firmware gave no record of the boot partition |
| Confirm | `NONOS needs a disk of at least` | use a disk of 2177 MiB or more |
| Confirm | `this disk uses 4096-byte blocks` | use a disk with 512-byte blocks |
| Confirm | `this boot's store is still loading; choose the disk again` | press Escape, wait, choose the disk again |
| Stopped | `the disk refused a transfer (status` and a number | the drive failed a write; the next lines name the request |
| Stopped | `sector` and a number, `read back different from what was written` | install again; if the read-back fails twice, the disk is failing |
| Stopped | `stopped by you before the table was written` | you pressed Escape; the disk has no partition table now |

The sources are `userland/capsule_install/src/install/`, `userland/nonos_blk_client/src/disks/` and `userland/nonos_disk/src/writer/error_text.rs`. When a write stops, the Stopped screen says whether the disk has a complete table and tells you not to boot it, names the request the driver refused, and says what to do next (`userland/capsule_install/src/install/ui/screens/failed.rs`).

## After installing

| Log line or message | Meaning |
|---|---|
| `[DATA] no machine key` | no TPM key, so the data volume stays closed |
| `[DATA] the volume holds data this key cannot open; not formatting over it` | the boot state changed since the volume was made ([Update](update.md)) |
| `data volume under another key` | the same, as a program sees it |
| `too little memory free for a volume in RAM` | a boot from the stick with too little free memory for `/data` |
| `[VFS] refused persist: amnesic boot` | the Mode step chose Amnesic, so nothing is kept |

The sources are `src/fs/blockfs_volume/`, `src/fs/vfs/map_volume_err.rs` and `userland/capsule_vfs/src/server/handlers/persist_gate.rs`.
