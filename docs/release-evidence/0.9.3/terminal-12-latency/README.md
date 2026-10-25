# Terminal 12: keystroke to screen, booted 25 October 2026

`latency` (f212cdd08d) times each key from its arrival in the window to the
frame it changed being painted.

## How it ran

- Image: `make dev-image` at 938b2475a4 (qemu-dev, throwaway test keys).
- Machine: QEMU TCG, no KVM, 4 CPUs, 8 GiB, fresh disk, 1920x1080.
- Driven over QMP at 120 ms a key: setup on its defaults, Ctrl+Alt+T, then
  `echo the quick brown fox jumps over the lazy dog`, `latency`,
  Ctrl+Shift+D, `neofetch`, Ctrl+Shift+Left, `ls`.

## What it showed

- `latency-tcg.png`: `keys to screen, last 56: middle 68 ms, 95% 86 ms,
  worst 106 ms` under TCG, every key a whole-window repaint.
- The same keystroke and whole repaint natively on the host (release build,
  demo_screen_proofs, frame and fit included): middle 8.0 ms, 95% 8.3 ms. TCG
  ran it 8.5 to 10 times slower, so on KVM-class hardware the 95% is about
  9 to 10 ms against a target of 16 ms.
- Since this image, 7cc9dc38dd repaints only the input row for a key that
  changes only the line: on the host 17 us middle, 25 us at 95%, against
  542 and 630 us for the content alone; its TCG numbers come with its boot.

## Open, seen in this run

- `echo` and eight words printed seven: the parser kept 8 words. Fixed in
  6fd1b2a0d9 (64 words, and a longer line refused).
