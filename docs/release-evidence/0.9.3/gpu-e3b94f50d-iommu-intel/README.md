# gpu: gate boot of e3b94f50d, 25 October 2026

Four changes, one boot (letter HANDOFF/letters/gpu/10-flush-focus-terminal-bgrt.md):
- the BGRT logo path removed;
- a FLUSH waits FLUSH_MS 400;
- the menu bar follows focus;
- one Terminal process for one window.

## How it ran

- Image: `make dev-image` at e3b94f50d (qemu-dev, throwaway keys).
- Machine: QEMU TCG, no KVM, 4 CPUs, 8 GiB, `--lane iommu=intel`, fresh disk.
- Driven over QMP: Enter through all 13 setup steps, 105 s, then a
  screendump. After that, the PS/2 pointer moved and clicked:
  - "Not now" on the live-session dialog;
  - the dock's Terminal;
  - the dock's Settings;
  - the Terminal's window.

## What it showed

- `serial.log`:
  - `[IOMMU] intel-vt-d present, enforcing=1`, `[SMP] answered=3 online=4`,
    `[DISPLAY] on driver.virtio_gpu0 1920x1080`.
  - No `ipc.call unanswered` line.
  - No `late driver.virtio_gpu0` and no `present refused`. The three boots
    before had two to four such lines each.
  - One `[SPAWN] name=app.terminal` (pid 0x3B, at boot) and no
    `app.terminal.1`: the click opened the boot process's window.
- Screenshots:
  - `desktop.png`: the desktop after setup, menu bar and dock, 3345 colours.
  - `1-terminal-opened.png`: the Terminal open; the menu bar says Terminal.
  - `2-settings.png`: Settings opened over it; the menu bar says Settings.
  - `3-terminal-raised.png`: the Terminal's window clicked. It is raised and
    the menu bar says Terminal again; before the fix it went on naming the
    app before.
  - `splash.png`: the splash.

## Seen in this run, not gpu's

- `ipc.call late` for `driver.virtio_net0` (net.core's polls) and `attest`:
  short caller deadlines under spawn load (hw's B-04).
