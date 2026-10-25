# Ctrl+Alt+T, booted 25 October 2026

Ctrl+Alt+T brings the Terminal forward or opens it; no focused window sees
it, and the shell never answers it over a prompt (ada76f8639).

## How it ran

- Image: `make dev-image` at ada76f8639 (qemu-dev, throwaway test keys).
- Machine: QEMU TCG, no KVM, 4 CPUs, 8 GiB, fresh disk, 1920x1080 on
  virtio-gpu.
- Driven over QMP: Enter through the 13 setup steps, "Not now" on the live
  session, then Ctrl+Alt+T on the desktop to open the Terminal, and keys
  typed as a person would.
- Before: `terminal-00-baseline` (main at 2c7f9d08, the same machine).

## What it showed

- Ctrl+Alt+T on the bare desktop opened the Terminal: `[SPAWN-INSTANCE]
  queued app.terminal -> ok`, then `[SPAWN] name=app.terminal.1` in serial.
- `1-processes-focused.png`: Ctrl+Alt+Esc brought Processes over it.
- `2-after-ctrl-alt-t.png`: Ctrl+Alt+T with Processes focused: the Terminal in
  front, its line still `draft text`, so no `t` reached either window.

## Open, seen in this run

- The menu bar still names Processes after the Terminal came forward.
- The chord opens through the shell's open path, not the dock's, so no
  "opening a new window" toast; it moves to the dock's path next.
