# Terminal 10: the theme picker, booted 25 October 2026

theme alone opens a picker that tries each theme on the window (4cd4630670).

## How it ran

- Image: `make dev-image` at 51d911fa8b (qemu-dev, throwaway test keys).
- Machine: QEMU TCG, no KVM, 4 CPUs, 8 GiB, fresh disk, 1920x1080.
- Driven over QMP: setup on its defaults, Ctrl+Alt+T, then typed keys:
  `neofetch`, `c` Tab, Ctrl+U, Ctrl+L, `market list`, `theme`, Down, Down, Esc.

## What it showed

- `1-down-twice-light.png`: `theme`, Down, Down: the panel marks `light` and the
  whole window is light, background (250, 250, 250).
- `2-esc-back-to-dark.png`: Esc: dark again, background (12, 14, 16).
