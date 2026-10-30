# Terminal 07: Tab's list, booted 25 October 2026

Candidates are a list beside the prompt that the next key clears (c6a08fbe85).

## How it ran

- Image: `make dev-image` at 5a0080a4d2 (qemu-dev, throwaway test keys).
- Machine: QEMU TCG, no KVM, 4 CPUs, 8 GiB, fresh disk, 1920x1080.
- Driven over QMP: setup on its defaults, Ctrl+Alt+T, then typed keys:
  `neofetch`, `c` Tab, Ctrl+U, Ctrl+L, `market list`, `theme`, Down, Down, Esc.

## What it showed

- `before-into-scrollback.png`: candidates printed twice into the last block.
- `after-c-tab.png`: `c` and Tab: ten names in aligned columns under the prompt,
  nothing written into the `neofetch` block above.
