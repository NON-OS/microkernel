# Code style: One concern per file

Part of [Code style](code-style.md).

## One concern per file

A file holds one concern and stays at or under 75 lines, and a `mod.rs` only
declares modules and re-exports names. `src/arch/aarch64/mod.rs` is a plain
example: module declarations, then `pub use` lines, nothing else. When a file
would grow past the limit, move a part into a child module; `table_geom.rs` does
that with `table_span` and says why
(`userland/capsule_process_manager/src/pm/ui/table_geom.rs:23-27`).

The static checks enforce this in the bootloader only:

- A `mod.rs` under `nonos-bootloader/src` may hold comments, blank lines,
  attributes, `mod` declarations and `pub use` re-exports, or `boot_mod_bodies`
  fails the check (`nonos-ci/run-static-checks.sh:2217-2237`). A plain `use` is
  not allowed there.
- Those `mod.rs` files stay at or under 75 lines (`boot_mod_oversize`,
  `nonos-ci/run-static-checks.sh:2240-2246`).
- Files under `nonos-bootloader/src/entry` stay at or under 75 lines and carry
  no comment after line 15, where the header ends (`boot_entry_oversize`,
  `boot_entry_comments`, `nonos-ci/run-static-checks.sh:2249-2264`).
- The bootloader's kernel verification module is held to the same two limits
  (`kernel_verify_oversize`, `nonos-ci/run-static-checks.sh:2267-2282`).

Everywhere else the limit is a house rule with no check behind it, and older
code does not meet it: at this commit 790 of the 5761 Rust files under `src/`
are longer than 75 lines.

## Comments

Say why, not what the next line does. A module opens with a `//!` comment that
says what the module is for and what it is not for; the one in
`src/arch/time_counter.rs:17-22` names what `x86_64` and `aarch64` read and why
callers must not use it as wall-clock time.

- An `unsafe fn` documents its contract in a `# Safety` section, as
  `enable_interrupts` does (`src/arch/abi.rs:42-46`).
- An `unsafe` block has a `// SAFETY:` comment above it naming the fact that
  makes it sound, as the call to `rdrand_u64` does
  (`src/arch/cpu_random/read.rs:33-36`).

No check requires a `SAFETY:` comment, so write one for every new `unsafe`
block. `tools/nonos_console.py` counts `unsafe {` sites and `SAFETY:` comments
across the tree and prints the comments, counted into `documented`, as a share
of the sites (`tools/nonos_console.py:1319-1324`).

Shipping comments carry no markers of unfinished work. The `hygiene` scan
rejects the four in `COMMENT_PATTERNS`, among them `FIXME`, `for now` and
`placeholder`, in any line that starts as a comment
(`nonos-verify/src/hygiene/patterns.rs:28-33`).
