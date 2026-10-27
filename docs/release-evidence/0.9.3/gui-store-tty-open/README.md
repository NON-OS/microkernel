# GUI 02: the Marketplace's Open of a terminal program runs it in the Terminal, booted 27 October 2026

Before: Open started app.linux.run with no terminal, so micro, vim or age
started with nothing to show them on. Now the run reads its program's box,
and one made for a terminal (TTY, no WAYLAND) asks the desktop shell to run
`linux <package>` in the Terminal (c55d58e10).

## How it ran

- Image: `make dev-image PROFILE=standard` at 934291c72 on s2/gui
  (standard-dev, throwaway keys), relay as in gui-swayimg.
- QEMU TCG, 4 CPUs, 8 GiB, `--tpm`, `--lane usb-tablet` (the pointer is
  placed with nonos-drive's `abs:`), fresh disk. `drive.steps`: setup, the
  Marketplace's dock icon, `/` `micro` Enter, Enter to install, Enter to
  open.

## What it showed

- `marketplace.png`: the Marketplace, opened from the dock.
- `micro-installed.png`: micro installed (`[LINUX-INSTALL] started, in its
  release's box linux.micro`, `[LINUX] installed`).
- `open-runs-in-terminal.png`: Enter on the installed micro. The serial
  shows `[LINUX-RUN] started micro`; the Terminal comes forward and runs
  `linux micro`, whose policy binds `FILES_HOME TTY`; the Marketplace says
  "starting it; a terminal app runs in the Terminal: linux micro".

## What does not work yet

- micro itself stops: `open /dev/tty: no such device or address`, then
  "Fatal: Micro could not initialize a Screen". A `linux` run in the
  Terminal has no /dev/tty; micro (tcell) opens it rather than its stdin.
  The same happens to `linux micro` typed by hand, before and after this
  change. /dev/tty is the controlling terminal, which the pseudo-terminals
  of linux/pty define; it is for after that branch lands.
