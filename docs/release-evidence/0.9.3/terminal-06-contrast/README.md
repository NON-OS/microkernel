# Terminal 06: WCAG AA on every theme, booted 25 October 2026

Every theme's text reaches 4.5:1 on the background and every bar and block
it is drawn on (286780597d), in an image built at dd24fca821; the proof over
the palette is `terminal_line_proofs/src/paint/contrast_tests.rs`.

## How it ran

As `terminal-05-prompt`, the same boot: `theme abyss`, `help`, `theme light`,
`theme dark`.

## What it showed

- `before-dark.png`: the baseline: the times and hints in dim text at 3.0:1.
- `dark.png`, `abyss.png`, `light.png`: the same screen on each theme; the times
  at the right of each block and the hint bar read clearly on all three.
