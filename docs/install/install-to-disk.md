# Install to disk

How the NONOS installer picks a disk, what it writes there, what it refuses, and how you know the install worked.

## Start the installer

The installer is `capsule_install`. It writes the system this machine is running onto a disk you name, with its package [store](../overview/glossary.md#store) and an encrypted [data volume](../overview/glossary.md#data-volume) (`userland/capsule_install/src/install/ui/screens/welcome.rs`). There are three ways in:

- Boot the stick and choose `Install NØNOS` in the boot menu. Setup opens with Install chosen on its Mode step, and when you finish the Review step the installer takes the whole screen, with no desktop behind it. If an earlier boot of the stick kept its answers, setup does not run and the installer opens at once (`userland/capsule_setup_wizard/src/main.rs`).
- Choose `Install to this computer` on setup's Mode step, on any boot that runs setup (`src/userspace/init/supervisor/after_setup.rs`).
- From the desktop, open the Install tile on the dock (`userland/capsule_desktop_shell/src/state/apps.rs`). The installer then runs in a window.

If you leave the full-screen installer without installing, the desktop starts, and the kernel log says `[INIT] installer ended without a restart; starting the desktop` (`src/userspace/init/supervisor/after_install.rs`). A kernel built without the installer stops an install boot with `Install NONOS: this image has no installer`, before anything starts (`src/kernel_core/init/entry/install_refusal.rs`).

## The screens

```mermaid
stateDiagram-v2
  Welcome --> Proofs
  Proofs --> Disks
  Disks --> Confirm
  Confirm --> Writing
  Writing --> Verifying
  Verifying --> Done
  Writing --> Failed
  Verifying --> Failed
  Failed --> Disks
```

The screens run in a fixed order (`userland/capsule_install/src/install/state/screen.rs`). Escape steps back. On Welcome and on Stopped it closes the installer, and on Done it closes a windowed installer. While the disk is written, Escape only stops the write early; during the read-back no key does anything (`userland/capsule_install/src/install/event/router.rs`, `userland/capsule_install/src/install/event/after.rs`).

| Screen | Title | What you do there |
|---|---|---|
| Welcome | Install NØNOS on this computer | Read what will be written: the loader and kernel sizes, the kernel measurement, the boot verdict, the firmware's Secure Boot state, the TPM. Enter goes on. |
| Proofs | What this boot proved | Read what each proof found. `VERIFIED` and `FAILED` are verdicts a check reached. `SELF-REPORTED` is a pass on the loader's word alone, with nothing measuring it. `NOT VERIFIED`, `NOT CHECKED` and `UNKNOWN` are not passes (`Mark` in `userland/capsule_install/src/install/proofs/mark.rs`). Enter lists the disks. |
| Disks | Choose the disk | Up and Down choose a disk, `r` looks again, Enter goes on. |
| Confirm | Everything on this disk will be erased | Read what is erased and written, type the disk's word, press Enter. |
| Writing | Writing | Watch the bar. Escape stops the write only until the new partition table starts, and leaves a disk with no partition table. |
| Verifying | Reading back | Every sector written is read back and compared. This cannot be stopped. |
| Done | Installed | Remove the USB stick, then press Enter to restart. |
| Failed | Stopped | Read what happened and what to do. Enter looks at the disks again and goes back to them. |

Nothing touches the disk before Enter on Confirm (`userland/capsule_install/src/install/job/prepare.rs`, `userland/capsule_install/src/install/job/start.rs`).

## Which disks are offered

The Disks screen lists every disk the NVMe, SATA and virtio-blk drivers serve, and the SATA driver also serves Intel eMMC hosts. Each row gives the model or the bus, the size, the serial, the block size when it is not 512 bytes, and what the disk holds now: `blank`, `NONOS installed`, `another system (GPT)`, `another system (MBR)`, `unrecognised contents` or `did not answer a read` (`userland/nonos_blk_client/src/disks/probe.rs`).

Some rows are not targets, and say why (`userland/nonos_blk_client/src/disks/scan.rs`):

- The stick this boot came from is left off the list, by the loader's record of its partition.
- A disk that holds a copy of the running loader, with no such record to tell it from the stick, reads `holds the loader this boot ran, so it may be the boot disk: not offered`.
- A controller whose driver did not start reads, for example, `NVMe controller present, its driver did not come up`.
- A disk whose first sectors do not read reads `its first sectors did not read:` with the driver's status, and `not offered`.

USB disks never appear: the list asks those three drivers only (`userland/nonos_blk_client/src/driver/table.rs`). The list looks again every two seconds, or four times as long as the last look took, while it has nothing to install to or a driver is missing, and `r` looks at once (`userland/capsule_install/src/install/rescan.rs`). With no disk and an Intel RST or VMD controller on the bus, the screen says `Intel RST/VMD is on: set the BIOS storage mode to AHCI (or turn VMD off), then boot this stick again.` It adds that a Windows already on the computer may need switching to AHCI first, or it will not start after the change.

## The confirmation word

The Confirm screen names the disk again, says what erasing it destroys, lists every region it erases and writes, and shows the word to type beside the field: `the word is` and the word. The word is the last four characters of the disk's serial, in lower case, when they are printable. Otherwise it is the bus name, `nvme`, `sata` or `virtio`, with the instance number after it past the first, as in `sata1` (`userland/nonos_blk_client/src/disks/describe.rs`, `userland/nonos_blk_client/src/disks/word.rs`).

What you type is turned to lower case. Enter does nothing until the word matches and the plan for the disk is made, so no single key can start an erase (`userland/capsule_install/src/install/event/confirm.rs`).

## What is refused before anything is written

The installer makes the whole plan when you choose the disk, and says on the Confirm screen why a disk cannot take NONOS:

| Message | Why |
|---|---|
| `the disk holds ...; NONOS needs a disk of at least ... (2177 MiB)` | the disk is smaller than 2177 MiB, the floor for every image whose boot files fit in 1 GiB |
| `this disk uses 4096-byte blocks, and NONOS lays its partition table out in 512-byte ones, which firmware would not find here` | the disk's logical blocks are not 512 bytes |
| `this boot's store is still loading; choose the disk again` | the running store is not loaded yet, so what it carries is not all there |
| `the kernel gave no entropy for the identifiers` and an errno | no randomness for the disk and partition GUIDs |
| `that disk has no working driver` | the row is not a disk the installer can write |

The sources are `userland/nonos_disk/src/writer/error_text.rs`, `userland/nonos_blk_client/src/disks/describe.rs` and `userland/capsule_install/src/install/job/prepare.rs`.

## What is written

The disk gets a GPT with four partitions, laid out in 512-byte sectors (`userland/nonos_disk/src/lib.rs`, `userland/nonos_disk_map/src/places.rs`, `userland/nonos_disk/src/layout/plan.rs`):

| Sectors | Partition | What it holds |
|---|---|---|
| 0 to 33 | | the protective MBR and the primary GPT |
| 256 to 245,759 | `NONOS-STORE` | the package store |
| 245,760 to 262,143 | `NONOS-PLAN` | the disk plan naming the data volume, then the key header, cleared |
| 262,144 to the ESP | `NONOS-DATA` | the data volume, everything between the plan and the ESP |
| 1 GiB, or whole MiBs more for larger boot files, ending on the last MiB boundary before the backup GPT | `NONOS-ESP` | the boot files |
| the last 33 sectors | | the backup GPT |

The [ESP](../overview/glossary.md#esp) holds `EFI/BOOT/BOOTX64.EFI`, then `kernel.bin`, `bootloader.trailer`, `boot_root.approval`, `boot.cfg` and, when the running image has one, `kernel.approval` under `EFI/nonos/`, and `startup.nsh` at the top (`userland/nonos_disk/src/image/nonos.rs`). The loader and kernel bytes are the ones the loader verified on this boot and handed to the kernel, read back with `mk_install_source`, not read again from the stick (`userland/capsule_install/src/install/source/load.rs`).

The store carries, in this order, while it has room: setup's answers and their marker, the wallpapers those answers keep, the signed programs under `/linux/` and then `/capsules/`, each program whole or not at all, then the files the Linux programs read (`userland/nonos_disk/src/carry/gather.rs`). Nothing else from the running session is carried: files you made, the consent to run installed software and remembered Wi-Fi networks stay behind. The store is not encrypted: the kernel writes it to the disk as it is given (`src/syscall/microkernel/store_write.rs`).

The data volume is the encrypted part. The installer clears its key header and zeroes its header ring, so the first boot from the disk keys the volume with the TPM and formats it (`userland/nonos_disk/src/lib.rs`).

The installer is not a secure wipe. It first wipes the old partition tables and the kernel's markers, then writes the ESP, the zeroed header ring and key header, the store, the disk plan and the new tables, in that order (`userland/nonos_disk/src/session/queue.rs`). The rest of the data region is not overwritten: what the disk held there stays on it until the new volume writes over it. NONOS never reads those old bytes, but they are not destroyed.

## How long it takes

The write and the read-back move 2 MiB per step (`BUDGET` in `userland/capsule_install/src/install/job/work.rs`). The screen shows the percentage, the bytes and the rate the disk acknowledged, and the Done screen gives the write time in seconds (`userland/capsule_install/src/install/ui/screens/done.rs`). This release records no typical duration.
