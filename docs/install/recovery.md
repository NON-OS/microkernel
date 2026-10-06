# Recovery

What the Recovery boot mode gives you, how to read what a disk holds, and how to start over when nothing else works.

## Start Recovery

Choose Recovery in the boot menu: press `5`, then Enter ([Boot modes](boot-modes.md)). The entry is on the stick and on an installed disk alike, because the installer writes the same loader that booted the stick.

Boot Recovery from the disk whose files you want, with no NONOS stick plugged in. The kernel keeps a boot's state on a USB stick that carries NONOS before any internal disk (`ORDER` in `src/hardware/block_device/select.rs`), so Recovery from the stick opens the stick's store and a volume in memory, not the installed system's.

The loader checks the kernel exactly as for a Standard boot, so a disk whose kernel the loader refuses does not start Recovery either. Recovery changes only what the kernel starts (`src/boot/handoff/api/profile.rs`).

## What runs

- No network driver and no network service start, and no program holds the Network capability, so nothing can reach a network (`src/kernel_core/process_spawn/capsule_spawn/runner/profile_gate.rs`).
- First-boot setup is skipped, and a Terminal opens once the desktop is up (`src/userspace/init/spawn_plan/wizard_plan.rs`).
- Of the optional apps, only Files and the Editor are available, beside the Terminal and Settings every boot has (`src/userspace/init/app_choice/profile.rs`).
- The Terminal's splash reads `NONOS, Recovery boot: no network` (`userland/capsule_terminal/src/paint/fetch_boot.rs`).
- The [store](../overview/glossary.md#store) and the [data volume](../overview/glossary.md#data-volume) open as on any other boot of the same disk: the boot mode decides which programs start, not which storage is read (`src/fs/blockfs_volume/open_machine.rs`).
