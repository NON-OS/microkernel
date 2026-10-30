# Terminal 11: the first screen, booted 25 October 2026

The splash names where to start (2cd5a2aad5).

## How it ran

- Image: `make dev-image` at 2cd5a2aad5 (qemu-dev, throwaway test keys).
- Machine: QEMU TCG, no KVM, 4 CPUs, 8 GiB, fresh disk, 1920x1080.
- Driven over QMP: setup on its defaults, Ctrl+Alt+T, then typed keys:
  `neofetch`, `c` Tab, Ctrl+U, Ctrl+L, `market list`, `theme`, Down, Down, Esc.

## What it showed

- `before-first-open.png`: `shell nox (type 'help')`, no key named.
- `splash-start-row.png`: `start help, Tab, Ctrl+R history, Ctrl+K`, `shell nox`.
