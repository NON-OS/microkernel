# Terminal 09: market list, booted 25 October 2026

market list prints as pacman prints a search (7cd56de2d7).

## How it ran

- Image: `make dev-image` at cf833f5f46 (qemu-dev, throwaway test keys).
- Machine: QEMU TCG, no KVM, 4 CPUs, 8 GiB, fresh disk, 1920x1080.
- Driven over QMP: setup on its defaults, Ctrl+Alt+T, then typed keys:
  `neofetch`, `c` Tab, Ctrl+U, Ctrl+L, `market list`, `theme`, Down, Down, Esc.

## What it showed

- `market-list.png`: `linux/<name>` in magenta and bold, the names lined up, each
  standing `[ready]` in cyan, the hint in dim under the rows.
