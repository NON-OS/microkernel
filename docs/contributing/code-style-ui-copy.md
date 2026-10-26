# Code style: UI copy

What a person reads on a NØNOS screen follows the Voice rules in
strings to the parts of them a machine can check. It runs in the `static-abi`
flake check, with its self-test.

## What it reads

The Rust string and byte-string literals in the source of each windowed
capsule (every crate that links `app_skeleton`), the desktop shell, setup,
login and the toolkit. It reads only strings that are prose: ones with a space
or a capital letter at the start. The Terminal is not read. A serial line, one
starting with `[`, is a log and not copy.

## What it refuses

- **Hype words:** seamless, unlock, empower, elevate, dive in, supercharge,
  effortless, magic, revolutionary, next-level, in any inflection. Say what
  happens instead: `open it with its passphrase`, not `unlock it`.
- **Emoji and pictographs**, the sparkle U+2728 among them.
- **An exclamation mark** ending a word. `n!`, a single lower-case token, is
  not copy.
- **Title Case:** three or more words each capitalised. A proper name in the
  script's `PROPER` list, such as a font's, is exempt.

Each refusal names the file, the line and the string. Fix the string. The
rules have no list of exceptions.

## Try it

    python3 nonos-ci/check-ui-copy.py --self-test
    python3 nonos-ci/check-ui-copy.py

Checked against the tree at the commit that added this page.
