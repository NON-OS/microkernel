# First boot

The thirteen steps of first-boot setup, in order: what each answer changes, and where it is kept.

## What starts before setup

After the boot menu, the loader checks the kernel, shows its proofs panel for a moment and hands over (`nonos-bootloader/src/display/boot/proofs/show.rs`). The kernel starts the compositor and the input router, and setup runs on them before any desktop app (`src/userspace/init/spawn_plan/wizard_plan.rs`).

Setup does not run:

- on a Recovery boot, which goes straight to the desktop with a Terminal open;
- when an earlier boot kept its answers: setup restores them and exits without drawing, and the kernel log says `[SETUP] kept from an earlier boot; starting the desktop`, or `opening the installer` on a boot from the menu's `Install NØNOS` entry (`userland/capsule_setup_wizard/src/main.rs`);
- on an image built with `install = false` in `nonos.toml`, which has no setup and no installer (`installFeatures` in `tools/nix/config.nix`).

There is no login and no password. The `login` service starts with nothing on screen, and no program in this release asks it to start or end a session (`userland/capsule_login/src/setup/run.rs`).

## Keys in setup

Enter goes to the next step and Escape to the one before. In a list, Up and Down or `k` and `j` move, Home and End jump to the ends, and a digit picks that row (`userland/capsule_setup_wizard/src/server/step.rs`). Ctrl+Alt+Space cycles the keyboard layout at any time, in the PS/2 and the USB keyboard drivers alike (`userland/capsule_driver_ps2_input/src/poll/absorb.rs`, `userland/capsule_driver_usb_hid/src/hid/keyboard/push_key.rs`).
