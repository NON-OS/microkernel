# Recovery

What the Recovery boot mode gives you, how to read what a disk holds, and how to start over when nothing else works.

## Start Recovery

Choose Recovery in the boot menu: press `5`, then Enter ([Boot modes](boot-modes.md)). The entry is on the stick and on an installed disk alike, because the installer writes the same loader that booted the stick.

Boot Recovery from the disk whose files you want, with no NONOS stick plugged in. The kernel keeps a boot's state on a USB stick that carries NONOS before any internal disk, so Recovery from the stick opens the stick's store and a volume in memory, not the installed system's.

The loader checks the kernel exactly as for a Standard boot, so a disk whose kernel the loader refuses does not start Recovery either. Recovery changes only what the kernel starts.

## What runs

- No network driver and no network service start, and no program holds the Network [capability](../overview/glossary.md#capability), so nothing can reach a network.
- First-boot setup is skipped, and a Terminal opens once the desktop is up.
- Of the optional apps, only Files and the Editor are available, beside the Terminal and Settings every boot has.
- The Terminal's splash reads `NONOS, Recovery boot: no network`.
- The [store](../overview/glossary.md#store) and the [data volume](../overview/glossary.md#data-volume) open as on any other boot of the same disk: the boot mode decides which programs start, not which storage is read.

## What you can do in Recovery

Read the kernel log in the Terminal. `log` shows the newest 200 lines, `log` with words shows only the lines that name one of them, and `log >` keeps the lines in a file:

```
log
log tpm data
log > boot-log.txt
```

Not tested in this release.

The log is there only on images built with capsule serial output, such as `standard` and `qemu`. A `hardened` or `airgapped` image keeps nothing, and `log` prints `log: no line matches`.

Look through your files in Files and the Editor. On an installed system a file reaches the disk in one of two ways: a program keeps it in the store, for example with `keep` and the file's path in the Terminal, or it is imported into the data volume, as Qwen models are. Files does not show the data volume: its `/data` folder belongs to the file store. Files that were never kept lived in memory and are gone.

## What Recovery cannot do

- It cannot copy files to a USB stick or another disk. The file service keeps files in memory and in the NONOS store, and mounts no other file system, and the USB storage driver serves sectors, not files.
- It cannot reach a network, by design.
- It cannot open a data volume the TPM no longer gives the key for.

## When the data volume does not open

The installed data volume opens only on the machine that made it, in the same boot state: its key is derived by the [TPM](../overview/glossary.md#tpm) under [PCRs](../overview/glossary.md#pcr) 0, 4, 7 and 9, and never stored. After a firmware update or a change to Secure Boot, the TPM gives another key, and the kernel leaves the volume closed rather than format over it. A program that asks for the volume then gets EIO, and the serial console says `[DATA] refused with EIO: Unopenable`, or `MachineKey(...)` in place of `Unopenable` when the TPM gave no key at all.

To get it back, put the firmware and the Secure Boot setting back as they were when the volume was made, and boot again. A reinstall erases the volume for good.

## Start over

If the installed system does not boot, or you want a clean one:

1. Plug the stick in and boot it. Open the firmware's boot menu and choose the stick.
2. Choose `Install NØNOS` and answer setup. If an earlier boot of the stick kept its answers, setup does not run and the installer opens at once.
3. Choose the old disk, type its word, and let the installer write and read it back ([Install to disk](install-to-disk.md)).

This erases the old store and data volume. If the stick itself is refused, the screen says why: see [Troubleshooting](troubleshooting.md).

## Where this comes from

- Start Recovery
  - A USB stick that carries NONOS comes first: `ORDER` in `src/hardware/block_device/select.rs:39-48`.
  - The loader ran the profile's checks, and the kernel changes what it starts: `BootProfile` in `src/boot/handoff/api/profile.rs:17-25`.
- What runs
  - No network driver or service starts: `refused` in `src/kernel_core/process_spawn/capsule_spawn/runner/profile_refuse.rs:34-39`.
  - No program keeps the Network capability: `caps` in `src/kernel_core/process_spawn/capsule_spawn/runner/profile_gate.rs:48-55`.
  - Setup skipped, then a Terminal: `skips_setup` in `src/userspace/init/spawn_plan/wizard_plan.rs:29-47`.
  - Only Files and the Editor: `withheld` in `src/userspace/init/app_choice/profile.rs:37-44`.
  - The splash line: `os_line` in `userland/capsule_terminal/src/paint/fetch_boot.rs:17-26`.
  - Storage opened the same on every boot mode: `open_machine_volume` in `src/fs/blockfs_volume/open_machine.rs:42-79`.
- What you can do in Recovery
  - The newest 200 lines, words and `>`: `NEWEST` in `userland/capsule_terminal/src/command/builtin/log.rs:17-31`.
  - The log kept only with capsule serial output: `keep` in `src/sys/serial/tail.rs:51-52`, and `log: no line matches` from `run` in `userland/capsule_terminal/src/command/builtin/log.rs:33-50`.
  - Keeping a file in the store: `persist` in `userland/capsule_terminal/src/command/builtin/nox/keep.rs:17-30`.
- What Recovery cannot do
  - Files in memory and in the store only: `capsule_vfs` in `userland/capsule_vfs/README.md:5-9`.
  - The USB storage driver serves sectors: `capsule_driver_usb_msc` in `userland/capsule_driver_usb_msc/README.md:5-8`.
- When the data volume does not open
  - PCRs 0, 4, 7 and 9: `BOUND_PCRS` in `src/security/tpm/machine_key/pcrs.rs:21-24`.
  - Left closed, never formatted over: `mount_or_format` in `src/fs/blockfs_volume/mount_or_format.rs:50-76`.
  - EIO, with `Unopenable` or `MachineKey(...)`: `errno` in `src/syscall/microkernel/data/errno.rs:30-53`.

## See also

- [Boot modes](boot-modes.md)
- [Update](update.md)
- [Troubleshooting](troubleshooting.md)
- [Terminal](../using/terminal.md)
- [Files](../using/files.md)
