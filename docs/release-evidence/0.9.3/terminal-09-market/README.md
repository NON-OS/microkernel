# Terminal 09: market list, booted 25 October 2026

market list prints as pacman prints a search (f70c90de15).

## How it ran

- Image: `make dev-image` at 51d911fa8b (qemu-dev, throwaway test keys).
- Machine: QEMU TCG, no KVM, 4 CPUs, 8 GiB, fresh disk, 1920x1080.
- Driven over QMP: setup on its defaults, Ctrl+Alt+T, then typed keys:
  `neofetch`, `c` Tab, Ctrl+U, Ctrl+L, `market list`, `theme`, Down, Down, Esc.

## What it showed

- `market-list.png`: `linux/<name>` in magenta and bold, the names lined up, each
  standing `[ready]` in cyan, the hint in dim under the rows.
