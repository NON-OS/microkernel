# Terminal 08: the 3D emblem, booted 25 October 2026

The logo's slashed zero in 3D: on the splash at screen resolution, in neofetch in half blocks (2035f28963, b66d452d66).

## How it ran

- Image: `make dev-image` at 51d911fa8b (qemu-dev, throwaway test keys).
- Machine: QEMU TCG, no KVM, 4 CPUs, 8 GiB, fresh disk, 1920x1080.
- Driven over QMP: setup on its defaults, Ctrl+Alt+T, then typed keys:
  `neofetch`, `c` Tab, Ctrl+U, Ctrl+L, `market list`, `theme`, Down, Down, Esc.

## What it showed

- `splash.png`: the fresh tab: the NONOS banner, the facts and beside them the
  slashed zero, its face lit from the upper left and its side in shadow.
- `neofetch.png`: the same zero in 24-bit half blocks beside the facts, all read
  from the system: memory 0.1 GiB / 7.0 GiB (2%), 58 processes, load
  2.96 2.11 0.99, terminal 170x30, theme dark.
