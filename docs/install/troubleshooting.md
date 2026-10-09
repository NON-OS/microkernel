# Troubleshooting

The messages NONOS shows when a boot, setup or an install stops, what each one means, and what to do about it.

## Where it stopped

Find the last thing the machine showed, then read its section.

| Last thing you saw | Read |
|---|---|
| no NONOS screen at all | [The firmware does not start the stick](#the-firmware-does-not-start-the-stick) |
| `REFUSED HERE: NO` under a boot menu entry | [The boot menu says REFUSED HERE](#the-boot-menu-says-refused-here) |
| `BOOT REFUSED` and a title | [The loader stops](#the-loader-stops) |
| a notice band, such as `NONOS BOOT STOPPED` | [The kernel stops](#the-kernel-stops) |
| the boot menu or the loader's proofs panel, then a screen that never changes | [The kernel stops](#the-kernel-stops) |
| a `[PROFILE]` line, or an app that does not open | [Programs that do not start](#programs-that-do-not-start) |
| a message in setup, or a Wi-Fi join that fails | [Setup and Wi-Fi](#setup-and-wi-fi) |
| a message in the installer | [The installer](#the-installer) |
| a `[DATA]` or `[VFS]` line after installing | [After installing](#after-installing) |

## Collect what NONOS says

Before you change anything, keep what the machine told you:

- The screen. A refusal from the loader stays up for 30 seconds or until you press a key, then the machine restarts. Write down its title and the text under `THE LOADER'S REASON`.
- The kernel log, from the Terminal, once the desktop is up. `log` shows the newest 200 lines, `log` with words shows only the lines that name one of them, and `log >` with a file name keeps them. The kernel keeps the first 64 KiB of the boot and the last 64 KiB after it.
- The [serial console](../overview/glossary.md#serial-console), if the machine has a serial port. Under QEMU it is the terminal `make boot` runs in, and with `--headless` the boot tool writes it to `target/qemu/serial.log`.

```
log
log rtl wifi
log > boot-log.txt
```

Not tested in this release.

The kernel keeps that log only on images built with capsule serial output, such as `standard` and `qemu`. A `hardened` or `airgapped` image keeps nothing, and `log` prints `log: no line matches`. If `log` prints `log: the kernel would not hand over its log (this terminal needs AttestRead)`, this Terminal was not granted the log.

To report a machine, follow [Reporting a machine](../hardware/report.md).

## The firmware does not start the stick

If no NONOS screen appears at all, either the firmware did not start the loader, or it gave the loader no graphics display, which the boot menu and the refusal screens need.

- Set the firmware to boot in UEFI mode. The stick has no legacy BIOS path ([Requirements](requirements.md#processor-and-firmware)).
- With [Secure Boot](../overview/glossary.md#secure-boot) on, the firmware starts the loader only if it trusts the NONOS db certificate, because the seal signs the loader with that key alone. Turn Secure Boot off and boot Standard ([Requirements](requirements.md#secure-boot-and-the-tpm)). Not tested in this release.
- Write the stick again and compare it with the image ([Check the write](usb-stick.md#check-the-write)).

## The boot menu says REFUSED HERE

Under each entry the menu says whether this machine meets what that entry needs. After `REFUSED HERE: NO` it names what is missing:

| Name | What to do |
|---|---|
| `CRYPTO SELF-TEST` | the loader's own BLAKE3 or Ed25519 gave a wrong answer; write the stick again |
| `SIGNING KEYS` | this loader carries no key to check the kernel with; use a sealed image |
| `HARDWARE RNG` | turn on RDRAND or the TPM in the firmware; a virtual machine needs virtio-rng |
| `SECURE BOOT`, `PK`, `DB` | Hardened, or any entry on a `hardened` or `airgapped` image: turn Secure Boot on, with its keys enrolled, or pick Standard on a `standard` image |
| `TPM 2.0` | Hardened, or any entry on a `hardened` or `airgapped` image: turn the TPM on in the firmware, or pick Standard on a `standard` image |

Air-Gapped also needs a TPM, though the menu does not list it there ([Boot modes](boot-modes.md)).

## The loader stops

A loader refusal shows `BOOT REFUSED`, a title, then three numbered parts: `WHAT HAPPENED`, `WHAT TO DO` and `THE LOADER'S REASON`. The title comes from the start of the reason. The titles this release shows:

| Title | Meaning | What to do |
|---|---|---|
| No kernel on this disk | no `EFI/nonos/kernel.bin` on the EFI partition | boot the stick you made, or write it again |
| The kernel is not signed | the kernel carries no release signature | boot a sealed image |
| The kernel signature does not match | Ed25519 and ML-DSA-65 did not both verify | write the stick again from one release: loader and kernel must match |
| The kernel's STARK attestation failed | the proof did not verify against the root this loader enrolled | use the loader and kernel of one release together |
| The kernel has no STARK attestation | this loader boots only an enrolled kernel | the same |
| The kernel could not be read | its program headers did not load | the media may be damaged; write it again |
| This kernel is older than allowed | its [rollback index](../overview/glossary.md#rollback-index) is below this machine's [TPM floor](../overview/glossary.md#rollback-floor); the reason reads `Rollback: tpm floor` and the two numbers | boot a release at least as new as the last one used here ([Update](update.md)) |
| This machine lacks what the entry requires | the entry's policy check failed; the reason reads `Security policy enforcement failed`, and the menu's `REFUSED HERE` line names what is missing | pick an entry the menu shows as ready, or turn on Secure Boot, the TPM and RDRAND |
| Not enough randomness | the hardware gave too little randomness for the boot's keys | turn on RDRAND or the TPM |
| Boot stopped | any other reason; the reason itself says what stopped | keep the reason and report it |

Three reasons, as the screen gives them:

- `This CPU has no NX bit` or `This CPU addresses under 36 bits of physical memory`, under `Boot stopped`. The processor lacks a feature NONOS needs. If the firmware can turn NX off, check that it is on.
- `Air-Gapped needs a TPM: its rollback floor keeps an older signed kernel from booting`, under `Boot stopped`, and the same for Hardened. Turn the TPM on, or pick another entry.
- `Rollback floor could not be raised to index` and a number, on Hardened and Air-Gapped. The TPM did not take the new floor. Because the reason starts with `Rollback`, the screen's title reads `This kernel is older than allowed`, though the kernel was not refused for its age. On other entries the loader only warns `Rollback floor not raised: an older kernel may still boot`.

The loader also has titles for each missing Secure Boot key, the TPM and the random source, but in this release every policy refusal reaches the screen as `Security policy enforcement failed`, so those titles are not shown.

## The kernel stops

The kernel draws a notice band when it cannot go on, and halts:

| On screen | Meaning | What to do |
|---|---|---|
| `NONOS BOOT STOPPED`, with a step and a detail | an early kernel step failed | keep the step and detail and report them |
| `The bootloader failed the kernel's check` | the loader's measurement, [boot-root record](../overview/glossary.md#boot-root-record) or [STARK proof](../overview/glossary.md#stark-proof) did not verify | boot an image whose loader is enrolled, written from one release |
| `The bootloader could not be checked` | the boot carried no boot-root record or no loader trailer | boot a sealed image |
| `Install NONOS: this image has no installer` | the image was built without setup or the installer | restart and pick another entry; nothing was written |

Some stops draw nothing new and leave the last screen up: a CPU exception in the kernel, and a stop before the kernel can write to the framebuffer. Only a serial port shows why. Restart and try Safe Mode, which starts no network driver, no audio and no optional app; if it boots, report which entry worked. [Panic and boot stop](../kernel/panic-and-boot-stop.md) explains the kernel side.

## Programs that do not start

On Safe Mode, Air-Gapped and Recovery boots the kernel refuses network drivers and services, and on Safe Mode also the audio driver and server, Snake and the hello demo. It logs each one as `[PROFILE] Safe Mode: not started: driver.hda0` and the like. That is the mode doing its job: boot Standard to have them ([Boot modes](boot-modes.md)).

## Setup and Wi-Fi

| Message | Meaning and what to do |
|---|---|
| `No Wi-Fi driver is running: none for this chip yet, or it did not start.` | no driver answers for this card; Settings names the chip. The screen also suggests a USB Wi-Fi adapter, but NONOS 0.9.2 has no USB Wi-Fi driver. Read from the code, a wired card on real hardware gets no address in this release either ([the receive fault](../drivers/ethernet/README.md#the-receive-fault)) |
| `card not supported yet; use Ethernet or USB Wi-Fi` | the iwlwifi driver took the card but has no air path for it. A wired card does not help on real hardware in this release, for the reason in the row above |
| `firmware stopped:` and a step | the RTL8821CE firmware load stopped at that step; report it with the log |
| `This boot runs no network: the boot menu chose it.` | Safe Mode or Air-Gapped; boot Standard for a network |
| `[SETUP] not started:` and a reason | setup could not draw; the desktop starts without it |
| `Not applied to this session:` and names | the settings service refused those answers; set them again in Settings |
| `Nothing was applied to this session: there is no settings service` | no answer took effect this boot |

A Wi-Fi join ends with one of these lines:

| Line | What to do |
|---|---|
| `The network was not heard on any channel` | move closer and scan again: `s` in setup, Enter on the Settings Wi-Fi page |
| `The access point refused the association` | the access point said no; try again |
| `The handshake did not finish; check the passphrase` | a wrong passphrase ends here; type it again |
| `A passphrase is 8 to 63 characters, or 64 hex digits` | the passphrase has the wrong length |
| `The network's security is not supported (open, TKIP or Enterprise)` | NONOS cannot join this network |
| `WPA3: the access point did not accept the password` | type the password again |
| `This driver cannot join networks yet` | the iwlwifi driver cannot join on this card |
| `The driver did not answer` | the driver stopped answering; collect the log |

Remembering a network can fail with `No NONOS store on this boot's disk`, `This boot keeps nothing across reboots` (Amnesic chosen), `No TPM to seal the passphrase with`, or, after a firmware or kernel change, `Sealed under a different boot state`.

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

When a write stops, the Stopped screen says whether the disk has a complete table and tells you not to boot it, names the request the driver refused, and says what to do next.

## After installing

| Log line or message | Meaning |
|---|---|
| `[DATA] refused with EIO: MachineKey(...)` | no TPM key, so the [data volume](../overview/glossary.md#data-volume) stays closed |
| `[DATA] refused with EIO: Unopenable` | the boot state changed since the volume was made, and the kernel will not format over it ([Update](update.md)) |
| `[DATA] live boot:` and `no volume in memory` | a boot from the stick with too little free memory for the data volume in RAM |
| `[VFS] refused persist: amnesic boot` | the Mode step chose Amnesic, so nothing is kept |

The two `refused with EIO` lines print when a program, such as the model fetcher, asks for the volume. The kernel's own lines for the first two, `[DATA] no machine key` and `[DATA] the volume holds data this key cannot open`, go to its structured log, which nothing installs in this release, so they never print ([Logging](../kernel/logging.md#the-structured-log-and-the-debug-ring)). `[DATA] formatted a volume of` goes straight to the console and does print.

## Where this comes from

- Collect what NONOS says
  - 30 seconds or a key: `HOLD_S` in `nonos-bootloader/src/display/boot/refusal/hold.rs:17-31`.
  - The refusal's title and its three parts: `draw_card` in `nonos-bootloader/src/display/boot/refusal/draw.rs:29-48`.
  - `log`, its words and `>`: `NEWEST` in `userland/capsule_terminal/src/command/builtin/log.rs:17-31`, and its two refusals in `run` at `userland/capsule_terminal/src/command/builtin/log.rs:33-50`.
  - The first and the last 64 KiB: `HEAD` and `CAPACITY` in `src/sys/serial/tail.rs:27-32`, kept only with capsule serial output by `keep` in `src/sys/serial/tail.rs:51-52`.
  - The serial log of a headless boot: `serial` in `tools/nonos_qemu/__main__.py:68-69`.
- The firmware does not start the stick
  - The menu needs a graphics display: `run` in `nonos-bootloader/src/bootmenu/run.rs:35-38`.
  - So does a refusal screen: `show_error_screen` in `nonos-bootloader/src/display/boot/error.rs:22-25`.
  - The loader signed with the db key alone: `secure_boot` in `tools/nonos_seal/chain.py:97-104`.
- The boot menu says REFUSED HERE
  - The names under each entry: `missing` in `nonos-bootloader/src/bootmenu/ready.rs:51-66`.
- The loader stops
  - The title from the start of the reason, or `Boot stopped`: `advice` in `nonos-bootloader/src/display/boot/refusal/advice.rs:28-41`.
  - The kernel titles: `KERNEL` in `nonos-bootloader/src/display/boot/refusal/kernel.rs:22-49`.
  - The rollback, policy and randomness titles: `PLATFORM` in `nonos-bootloader/src/display/boot/refusal/platform.rs:21-36`.
  - The `Rollback: tpm floor` reason: `enforce_floor` in `nonos-bootloader/src/boot/crypto/rollback/floor.rs:28-39`.
  - The NX and 36-bit reasons: `verify_hardware_requirements` in `nonos-bootloader/src/boot/security/hardware.rs:27-35`.
  - A TPM needed for Hardened and Air-Gapped: `Floor::Refuse` in `nonos-bootloader/src/boot/crypto/rollback/floor.rs:41-51`.
  - The floor not raised: `raise_failed` in `nonos-bootloader/src/boot/crypto/rollback/raise.rs:33-45`.
  - The per-key titles, never reached: `POLICY` in `nonos-bootloader/src/display/boot/refusal/policy.rs:28`, against `enforce_policy` in `nonos-bootloader/src/boot/security/policy.rs:28-39`.
- The kernel stops
  - `NONOS BOOT STOPPED`, a step and a detail: `stop` in `src/boot/stop.rs:25-41`.
  - The two loader checks: `refuse_unchecked_loader` in `src/kernel_core/init/entry/loader_refusal.rs:27-45`.
  - No installer in the image: `refuse_install_without_installer` in `src/kernel_core/init/entry/install_refusal.rs:34-47`.
- Programs that do not start
  - What each mode refuses: `refused` in `src/kernel_core/process_spawn/capsule_spawn/runner/profile_refuse.rs:21-39`.
  - The `[PROFILE]` line: `check` in `src/kernel_core/process_spawn/capsule_spawn/runner/profile_gate.rs:31-44`.
- Setup and Wi-Fi
  - `card not supported yet`: `NoAirPath` in `userland/nonos_wifi_client/src/driver/stage.rs:60`.
  - The join lines: `join_text` in `userland/nonos_wifi_client/src/driver/join_text.rs:15-34`.
  - Why a network was not remembered: `text` in `userland/nonos_wifi_client/src/saved/error.rs:31-36`.
- The installer
  - The Welcome messages: `load` in `userland/capsule_install/src/install/source/load.rs:51-64`.
  - No disk, and RST or VMD: `NO_DISK` and `RAID` in `userland/capsule_install/src/install/ui/screens/disks.rs:34-36`.
  - The boot disk not offered: `withheld` in `userland/nonos_blk_client/src/disks/scan.rs:139-150`.
  - Too small, a refused transfer, a failed read-back: `WriteError` in `userland/nonos_disk/src/writer/error_text.rs:26-41`.
  - 4096-byte blocks: `refusal` in `userland/nonos_blk_client/src/disks/describe.rs:48-56`.
  - The store still loading: `store_settled` in `userland/capsule_install/src/install/job/prepare.rs:46-48`.
  - Stopped by you: `Outcome` in `userland/capsule_install/src/install/event/cancel.rs:36-43`.
  - The Stopped screen: `paint` in `userland/capsule_install/src/install/ui/screens/failed.rs:37-57`.
- After installing
  - `[DATA] no machine key` and `MachineKey`: `open_machine_volume` in `src/fs/blockfs_volume/open_machine.rs:73-77`.
  - `Unopenable`, and `[DATA] formatted a volume of` on the console: `mount_or_format` in `src/fs/blockfs_volume/mount_or_format.rs:64-76`.
  - The live boot line: `LEAST` in `src/fs/blockfs_volume/session.rs:34-44`.
  - The `refused with EIO` lines on the console: `ERRNO_IO` in `src/syscall/microkernel/data/errno.rs:47-53`.
  - The amnesic refusal: `require_persistent` in `userland/capsule_vfs/src/server/handlers/persist_gate.rs:35-39`.

## See also

- [Reporting a machine](../hardware/report.md)
- [Recovery](recovery.md)
- [Boot modes](boot-modes.md)
- [Install to disk](install-to-disk.md)
- [Panic and boot stop](../kernel/panic-and-boot-stop.md)
- [Logging](../kernel/logging.md)
