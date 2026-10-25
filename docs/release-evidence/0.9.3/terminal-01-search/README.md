# Terminal 01: history search, booted 25 October 2026

Esc leaves Ctrl+R with the line put back, the prompt names the search and
shows its needle whole, and Enter runs nothing on no match (4f3dd5ebe1).

## How it ran

- Image: `make dev-image` at ada76f8639 (qemu-dev, throwaway test keys).
- Machine: QEMU TCG, no KVM, 4 CPUs, 8 GiB, fresh disk, 1920x1080 on
  virtio-gpu.
- Driven over QMP: Enter through the 13 setup steps, "Not now" on the live
  session, then Ctrl+Alt+T on the desktop to open the Terminal, and keys
  typed as a person would.
- Before: `terminal-00-baseline` (main at 2c7f9d08, the same machine).

## What it showed

- `before-history-search.png`: `he` typed at `~` shown as `?h`.
- `before-esc-ran-help.png`: after Esc, `ls -l /` and Enter, `help` ran.
- `after-1-history-he.png`: `draft text` typed, Ctrl+R, `he`: the prompt reads
  `history he` and the line shows `help`; the bar names Enter, Ctrl+R, Esc.
- `after-2-no-match.png`: `zz` added: `no match hezz` in red.
- `after-3-enter-ran-nothing.png`: Enter: the screen is pixel for pixel the one
  before it; nothing ran.
- `after-4-esc-line-back.png`: Esc: the prompt is back with `draft text`.
