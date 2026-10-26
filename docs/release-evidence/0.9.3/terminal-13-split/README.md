# Terminal 13: split view, booted 25 October 2026

Two tabs side by side in one window (938b2475a4).

## How it ran

- Image: `make dev-image` at 938b2475a4 (qemu-dev, throwaway test keys).
- Machine: QEMU TCG, no KVM, 4 CPUs, 8 GiB, fresh disk, 1920x1080.
- Driven over QMP at 120 ms a key: setup on its defaults, Ctrl+Alt+T, then
  `echo the quick brown fox jumps over the lazy dog`, `latency`,
  Ctrl+Shift+D, `neofetch`, Ctrl+Shift+Left, `ls`.

## What it showed

- `1-split-neofetch-right.png`: Ctrl+Shift+D: a second pane on the right with
  the keys and an accent line over it; `neofetch` there reports `terminal
  82x30`, its own half; the titlebar shows both tabs.
- `2-keys-moved-left.png`: Ctrl+Shift+Left: `ls` ran in the left pane.
