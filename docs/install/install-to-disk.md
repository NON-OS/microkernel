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
