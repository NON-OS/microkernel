# Terminal 02: the hint bar, booted 25 October 2026

The bar names the keys of the state the reader is in, and help keys says
what Ctrl+K does (e8390dee7a).

## How it ran

- Image: `make dev-image` at eb520916d8 (qemu-dev, throwaway test keys).
- Machine: QEMU TCG, no KVM, 4 CPUs, 8 GiB, fresh disk, 1920x1080 on
  virtio-gpu.
- Driven over QMP: Enter through the 13 setup steps, "Not now" on the live
  session, then Ctrl+Alt+T on the desktop to open the Terminal, and keys
  typed as a person would.
- Before: `terminal-00-baseline` (main at 2c7f9d08, the same machine).

## What it showed

- `before-ctrl-k-kill.png`: the one fixed row, `^K kill`, while Ctrl+K opened
  the palette.
- `after-prompt-hints.png`: at the prompt: Tab complete, Ctrl+R history,
  Ctrl+K commands, Ctrl+Shift+F find, Ctrl+Shift+T new tab, Ctrl+L clear,
  help keys.
- `after-search-hints.png`: in a search: Enter run, Ctrl+R older, Esc back to
  your line, Bksp edit.
