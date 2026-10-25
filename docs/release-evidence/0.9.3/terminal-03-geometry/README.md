# Terminal 03: where it opens and where the prompt sits, booted 25 October 2026

Opens at 1240x720 centred on a 1080p screen, and the prompt sits under the
last output until the body is full (982944ea06).

## How it ran

- Image: `make dev-image` at ada76f8639 (qemu-dev, throwaway test keys).
- Machine: QEMU TCG, no KVM, 4 CPUs, 8 GiB, fresh disk, 1920x1080 on
  virtio-gpu.
- Driven over QMP: Enter through the 13 setup steps, "Not now" on the live
  session, then Ctrl+Alt+T on the desktop to open the Terminal, and keys
  typed as a person would.
- Before: `terminal-00-baseline` (main at 2c7f9d08, the same machine).

## What it showed

- `before-first-open.png`: 960x540, a quarter of the screen, 20 rows.
- `before-dead-band.png`: maximised, output at the top, the prompt at the foot.
- `after-open-1080p.png`: the whole screen: the window centred between the
  menu bar and the dock, about two thirds of the width.
- `after-prompt-follows.png`: after `help` and `ls -l`, the prompt on the
  line under the output.
