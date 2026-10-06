# Install to disk

How the NONOS installer picks a disk, what it writes there, what it refuses, and how you know the install worked.

## Start the installer

The installer is `capsule_install`. It writes the system this machine is running onto a disk you name, with its package [store](../overview/glossary.md#store) and an encrypted [data volume](../overview/glossary.md#data-volume) (`userland/capsule_install/src/install/ui/screens/welcome.rs`). There are three ways in:

- Boot the stick and choose `Install NØNOS` in the boot menu. Setup opens with Install chosen on its Mode step, and when you finish the Review step the installer takes the whole screen, with no desktop behind it. If an earlier boot of the stick kept its answers, setup does not run and the installer opens at once (`userland/capsule_setup_wizard/src/main.rs`).
- Choose `Install to this computer` on setup's Mode step, on any boot that runs setup (`src/userspace/init/supervisor/after_setup.rs`).
- From the desktop, open the Install tile on the dock (`userland/capsule_desktop_shell/src/state/apps.rs`). The installer then runs in a window.

If you leave the full-screen installer without installing, the desktop starts, and the kernel log says `[INIT] installer ended without a restart; starting the desktop` (`src/userspace/init/supervisor/after_install.rs`). A kernel built without the installer stops an install boot with `Install NONOS: this image has no installer`, before anything starts (`src/kernel_core/init/entry/install_refusal.rs`).
