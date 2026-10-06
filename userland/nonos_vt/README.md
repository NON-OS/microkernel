# nonos_vt

`nonos_vt` is the terminal emulator the NONOS terminal draws each tab's
scrollback with: an xterm-compatible screen of cells, a parser for what a
program writes, and an encoder for the keys and pointer events a person sends.
It is a `no_std` library with no capsule, service or capability word of its
own. `capsule_terminal` uses it to show the output of the programs it starts
(tool capsules, the Linux personality, `qwen`), and `terminal_line_proofs`
compiles it on the host. The terminal is described in
[docs/handbook/apps/terminal.md](../../docs/handbook/apps/terminal.md).

## What is in it

- `parser`, `params`, `utf8`: the DEC state machine, so every byte has one
  meaning in every state and no output can leave the parser stuck.
- `term`, `cell`, `line`, `color`, `charset`, `width`: the screen, its cells,
  pens, colours, character sets and character widths, with scrollback,
  resizing by re-wrapping lines, modes, cursor shape, the title stack,
  clipboard requests and OSC 8 links (`url`).
- `input`: keys and pointer events encoded as the bytes the program reads,
  following the modes the program set.
- `limits`: a fixed ceiling on everything hostile output could grow. The
  screen is held between 2 by 1 and 1000 by 500 cells, scrollback defaults to
  5000 lines and stops at 100,000, an OSC payload past 4096 bytes is dropped
  whole, and replies to a program that does not read them stop at 4096 bytes.

## Tests

69 `#[test]` functions under `tests/`: the parser and its string sequences,
the screen (characters, colour, cursor, scroll regions), reflow on resize,
reports, input encoding, refusals past the limits, and random noise fed to
the parser.

```sh
cd userland/nonos_vt
cargo test --release
```

No CI job names this crate directly.
